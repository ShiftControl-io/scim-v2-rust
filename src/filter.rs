//! SCIM filter expressions (RFC 7644 §3.4.2.2) and PATCH paths (RFC 7644 §3.5.2).
//!
//! # Overview
//!
//! This module provides the AST types for SCIM filter expressions and PATCH operation
//! paths. The two main entry points are [`Filter`] and [`PatchPath`].
//!
//! # SCIM server: evaluating an incoming filter
//!
//! Filters arrive already deserialized inside other SCIM message types:
//!
//! - `SearchRequest::filter` — from a
//!   `POST /.search` body.
//! - `ListQuery::filter` — from a `GET`
//!   query string (`?filter=...`), parsed by your web framework into this struct.
//! - `PatchOperation` — each operation's
//!   `path` field deserializes as a [`PatchPath`].
//!
//! By default these types fail deserialization when the filter expression is
//! malformed, which takes `start_index`, `count`, and other fields down with
//! it. To produce an RFC 7644 §3.12 `invalidFilter` response instead, use the
//! `TolerantListQuery` /
//! `TolerantSearchRequest`
//! aliases (equivalently `ListQuery<MaybeFilter>` / `SearchRequest<MaybeFilter>`)
//! and match on [`MaybeFilter::Valid`] vs [`MaybeFilter::Invalid`] to build
//! the error body from the captured `raw` string and [`ParseError`].
//!
//! Once you have a [`Filter`], recursively `match` on its variants to evaluate it:
//!
//! ```
//! # use scim_v2::filter::*;
//! fn matches_user(filter: &Filter, user_name: &str) -> bool {
//!     match filter {
//!         Filter::Attr(AttrExp::Comparison(path, CompareOp::Eq, CompValue::Str(v)))
//!             if path.name == "userName" =>
//!         {
//!             user_name == v
//!         }
//!         Filter::And(items) => items.iter().all(|f| matches_user(f, user_name)),
//!         Filter::Or(items) => items.iter().any(|f| matches_user(f, user_name)),
//!         Filter::Not(inner) => !matches_user(inner, user_name),
//!         _ => false,
//!     }
//! }
//!
//! let filter: Filter = r#"userName eq "bjensen""#.parse().unwrap();
//! assert!(matches_user(&filter, "bjensen"));
//! assert!(!matches_user(&filter, "jsmith"));
//! ```
//!
//! # Depth and size limits
//!
//! [`Filter::from_str`](std::str::FromStr::from_str) and [`PatchPath::from_str`](std::str::FromStr::from_str) (and the corresponding
//! [`Deserialize`] impls, which delegate to
//! [`FromStr`](std::str::FromStr)) reject any input whose parsed AST would exceed
//! [`MAX_FILTER_DEPTH`] levels of nesting or [`MAX_FILTER_TERMS`] attribute
//! expressions. The depth bound is what keeps the crate's own recursive
//! [`Display`], derived [`PartialEq`] / [`Debug`], serialization, and `Drop`
//! impls off the end of the stack, so a hostile `?filter=` value with
//! thousands of nested `not (…)` cannot crash the server after the filter has
//! been accepted. The term bound is memory hygiene for the other axis. Both
//! are enforced while the parser runs, so an input that crosses either is
//! rejected at that point with the rest unread, and peak allocation for a
//! hostile input is bounded by the limits rather than by its length.
//!
//! The two are independent because `and`/`or` are n-ary: `a or b or c …` is
//! one [`Filter::Or`] however many operands it has, so a hundred-id batch
//! lookup has depth 2 and parses, while the same hundred terms would have
//! been depth 100 in a binary tree. Depth counts only genuine nesting —
//! `not`, grouping that changes the operator, a value path's inner filter —
//! and one filter has one budget: a value path's inner filter continues the
//! count from where its enclosing filter left off rather than starting over.
//! Hand-constructed [`Filter`] values bypass both checks and are the caller's
//! responsibility; build them with [`Filter::and`] / [`Filter::or`] to keep
//! the shape the parser would produce.
//!
//! # SCIM client: building a filter to send in a request
//!
//! Construct the AST directly and assign it to the `filter` field of
//! `SearchRequest` or
//! `ListQuery`. Those types implement
//! `serde::Serialize`, so the filter is serialized automatically as a JSON
//! string when you serialize the containing struct. You can also call
//! `.to_string()` directly when you need the raw filter string for a query
//! parameter:
//!
//! ```
//! # use scim_v2::filter::*;
//! let filter = Filter::and(
//!     Filter::Attr(AttrExp::Present(AttrPath::with_name("title"))),
//!     Filter::Attr(AttrExp::Comparison(
//!         AttrPath::with_name("userType"),
//!         CompareOp::Eq,
//!         "Employee".into(),
//!     )),
//! );
//! assert_eq!(filter.to_string(), r#"title pr and userType eq "Employee""#);
//!
//! // `and`/`or` are n-ary: chaining flattens rather than nesting.
//! let three = Filter::and(filter, Filter::Attr(AttrExp::Present(AttrPath::with_name("active"))));
//! assert!(matches!(&three, Filter::And(items) if items.len() == 3));
//! assert_eq!(three.to_string(), r#"title pr and userType eq "Employee" and active pr"#);
//! ```

use fluent_uri::Uri;
use lalrpop_util::ParseError as LalrParseError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::Cell;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::OnceLock;
use thiserror::Error;

/// The generated parsers, built once. Constructing one compiles the lexer's
/// regex set (about 400 KiB and measurable time), which used to happen on
/// every `from_str`; the parser itself is stateless, so sharing is free.
fn filter_parser() -> &'static crate::filter_parser::FilterParser {
    static PARSER: OnceLock<crate::filter_parser::FilterParser> = OnceLock::new();
    PARSER.get_or_init(crate::filter_parser::FilterParser::new)
}

fn path_parser() -> &'static crate::filter_parser::PathParser {
    static PARSER: OnceLock<crate::filter_parser::PathParser> = OnceLock::new();
    PARSER.get_or_init(crate::filter_parser::PathParser::new)
}

/// Maximum allowed nesting depth for a parsed [`Filter`] or [`ValFilter`] tree.
///
/// [`Filter::from_str`](std::str::FromStr::from_str) and [`PatchPath::from_str`](std::str::FromStr::from_str) reject any input whose parsed
/// AST would exceed this depth, returning
/// [`ParseError::User`] wrapping [`FilterActionError::DepthExceeded`]. The same
/// enforcement runs on the [`Deserialize`] paths (SCIM `SearchRequest.filter`,
/// `ListQuery.filter`, `PatchOperation.path`), so a remote attacker cannot submit
/// a pathologically nested filter like `(((…(title pr)…)))` or
/// `a pr and a pr and …` that would later overflow the call stack when the
/// library's recursive [`Display`], derived [`PartialEq`] / [`Debug`],
/// [`Serialize`], or [`Drop`] impls walk the tree.
///
/// Hand-constructed [`Filter`] values that bypass the parser do not pass through
/// this check; it is the caller's responsibility to keep programmatically built
/// filters within this limit.
pub const MAX_FILTER_DEPTH: usize = 64;

/// Maximum number of attribute expressions (terms) in one filter or PATCH
/// path, across every `and`/`or` chain and value path.
///
/// Distinct from [`MAX_FILTER_DEPTH`] on purpose. Since the AST is n-ary, a
/// flat `a or b or … ` chain has depth 2 however long it is, so depth no
/// longer bounds the *size* of an accepted filter; this does. It is generous
/// — a client resolving a batch of a hundred ids in one `or` chain is an
/// ordinary request and parses — while still keeping a hostile `?filter=`
/// from allocating without limit. RFC 7644 §3.4.2.2 sets no term limit and
/// `ServiceProviderConfig.filter.maxResults` bounds results, not terms, so
/// this is the crate's own hygiene bound.
///
/// Exceeding it is [`FilterActionError::TooManyTerms`], which a server should
/// answer with RFC 7644 §3.12 `tooMany` or `invalidFilter`.
///
/// Both this and [`MAX_FILTER_DEPTH`] are enforced *during* parsing, not
/// after: the parser charges each attribute expression against a budget as it
/// is reduced and tracks open brackets as it shifts them, so an input that
/// crosses a limit is rejected at that point and the rest of it is never read,
/// let alone allocated. Peak memory for a hostile filter is therefore bounded
/// by the limits, not by the input's length.
pub const MAX_FILTER_TERMS: usize = 1024;

/// Parse-time resource budget, threaded through the generated parser.
///
/// Charges each attribute expression against [`MAX_FILTER_TERMS`] and each
/// open `(` / `[` against [`MAX_FILTER_DEPTH`], failing the parse the moment
/// either is exceeded. This is what makes the limits a bound on allocation
/// rather than a check on an already-built tree.
#[derive(Debug, Default)]
pub(crate) struct ParseBudget {
    terms: Cell<usize>,
    depth: Cell<usize>,
}

impl ParseBudget {
    /// Charge one attribute expression.
    pub(crate) fn term(&self) -> Result<(), FilterActionError> {
        let n = self.terms.get() + 1;
        if n > MAX_FILTER_TERMS {
            return Err(FilterActionError::TooManyTerms(n));
        }
        self.terms.set(n);
        Ok(())
    }

    /// Enter one level of syntactic nesting (`(`, `not (`, or `[`).
    pub(crate) fn enter(&self) -> Result<(), FilterActionError> {
        let d = self.depth.get() + 1;
        if d > MAX_FILTER_DEPTH {
            return Err(FilterActionError::DepthExceeded(d));
        }
        self.depth.set(d);
        Ok(())
    }

    /// Leave one level of syntactic nesting.
    pub(crate) fn leave(&self) {
        self.depth.set(self.depth.get().saturating_sub(1));
    }
}

/// Error produced by fallible grammar actions (`=>?` rules in the LALRPOP grammar)
/// and by post-parse validation.
/// `#[non_exhaustive]`: further grammar and depth diagnostics will be added
/// in minor releases.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum FilterActionError {
    /// An `attrPath` token contained more than one sub-attribute segment.
    #[error("attrPath '{0}' has more than one sub-attribute segment")]
    InvalidAttrPath(String),
    /// A comparison value contained an invalid JSON escape or format.
    #[error("invalid comparison value: {0}")]
    InvalidCompValue(#[from] serde_json::Error),
    /// The filter's nesting depth exceeded [`MAX_FILTER_DEPTH`].
    ///
    /// The wrapped value is the depth at which the limit was first breached.
    /// Nesting is measured two ways, both against the same limit: the number
    /// of brackets open at once (`(`, `not (`, `[`) while parsing, which
    /// rejects an over-deep input before its interior is read; and the depth
    /// of the finished AST, where `not (…)`, a value path's inner filter, or
    /// an operator change such as an `or` inside an `and` each add a level.
    /// Neither counts the length of a same-operator chain, which is bounded
    /// by [`TooManyTerms`](FilterActionError::TooManyTerms) instead.
    #[error("filter nesting depth exceeds maximum of {MAX_FILTER_DEPTH} (at depth {0})")]
    DepthExceeded(usize),
    /// The filter contains more attribute expressions than [`MAX_FILTER_TERMS`].
    ///
    /// The wrapped value is the count that breached the limit.
    #[error("filter has too many terms: {0} exceeds the maximum of {MAX_FILTER_TERMS}")]
    TooManyTerms(usize),
}

/// Error returned by [`Filter::from_str`](std::str::FromStr::from_str) and [`PatchPath::from_str`](std::str::FromStr::from_str) when the
/// input is not a valid filter or path expression.
pub type ParseError = LalrParseError<usize, String, FilterActionError>;

/// Payload of [`MaybeFilter::Invalid`] and error type of the
/// `TryFrom<Tolerant*>` / `into_strict` conversions on
/// `TolerantListQuery` and
/// `TolerantSearchRequest`.
///
/// Captures the original filter string and underlying [`ParseError`] so
/// callers can build an RFC 7644 §3.12 `invalidFilter` response body
/// (typically via `.map_err(...)` into the caller's SCIM error type).
#[derive(Debug, Error)]
#[error("invalid SCIM filter {raw:?}: {error}")]
pub struct InvalidFilterError {
    /// The original filter string as received from the wire.
    pub raw: String,
    /// The underlying parse failure.
    pub error: ParseError,
}

/// A SCIM filter expression (RFC 7644 §3.4.2.2).
///
/// Filters are used in SCIM `GET` requests (`?filter=...`) and in conditional
/// operations. The expression forms a tree of logical and comparison nodes.
///
/// ## Operator precedence
///
/// From highest to lowest: `not` > `and` > `or`. [`Display`] parenthesises
/// sub-expressions automatically so the output round-trips correctly.
///
/// ## Construction
///
/// Build filters from the enum variants and the supporting types:
///
/// ```
/// # use scim_v2::filter::*;
/// let work_emails = Filter::ValuePath(ValuePath {
///     attr: AttrPath::with_name("emails"),
///     filter: Box::new(ValFilter::Attr(AttrExp::Comparison(
///         AttrPath::with_name("type"),
///         CompareOp::Eq,
///         "work".into(),
///     ))),
/// });
/// assert_eq!(work_emails.to_string(), r#"emails[type eq "work"]"#);
/// ```
///
/// ## Matching
///
/// Use a recursive `match` to walk the tree when evaluating a filter against a
/// resource. See the [module-level examples](self) for a complete pattern.
///
/// ## Serde
///
/// [`Filter`] serializes as a JSON string (the filter expression) and
/// deserializes from a JSON string via [`FromStr`](std::str::FromStr).
#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    /// A single attribute expression: a presence test or a comparison.
    ///
    /// Examples: `title pr`, `userName eq "bjensen"`.
    Attr(AttrExp),

    /// An attribute path with a bracketed sub-filter applied to its values.
    ///
    /// Example: `emails[type eq "work" and value co "@example.com"]`.
    ///
    /// This form filters the elements of a multi-valued attribute; only elements
    /// matching the inner [`ValFilter`] are considered.
    ValuePath(ValuePath),

    /// Logical negation of the inner filter.
    ///
    /// Serializes as `not (<inner>)`. The inner filter is always parenthesised.
    Not(Box<Filter>),

    /// Logical conjunction — both operands must match.
    ///
    /// Binds more tightly than [`Or`](Filter::Or).
    ///
    /// N-ary: `a and b and c` is one `And` with three operands, not two nested
    /// pairs. The grammar flattens same-operator chains on the way in, so the
    /// tree's depth reflects genuine nesting (`not`, grouping, a value path, an
    /// `or` inside an `and`) and never the length of a chain. That is what lets
    /// a 100-term batch lookup parse while [`MAX_FILTER_DEPTH`] still bounds the
    /// recursion of the derived impls. [`Operands`] guarantees at least two
    /// operands however the value was built.
    And(Operands<Filter>),

    /// Logical disjunction — at least one operand must match.
    ///
    /// Lowest precedence operator. When an `Or` expression appears as a
    /// direct child of an `And`, [`Display`] wraps it in parentheses to
    /// preserve the original precedence on round-trip.
    ///
    /// N-ary, as [`And`](Filter::And).
    Or(Operands<Filter>),
}

/// A logical operator was built with fewer than two operands.
///
/// Returned by [`Operands::new`]; the wrapped value is the count supplied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("a logical operator needs at least two operands, got {0}")]
pub struct TooFewOperands(pub usize);

/// The operands of an n-ary [`Filter::And`] / [`Filter::Or`] (and the
/// [`ValFilter`] pair): at least two, by construction.
///
/// The SCIM grammar has no way to write a conjunction of one thing or of
/// nothing, so a tree holding one would not survive `Display` and re-parse.
/// Rather than check that at every use, the type rules it out: the only ways
/// to obtain an `Operands` are [`Operands::new`] (which refuses fewer than
/// two), [`Operands::pair`], the flattening constructors [`Filter::and`] /
/// [`Filter::or`], and the parser. Dereferences to a slice, so `.len()`,
/// `.iter()`, indexing and slice patterns all work; [`push`](Operands::push)
/// is the only way to grow it and there is no way to shrink it.
#[derive(Debug, Clone, PartialEq)]
pub struct Operands<T>(Vec<T>);

impl<T> Operands<T> {
    /// Wrap `items`, which must hold at least two operands.
    pub fn new(items: Vec<T>) -> Result<Self, TooFewOperands> {
        if items.len() < 2 {
            return Err(TooFewOperands(items.len()));
        }
        Ok(Self(items))
    }

    /// Exactly two operands.
    pub fn pair(first: T, second: T) -> Self {
        Self(vec![first, second])
    }

    /// Append an operand.
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    /// The operands as a slice.
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Take the operands back as a `Vec`.
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    fn extend_from(&mut self, other: Operands<T>) {
        self.0.extend(other.0);
    }
}

impl<T> Deref for Operands<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.0
    }
}

/// Elements may be replaced in place; the length cannot change through a
/// slice, so the two-operand floor holds.
impl<T> DerefMut for Operands<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<T> IntoIterator for Operands<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Operands<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Filter {
    /// `lhs and rhs`, flattening: an `And` operand contributes its operands
    /// rather than nesting, since `and` is associative. This is the
    /// constructor the parser uses, and the one to use when building filters
    /// programmatically so the tree keeps the shape the parser would produce.
    pub fn and(lhs: Filter, rhs: Filter) -> Filter {
        let mut items = match lhs {
            Filter::And(items) => items,
            other => Operands(vec![other]),
        };
        match rhs {
            Filter::And(more) => items.extend_from(more),
            other => items.push(other),
        }
        Filter::And(items)
    }

    /// `lhs or rhs`, flattening as [`Filter::and`] does.
    pub fn or(lhs: Filter, rhs: Filter) -> Filter {
        let mut items = match lhs {
            Filter::Or(items) => items,
            other => Operands(vec![other]),
        };
        match rhs {
            Filter::Or(more) => items.extend_from(more),
            other => items.push(other),
        }
        Filter::Or(items)
    }

    /// The conjunction of `items`: `None` for none, the item itself for one,
    /// and a flattened [`And`](Filter::And) for more.
    ///
    /// The constructor for folding a computed list of conditions — an
    /// allow-list, say — where an empty list must reach the caller's control
    /// flow rather than become an empty filter that a server could read as
    /// "match everything".
    pub fn all(items: impl IntoIterator<Item = Filter>) -> Option<Filter> {
        items.into_iter().reduce(Filter::and)
    }

    /// The disjunction of `items`, as [`Filter::all`].
    pub fn any(items: impl IntoIterator<Item = Filter>) -> Option<Filter> {
        items.into_iter().reduce(Filter::or)
    }
}

impl ValFilter {
    /// `lhs and rhs`, flattening as [`Filter::and`].
    pub fn and(lhs: ValFilter, rhs: ValFilter) -> ValFilter {
        let mut items = match lhs {
            ValFilter::And(items) => items,
            other => Operands(vec![other]),
        };
        match rhs {
            ValFilter::And(more) => items.extend_from(more),
            other => items.push(other),
        }
        ValFilter::And(items)
    }

    /// `lhs or rhs`, flattening as [`Filter::or`].
    pub fn or(lhs: ValFilter, rhs: ValFilter) -> ValFilter {
        let mut items = match lhs {
            ValFilter::Or(items) => items,
            other => Operands(vec![other]),
        };
        match rhs {
            ValFilter::Or(more) => items.extend_from(more),
            other => items.push(other),
        }
        ValFilter::Or(items)
    }

    /// The conjunction of `items`, as [`Filter::all`].
    pub fn all(items: impl IntoIterator<Item = ValFilter>) -> Option<ValFilter> {
        items.into_iter().reduce(ValFilter::and)
    }

    /// The disjunction of `items`, as [`Filter::any`].
    pub fn any(items: impl IntoIterator<Item = ValFilter>) -> Option<ValFilter> {
        items.into_iter().reduce(ValFilter::or)
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::Attr(e) => write!(f, "{e}"),
            Filter::ValuePath(vp) => write!(f, "{}[{}]", vp.attr, vp.filter),
            Filter::Not(inner) => write!(f, "not ({inner})"),
            Filter::And(items) => {
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " and ")?;
                    }
                    fmt_and_operand_filter(item, f)?;
                }
                Ok(())
            }
            Filter::Or(items) => {
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " or ")?;
                    }
                    write!(f, "{item}")?;
                }
                Ok(())
            }
        }
    }
}

fn fmt_and_operand_filter(operand: &Filter, f: &mut Formatter<'_>) -> std::fmt::Result {
    if matches!(operand, Filter::Or(_)) {
        write!(f, "({operand})")
    } else {
        write!(f, "{operand}")
    }
}

impl Serialize for Filter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Filter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// Deserialize-side wrapper that rescues invalid filter expressions.
///
/// SCIM handlers that receive a malformed `?filter=...` need to return a
/// `400 invalidFilter` response per RFC 7644 §3.12. Using this wrapper in
/// place of [`Filter`] lets the surrounding
/// `ListQuery` /
/// `SearchRequest` deserialize
/// successfully so the handler can inspect `start_index`, `count`, etc. and
/// produce an RFC-compliant error body instead of a generic 400.
///
/// `MaybeFilter` intentionally does **not** implement `Serialize`. To emit a
/// filter on the wire, pattern-match the [`MaybeFilter::Valid`] variant and
/// serialize the inner [`Filter`] — `Filter` itself is `Serialize`.
#[derive(Debug)]
pub enum MaybeFilter {
    /// The filter string parsed successfully.
    Valid(Filter),
    /// The filter string did not parse. The wrapped [`InvalidFilterError`]
    /// carries the original input and underlying [`ParseError`] for
    /// constructing an RFC 7644 §3.12 `invalidFilter` error response.
    Invalid(InvalidFilterError),
}

impl<'de> Deserialize<'de> for MaybeFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.parse::<Filter>() {
            Ok(f) => Ok(MaybeFilter::Valid(f)),
            Err(e) => Ok(MaybeFilter::Invalid(InvalidFilterError {
                raw: s,
                error: e,
            })),
        }
    }
}

impl std::str::FromStr for Filter {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The budget bounds terms and syntactic nesting while parsing; the AST
        // depth walk below is the exact structural check on what came out.
        let budget = ParseBudget::default();
        let parsed = filter_parser()
            .parse(&budget, s.trim())
            .map_err(|e| e.map_token(|t| t.to_string()))?;
        if let Some(depth) = filter_depth_exceeds(&parsed, MAX_FILTER_DEPTH) {
            drop_filter_iteratively(parsed);
            return Err(LalrParseError::User {
                error: FilterActionError::DepthExceeded(depth),
            });
        }
        Ok(parsed)
    }
}

/// The `attrPath "[" valFilter "]"` form of a filter (RFC 7644 §3.4.2.2).
///
/// Selects elements of a multi-valued attribute that satisfy the inner filter.
///
/// ```
/// # use scim_v2::filter::*;
/// let vp = ValuePath {
///     attr: AttrPath::with_name("emails"),
///     filter: Box::new(ValFilter::Attr(AttrExp::Comparison(
///         AttrPath::with_name("type"),
///         CompareOp::Eq,
///         "work".into(),
///     ))),
/// };
/// assert_eq!(Filter::ValuePath(vp).to_string(), r#"emails[type eq "work"]"#);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ValuePath {
    /// The multi-valued attribute being filtered.
    pub attr: AttrPath,
    /// The filter applied to each element of the attribute's values.
    pub filter: Box<ValFilter>,
}

/// A filter expression used inside `[...]` brackets (RFC 7644 §3.4.2.2).
///
/// Structurally identical to [`Filter`] but does not allow nested
/// [`ValuePath`] expressions. The RFC specifies that the expression inside
/// square brackets must be a valid filter expression based on sub-attributes
/// of the parent attribute.
///
/// Used as the inner filter of [`ValuePath`] and [`PatchValuePath`].
#[derive(Debug, Clone, PartialEq)]
pub enum ValFilter {
    /// A single attribute expression (presence test or comparison).
    Attr(AttrExp),
    /// Logical negation. Serializes as `not (<inner>)`.
    Not(Box<ValFilter>),
    /// Logical conjunction — both operands must match.
    /// N-ary, as [`Filter::And`].
    And(Operands<ValFilter>),
    /// Logical disjunction — at least one operand must match.
    /// N-ary, as [`Filter::Or`].
    Or(Operands<ValFilter>),
}

/// An atomic filter expression: a presence test or an attribute comparison.
///
/// ```
/// # use scim_v2::filter::*;
/// // "title pr" — attribute is present and non-null
/// let present = AttrExp::Present(AttrPath::with_name("title"));
/// assert_eq!(Filter::Attr(present).to_string(), "title pr");
///
/// // "userName eq \"bjensen\""
/// let cmp = AttrExp::Comparison(
///     AttrPath::with_name("userName"),
///     CompareOp::Eq,
///     "bjensen".into(),
/// );
/// assert_eq!(Filter::Attr(cmp).to_string(), r#"userName eq "bjensen""#);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum AttrExp {
    /// The `pr` (present) operator. Matches when the attribute exists and is not null.
    ///
    /// Example: `title pr`
    Present(AttrPath),

    /// A comparison operator with a value.
    ///
    /// Example: `userName eq "bjensen"`, `meta.lastModified gt "2024-01-01T00:00:00Z"`
    Comparison(AttrPath, CompareOp, CompValue),
}

/// A SCIM attribute path, optionally qualified with a schema URI and/or a sub-attribute.
///
/// An attribute path can take three forms:
///
/// | Form | Example string | Constructor |
/// |------|---------------|-------------|
/// | Simple name | `userName` | [`AttrPath::with_name("userName")`](AttrPath::with_name) |
/// | Sub-attribute | `name.familyName` | [`AttrPath::with_sub_attr("name", "familyName")`](AttrPath::with_sub_attr) |
/// | URI-prefixed | `urn:ietf:params:scim:schemas:core:2.0:User:userName` | struct literal with `uri: Some(...)` |
///
/// ```
/// # use scim_v2::filter::*;
/// assert_eq!(AttrPath::with_name("userName").to_string(), "userName");
/// assert_eq!(AttrPath::with_sub_attr("name", "familyName").to_string(), "name.familyName");
///
/// // URI-prefixed form requires the struct literal
/// let path = AttrPath {
///     uri: Some("urn:ietf:params:scim:schemas:core:2.0:User".into()),
///     name: "userName".into(),
///     sub_attr: None,
/// };
/// assert_eq!(path.to_string(), "urn:ietf:params:scim:schemas:core:2.0:User:userName");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AttrPath {
    /// Optional schema URI prefix, e.g. `"urn:ietf:params:scim:schemas:core:2.0:User"`.
    pub uri: Option<String>,
    /// Attribute name, e.g. `"userName"`.
    pub name: String,
    /// Sub-attribute, e.g. `"familyName"` from `"name.familyName"`.
    pub sub_attr: Option<String>,
}

impl AttrPath {
    /// Create an `AttrPath` for a simple attribute name (no URI prefix, no sub-attribute).
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            uri: None,
            name: name.into(),
            sub_attr: None,
        }
    }

    /// Create an `AttrPath` for a dotted sub-attribute path (e.g. `name.familyName`).
    pub fn with_sub_attr(name: impl Into<String>, sub_attr: impl Into<String>) -> Self {
        Self {
            uri: None,
            name: name.into(),
            sub_attr: Some(sub_attr.into()),
        }
    }
}

/// SCIM comparison operators (RFC 7644 §3.4.2.2, Table 3).
///
/// Operators are case-insensitive when parsed (`EQ`, `Eq`, and `eq` are all
/// accepted) but are always displayed in lowercase.
#[derive(Debug, Clone, PartialEq)]
pub enum CompareOp {
    /// `eq` — equal.
    Eq,
    /// `ne` — not equal.
    Ne,
    /// `co` — contains (substring match).
    Co,
    /// `sw` — starts with.
    Sw,
    /// `ew` — ends with.
    Ew,
    /// `gt` — greater than.
    Gt,
    /// `lt` — less than.
    Lt,
    /// `ge` — greater than or equal to.
    Ge,
    /// `le` — less than or equal to.
    Le,
}

/// The comparison value (right-hand side of a filter comparison).
///
/// Corresponds to the JSON scalar types allowed in SCIM filter expressions.
/// [`Display`] produces the JSON representation (e.g. `Str("foo")` → `"foo"`
/// with surrounding quotes and proper escaping).
///
/// String values can be constructed with `.into()`:
///
/// ```
/// # use scim_v2::filter::*;
/// assert_eq!(CompValue::True.to_string(), "true");
/// assert_eq!(CompValue::Null.to_string(), "null");
/// let v: CompValue = "hello\nworld".into();
/// assert_eq!(v.to_string(), r#""hello\nworld""#);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum CompValue {
    /// JSON `false`.
    False,
    /// JSON `null`.
    Null,
    /// JSON `true`.
    True,
    /// A JSON number (integer or decimal), e.g. `42` or `3.14`.
    Number(serde_json::Number),
    /// An unescaped string value. [`Display`] produces the JSON string literal
    /// (with surrounding double quotes and escape sequences).
    Str(String),
}

impl From<String> for CompValue {
    fn from(s: String) -> Self {
        CompValue::Str(s)
    }
}

impl From<&str> for CompValue {
    fn from(s: &str) -> Self {
        CompValue::Str(s.to_owned())
    }
}

/// Parse the raw attrPath token (e.g. "name.givenName" or
/// "urn:ietf:params:scim:schemas:core:2.0:User:userName") into an AttrPath.
pub(crate) fn parse_attr_path(s: &str) -> Result<AttrPath, FilterActionError> {
    // Attempt to detect a URI prefix: a URI prefix contains ':' and the last
    // colon-separated segment is a plain identifier (no further ':').
    let (uri, rest) = if let Some((before, after)) = s.rsplit_once(':') {
        // Delegate URI parsing to fluent_uri to keep filter.lalrpop simple
        if Uri::parse(before).is_ok() {
            (Some(before.to_owned()), after)
        } else {
            (None, s)
        }
    } else {
        (None, s)
    };

    // RFC 7644 §3.4.2.2: attrPath = [URI ":"] ATTRNAME *1subAttr
    // "*1" means at most one sub-attribute segment.
    if rest.chars().filter(|&c| c == '.').count() > 1 {
        return Err(FilterActionError::InvalidAttrPath(s.to_string()));
    }

    let (name, sub_attr) = rest
        .split_once('.')
        .map_or((rest, None), |(name, sub_attr)| {
            (name, Some(sub_attr.to_string()))
        });

    Ok(AttrPath {
        uri,
        name: name.to_string(),
        sub_attr,
    })
}

/// A SCIM PATCH operation path (RFC 7644 §3.5.2).
///
/// The `path` field of a PATCH operation identifies which attribute (or subset
/// of attribute values) to modify. It takes one of two forms:
///
/// | Form | Example | Variant |
/// |------|---------|---------|
/// | Plain attribute path | `userName`, `name.familyName` | [`PatchPath::Attr`] |
/// | Value path with optional sub-attribute | `emails[type eq "work"]`, `emails[type eq "work"].value` | [`PatchPath::Value`] |
///
/// ## SCIM server: inspecting an incoming PATCH path
///
/// ```
/// # use scim_v2::filter::*;
/// fn describe_path(path: &PatchPath) -> String {
///     match path {
///         PatchPath::Attr(attr) => format!("set attribute '{attr}'"),
///         PatchPath::Value(vp) => {
///             let sub = vp.sub_attr.as_deref().unwrap_or("(all fields)");
///             format!("set '{}' on matching '{}' values", sub, vp.attr)
///         }
///     }
/// }
///
/// let path: PatchPath = r#"emails[type eq "work"].value"#.parse().unwrap();
/// assert_eq!(describe_path(&path), "set 'value' on matching 'emails' values");
/// ```
///
/// ## SCIM client: building a PATCH path
///
/// ```
/// # use scim_v2::filter::*;
/// // emails[type eq "work"].value
/// let path = PatchPath::Value(PatchValuePath {
///     attr: AttrPath::with_name("emails"),
///     filter: ValFilter::Attr(AttrExp::Comparison(
///         AttrPath::with_name("type"),
///         CompareOp::Eq,
///         "work".into(),
///     )),
///     sub_attr: Some("value".into()),
/// });
/// assert_eq!(path.to_string(), r#"emails[type eq "work"].value"#);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum PatchPath {
    /// Plain attribute path (e.g. `"userName"`, `"name.familyName"`).
    Attr(AttrPath),
    /// Value path with optional trailing sub-attribute
    /// (e.g. `emails[type eq "work"]` or `emails[type eq "work"].value`).
    Value(PatchValuePath),
}

/// The `valuePath [subAttr]` form of a PATCH path (RFC 7644 §3.5.2).
///
/// Identifies a specific subset of a multi-valued attribute's elements, and
/// optionally a sub-attribute within those elements.
///
/// ```
/// # use scim_v2::filter::*;
/// let pvp = PatchValuePath {
///     attr: AttrPath::with_name("emails"),
///     filter: ValFilter::Attr(AttrExp::Comparison(
///         AttrPath::with_name("type"),
///         CompareOp::Eq,
///         "work".into(),
///     )),
///     sub_attr: Some("value".into()),
/// };
/// assert_eq!(pvp.to_string(), r#"emails[type eq "work"].value"#);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PatchValuePath {
    /// The multi-valued attribute being targeted.
    pub attr: AttrPath,
    /// The filter that selects which elements of the attribute to modify.
    pub filter: ValFilter,
    /// Optional sub-attribute after the closing bracket (e.g. `"value"` in
    /// `emails[type eq "work"].value`). When `None`, the operation targets
    /// the entire matching element.
    pub sub_attr: Option<String>,
}

impl Display for AttrPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(uri) = &self.uri {
            write!(f, "{uri}:")?;
        }
        write!(f, "{}", self.name)?;
        if let Some(sub) = &self.sub_attr {
            write!(f, ".{sub}")?;
        }
        Ok(())
    }
}

impl Display for CompareOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CompareOp::Eq => "eq",
            CompareOp::Ne => "ne",
            CompareOp::Co => "co",
            CompareOp::Sw => "sw",
            CompareOp::Ew => "ew",
            CompareOp::Gt => "gt",
            CompareOp::Lt => "lt",
            CompareOp::Ge => "ge",
            CompareOp::Le => "le",
        };
        write!(f, "{s}")
    }
}

impl Display for CompValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompValue::False => write!(f, "false"),
            CompValue::Null => write!(f, "null"),
            CompValue::True => write!(f, "true"),
            CompValue::Number(n) => write!(f, "{n}"),
            CompValue::Str(s) => {
                // Produce a JSON-encoded string (with surrounding quotes and proper escaping)
                let encoded = serde_json::to_string(s).expect("string serialization never fails");
                write!(f, "{encoded}")
            }
        }
    }
}

impl Display for AttrExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AttrExp::Present(path) => write!(f, "{path} pr"),
            AttrExp::Comparison(path, op, val) => write!(f, "{path} {op} {val}"),
        }
    }
}

impl Display for ValFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValFilter::Attr(e) => write!(f, "{e}"),
            ValFilter::Not(inner) => write!(f, "not ({inner})"),
            ValFilter::And(items) => {
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " and ")?;
                    }
                    fmt_and_operand_val_filter(item, f)?;
                }
                Ok(())
            }
            ValFilter::Or(items) => {
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " or ")?;
                    }
                    write!(f, "{item}")?;
                }
                Ok(())
            }
        }
    }
}

fn fmt_and_operand_val_filter(operand: &ValFilter, f: &mut Formatter<'_>) -> std::fmt::Result {
    if matches!(operand, ValFilter::Or(_)) {
        write!(f, "({operand})")
    } else {
        write!(f, "{operand}")
    }
}

impl Display for PatchValuePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.attr, self.filter)?;
        if let Some(sub) = &self.sub_attr {
            write!(f, ".{sub}")?;
        }
        Ok(())
    }
}

impl Display for PatchPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PatchPath::Attr(p) => write!(f, "{p}"),
            PatchPath::Value(vp) => write!(f, "{vp}"),
        }
    }
}

impl Serialize for PatchPath {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PatchPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl std::str::FromStr for PatchPath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let budget = ParseBudget::default();
        let parsed = path_parser()
            .parse(&budget, s.trim())
            .map_err(|e| e.map_token(|t| t.to_string()))?;
        match parsed {
            PatchPath::Attr(a) => Ok(PatchPath::Attr(a)),
            PatchPath::Value(vp) => {
                if let Some(depth) = val_filter_depth_exceeds(&vp.filter, MAX_FILTER_DEPTH) {
                    drop_val_filter_iteratively(vp.filter);
                    return Err(LalrParseError::User {
                        error: FilterActionError::DepthExceeded(depth),
                    });
                }
                Ok(PatchPath::Value(vp))
            }
        }
    }
}

// --- Depth enforcement helpers ----------------------------------------------
//
// Both the depth check and the over-deep drop path walk `Filter` / `ValFilter`
// iteratively via an explicit worklist so that a pathological input cannot
// overflow the call stack before we manage to reject it.

/// Return `Some(depth)` as soon as any node's depth in `root` exceeds `limit`,
/// else `None`.
fn filter_depth_exceeds(root: &Filter, limit: usize) -> Option<usize> {
    let mut worklist: Vec<(&Filter, usize)> = vec![(root, 1)];
    while let Some((node, depth)) = worklist.pop() {
        if depth > limit {
            return Some(depth);
        }
        match node {
            Filter::Attr(_) => {}
            Filter::ValuePath(vp) => {
                // One budget for the whole filter: the inner filter continues
                // from this node's depth rather than starting a fresh count.
                if let Some(d) = val_filter_depth_exceeds_from(&vp.filter, depth + 1, limit) {
                    return Some(d);
                }
            }
            Filter::Not(inner) => worklist.push((inner, depth + 1)),
            Filter::And(items) | Filter::Or(items) => {
                worklist.extend(items.iter().map(|item| (item, depth + 1)));
            }
        }
    }
    None
}

/// `ValFilter` mirror of [`filter_depth_exceeds`].
fn val_filter_depth_exceeds(root: &ValFilter, limit: usize) -> Option<usize> {
    val_filter_depth_exceeds_from(root, 1, limit)
}

/// [`val_filter_depth_exceeds`] with `root` at `start` rather than 1, so a
/// value path's inner filter is charged against the budget its enclosing
/// filter has already spent.
fn val_filter_depth_exceeds_from(root: &ValFilter, start: usize, limit: usize) -> Option<usize> {
    let mut worklist: Vec<(&ValFilter, usize)> = vec![(root, start)];
    while let Some((node, depth)) = worklist.pop() {
        if depth > limit {
            return Some(depth);
        }
        match node {
            ValFilter::Attr(_) => {}
            ValFilter::Not(inner) => worklist.push((inner, depth + 1)),
            ValFilter::And(items) | ValFilter::Or(items) => {
                worklist.extend(items.iter().map(|item| (item, depth + 1)));
            }
        }
    }
    None
}

/// Drop a [`Filter`] without recursing, so an over-deep AST can be rejected
/// without the derived recursive `Drop` overflowing the stack.
fn drop_filter_iteratively(root: Filter) {
    let mut stack: Vec<Filter> = vec![root];
    while let Some(node) = stack.pop() {
        match node {
            Filter::Attr(_) => {}
            Filter::ValuePath(vp) => drop_val_filter_iteratively(*vp.filter),
            Filter::Not(inner) => stack.push(*inner),
            Filter::And(items) | Filter::Or(items) => stack.extend(items),
        }
    }
}

/// `ValFilter` mirror of [`drop_filter_iteratively`].
fn drop_val_filter_iteratively(root: ValFilter) {
    let mut stack: Vec<ValFilter> = vec![root];
    while let Some(node) = stack.pop() {
        match node {
            ValFilter::Attr(_) => {}
            ValFilter::Not(inner) => stack.push(*inner),
            ValFilter::And(items) | ValFilter::Or(items) => stack.extend(items),
        }
    }
}

#[cfg(test)]
mod tests;
