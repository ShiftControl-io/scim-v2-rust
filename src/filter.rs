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
//! - [`SearchRequest::filter`](crate::models::others::SearchRequest::filter) — from a
//!   `POST /.search` body.
//! - [`ListQuery::filter`](crate::models::others::ListQuery::filter) — from a `GET`
//!   query string (`?filter=...`), parsed by your web framework into this struct.
//! - [`PatchOperation`](crate::models::others::PatchOperation) — each operation's
//!   `path` field deserializes as a [`PatchPath`].
//!
//! By default these types fail deserialization when the filter expression is
//! malformed, which takes `start_index`, `count`, and other fields down with
//! it. To produce an RFC 7644 §3.12 `invalidFilter` response instead, use the
//! [`TolerantListQuery`](crate::models::others::TolerantListQuery) /
//! [`TolerantSearchRequest`](crate::models::others::TolerantSearchRequest)
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
//!         Filter::And(lhs, rhs) => matches_user(lhs, user_name) && matches_user(rhs, user_name),
//!         Filter::Or(lhs, rhs) => matches_user(lhs, user_name) || matches_user(rhs, user_name),
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
//! # Depth limit
//!
//! [`Filter::from_str`] and [`PatchPath::from_str`] (and the corresponding
//! [`Deserialize`](serde::Deserialize) impls, which delegate to
//! [`FromStr`](std::str::FromStr)) reject any input whose parsed AST would exceed
//! [`MAX_FILTER_DEPTH`]. This bounds the call stack used by the crate's own
//! recursive [`Display`], derived [`PartialEq`] / [`Debug`], serialization, and
//! `Drop` impls, so a hostile `?filter=` value with thousands of chained
//! `not`/`and`/`or` operators cannot crash the server after the filter has
//! been accepted. Hand-constructed [`Filter`] values bypass this check and are
//! the caller's responsibility.
//!
//! # SCIM client: building a filter to send in a request
//!
//! Construct the AST directly and assign it to the `filter` field of
//! [`SearchRequest`](crate::models::others::SearchRequest) or
//! [`ListQuery`](crate::models::others::ListQuery). Those types implement
//! `serde::Serialize`, so the filter is serialized automatically as a JSON
//! string when you serialize the containing struct. You can also call
//! `.to_string()` directly when you need the raw filter string for a query
//! parameter:
//!
//! ```
//! # use scim_v2::filter::*;
//! let filter = Filter::And(
//!     Box::new(Filter::Attr(AttrExp::Present(AttrPath::with_name("title")))),
//!     Box::new(Filter::Attr(AttrExp::Comparison(
//!         AttrPath::with_name("userType"),
//!         CompareOp::Eq,
//!         "Employee".into(),
//!     ))),
//! );
//! assert_eq!(filter.to_string(), r#"title pr and userType eq "Employee""#);
//! ```

use fluent_uri::Uri;
use lalrpop_util::ParseError as LalrParseError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// Maximum allowed nesting depth for a parsed [`Filter`] or [`ValFilter`] tree.
///
/// [`Filter::from_str`] and [`PatchPath::from_str`] reject any input whose parsed
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

/// Error produced by fallible grammar actions (`=>?` rules in the LALRPOP grammar)
/// and by post-parse validation.
#[derive(Debug, Error)]
pub enum FilterActionError {
    /// An `attrPath` token contained more than one sub-attribute segment.
    #[error("attrPath '{0}' has more than one sub-attribute segment")]
    InvalidAttrPath(String),
    /// A comparison value contained an invalid JSON escape or format.
    #[error("invalid comparison value: {0}")]
    InvalidCompValue(#[from] serde_json::Error),
    /// The parsed filter's nesting depth exceeded [`MAX_FILTER_DEPTH`].
    ///
    /// The wrapped value is the depth at which the limit was first breached.
    #[error("filter nesting depth exceeds maximum of {MAX_FILTER_DEPTH} (at depth {0})")]
    DepthExceeded(usize),
}

/// Error returned by [`Filter::from_str`] and [`PatchPath::from_str`] when the
/// input is not a valid filter or path expression.
pub type ParseError = LalrParseError<usize, String, FilterActionError>;

/// Payload of [`MaybeFilter::Invalid`] and error type of the
/// `TryFrom<Tolerant*>` / `into_strict` conversions on
/// [`TolerantListQuery`](crate::models::others::TolerantListQuery) and
/// [`TolerantSearchRequest`](crate::models::others::TolerantSearchRequest).
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
    And(Box<Filter>, Box<Filter>),

    /// Logical disjunction — at least one operand must match.
    ///
    /// Lowest precedence operator. When an `Or` expression appears as a
    /// direct child of an `And`, [`Display`] wraps it in parentheses to
    /// preserve the original precedence on round-trip.
    Or(Box<Filter>, Box<Filter>),
}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::Attr(e) => write!(f, "{e}"),
            Filter::ValuePath(vp) => write!(f, "{}[{}]", vp.attr, vp.filter),
            Filter::Not(inner) => write!(f, "not ({inner})"),
            Filter::And(lhs, rhs) => {
                fmt_and_operand_filter(lhs, f)?;
                write!(f, " and ")?;
                fmt_and_operand_filter(rhs, f)
            }
            Filter::Or(lhs, rhs) => write!(f, "{lhs} or {rhs}"),
        }
    }
}

fn fmt_and_operand_filter(operand: &Filter, f: &mut Formatter<'_>) -> std::fmt::Result {
    if matches!(operand, Filter::Or(_, _)) {
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
/// [`ListQuery`](crate::models::others::ListQuery) /
/// [`SearchRequest`](crate::models::others::SearchRequest) deserialize
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
        let parsed = crate::filter_parser::FilterParser::new()
            .parse(s.trim())
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
    And(Box<ValFilter>, Box<ValFilter>),
    /// Logical disjunction — at least one operand must match.
    Or(Box<ValFilter>, Box<ValFilter>),
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
            ValFilter::And(lhs, rhs) => {
                fmt_and_operand_val_filter(lhs, f)?;
                write!(f, " and ")?;
                fmt_and_operand_val_filter(rhs, f)
            }
            ValFilter::Or(lhs, rhs) => write!(f, "{lhs} or {rhs}"),
        }
    }
}

fn fmt_and_operand_val_filter(operand: &ValFilter, f: &mut Formatter<'_>) -> std::fmt::Result {
    if matches!(operand, ValFilter::Or(_, _)) {
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
        let parsed = crate::filter_parser::PathParser::new()
            .parse(s.trim())
            .map_err(|e| e.map_token(|t| t.to_string()))?;
        match parsed {
            PatchPath::Attr(a) => Ok(PatchPath::Attr(a)),
            PatchPath::Value(vp) => match val_filter_depth_exceeds(&vp.filter, MAX_FILTER_DEPTH) {
                Some(depth) => {
                    drop_val_filter_iteratively(vp.filter);
                    Err(LalrParseError::User {
                        error: FilterActionError::DepthExceeded(depth),
                    })
                }
                None => Ok(PatchPath::Value(vp)),
            },
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
                if let Some(d) = val_filter_depth_exceeds(&vp.filter, limit) {
                    return Some(d);
                }
            }
            Filter::Not(inner) => worklist.push((inner, depth + 1)),
            Filter::And(lhs, rhs) | Filter::Or(lhs, rhs) => {
                worklist.push((lhs, depth + 1));
                worklist.push((rhs, depth + 1));
            }
        }
    }
    None
}

/// `ValFilter` mirror of [`filter_depth_exceeds`].
fn val_filter_depth_exceeds(root: &ValFilter, limit: usize) -> Option<usize> {
    let mut worklist: Vec<(&ValFilter, usize)> = vec![(root, 1)];
    while let Some((node, depth)) = worklist.pop() {
        if depth > limit {
            return Some(depth);
        }
        match node {
            ValFilter::Attr(_) => {}
            ValFilter::Not(inner) => worklist.push((inner, depth + 1)),
            ValFilter::And(lhs, rhs) | ValFilter::Or(lhs, rhs) => {
                worklist.push((lhs, depth + 1));
                worklist.push((rhs, depth + 1));
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
            Filter::And(lhs, rhs) | Filter::Or(lhs, rhs) => {
                stack.push(*lhs);
                stack.push(*rhs);
            }
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
            ValFilter::And(lhs, rhs) | ValFilter::Or(lhs, rhs) => {
                stack.push(*lhs);
                stack.push(*rhs);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_scim_filter_simple_eq() {
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"userName eq "bjensen""#)
            .unwrap();
        assert_eq!(
            f,
            Filter::Attr(AttrExp::Comparison(
                AttrPath {
                    uri: None,
                    name: "userName".into(),
                    sub_attr: None
                },
                CompareOp::Eq,
                CompValue::Str("bjensen".into()),
            ))
        );
    }

    #[test]
    fn test_scim_filter_case_insensitive_op() {
        // RFC: "Attribute names and attribute operators used in filters are case insensitive"
        let f1 = crate::filter_parser::FilterParser::new()
            .parse(r#"userName Eq "john""#)
            .unwrap();
        let f2 = crate::filter_parser::FilterParser::new()
            .parse(r#"userName eq "john""#)
            .unwrap();
        assert_eq!(f1, f2);
    }

    #[test]
    fn test_scim_filter_pr() {
        let f = crate::filter_parser::FilterParser::new()
            .parse("title pr")
            .unwrap();
        assert_eq!(
            f,
            Filter::Attr(AttrExp::Present(AttrPath {
                uri: None,
                name: "title".into(),
                sub_attr: None,
            }))
        );
    }

    #[test]
    fn test_scim_filter_sub_attr() {
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"name.familyName co "O'Malley""#)
            .unwrap();
        assert_eq!(
            f,
            Filter::Attr(AttrExp::Comparison(
                AttrPath {
                    uri: None,
                    name: "name".into(),
                    sub_attr: Some("familyName".into()),
                },
                CompareOp::Co,
                CompValue::Str("O'Malley".into()),
            ))
        );
    }

    #[test]
    fn test_scim_filter_uri_prefix() {
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"urn:ietf:params:scim:schemas:core:2.0:User:userName sw "J""#)
            .unwrap();
        assert_eq!(
            f,
            Filter::Attr(AttrExp::Comparison(
                AttrPath {
                    uri: Some("urn:ietf:params:scim:schemas:core:2.0:User".into()),
                    name: "userName".into(),
                    sub_attr: None,
                },
                CompareOp::Sw,
                CompValue::Str("J".into()),
            ))
        );
    }

    #[test]
    fn test_scim_filter_and() {
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"title pr and userType eq "Employee""#)
            .unwrap();
        assert_eq!(
            f,
            Filter::And(
                Box::new(Filter::Attr(AttrExp::Present(AttrPath {
                    uri: None,
                    name: "title".into(),
                    sub_attr: None,
                }))),
                Box::new(Filter::Attr(AttrExp::Comparison(
                    AttrPath {
                        uri: None,
                        name: "userType".into(),
                        sub_attr: None
                    },
                    CompareOp::Eq,
                    CompValue::Str("Employee".into()),
                ))),
            )
        );
    }

    #[test]
    fn test_scim_filter_or() {
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"title pr or userType eq "Intern""#)
            .unwrap();
        assert_eq!(
            f,
            Filter::Or(
                Box::new(Filter::Attr(AttrExp::Present(AttrPath {
                    uri: None,
                    name: "title".into(),
                    sub_attr: None,
                }))),
                Box::new(Filter::Attr(AttrExp::Comparison(
                    AttrPath {
                        uri: None,
                        name: "userType".into(),
                        sub_attr: None
                    },
                    CompareOp::Eq,
                    CompValue::Str("Intern".into()),
                ))),
            )
        );
    }

    #[test]
    fn test_scim_filter_and_precedence_over_or() {
        // "A and B or C" should parse as "(A and B) or C"
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"title pr and userType eq "Employee" or emails pr"#)
            .unwrap();

        assert!(matches!(f, Filter::Or(_, _)));
        if let Filter::Or(left, _) = f {
            assert!(matches!(*left, Filter::And(_, _)));
        }
    }

    #[test]
    fn test_scim_filter_not() {
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"not (emails co "example.com")"#)
            .unwrap();
        assert!(matches!(f, Filter::Not(_)));
    }

    #[test]
    fn test_scim_filter_grouping() {
        // Parens change precedence: "A or (B and C)" — inner group is And
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"userType eq "Employee" and (emails co "example.com" or emails.value co "example.org")"#)
            .unwrap();
        assert!(matches!(f, Filter::And(_, _)));
    }

    #[test]
    fn test_scim_filter_value_path() {
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"emails[type eq "work" and value co "@example.com"]"#)
            .unwrap();
        assert!(matches!(f, Filter::ValuePath(_)));
        if let Filter::ValuePath(vp) = f {
            assert_eq!(vp.attr.name, "emails");
            assert!(matches!(*vp.filter, ValFilter::And(_, _)));
        }
    }

    // All RFC Figure 2 examples: parse must succeed and display must round-trip.
    #[test_case(r#"userName eq "bjensen""# ; "simple_eq")]
    #[test_case(r#"name.familyName co "O'Malley""# ; "sub_attr_co")]
    #[test_case(r#"userName sw "J""# ; "sw")]
    #[test_case(r#"urn:ietf:params:scim:schemas:core:2.0:User:userName sw "J""# ; "uri_prefix_sw")]
    #[test_case("title pr" ; "pr")]
    #[test_case(r#"meta.lastModified gt "2011-05-13T04:42:34Z""# ; "datetime_gt")]
    #[test_case(r#"meta.lastModified ge "2011-05-13T04:42:34Z""# ; "datetime_ge")]
    #[test_case(r#"meta.lastModified lt "2011-05-13T04:42:34Z""# ; "datetime_lt")]
    #[test_case(r#"meta.lastModified le "2011-05-13T04:42:34Z""# ; "datetime_le")]
    #[test_case(r#"title pr and userType eq "Employee""# ; "and")]
    #[test_case(r#"title pr or userType eq "Intern""# ; "or")]
    #[test_case(r#"schemas eq "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User""# ; "urn_value")]
    #[test_case(r#"userType eq "Employee" and (emails co "example.com" or emails.value co "example.org")"# ; "and_grouped_or")]
    #[test_case(r#"userType ne "Employee" and not (emails co "example.com" or emails.value co "example.org")"# ; "and_not_grouped_or")]
    #[test_case(r#"userType eq "Employee" and (emails.type eq "work")"# ; "and_grouped_sub_attr")]
    #[test_case(r#"userType eq "Employee" and emails[type eq "work" and value co "@example.com"]"# ; "and_value_path")]
    #[test_case(r#"emails[type eq "work" and value co "@example.com"] or ims[type eq "xmpp" and value co "@foo.com"]"# ; "two_value_paths_or")]
    fn filter_parses_and_round_trips(s: &str) {
        let f: Filter = s.parse().unwrap_or_else(|e| panic!("parse {s:?}: {e:?}"));
        let displayed = f.to_string();
        let reparsed: Filter = displayed
            .parse()
            .unwrap_or_else(|e| panic!("re-parse of {displayed:?}: {e:?}"));
        assert_eq!(f, reparsed, "round-trip mismatch for {s:?}");
    }

    // ---------------------------------------------------------------------------
    // SCIM PATCH path tests (RFC 7644 §3.5.2)
    // ---------------------------------------------------------------------------

    #[test]
    fn test_scim_patch_path_attr() {
        let p = crate::filter_parser::PathParser::new()
            .parse("userName")
            .unwrap();
        assert_eq!(
            p,
            PatchPath::Attr(AttrPath {
                uri: None,
                name: "userName".into(),
                sub_attr: None
            })
        );
    }

    #[test]
    fn test_scim_patch_path_sub_attr() {
        let p = crate::filter_parser::PathParser::new()
            .parse("name.familyName")
            .unwrap();
        assert_eq!(
            p,
            PatchPath::Attr(AttrPath {
                uri: None,
                name: "name".into(),
                sub_attr: Some("familyName".into()),
            })
        );
    }

    #[test]
    fn test_scim_patch_path_value_path() {
        let p = crate::filter_parser::PathParser::new()
            .parse(r#"emails[type eq "work"]"#)
            .unwrap();
        let PatchPath::Value(vp) = p else {
            panic!("expected PatchPath::Value");
        };
        assert_eq!(vp.attr.name, "emails");
        assert!(vp.sub_attr.is_none());
    }

    #[test]
    fn test_scim_patch_path_value_path_sub_attr() {
        let p = crate::filter_parser::PathParser::new()
            .parse(r#"emails[type eq "work"].value"#)
            .unwrap();
        let PatchPath::Value(vp) = p else {
            panic!("expected PatchPath::Value");
        };
        assert_eq!(vp.attr.name, "emails");
        assert_eq!(vp.sub_attr.as_deref(), Some("value"));
    }

    // Patch paths that must be rejected by the parser.
    #[test_case(r#"emails[type eq "work"].value.extra"# ; "dotted_sub_attr")]
    #[test_case(r#"emails[type eq "work"].urn:foo:bar"# ; "trailing_urn")]
    #[test_case("name.family.given" ; "too_many_sub_attrs")]
    fn path_rejected(s: &str) {
        assert!(crate::filter_parser::PathParser::new().parse(s).is_err());
    }

    // Filter strings with invalid comp-values that must be rejected.
    #[test_case(r#"userName eq "\q""# ; "unknown_escape")]
    #[test_case(r#"userName eq "\uGHIJ""# ; "invalid_unicode_escape")]
    fn filter_comp_value_rejected(s: &str) {
        assert!(crate::filter_parser::FilterParser::new().parse(s).is_err());
    }

    // Invalid attrPaths that do not conform to RFC 7644 Figure 1.
    #[test_case("name." ; "empty_sub_attr")]
    #[test_case("name.family.given pr" ; "too_many_sub_attrs")]
    #[test_case(r#"emails[name.family.given eq "x"]"# ; "value_path_inner_too_many_sub_attrs")]
    fn invalid_attr_path_rejected(s: &str) {
        assert!(crate::filter_parser::FilterParser::new().parse(s).is_err());
    }

    // The RFC ABNF requires SP separators around compare/logical operators.
    #[test_case(r#"userName eq"bjensen""# ; "missing_space_before_comp_value")]
    #[test_case(r#"emails[type eq"work"]"# ; "missing_space_in_value_path")]
    #[test_case(r#"title prand userType eq "Employee""# ; "missing_space_before_and")]
    fn filter_requires_required_spaces(s: &str) {
        assert!(crate::filter_parser::FilterParser::new().parse(s).is_err());
    }

    #[test]
    fn test_comp_value_surrogate_pair_handled() {
        // \uD800\uDC00 is a surrogate pair representing U+10000
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"userName eq "\uD800\uDC00""#)
            .unwrap();
        if let Filter::Attr(AttrExp::Comparison(_, _, CompValue::Str(s))) = f {
            assert_eq!(s, "\u{10000}");
        } else {
            panic!("unexpected parse result");
        }
    }

    #[test]
    fn test_scim_patch_path_complex_filter() {
        let p = crate::filter_parser::PathParser::new()
            .parse(r#"emails[type eq "work" and value co "@example.com"].value"#)
            .unwrap();
        let PatchPath::Value(vp) = p else {
            panic!("expected PatchPath::Value");
        };
        let ValFilter::And(_, _) = vp.filter else {
            panic!("expected ValFilter::And, got {:?}", vp.filter);
        };
        assert_eq!(vp.sub_attr.as_deref(), Some("value"));
    }

    #[test]
    fn test_val_filter_parenthesized_grouping() {
        // Parenthesized grouping inside [...] should be accepted
        let f = crate::filter_parser::FilterParser::new()
            .parse(r#"emails[(type eq "work") and value pr]"#)
            .unwrap();
        assert!(matches!(f, Filter::ValuePath(_)));
        if let Filter::ValuePath(vp) = f {
            assert!(matches!(*vp.filter, ValFilter::And(_, _)));
        }
    }

    #[test]
    fn test_filter_from_str() {
        let f: Filter = r#"userName eq "bjensen""#.parse().unwrap();
        assert!(matches!(f, Filter::Attr(AttrExp::Comparison(..))));
    }

    #[test]
    fn test_patch_path_from_str() {
        let p: PatchPath = r#"emails[type eq "work"].value"#.parse().unwrap();
        assert!(matches!(p, PatchPath::Value(_)));
    }

    #[test]
    fn test_filter_from_str_error() {
        assert!(r#"not a valid filter !!!"#.parse::<Filter>().is_err());
    }

    // ---------------------------------------------------------------------------
    // Display / round-trip tests
    // ---------------------------------------------------------------------------

    #[test_case("userName" ; "simple_attr")]
    #[test_case("name.familyName" ; "sub_attr")]
    #[test_case(r#"emails[type eq "work"]"# ; "value_path")]
    #[test_case(r#"emails[type eq "work"].value"# ; "value_path_sub_attr")]
    #[test_case(r#"emails[type eq "work" and value co "@example.com"].value"# ; "complex_filter")]
    fn patch_path_round_trips(s: &str) {
        let p: PatchPath = s.parse().unwrap_or_else(|e| panic!("parse {s:?}: {e:?}"));
        let displayed = p.to_string();
        let reparsed: PatchPath = displayed
            .parse()
            .unwrap_or_else(|e| panic!("re-parse of {displayed:?}: {e:?}"));
        assert_eq!(p, reparsed, "round-trip mismatch for {s:?}");
    }

    #[test]
    fn test_display_attr_path() {
        let p = AttrPath {
            uri: Some("urn:ietf:params:scim:schemas:core:2.0:User".into()),
            name: "userName".into(),
            sub_attr: None,
        };
        assert_eq!(
            p.to_string(),
            "urn:ietf:params:scim:schemas:core:2.0:User:userName"
        );
    }

    #[test]
    fn test_display_comp_value_string_escaping() {
        // Newline in string must be JSON-escaped
        let v = CompValue::Str("foo\nbar".into());
        assert_eq!(v.to_string(), r#""foo\nbar""#);
    }

    #[test]
    fn test_display_and_wraps_or_children() {
        // "(A or B) and C" — Or child must be wrapped in parens
        let f: Filter = r#"(title pr or userType eq "Intern") and emails pr"#
            .parse()
            .unwrap();
        let s = f.to_string();
        // Re-parse must yield the same tree
        let reparsed: Filter = s.parse().unwrap();
        assert_eq!(f, reparsed);
        // The output must contain parens around the Or
        assert!(s.contains('('), "expected parens in {s:?}");
    }

    // ---------------------------------------------------------------------------
    // Depth-limit enforcement (DoS hardening against deeply nested filters)
    // ---------------------------------------------------------------------------

    fn assert_depth_exceeded<T: std::fmt::Debug>(res: Result<T, ParseError>) {
        match res {
            Err(LalrParseError::User {
                error: FilterActionError::DepthExceeded(_),
            }) => {}
            Err(other) => panic!("expected ParseError::User(DepthExceeded), got {other:?}"),
            Ok(ok) => panic!("expected rejection, but parse succeeded: {ok:?}"),
        }
    }

    // Build a filter string whose parsed AST has depth exactly `depth` using
    // nested `not`s: `not (not (... (title pr) ...))` with `depth - 1` `not`s
    // gives a Not-chain with a leaf Attr underneath → AST depth = depth.
    fn not_chain(depth: usize) -> String {
        assert!(depth >= 1);
        let nots = depth - 1;
        format!("{}title pr{}", "not (".repeat(nots), ")".repeat(nots))
    }

    // Same, for `ValFilter` content inside `emails[...]`.
    fn val_not_chain(depth: usize) -> String {
        assert!(depth >= 1);
        let nots = depth - 1;
        format!(
            r#"{}type eq "work"{}"#,
            "not (".repeat(nots),
            ")".repeat(nots)
        )
    }

    #[test]
    fn not_chain_within_limit_parses() {
        let s = not_chain(MAX_FILTER_DEPTH);
        s.parse::<Filter>()
            .unwrap_or_else(|e| panic!("expected success at depth {MAX_FILTER_DEPTH}: {e:?}"));
    }

    #[test]
    fn not_chain_exceeding_limit_rejected() {
        let s = not_chain(MAX_FILTER_DEPTH + 5);
        assert_depth_exceeded(s.parse::<Filter>());
    }

    #[test]
    fn and_chain_within_limit_parses() {
        // `a and b and c` is left-associative, so N leaves produce a depth-N AST
        // (N-1 And nodes on the left spine + 1 leaf).
        let n = MAX_FILTER_DEPTH;
        let mut s = String::from("title pr");
        for _ in 0..n - 1 {
            s.push_str(" and title pr");
        }
        s.parse::<Filter>()
            .unwrap_or_else(|e| panic!("expected success with {n} leaves: {e:?}"));
    }

    #[test]
    fn and_chain_exceeding_limit_rejected() {
        let n = MAX_FILTER_DEPTH + 10;
        let mut s = String::from("title pr");
        for _ in 0..n - 1 {
            s.push_str(" and title pr");
        }
        assert_depth_exceeded(s.parse::<Filter>());
    }

    #[test]
    fn or_chain_exceeding_limit_rejected() {
        let n = MAX_FILTER_DEPTH + 10;
        let mut s = String::from("title pr");
        for _ in 0..n - 1 {
            s.push_str(" or title pr");
        }
        assert_depth_exceeded(s.parse::<Filter>());
    }

    #[test]
    fn value_path_inner_depth_bounded() {
        // Over-deep ValFilter inside a ValuePath triggers val_filter_depth_exceeds.
        let s = format!("emails[{}]", val_not_chain(MAX_FILTER_DEPTH + 5));
        assert_depth_exceeded(s.parse::<Filter>());
    }

    #[test]
    fn patch_path_value_path_depth_bounded() {
        // Same, routed through PatchPath::from_str.
        let s = format!("emails[{}]", val_not_chain(MAX_FILTER_DEPTH + 5));
        assert_depth_exceeded(s.parse::<PatchPath>());
    }

    #[test]
    fn patch_path_plain_attr_not_affected() {
        // PatchPath::Attr has no recursive structure — depth check is a no-op.
        let p: PatchPath = "name.familyName".parse().unwrap();
        assert!(matches!(p, PatchPath::Attr(_)));
    }

    #[test]
    fn deserialize_rejects_deep_nesting() {
        // The Deserialize path delegates to FromStr; ensure a JSON-wrapped
        // over-deep filter is rejected.
        let raw = not_chain(MAX_FILTER_DEPTH + 5);
        let json = serde_json::to_string(&raw).unwrap();
        let err = serde_json::from_str::<Filter>(&json).unwrap_err();
        assert!(
            err.to_string().contains("depth"),
            "expected depth error, got: {err}"
        );
    }

    #[test]
    fn display_round_trip_at_limit_does_not_overflow() {
        // Confirm the whole recursive path (Display, PartialEq, Debug,
        // Serialize) is safe at the limit.
        let s = not_chain(MAX_FILTER_DEPTH);
        let f: Filter = s.parse().unwrap();
        let displayed = f.to_string();
        let reparsed: Filter = displayed.parse().unwrap();
        assert_eq!(f, reparsed);
        let _ = format!("{f:?}");
        let _ = serde_json::to_string(&f).unwrap();
    }

    #[test]
    fn extremely_deep_input_rejected_without_panic() {
        // Regression for the H-1 finding: 10_000 nested `not`s must return an
        // error rather than overflowing the stack during parse, depth-check,
        // or drop of the rejected AST.
        let s = not_chain(10_000);
        assert_depth_exceeded(s.parse::<Filter>());
    }
}
