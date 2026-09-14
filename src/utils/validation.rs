//! Validation of SCIM resources against the required-attribute rules the
//! type system cannot express.
//!
//! Nearly every attribute in RFC 7643 is optional. The models therefore make
//! almost everything `Option` or defaulted. `serde` cannot enforce the handful
//! of attributes the RFC marks REQUIRED. [`Validate`] carries those checks.
//! [`Validate`] reports a failure with the **wire** attribute name
//! (`userName`, not `user_name`). A server can then echo the path straight
//! back in an RFC 7644 §3.12 error response.

use thiserror::Error;

#[cfg(feature = "models")]
use crate::models::errors::{ScimHttpError, ScimType};
#[cfg(feature = "models")]
use crate::schema_urns;

/// Why a resource fails validation.
///
/// `#[non_exhaustive]`: further checks (attribute-type and mutability
/// conformance, canonical-value membership) will add variants in minor
/// releases.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidationErrorKind {
    /// An attribute RFC 7643 marks REQUIRED is absent, `null`, or empty.
    #[error("required attribute is missing or empty")]
    MissingRequiredAttribute,
    /// The attribute is present, but the value is not permissible.
    #[error("{0}")]
    InvalidValue(String),
}

/// A single validation failure, located by its SCIM wire path.
///
/// `path` is dotted wire notation, as a filter or a PATCH path spells it:
/// `userName`, `name.familyName`, `meta.resourceType`. `path` is never the
/// Rust field name.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{path}: {kind}")]
pub struct ValidationError {
    path: String,
    kind: ValidationErrorKind,
}

impl ValidationError {
    /// A REQUIRED attribute is missing or empty.
    pub fn missing_required(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            kind: ValidationErrorKind::MissingRequiredAttribute,
        }
    }

    /// An attribute carries a value that is not permissible.
    pub fn invalid_value(path: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            kind: ValidationErrorKind::InvalidValue(detail.into()),
        }
    }

    /// The same failure, located under `parent`, such as `Resources[2]` or
    /// `authenticationSchemes[0]`. A container reports a nested resource's
    /// error with the full wire path, the kind and the detail intact. The
    /// container does not flatten every nested failure into one kind.
    pub fn under(mut self, parent: &str) -> Self {
        self.path = format!("{parent}.{}", self.path);
        self
    }

    /// The SCIM wire path of the attribute at fault.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Why the validation fails.
    pub fn kind(&self) -> &ValidationErrorKind {
        &self.kind
    }

    /// The RFC 7644 §3.12 `scimType` keyword for this failure, as its wire
    /// string. `scim_type()` (with the `models` feature) gives the typed form.
    pub fn scim_type_str(&self) -> &'static str {
        match self.kind {
            ValidationErrorKind::MissingRequiredAttribute
            | ValidationErrorKind::InvalidValue(_) => "invalidValue",
        }
    }

    /// The RFC 7644 §3.12 `scimType` keyword for this failure.
    ///
    /// Every current variant maps to `invalidValue`, which §3.12 defines as
    /// "A required value was missing, or the value specified was not
    /// compatible with the operation or attribute type ... or resource
    /// schema".
    #[cfg(feature = "models")]
    pub fn scim_type(&self) -> ScimType {
        match self.kind {
            ValidationErrorKind::MissingRequiredAttribute
            | ValidationErrorKind::InvalidValue(_) => ScimType::InvalidValue,
        }
    }

    /// Build the RFC 7644 §3.12 error body a server should return.
    ///
    /// Every current [`ValidationErrorKind`] is a `400 Bad Request`. The
    /// method therefore fixes the status here, rather than take it as a
    /// parameter a caller could get wrong. For a body that needs another
    /// status, build a [`ScimHttpError`] directly and check it with its own
    /// `Validate`.
    ///
    /// Requires the `models` feature, which supplies [`ScimHttpError`].
    #[cfg(feature = "models")]
    pub fn to_http_error(&self) -> ScimHttpError {
        ScimHttpError {
            schemas: vec![schema_urns::ERROR.to_string()],
            scim_type: Some(self.scim_type()),
            detail: Some(self.to_string()),
            status: "400".to_string(),
        }
    }
}

/// RFC 7643 §3: `schemas` is REQUIRED and "MUST include a non-empty array".
/// §3 also requires "each String value must be a unique URI" and says
/// "duplicate values MUST NOT be included". A resource's `schemas` "MUST only
/// contain values defined as schema and schemaExtensions for the resource's
/// defined resourceType". The protocol messages of RFC 7644 each carry one
/// fixed URN. The function checks three things: the array is not empty, the
/// values are unique, and `required_urn` is present. The function deliberately
/// does *not* reject an additional URN. A resource may carry an extension this
/// crate does not model, and the `Resource` deserializer in `models::others`
/// likewise ignores a URN it does not recognise. The uniqueness comparison is
/// byte-exact, the same comparison the membership check uses.
pub fn require_schema_urn(schemas: &[String], required_urn: &str) -> Result<(), ValidationError> {
    if schemas.is_empty() {
        return Err(ValidationError::missing_required("schemas"));
    }
    // One pass, linear: `schemas` is attacker-sized and this runs inside
    // `Strict`'s deserialization, so a quadratic scan here was a CPU
    // amplifier on the success path.
    let mut seen = std::collections::HashSet::with_capacity(schemas.len());
    let mut has_required = false;
    for s in schemas {
        if !seen.insert(s.as_str()) {
            return Err(ValidationError::invalid_value(
                "schemas",
                format!("duplicate value {s}; RFC 7643 §3 requires each URI to be unique"),
            ));
        }
        has_required |= s == required_urn;
    }
    if !has_required {
        return Err(ValidationError::invalid_value(
            "schemas",
            format!("must include {required_urn}"),
        ));
    }
    Ok(())
}

/// RFC 7643 §2.4: for a multi-valued attribute, "The primary attribute value
/// `true` MUST appear no more than once." `primary` extracts the flag from
/// each value. `path` is the attribute's wire name for the error.
pub fn at_most_one_primary<T>(
    values: &[T],
    primary: impl Fn(&T) -> Option<bool>,
    path: &str,
) -> Result<(), ValidationError> {
    let count = values.iter().filter(|v| primary(v) == Some(true)).count();
    if count > 1 {
        return Err(ValidationError::invalid_value(
            path,
            format!("primary is true on {count} values; RFC 7643 §2.4 allows at most one"),
        ));
    }
    Ok(())
}

/// The direction a payload travels. The direction decides which conformance
/// rules apply.
///
/// RFC 7643 §3.1 makes `id` REQUIRED in "each representation of the resource"
/// a server returns. A client "MUST NOT" specify `id` on create. §4.1 gives
/// `password` `returned: never`, so `password` may travel *to* a server and
/// must never travel *from* a server. A single direction-agnostic check cannot
/// express either rule. [`Validate::validate_as`] therefore takes a context.
///
/// `#[non_exhaustive]`: minor releases may add further contexts (bulk, search
/// results as a distinct case).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Context {
    /// A client's `POST` body. `id` MUST NOT be present (RFC 7643 §3.1).
    CreateRequest,
    /// A client's `PUT` body. RFC 7644 §3.5.1 has the server ignore
    /// `readOnly` attributes such as `id`, rather than reject them. The
    /// §3.5.1 example itself carries `id`. This context therefore tolerates
    /// `id`.
    ReplaceRequest,
    /// A server's response body. `id` is REQUIRED (§3.1, except
    /// `ServiceProviderConfig` per §5). A `returned: never` attribute such as
    /// `password` MUST be absent.
    Response,
}

impl Context {
    /// Human-readable name for messages.
    pub fn as_str(&self) -> &'static str {
        match self {
            Context::CreateRequest => "create request",
            Context::ReplaceRequest => "replace request",
            Context::Response => "response",
        }
    }
}

/// Check a resource against the attributes RFC 7643 marks REQUIRED.
///
/// Import the trait to call it: `use scim_v2::Validate;`
///
/// [`validate`](Self::validate) holds the rules that apply in every
/// direction. [`validate_as`](Self::validate_as) adds the direction-specific
/// rules, such as `id` REQUIRED on a response and forbidden on a create.
/// [`Valid`] requires [`validate_as`](Self::validate_as). No method here runs
/// during deserialization. The wire parsers stay lenient, so a caller can read
/// a real provider's payload. The crate asserts conformance at the point where
/// the crate's user needs the assertion. [`Strict`] moves that point to the
/// parse boundary, for a caller who wants the check there.
pub trait Validate {
    /// `Ok(())` when every REQUIRED attribute is present and permissible,
    /// regardless of direction.
    fn validate(&self) -> Result<(), ValidationError>;

    /// The rules that depend on direction. The default implementation has no
    /// such rule. A resource overrides the default.
    fn validate_context(&self, _ctx: Context) -> Result<(), ValidationError> {
        Ok(())
    }

    /// [`validate`](Self::validate) plus the rules for `ctx`.
    fn validate_as(&self, ctx: Context) -> Result<(), ValidationError> {
        self.validate()?;
        self.validate_context(ctx)
    }
}

/// A value that has passed [`Validate::validate_as`] for a known
/// [`Context`].
///
/// [`Valid::new`] is the only way to obtain one. A caller therefore cannot
/// give an unvalidated resource to a handler that takes `Valid<User>`. A
/// caller who omits the check gets a compile error, not a 500 response later.
/// The wrapper costs nothing at run time beyond the validation itself. The
/// wrapper dereferences to the inner value. The wrapper deliberately has no
/// `DerefMut`, because a mutation could invalidate the inner value. Use
/// [`into_inner`](Self::into_inner) to take ownership back.
///
/// ```
/// # #[cfg(feature = "models")] {
/// use scim_v2::{Context, Valid, models::user::User};
///
/// let user = User::<String> {
///     schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".to_string()],
///     user_name: "bjensen".to_string(),
///     ..Default::default()
/// };
///
/// // A create request may not carry an id, and this one does not.
/// let valid = Valid::new(user, Context::CreateRequest).unwrap();
/// assert_eq!(valid.user_name, "bjensen");
///
/// // The same value is not a conformant *response*: a response needs an id.
/// let err = Valid::new(valid.into_inner(), Context::Response).unwrap_err();
/// assert_eq!(err.path(), "id");
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Valid<T> {
    inner: T,
    context: Context,
}

impl<T: Validate> Valid<T> {
    /// Validate `value` for `context`. On success, the method wraps `value`.
    pub fn new(value: T, context: Context) -> Result<Self, ValidationError> {
        value.validate_as(context)?;
        Ok(Self {
            inner: value,
            context,
        })
    }
}

impl<T> Valid<T> {
    /// The context this value is valid for.
    pub fn context(&self) -> Context {
        self.context
    }

    /// Take the inner value back. A caller can mutate the returned value, so
    /// the value is no longer known to be valid. The method therefore consumes
    /// the wrapper.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> std::ops::Deref for Valid<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: serde::Serialize> serde::Serialize for Valid<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(serializer)
    }
}

mod marker_sealed {
    pub trait Sealed {}
}

/// Type-level [`Context`], for [`Strict`].
pub trait ContextMarker: marker_sealed::Sealed {
    /// The context this marker stands for.
    const CONTEXT: Context;
}

/// Marker for [`Context::CreateRequest`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateRequest;
/// Marker for [`Context::ReplaceRequest`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplaceRequest;
/// Marker for [`Context::Response`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Response;

impl marker_sealed::Sealed for CreateRequest {}
impl marker_sealed::Sealed for ReplaceRequest {}
impl marker_sealed::Sealed for Response {}
impl ContextMarker for CreateRequest {
    const CONTEXT: Context = Context::CreateRequest;
}
impl ContextMarker for ReplaceRequest {
    const CONTEXT: Context = Context::ReplaceRequest;
}
impl ContextMarker for Response {
    const CONTEXT: Context = Context::Response;
}

/// Deserialize-and-validate in one step.
///
/// The plain models deserialize leniently, so a caller can always read a real
/// provider's payload. A server that parses an inbound request usually wants
/// the opposite behaviour. The server rejects a non-conformant body at the
/// boundary and answers with the RFC 7644 §3.12 error. `Strict<T, M>` gives
/// the server that behaviour. `Strict<T, M>` deserializes `T` and runs
/// [`Validate::validate_as`] for `M`'s [`Context`]. A failed check fails the
/// deserialization, with the [`ValidationError`]'s message. On success,
/// `Strict<T, M>` holds a [`Valid<T>`].
///
/// ```
/// # #[cfg(feature = "models")] {
/// use scim_v2::{CreateRequest, Strict, Valid, models::user::User};
///
/// let body = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"bjensen"}"#;
/// let valid: Valid<User<String>> =
///     serde_json::from_str::<Strict<User<String>, CreateRequest>>(body).unwrap().into_valid();
/// assert_eq!(valid.user_name, "bjensen");
///
/// // A create body carrying an id is rejected at parse time.
/// let with_id = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"bjensen","id":"x"}"#;
/// assert!(serde_json::from_str::<Strict<User<String>, CreateRequest>>(with_id).is_err());
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strict<T, M: ContextMarker>(Valid<T>, std::marker::PhantomData<M>);

impl<T, M: ContextMarker> Strict<T, M> {
    /// The validated value.
    pub fn into_valid(self) -> Valid<T> {
        self.0
    }
}

impl<T, M: ContextMarker> std::ops::Deref for Strict<T, M> {
    type Target = Valid<T>;
    fn deref(&self) -> &Valid<T> {
        &self.0
    }
}

impl<'de, T, M> serde::Deserialize<'de> for Strict<T, M>
where
    T: serde::Deserialize<'de> + Validate,
    M: ContextMarker,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = T::deserialize(deserializer)?;
        let valid = Valid::new(value, M::CONTEXT).map_err(serde::de::Error::custom)?;
        Ok(Strict(valid, std::marker::PhantomData))
    }
}

impl<T: serde::Serialize, M: ContextMarker> serde::Serialize for Strict<T, M> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(test)]
mod tests;
