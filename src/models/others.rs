use serde::de::{DeserializeOwned, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(feature = "filter")]
use crate::filter::{AttrExp, Filter, InvalidFilterError, MaybeFilter, PatchPath};
use crate::models::group::Group;
use crate::models::resource_types::ResourceType;
use crate::models::scim_schema::Schema;
use crate::models::user::User;
use crate::schema_urns;
use crate::utils::validation::{Context, Validate, ValidationError, require_schema_urn};

#[cfg(feature = "filter")]
/// Server-side variant of [`ListQuery`] that tolerates malformed filter
/// expressions so the handler can produce an RFC 7644 §3.12 `invalidFilter`
/// error response instead of aborting deserialization of the whole query.
pub type TolerantListQuery = ListQuery<MaybeFilter>;

#[cfg(feature = "filter")]
/// Server-side variant of [`SearchRequest`] with the same tolerant filter
/// behavior as [`TolerantListQuery`].
pub type TolerantSearchRequest = SearchRequest<MaybeFilter>;

#[cfg(feature = "filter")]
/// [`ListQuery`] with a fully-parsed [`Filter`]. Equivalent to `ListQuery` with
/// its default type parameter; provided as a named alias for symmetry with
/// [`TolerantListQuery`] and as the `Ok` type of
/// [`ListQuery::<MaybeFilter>::into_strict`].
pub type StrictListQuery = ListQuery<Filter>;

#[cfg(feature = "filter")]
/// [`SearchRequest`] with a fully-parsed [`Filter`]. Equivalent to
/// `SearchRequest` with its default type parameter; provided as a named alias
/// for symmetry with [`TolerantSearchRequest`] and as the `Ok` type of
/// [`SearchRequest::<MaybeFilter>::into_strict`].
pub type StrictSearchRequest = SearchRequest<Filter>;

#[cfg(feature = "filter")]
impl TryFrom<TolerantListQuery> for StrictListQuery {
    type Error = InvalidFilterError;

    fn try_from(q: TolerantListQuery) -> Result<Self, Self::Error> {
        let filter = match q.filter {
            None => None,
            Some(MaybeFilter::Valid(f)) => Some(f),
            Some(MaybeFilter::Invalid(err)) => return Err(err),
        };
        Ok(ListQuery {
            filter,
            sort_by: q.sort_by,
            sort_order: q.sort_order,
            start_index: q.start_index,
            count: q.count,
            attributes: q.attributes,
            excluded_attributes: q.excluded_attributes,
        })
    }
}

#[cfg(feature = "filter")]
impl TryFrom<TolerantSearchRequest> for StrictSearchRequest {
    type Error = InvalidFilterError;

    fn try_from(r: TolerantSearchRequest) -> Result<Self, Self::Error> {
        let filter = match r.filter {
            None => None,
            Some(MaybeFilter::Valid(f)) => Some(f),
            Some(MaybeFilter::Invalid(err)) => return Err(err),
        };
        Ok(SearchRequest {
            schemas: r.schemas,
            attributes: r.attributes,
            excluded_attributes: r.excluded_attributes,
            filter,
            sort_by: r.sort_by,
            sort_order: r.sort_order,
            start_index: r.start_index,
            count: r.count,
        })
    }
}

#[cfg(feature = "filter")]
impl ListQuery<MaybeFilter> {
    /// Convert a [`TolerantListQuery`] into a [`StrictListQuery`], failing if
    /// the embedded filter is [`MaybeFilter::Invalid`].
    ///
    /// Callers typically pair this with `.map_err(...)` to turn the
    /// [`InvalidFilterError`] into their own RFC 7644 §3.12 `invalidFilter`
    /// error response.
    pub fn into_strict(self) -> Result<StrictListQuery, InvalidFilterError> {
        StrictListQuery::try_from(self)
    }
}

#[cfg(feature = "filter")]
impl SearchRequest<MaybeFilter> {
    /// Convert a [`TolerantSearchRequest`] into a [`StrictSearchRequest`],
    /// failing if the embedded filter is [`MaybeFilter::Invalid`].
    ///
    /// Callers typically pair this with `.map_err(...)` to turn the
    /// [`InvalidFilterError`] into their own RFC 7644 §3.12 `invalidFilter`
    /// error response.
    pub fn into_strict(self) -> Result<StrictSearchRequest, InvalidFilterError> {
        StrictSearchRequest::try_from(self)
    }
}

/// RFC 7644 §3.4.2.3 `sortOrder`. "Allowed values are `ascending` and
/// `descending`. If a value for `sortBy` is provided and no `sortOrder` is
/// specified, `sortOrder` SHALL default to ascending."
///
/// Exhaustive on purpose: the RFC closes the set. Deserialization is
/// case-insensitive, matching the crate's posture on provider spelling.
#[cfg(feature = "filter")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    #[default]
    #[serde(alias = "Ascending", alias = "ASCENDING")]
    Ascending,
    #[serde(alias = "Descending", alias = "DESCENDING")]
    Descending,
}

#[cfg(feature = "filter")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest<F = Filter> {
    pub schemas: Vec<String>,
    /// RFC 7644 §3.9 attribute selection, not a resource attribute — so unlike
    /// the multi-valued attributes on `User` and `Group`, an empty list is
    /// omitted rather than sent as `[]`. §3.5.1\'s "an empty array … for a multi-valued attribute, to clear all
    /// values" is about a resource's own attributes; an empty *selection* is
    /// not an assertion about anything, and a server could reasonably read
    /// `"attributes": []` as a request for no attributes at all.
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attributes: Vec<String>,
    /// RFC 7644 §3.9 attribute selection, not a resource attribute — so unlike
    /// the multi-valued attributes on `User` and `Group`, an empty list is
    /// omitted rather than sent as `[]`. §3.5.1\'s "an empty array … for a multi-valued attribute, to clear all
    /// values" is about a resource's own attributes; an empty *selection* is
    /// not an assertion about anything, and a server could reasonably read
    /// `"attributes": []` as a request for no attributes at all.
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub excluded_attributes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<F>,
    /// RFC 7644 §3.4.2.3: the attribute whose value orders the results,
    /// e.g. `userName` or `name.familyName`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// RFC 7644 §3.4.2.3. When `sort_by` is set and this is `None`, the
    /// server SHALL treat it as [`SortOrder::Ascending`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    /// RFC 7644 §3.4.2.4 Table 6: 1-based; "a value less than 1 SHALL be
    /// interpreted as 1", so an out-of-range value is not an error — see
    /// [`effective_start_index`](Self::effective_start_index).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    /// RFC 7644 §3.4.2.4 Table 6: "a negative value SHALL be interpreted as
    /// 0", so it is not an error — see [`effective_count`](Self::effective_count).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

#[cfg(feature = "filter")]
impl<F> SearchRequest<F> {
    /// `startIndex` as RFC 7644 §3.4.2.4 Table 6 has a server read it: the
    /// default 1 when absent, and 1 for any value below it.
    pub fn effective_start_index(&self) -> i64 {
        self.start_index.map_or(1, |s| s.max(1))
    }

    /// `count` as Table 6 has a server read it: `None` when absent (the
    /// server's own maximum applies), and 0 for a negative value, which the
    /// table says "indicates that no resource results are to be returned
    /// except for totalResults".
    pub fn effective_count(&self) -> Option<i64> {
        self.count.map(|c| c.max(0))
    }
}

#[cfg(feature = "filter")]
impl<F> Default for SearchRequest<F> {
    fn default() -> Self {
        SearchRequest {
            schemas: vec![schema_urns::SEARCH_REQUEST.to_string()],
            attributes: Vec::new(),
            excluded_attributes: Vec::new(),
            filter: None,
            sort_by: None,
            sort_order: None,
            start_index: Some(1),
            count: Some(100),
        }
    }
}

#[cfg(feature = "filter")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery<F = Filter> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<F>,
    /// RFC 7644 §3.4.2.3 `sortBy` query parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// RFC 7644 §3.4.2.3 `sortOrder`; defaults to ascending when `sort_by` is
    /// set and this is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    /// RFC 7644 §3.4.2.4 Table 6: "a value less than 1 SHALL be interpreted
    /// as 1" — see [`effective_start_index`](Self::effective_start_index).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    /// RFC 7644 §3.4.2.4 Table 6: "a negative value SHALL be interpreted as
    /// 0" — see [`effective_count`](Self::effective_count).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// RFC 7644 §3.9: a comma-separated list of attribute names to return,
    /// "mutually exclusive" with `excludedAttributes`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// RFC 7644 §3.9: a comma-separated list of attribute names to omit,
    /// mutually exclusive with `attributes`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_attributes: Option<String>,
}

#[cfg(feature = "filter")]
impl<F> ListQuery<F> {
    /// `startIndex` as RFC 7644 §3.4.2.4 Table 6 has a server read it: the
    /// default 1 when absent, and 1 for any value below it.
    pub fn effective_start_index(&self) -> i64 {
        self.start_index.map_or(1, |s| s.max(1))
    }

    /// `count` as Table 6 has a server read it: `None` when absent, and 0 for
    /// a negative value.
    pub fn effective_count(&self) -> Option<i64> {
        self.count.map(|c| c.max(0))
    }
}

#[cfg(feature = "filter")]
impl<F> Default for ListQuery<F> {
    fn default() -> Self {
        ListQuery {
            filter: None,
            sort_by: None,
            sort_order: None,
            start_index: Some(1),
            count: Some(100),
            // Omitted, not empty: RFC 7644 §3.9 makes the two mutually
            // exclusive, so a default carrying both — as empty strings that
            // select nothing — is a query the crate's own `validate` would
            // have to reject.
            attributes: None,
            excluded_attributes: None,
        }
    }
}

/// Heterogeneous SCIM resource type used inside [`ListResponse`].
///
/// Deserialization dispatches on the SCIM schema URN carried in the payload's
/// `schemas` attribute (RFC 7643 §3). `Schema` and `ResourceType` resources,
/// which are often served without a `schemas` field — RFC 7643's own §8.7
/// schema representations carry none — are disambiguated by structural
/// markers: the `attributes` array (Schema) or
/// the `endpoint` + `schema` fields (ResourceType). Payloads that do not
/// carry a recognized discriminator are rejected rather than silently
/// classified, to prevent type confusion.
/// `#[non_exhaustive]`: RFC 7643 §6 lets a server define resource types
/// beyond the four this crate models, so variants will be added in minor
/// releases. Match with a trailing `_ =>` arm.
#[non_exhaustive]
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Resource<T> {
    User(Box<User<T>>),
    Schema(Box<Schema>),
    Group(Box<Group<T>>),
    ResourceType(Box<ResourceType>),
}

impl<'de, T> Deserialize<'de> for Resource<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = serde_json::Map::deserialize(deserializer)?;
        let value = Value::Object(map);

        let schemas_iter = value
            .get("schemas")
            .map(|v| {
                let schema_array = v.as_array().ok_or_else(|| {
                    serde::de::Error::custom("\"schemas\" must be a JSON array of strings")
                })?;

                let iter = schema_array.iter().map(|item| {
                    item.as_str().ok_or_else(|| {
                        serde::de::Error::custom("\"schemas\" entries must be strings")
                    })
                });
                Ok(iter)
            })
            .transpose()?;

        if let Some(urns) = schemas_iter {
            let mut total: usize = 0;
            let mut is_user = false;
            let mut is_group = false;
            let mut is_schema = false;
            let mut is_resource_type = false;
            for urn in urns {
                let urn = urn?;
                total += 1;
                match urn {
                    schema_urns::ENTERPRISE_USER | schema_urns::USER => is_user = true,
                    schema_urns::GROUP => is_group = true,
                    schema_urns::SCHEMA => is_schema = true,
                    schema_urns::RESOURCE_TYPE => is_resource_type = true,
                    _ => continue,
                }
            }
            if total == 0 {
                return Err(serde::de::Error::custom(
                    "\"schemas\" array is empty; cannot determine SCIM resource type",
                ));
            }
            let matched = is_user as u8 + is_group as u8 + is_schema as u8 + is_resource_type as u8;
            if matched > 1 {
                return Err(serde::de::Error::custom(
                    "ambiguous schemas: multiple SCIM resource-type URNs present",
                ));
            }

            if is_user {
                return serde_json::from_value::<User<T>>(value)
                    .map(|u| Resource::User(Box::new(u)))
                    .map_err(serde::de::Error::custom);
            } else if is_group {
                return serde_json::from_value::<Group<T>>(value)
                    .map(|g| Resource::Group(Box::new(g)))
                    .map_err(serde::de::Error::custom);
            } else if is_schema {
                return serde_json::from_value::<Schema>(value)
                    .map(|s| Resource::Schema(Box::new(s)))
                    .map_err(serde::de::Error::custom);
            } else if is_resource_type {
                return serde_json::from_value::<ResourceType>(value)
                    .map(|r| Resource::ResourceType(Box::new(r)))
                    .map_err(serde::de::Error::custom);
            }

            return Err(serde::de::Error::custom(
                "\"schemas\" contains no recognized SCIM resource-type URN",
            ));
        }

        // No "schemas" field. Schema and ResourceType resources are seen
        // without one in practice — RFC 7643's own §8.7 schema representations
        // carry none — so they get a structural fallback. User and Group MUST
        // carry their URN and are not eligible for it.
        let has_attributes = value
            .get("attributes")
            .map(Value::is_array)
            .unwrap_or(false);
        let has_endpoint = value.get("endpoint").map(Value::is_string).unwrap_or(false);
        let has_schema_field = value.get("schema").map(Value::is_string).unwrap_or(false);

        if has_attributes {
            return serde_json::from_value::<Schema>(value)
                .map(|s| Resource::Schema(Box::new(s)))
                .map_err(serde::de::Error::custom);
        }
        if has_endpoint && has_schema_field {
            return serde_json::from_value::<ResourceType>(value)
                .map(|r| Resource::ResourceType(Box::new(r)))
                .map_err(serde::de::Error::custom);
        }

        Err(serde::de::Error::custom(
            "cannot determine SCIM resource type: missing \"schemas\" field and no structural discriminator",
        ))
    }
}

mod sealed {
    /// Private supertrait of [`ScimResource`]. Nothing outside this crate can
    /// name it, so nothing outside this crate can implement `ScimResource`.
    pub trait Sealed {}
}

/// A resource that may appear in the `Resources` array of a
/// [`ListResponse`].
///
/// **Sealed.** [`ScimResource`] has a private supertrait, so the implementing
/// set is fixed by this crate: [`User`], [`Group`], [`Schema`],
/// [`ResourceType`], and [`Resource`] for the heterogeneous case. The bound
/// exists to make `ListResponse<R>` reject a nonsense `R` at compile time —
/// before 1.0 the type parameter was the *id* type, so `ListResponse<Resource<String>>`
/// meant "ids are strings"; it now means "resources are strings", which the
/// sealing turns into a compile error rather than a runtime deserialization
/// failure.
///
/// `EnterpriseUser` is deliberately absent: RFC 7643 §4.3 makes it a schema
/// *extension* carried inside a `User`, never a resource in its own right.
/// `ServiceProviderConfig` is absent because RFC 7643 §5 serves it as a
/// singleton, not a list.
///
/// # The bound is what makes the 1.0 change safe
///
/// `ListResponse<String>` compiled before 1.0 and meant "ids are strings".
/// Reusing the parameter for the resource would have left that code compiling
/// with a new meaning and failing only at runtime, on deserialize. The bound
/// turns it into a compile error instead. Each snippet below inlines the full
/// path deliberately: a `use` line could fail for an unrelated reason — a
/// renamed module, a moved re-export — and `compile_fail` would still pass
/// while the bound it is guarding had been relaxed.
///
/// Doctest error-code annotations are inert on stable, so each snippet below is
/// kept to the single statement whose failure it is meant to prove, and names
/// the mutation it catches.
///
/// Catches: relaxing `R: ScimResource` on `ListResponse`.
///
/// ```compile_fail
/// // `String` is not a SCIM resource, so this does not compile.
/// let _list: scim_v2::models::others::ListResponse<String> = unimplemented!();
/// ```
///
/// The sealing is why no downstream crate can widen that set. Catches:
/// removing `: sealed::Sealed` from the trait. The impl is complete on purpose
/// — with `declared_schemas` missing it failed for that reason instead and
/// proved nothing about the seal.
///
/// ```compile_fail
/// struct MyResource;
/// // `sealed::Sealed` is private to scim_v2, so this cannot be satisfied.
/// impl scim_v2::models::others::ScimResource for MyResource {
///     fn schema_urn(&self) -> &'static str { "urn:example" }
///     fn declared_schemas(&self) -> &[String] { &[] }
/// }
/// ```
///
/// And the seal holds because the supertrait's module is private. Naming it
/// from outside is an E0603, which is what keeps a later `pub mod sealed` —
/// added to quiet a `private_interfaces` warning, say — from silently
/// unsealing the trait: this snippet starts compiling, and therefore starts
/// failing, the moment the module is public.
///
/// Catches: `pub mod sealed`.
///
/// ```compile_fail
/// struct MyResource;
/// impl scim_v2::models::others::sealed::Sealed for MyResource {}
/// ```
pub trait ScimResource: sealed::Sealed + Validate {
    /// The RFC 7643 schema URN for this resource's type.
    fn schema_urn(&self) -> &'static str;

    /// The `schemas` attribute this particular instance carries on the wire.
    ///
    /// Empty when the payload omitted it, as discovery payloads sometimes do
    /// (RFC 7643's own §8.7 schema representations carry none); whether that
    /// is acceptable is each resource's own [`Validate`] decision.
    /// [`ListResponse::validate`] compares a non-empty value against
    /// [`schema_urn`](ScimResource::schema_urn) so the typed path performs the
    /// same discriminator check the [`Resource`] deserializer does.
    fn declared_schemas(&self) -> &[String];
}

impl<T> sealed::Sealed for User<T> {}
impl<T: std::fmt::Display> ScimResource for User<T> {
    fn declared_schemas(&self) -> &[String] {
        &self.schemas
    }

    fn schema_urn(&self) -> &'static str {
        schema_urns::USER
    }
}

impl<T> sealed::Sealed for Group<T> {}
impl<T: std::fmt::Display> ScimResource for Group<T> {
    fn declared_schemas(&self) -> &[String] {
        &self.schemas
    }

    fn schema_urn(&self) -> &'static str {
        schema_urns::GROUP
    }
}

impl sealed::Sealed for Schema {}
impl ScimResource for Schema {
    fn declared_schemas(&self) -> &[String] {
        &self.schemas
    }

    fn schema_urn(&self) -> &'static str {
        schema_urns::SCHEMA
    }
}

impl sealed::Sealed for ResourceType {}
impl ScimResource for ResourceType {
    fn declared_schemas(&self) -> &[String] {
        &self.schemas
    }

    fn schema_urn(&self) -> &'static str {
        schema_urns::RESOURCE_TYPE
    }
}

impl<T> sealed::Sealed for Resource<T> {}
impl<T: std::fmt::Display> Validate for Resource<T> {
    /// Delegates to the wrapped resource's own rules.
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Resource::User(u) => u.validate(),
            Resource::Group(g) => g.validate(),
            Resource::Schema(s) => s.validate(),
            Resource::ResourceType(r) => r.validate(),
        }
    }

    fn validate_context(&self, ctx: Context) -> Result<(), ValidationError> {
        match self {
            Resource::User(u) => u.validate_context(ctx),
            Resource::Group(g) => g.validate_context(ctx),
            Resource::Schema(s) => s.validate_context(ctx),
            Resource::ResourceType(r) => r.validate_context(ctx),
        }
    }
}

impl<T: std::fmt::Display> ScimResource for Resource<T> {
    fn declared_schemas(&self) -> &[String] {
        match self {
            Resource::User(u) => &u.schemas,
            Resource::Group(g) => &g.schemas,
            Resource::Schema(s) => &s.schemas,
            Resource::ResourceType(r) => &r.schemas,
        }
    }

    fn schema_urn(&self) -> &'static str {
        match self {
            Resource::User(_) => schema_urns::USER,
            Resource::Group(_) => schema_urns::GROUP,
            Resource::Schema(_) => schema_urns::SCHEMA,
            Resource::ResourceType(_) => schema_urns::RESOURCE_TYPE,
        }
    }
}

/// RFC 7644 §3.4.2 `ListResponse`.
///
/// Generic over the resource, not over the id type. Use a concrete resource
/// when the endpoint returns one kind — `GET /Users` is
/// `ListResponse<User<String>>`, deserializing straight into
/// `Vec<User<String>>` with no enum to match through and one allocation
/// instead of one per resource. Use [`Resource`] for the heterogeneous case,
/// which §3.4.3 allows when querying the root `/.search` endpoint:
/// `ListResponse<Resource<String>>`.
///
/// `R` is bounded by the sealed [`ScimResource`] trait, so a type that is not
/// a SCIM resource is rejected at compile time.
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse<R: ScimResource = Resource<String>> {
    /// RFC 7644 §3.4.2: REQUIRED when partial results are returned due to
    /// pagination; omitted otherwise. Not enforced by the type — see
    /// [`ListResponse::validate`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_per_page: Option<i64>,
    pub total_results: i64,
    /// RFC 7644 §3.4.2: REQUIRED when partial results are returned due to
    /// pagination; omitted otherwise. Not enforced by the type — see
    /// [`ListResponse::validate`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    pub schemas: Vec<String>,
    /// RFC 7644 §3.4.2: "REQUIRED if "totalResults" is non-zero", so a query
    /// returning no matches may omit it. Absence *with* a non-zero
    /// `totalResults` is refused during deserialization rather than by
    /// [`validate`](Validate::validate), because presence is a fact about the
    /// JSON and not about the value: an omitted member, `null` and `[]` all
    /// have to land on the same empty `Vec` for the type to stay usable, and
    /// carrying a hidden presence flag would make this struct
    /// unconstructible by a literal. `OperationTarget` refuses a pathless
    /// operation with no `value` at the same boundary and for the same
    /// reason.
    #[serde(rename = "Resources")]
    pub resources: Vec<R>,
}

/// The wire shape of [`ListResponse`], with `Resources` presence preserved
/// so the RFC 7644 §3.4.2 REQUIRED can be checked before it is erased.
#[derive(Deserialize)]
#[serde(
    rename_all = "camelCase",
    bound(deserialize = "R: ScimResource + DeserializeOwned")
)]
struct ListResponseWire<R: ScimResource> {
    items_per_page: Option<i64>,
    total_results: i64,
    start_index: Option<i64>,
    schemas: Vec<String>,
    #[serde(
        rename = "Resources",
        default,
        deserialize_with = "deserialize_present_resources"
    )]
    resources: Option<Vec<R>>,
}

/// `Resources` as it appeared: `None` only when the member was absent, since
/// `null` is a form providers send for an empty page (RFC 7643 §2.5 makes it
/// equivalent to `[]`).
fn deserialize_present_resources<'de, D, R>(deserializer: D) -> Result<Option<Vec<R>>, D::Error>
where
    D: Deserializer<'de>,
    R: DeserializeOwned,
{
    Ok(Some(
        Option::<Vec<R>>::deserialize(deserializer)?.unwrap_or_default(),
    ))
}

impl<'de, R> Deserialize<'de> for ListResponse<R>
where
    R: ScimResource + DeserializeOwned,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = ListResponseWire::<R>::deserialize(deserializer)?;
        // §3.4.2: `Resources` is "REQUIRED if "totalResults" is non-zero".
        // A page claiming matches and carrying none is not a page a caller can
        // act on, and after this point the absence is indistinguishable from
        // a conformant empty `[]`.
        let resources = match wire.resources {
            Some(resources) => resources,
            None if wire.total_results == 0 => Vec::new(),
            None => return Err(serde::de::Error::missing_field("Resources")),
        };
        Ok(ListResponse {
            items_per_page: wire.items_per_page,
            total_results: wire.total_results,
            start_index: wire.start_index,
            schemas: wire.schemas,
            resources,
        })
    }
}

impl<R: ScimResource> Validate for ListResponse<R> {
    /// Validates a `ListResponse` against RFC 7644 §3.4.2.
    ///
    /// Checks performed:
    ///
    /// * `schemas` is present.
    /// * `totalResults` is not negative, and is not smaller than the number of
    ///   entries in `Resources` — §3.4.2 describes it as a count that "may be
    ///   larger than the number of resources returned".
    /// * A present `startIndex` is at least 1 (§3.4.2: "the 1-based index of
    ///   the first result") and a present `itemsPerPage` is not negative.
    /// * Every resource declares the schema of the type it was parsed as, and
    ///   passes its own [`Validate::validate`]; a failure is reported under
    ///   `Resources[i]`. [`validate_as`](Validate::validate_as) carries the
    ///   [`Context`] into each resource too, so a `Response` page needs an
    ///   `id` on every entry.
    /// * When the response carries a *partial* result set — fewer entries in
    ///   `Resources` than `totalResults` — both `startIndex` and `itemsPerPage`
    ///   are present. §3.4.2 makes them REQUIRED "when partial results are
    ///   returned due to pagination", and pagination is the only mechanism
    ///   §3.4.2 gives for returning fewer resources than `totalResults`, so a
    ///   short page without markers is treated as non-conformant rather than
    ///   as an unpaginated page of unexplained size.
    ///
    /// This last invariant cannot be encoded in the type: `startIndex` and
    /// `itemsPerPage` are `Option<i64>` so that a wire `0` stays distinct from
    /// an omitted field, which leaves a paginated-but-incomplete response
    /// expressible. Call this before serializing a response assembled by hand.
    ///
    /// # Returns
    ///
    /// * `Ok(())` — the response is conformant.
    /// * `Err(ValidationError)` — `schemas` is empty, or a pagination marker is
    ///   absent from a partial result set. The error names the offending
    ///   attribute by its wire path.
    ///
    /// # Example
    ///
    /// A homogeneous page from `GET /Users` deserializes straight into
    /// `Vec<User<String>>`, with no enum to match through:
    ///
    /// ```
    /// use scim_v2::{Validate, models::{others::ListResponse, user::User}};
    ///
    /// let body = r#"{
    ///   "schemas": ["urn:ietf:params:scim:api:messages:2.0:ListResponse"],
    ///   "totalResults": 2,
    ///   "startIndex": 1,
    ///   "itemsPerPage": 2,
    ///   "Resources": [
    ///     {"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"bjensen"},
    ///     {"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"jsmith"}
    ///   ]
    /// }"#;
    ///
    /// let list: ListResponse<User<String>> = serde_json::from_str(body)?;
    /// list.validate()?;
    /// assert_eq!(list.resources[0].user_name, "bjensen");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// A partial page that omits its pagination markers is rejected:
    ///
    /// ```
    /// use scim_v2::{Validate, models::{others::{ListResponse, Resource}, user::User}};
    ///
    /// let list: ListResponse<Resource<String>> = ListResponse {
    ///     schemas: vec!["urn:ietf:params:scim:api:messages:2.0:ListResponse".to_string()],
    ///     total_results: 100,
    ///     items_per_page: None,
    ///     start_index: None,
    ///     resources: vec![],
    /// };
    ///
    /// let err = list.validate().unwrap_err();
    /// assert_eq!(err.path(), "startIndex");
    /// ```
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, schema_urns::LIST_RESPONSE)?;

        // §3.4.2 describes `totalResults` as a count that "may be larger than
        // the number of resources returned" — larger, never smaller, and never
        // negative. Both directions matter, because the doc above presents this
        // as the check to run on a response assembled by hand, and filling
        // `Resources` while leaving `totalResults` at its default is the
        // mistake that shape invites.
        if self.total_results < 0 {
            return Err(ValidationError::invalid_value(
                "totalResults",
                format!("must not be negative, got {}", self.total_results),
            ));
        }
        let returned = self.resources.len() as i64;
        if returned > self.total_results {
            return Err(ValidationError::invalid_value(
                "totalResults",
                format!(
                    "{returned} resources returned but totalResults is {}",
                    self.total_results
                ),
            ));
        }

        // A short page — fewer `Resources` returned than the total that match —
        // is "partial results ... returned due to pagination" per §3.4.2, which
        // makes both pagination markers REQUIRED. §3.4.2 offers no other reason
        // for a short page: `Resources` "MAY be a subset of the full set of
        // resources if pagination is requested", `totalResults` "may be larger
        // than the number of resources returned, such as when returning a
        // single page", and Table 6 makes `count` — including a server-side
        // default — a pagination parameter. A short page is a paginated page.
        if returned < self.total_results {
            if self.start_index.is_none() {
                return Err(ValidationError::missing_required("startIndex"));
            }
            if self.items_per_page.is_none() {
                return Err(ValidationError::missing_required("itemsPerPage"));
            }
        }

        // The typed path chooses `R` at the call site, so nothing has compared
        // the payload's declared `schemas` against the type it was parsed as.
        // `Resource`'s own deserializer dispatches on that URN specifically
        // "to prevent type confusion"; without this, a Group payload carrying
        // a `userName` deserialized cleanly into `ListResponse<User<String>>`
        // and both validators returned Ok. Then each resource's own rules, with
        // its index in the path: a page is not conformant if what it carries
        // is not (whether an *absent* `schemas` is acceptable is decided
        // there — only `Schema` tolerates it, after RFC 7643 §8.7's examples).
        for (i, resource) in self.resources.iter().enumerate() {
            let declared = resource.declared_schemas();
            if !declared.is_empty() {
                let expected = resource.schema_urn();
                if !declared.iter().any(|s| s == expected) {
                    return Err(ValidationError::invalid_value(
                        format!("Resources[{i}].schemas"),
                        format!(
                            "does not declare {expected}, the schema of the type it was parsed as"
                        ),
                    ));
                }
            }
            resource
                .validate()
                .map_err(|e| e.under(&format!("Resources[{i}]")))?;
        }

        // §3.4.2 defines `startIndex` as "The 1-based index of the first
        // result". A present-but-nonsensical marker is worth catching, and
        // this is the only place it can be: the fields deliberately keep a
        // wire `0` distinct from an omitted field, so serde will not reject it.
        // No `let` chains: they stabilised in Rust 1.88 and the MSRV is 1.86.
        if let Some(start) = self.start_index.filter(|s| *s < 1) {
            return Err(ValidationError::invalid_value(
                "startIndex",
                format!("is 1-based, got {start}"),
            ));
        }
        if let Some(per_page) = self.items_per_page.filter(|p| *p < 0) {
            return Err(ValidationError::invalid_value(
                "itemsPerPage",
                format!("must not be negative, got {per_page}"),
            ));
        }

        Ok(())
    }

    /// The direction applies to every resource on the page: for
    /// [`Context::Response`] each one needs its `id` (RFC 7643 §3.1) and may
    /// not carry `password`. A list response is only ever a response, but the
    /// context is the caller's to state, as everywhere else.
    fn validate_context(&self, ctx: Context) -> Result<(), ValidationError> {
        for (i, resource) in self.resources.iter().enumerate() {
            resource
                .validate_context(ctx)
                .map_err(|e| e.under(&format!("Resources[{i}]")))?;
        }
        Ok(())
    }
}

#[cfg(feature = "filter")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PatchOp {
    pub schemas: Vec<String>,
    #[serde(rename = "Operations")]
    pub operations: Vec<PatchOperation>,
}

#[cfg(feature = "filter")]
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
#[expect(clippy::large_enum_variant)]
pub enum OperationTarget {
    WithPath {
        path: PatchPath,
        /// `None` when the operation carried no `value` member at all, which
        /// an `add` may not do — RFC 7644 §3.5.2.1: "The operation MUST
        /// contain a "value" member whose content specifies the value to be
        /// added". Kept distinct from `Some(Value::Null)` so the omission is
        /// reportable by [`PatchOp::validate`] rather than collapsed into an
        /// explicit null on the way in and emitted as one on the way out.
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<Value>,
    },
    WithoutPath {
        value: serde_json::Map<String, Value>,
    },
}

#[cfg(feature = "filter")]
impl<'de> Deserialize<'de> for OperationTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;

        if let Some(path_value) = map.remove("path") {
            let path_str = path_value
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("\"path\" must be a string"))?;
            let path: PatchPath = path_str
                .parse()
                .map_err(|e| serde::de::Error::custom(format!("invalid SCIM path: {e}")))?;
            let value = map.remove("value");
            Ok(OperationTarget::WithPath { path, value })
        } else {
            let value = match map.remove("value") {
                Some(Value::Object(m)) => m,
                Some(_) => {
                    return Err(serde::de::Error::custom(
                        "\"value\" must be a JSON object when \"path\" is absent",
                    ));
                }
                None => return Err(serde::de::Error::missing_field("value")),
            };
            Ok(OperationTarget::WithoutPath { value })
        }
    }
}

#[cfg(feature = "filter")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "op")]
pub enum PatchOperation {
    #[serde(rename = "add", alias = "Add", alias = "ADD")]
    Add(OperationTarget),
    #[serde(rename = "remove", alias = "Remove", alias = "REMOVE")]
    Remove {
        path: PatchPath,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<Value>,
    },
    #[serde(rename = "replace", alias = "Replace", alias = "REPLACE")]
    Replace(OperationTarget),
}

/// Whether `s` is RFC 7644 §3.10 standard attribute notation — an `attrPath`,
/// optionally URN-prefixed, with at most one sub-attribute. Decided by the
/// crate's own grammar rather than a second-guess at it: `<s> pr` must parse
/// to a bare presence test.
#[cfg(feature = "filter")]
fn is_attribute_path(s: &str) -> bool {
    format!("{s} pr")
        .parse::<Filter>()
        .is_ok_and(|f| matches!(f, Filter::Attr(AttrExp::Present(_))))
}

/// The RFC 7644 §3.4.2.3 sort rules, shared by the two query carriers:
/// `sortOrder` is "the order in which the "sortBy" parameter is applied", so
/// on its own it orders nothing, and §3.4.3 requires that "the "sortBy"
/// attribute MUST be in standard attribute notation (Section 3.10) form".
#[cfg(feature = "filter")]
fn validate_sort(sort_by: Option<&str>, sort_order_present: bool) -> Result<(), ValidationError> {
    if sort_order_present && sort_by.is_none() {
        return Err(ValidationError::invalid_value(
            "sortOrder",
            "present without sortBy; RFC 7644 §3.4.2.3 defines it as the order in which sortBy is applied",
        ));
    }
    if let Some(sort_by) = sort_by.filter(|s| !is_attribute_path(s)) {
        return Err(ValidationError::invalid_value(
            "sortBy",
            format!("{sort_by:?} is not in standard attribute notation (RFC 7644 §3.4.3, §3.10)"),
        ));
    }
    Ok(())
}

/// The RFC 7644 §3.9 attribute-selection rules, shared by the two query
/// carriers. §3.9 offers "either of the mutually exclusive URL query
/// parameters "attributes" or "excludedAttributes"", so a request carrying
/// both asserts two incompatible projections; and §§3.4.2.5 and 3.4.3 both
/// say of each parameter that "Attribute names MUST be in standard attribute
/// notation (Section 3.10) form", so a name a server cannot resolve to an
/// attribute is a client error rather than something to ignore.
///
/// Each selection is passed as its individual names. A wholly empty
/// selection (`?attributes=`, or an empty `Vec`) is treated as absent, which
/// is the only reading that asserts nothing; an empty name *within* a list
/// (`?attributes=userName,,emails`) is malformed and rejected.
#[cfg(feature = "filter")]
fn validate_attribute_selection<'a>(
    attributes: impl Iterator<Item = &'a str>,
    excluded: impl Iterator<Item = &'a str>,
) -> Result<(), ValidationError> {
    fn check_names<'a>(
        path: &str,
        names: impl Iterator<Item = &'a str>,
    ) -> Result<bool, ValidationError> {
        let mut present = false;
        for name in names {
            present = true;
            if !is_attribute_path(name) {
                return Err(ValidationError::invalid_value(
                    path,
                    format!(
                        "{name:?} is not in standard attribute notation (RFC 7644 §3.4.2.5, §3.10)"
                    ),
                ));
            }
        }
        Ok(present)
    }

    let attributes_present = check_names("attributes", attributes)?;
    let excluded_present = check_names("excludedAttributes", excluded)?;
    if attributes_present && excluded_present {
        return Err(ValidationError::invalid_value(
            "excludedAttributes",
            "may not be combined with attributes; RFC 7644 §3.9 makes them mutually exclusive",
        ));
    }
    Ok(())
}

/// The individual names in a `ListQuery`'s comma-separated selection. Empty
/// overall means no selection; an empty entry inside a non-empty list is
/// kept, so [`validate_attribute_selection`] can reject it.
#[cfg(feature = "filter")]
fn selection_names(raw: Option<&str>) -> impl Iterator<Item = &str> {
    raw.filter(|s| !s.is_empty())
        .into_iter()
        .flat_map(|s| s.split(','))
        .map(str::trim)
}

#[cfg(feature = "filter")]
impl<F> Validate for ListQuery<F> {
    /// The `GET` half of the rules [`SearchRequest`] enforces for `POST
    /// /.search`: RFC 7644 §3.4.2.3's sort pair and §3.9's mutually exclusive
    /// attribute selection. There is no `schemas` to check, since these are
    /// query parameters rather than a body. An out-of-range `count` or
    /// `startIndex` is interpreted rather than rejected (§3.4.2.4 Table 6) —
    /// see [`effective_count`](Self::effective_count) and
    /// [`effective_start_index`](Self::effective_start_index).
    fn validate(&self) -> Result<(), ValidationError> {
        validate_sort(self.sort_by.as_deref(), self.sort_order.is_some())?;
        validate_attribute_selection(
            selection_names(self.attributes.as_deref()),
            selection_names(self.excluded_attributes.as_deref()),
        )
    }
}

#[cfg(feature = "filter")]
impl<F> Validate for SearchRequest<F> {
    /// RFC 7644 §3.4.3: the body carries the SearchRequest URN, and "the
    /// sortBy attribute MUST be in standard attribute notation (Section 3.10)
    /// form". §3.4.2.3: `sortOrder` is "the order in which the sortBy
    /// parameter is applied", so without `sortBy` it orders nothing and is
    /// rejected rather than silently ignored. §3.9 calls `attributes` and
    /// `excludedAttributes` "mutually exclusive URL query parameters", so
    /// both at once is rejected. A negative `count` or a `startIndex` below 1
    /// is *not* an error: §3.4.2.4 Table 6 says each "SHALL be interpreted"
    /// as 0 and 1 respectively, which
    /// [`effective_count`](Self::effective_count) and
    /// [`effective_start_index`](Self::effective_start_index) do.
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, schema_urns::SEARCH_REQUEST)?;
        validate_sort(self.sort_by.as_deref(), self.sort_order.is_some())?;
        validate_attribute_selection(
            self.attributes.iter().map(String::as_str),
            self.excluded_attributes.iter().map(String::as_str),
        )
    }
}

#[cfg(feature = "filter")]
impl Validate for PatchOp {
    /// RFC 7644 §3.5.2: the body carries the PatchOp URN and "an array of one
    /// or more PATCH operations". Most of each operation's shape — `remove`
    /// needs a path, a pathless `add`/`replace` needs an object — is enforced
    /// by the [`PatchOperation`] type. What the type cannot enforce is
    /// §3.5.2.1's "The operation MUST contain a "value" member whose content
    /// specifies the value to be added", since a path-carrying operation may
    /// legitimately omit `value` when it is a `remove`.
    ///
    /// A path-carrying `replace` is held to the same rule. §3.5.2.3 has no
    /// single blanket MUST, but it opens with "The "replace" operation
    /// replaces the value at the target location specified by the "path"" and
    /// then requires the member twice: "the "value" attribute SHALL contain a
    /// list of one or more attributes that are to be replaced" for the
    /// pathless form, and "a set of sub-attributes SHALL be specified in the
    /// "value" parameter" where the path names a complex attribute. Telling a
    /// complex target from a single-valued one needs the resource's schema,
    /// which this crate does not consult, and no reading of §3.5.2.3 gives a
    /// valueless replace a defined outcome — so the check is uniform.
    ///
    /// An explicit `"value": null` counts as present, which is why the field
    /// distinguishes the two: what a null means for the targeted attribute is
    /// the server's decision, and absence is not.
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, schema_urns::PATCH_OP)?;
        if self.operations.is_empty() {
            return Err(ValidationError::invalid_value(
                "Operations",
                "must contain at least one operation (RFC 7644 §3.5.2)",
            ));
        }
        for (i, operation) in self.operations.iter().enumerate() {
            if matches!(
                operation,
                PatchOperation::Add(OperationTarget::WithPath { value: None, .. })
                    | PatchOperation::Replace(OperationTarget::WithPath { value: None, .. })
            ) {
                return Err(ValidationError::missing_required(format!(
                    "Operations[{i}].value"
                )));
            }
        }
        Ok(())
    }
}

/// Tests for the parts of this module that do not depend on the `filter`
/// feature — [`Resource`], [`ListResponse`] and [`ScimResource`] — so they
/// still run under `--no-default-features --features models`.
#[cfg(test)]
mod resource_tests;

#[cfg(all(test, feature = "filter"))]
mod tests;
