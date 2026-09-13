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
/// Server-side variant of [`ListQuery`] that tolerates a malformed filter
/// expression. This tolerance lets a handler return an RFC 7644 §3.12
/// `invalidFilter` error response. Without it, a malformed filter would
/// abort deserialization of the whole query.
pub type TolerantListQuery = ListQuery<MaybeFilter>;

#[cfg(feature = "filter")]
/// Server-side variant of [`SearchRequest`] with the same tolerant filter
/// behavior as [`TolerantListQuery`].
pub type TolerantSearchRequest = SearchRequest<MaybeFilter>;

#[cfg(feature = "filter")]
/// [`ListQuery`] with a fully-parsed [`Filter`]. This type is equivalent to
/// `ListQuery` with its default type parameter. The crate provides it as a
/// named alias for symmetry with [`TolerantListQuery`]. It is also the
/// `Ok` type of [`ListQuery::<MaybeFilter>::into_strict`].
pub type StrictListQuery = ListQuery<Filter>;

#[cfg(feature = "filter")]
/// [`SearchRequest`] with a fully-parsed [`Filter`]. This type is
/// equivalent to `SearchRequest` with its default type parameter. The
/// crate provides it as a named alias for symmetry with
/// [`TolerantSearchRequest`]. It is also the `Ok` type of
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
    /// Converts a [`TolerantListQuery`] into a [`StrictListQuery`]. The
    /// method fails when the embedded filter is [`MaybeFilter::Invalid`].
    ///
    /// A caller typically pairs this method with `.map_err(...)`. That
    /// pairing turns the [`InvalidFilterError`] into the caller's own RFC
    /// 7644 §3.12 `invalidFilter` error response.
    pub fn into_strict(self) -> Result<StrictListQuery, InvalidFilterError> {
        StrictListQuery::try_from(self)
    }
}

#[cfg(feature = "filter")]
impl SearchRequest<MaybeFilter> {
    /// Converts a [`TolerantSearchRequest`] into a [`StrictSearchRequest`].
    /// The method fails when the embedded filter is
    /// [`MaybeFilter::Invalid`].
    ///
    /// A caller typically pairs this method with `.map_err(...)`. That
    /// pairing turns the [`InvalidFilterError`] into the caller's own RFC
    /// 7644 §3.12 `invalidFilter` error response.
    pub fn into_strict(self) -> Result<StrictSearchRequest, InvalidFilterError> {
        StrictSearchRequest::try_from(self)
    }
}

/// RFC 7644 §3.4.2.3 `sortOrder`. "Allowed values are `ascending` and
/// `descending`. If a value for `sortBy` is provided and no `sortOrder` is
/// specified, `sortOrder` SHALL default to ascending."
///
/// This enum is exhaustive. The RFC closes the set of values.
/// Deserialization is case-insensitive. This choice matches the crate's
/// posture on provider spelling.
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
    /// RFC 7644 §3.9 defines this field as attribute selection, not a
    /// resource attribute. The multi-valued attributes on `User` and
    /// `Group` send an empty list as `[]`. This field omits an empty list
    /// instead. RFC 7644 §3.5.1 says "an empty array … for a multi-valued
    /// attribute, to clear all values". That sentence is about a
    /// resource's own attributes. An empty *selection* asserts nothing. A
    /// server could reasonably read `"attributes": []` as a request for no
    /// attributes at all.
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attributes: Vec<String>,
    /// RFC 7644 §3.9 defines this field as attribute selection, not a
    /// resource attribute. The multi-valued attributes on `User` and
    /// `Group` send an empty list as `[]`. This field omits an empty list
    /// instead. RFC 7644 §3.5.1 says "an empty array … for a multi-valued
    /// attribute, to clear all values". That sentence is about a
    /// resource's own attributes. An empty *selection* asserts nothing. A
    /// server could reasonably read `"attributes": []` as a request for no
    /// attributes at all.
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub excluded_attributes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<F>,
    /// RFC 7644 §3.4.2.3: the attribute whose value orders the results, for
    /// example `userName` or `name.familyName`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// RFC 7644 §3.4.2.3. When `sort_by` is set and this is `None`, the
    /// server SHALL treat it as [`SortOrder::Ascending`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    /// RFC 7644 §3.4.2.4 Table 6 makes this value 1-based. The table says "a
    /// value less than 1 SHALL be interpreted as 1". An out-of-range value
    /// is not an error. See
    /// [`effective_start_index`](Self::effective_start_index).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    /// RFC 7644 §3.4.2.4 Table 6 says "a negative value SHALL be
    /// interpreted as 0". A negative value is not an error. See
    /// [`effective_count`](Self::effective_count).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

#[cfg(feature = "filter")]
impl<F> SearchRequest<F> {
    /// Reads `startIndex` the way RFC 7644 §3.4.2.4 Table 6 has a server
    /// read it. The default is 1 when the value is absent. The result is 1
    /// for any value below 1.
    pub fn effective_start_index(&self) -> i64 {
        self.start_index.map_or(1, |s| s.max(1))
    }

    /// Reads `count` the way Table 6 has a server read it. The result is
    /// `None` when the value is absent. The server's own maximum then
    /// applies. The result is 0 for a negative value. Table 6 says a 0
    /// count "indicates that no resource results are to be returned except
    /// for totalResults".
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
    /// RFC 7644 §3.4.2.3 `sortOrder`. This value defaults to ascending when
    /// `sort_by` is set and this value is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    /// RFC 7644 §3.4.2.4 Table 6 says "a value less than 1 SHALL be
    /// interpreted as 1". See
    /// [`effective_start_index`](Self::effective_start_index).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    /// RFC 7644 §3.4.2.4 Table 6 says "a negative value SHALL be
    /// interpreted as 0". See [`effective_count`](Self::effective_count).
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
    /// Reads `startIndex` the way RFC 7644 §3.4.2.4 Table 6 has a server
    /// read it. The default is 1 when the value is absent. The result is 1
    /// for any value below 1.
    pub fn effective_start_index(&self) -> i64 {
        self.start_index.map_or(1, |s| s.max(1))
    }

    /// Reads `count` the way Table 6 has a server read it. The result is
    /// `None` when the value is absent. The result is 0 for a negative
    /// value.
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
/// Deserialization dispatches on the SCIM schema URN in the payload's
/// `schemas` attribute (RFC 7643 §3). `Schema` and `ResourceType` resources
/// often carry no `schemas` field. RFC 7643's own §8.7 schema
/// representations carry none. The deserializer disambiguates these two
/// types by structural markers instead. The `attributes` array marks a
/// `Schema`. The `endpoint` and `schema` fields together mark a
/// `ResourceType`. The deserializer rejects a payload with no recognized
/// discriminator. It never classifies such a payload silently. This design
/// prevents type confusion.
///
/// This type carries `#[non_exhaustive]`. RFC 7643 §6 lets a server define
/// resource types beyond the four this crate models. New variants will
/// therefore appear in a minor release. Match on this enum with a
/// trailing `_ =>` arm.
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
    /// Private supertrait of [`ScimResource`]. No code outside this crate
    /// can name this trait. Because of that, no code outside this crate
    /// can implement `ScimResource`.
    pub trait Sealed {}
}

/// A resource that may appear in the `Resources` array of a
/// [`ListResponse`].
///
/// **Sealed.** [`ScimResource`] has a private supertrait. Because of that,
/// this crate fixes the implementing set: [`User`], [`Group`], [`Schema`],
/// [`ResourceType`], and [`Resource`] for the heterogeneous case. The bound
/// exists to make `ListResponse<R>` reject a nonsense `R` at compile time.
/// Before 1.0, the type parameter was the *id* type.
/// `ListResponse<Resource<String>>` then meant "ids are strings". The
/// same type now means "resources are strings". The sealing turns a
/// mismatch into a compile error instead of a runtime deserialization
/// failure.
///
/// `EnterpriseUser` is deliberately absent from this list. RFC 7643 §4.3
/// makes it a schema *extension* carried inside a `User`. It is never a
/// resource in its own right. `ServiceProviderConfig` is absent too. RFC
/// 7643 §5 serves it as a singleton, not a list.
///
/// # The bound is what makes the 1.0 change safe
///
/// `ListResponse<String>` compiled before 1.0. It meant "ids are strings".
/// Reusing the parameter for the resource would have left that code
/// compiling with a new meaning. That code would then fail only at
/// runtime, on deserialize. The bound turns such a mismatch into a
/// compile error instead. Each snippet below inlines the full path on
/// purpose. A `use` line could fail for an unrelated reason, such as a
/// renamed module or a moved re-export. If a snippet used a `use` line,
/// `compile_fail` would still pass even after the bound it guards had
/// been relaxed.
///
/// Doctest error-code annotations are inert on stable Rust. Because of
/// that, each snippet below contains only the one statement whose failure
/// proves the point. Each snippet also names the mutation it catches.
///
/// Catches: relaxing `R: ScimResource` on `ListResponse`.
///
/// ```compile_fail
/// // `String` is not a SCIM resource, so this does not compile.
/// let _list: scim_v2::models::others::ListResponse<String> = unimplemented!();
/// ```
///
/// The sealing is why no downstream crate can widen that set. Catches:
/// removing `: sealed::Sealed` from the trait. The impl below is
/// deliberately complete. An earlier version left out `declared_schemas`.
/// That earlier version failed for the missing method instead. It proved
/// nothing about the seal.
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
/// The seal holds because the supertrait's module is private. Naming that
/// module from outside code currently produces error E0603. A developer
/// might later make the module `pub`, for example to quiet a
/// `private_interfaces` warning. If that happens, the snippet below starts
/// to compile. A `compile_fail` snippet that compiles is a failing test.
/// The test failure is what reveals the trait has become unsealed.
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
    /// This value is empty when the payload omitted the attribute. A
    /// discovery payload sometimes omits it. RFC 7643's own §8.7 schema
    /// representations carry no `schemas` attribute. Whether an empty
    /// value is acceptable is each resource's own [`Validate`] decision.
    /// [`ListResponse::validate`] compares a non-empty value against
    /// [`schema_urn`](ScimResource::schema_urn). This comparison performs
    /// the same discriminator check that the [`Resource`] deserializer
    /// performs.
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
/// This type is generic over the resource, not over the id type. Use a
/// concrete resource type when the endpoint returns one kind of resource.
/// `GET /Users` returns `ListResponse<User<String>>`. This return type
/// deserializes straight into `Vec<User<String>>`. The deserializer needs
/// no enum to match through. It also performs one allocation for the
/// whole list, instead of one allocation per resource. Use [`Resource`]
/// for the heterogeneous case. RFC 7644 §3.4.3 allows this case when a
/// client queries the root `/.search` endpoint:
/// `ListResponse<Resource<String>>`.
///
/// `R` is bounded by the sealed [`ScimResource`] trait. This bound rejects
/// a type that is not a SCIM resource at compile time.
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse<R: ScimResource = Resource<String>> {
    /// RFC 7644 §3.4.2 marks this field REQUIRED when the response returns
    /// partial results due to pagination. Omit the field otherwise. The
    /// type does not enforce this rule. See [`ListResponse::validate`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_per_page: Option<i64>,
    pub total_results: i64,
    /// RFC 7644 §3.4.2 marks this field REQUIRED when the response returns
    /// partial results due to pagination. Omit the field otherwise. The
    /// type does not enforce this rule. See [`ListResponse::validate`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    pub schemas: Vec<String>,
    /// RFC 7644 §3.4.2 says this field is "REQUIRED if "totalResults" is
    /// non-zero". A query that returns no matches may omit the field for
    /// that reason. Deserialization refuses an absent field when
    /// `totalResults` is non-zero. [`validate`](Validate::validate) does
    /// not perform this check. Presence is a fact about the JSON, not
    /// about the value. An omitted member, `null`, and `[]` must all land
    /// on the same empty `Vec` for the type to stay usable. A hidden
    /// presence flag would make this struct impossible to build from a
    /// literal. `OperationTarget` refuses a pathless operation with no
    /// `value` at the same boundary, for the same reason.
    #[serde(rename = "Resources")]
    pub resources: Vec<R>,
}

/// The wire shape of [`ListResponse`]. This shape preserves whether
/// `Resources` was present. The check for the RFC 7644 §3.4.2 REQUIRED
/// rule needs that presence before the type erases it.
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

/// `Resources` as the payload carried it. The result is `None` only when
/// the member was absent. Some providers send `null` for an empty page.
/// RFC 7643 §2.5 makes `null` equivalent to `[]` here.
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
    /// * `totalResults` is not negative. `totalResults` is not smaller than
    ///   the number of entries in `Resources`. §3.4.2 describes
    ///   `totalResults` as a count that "may be larger than the number of
    ///   resources returned".
    /// * A present `startIndex` is at least 1. §3.4.2 calls it "the
    ///   1-based index of the first result". A present `itemsPerPage` is
    ///   not negative.
    /// * Every resource declares the schema of the type it was parsed as.
    ///   Every resource also passes its own [`Validate::validate`]. A
    ///   failure is reported under `Resources[i]`.
    ///   [`validate_as`](Validate::validate_as) carries the [`Context`]
    ///   into each resource too. Because of that, a `Response` page needs
    ///   an `id` on every entry.
    /// * When the response carries a partial result set, both
    ///   `startIndex` and `itemsPerPage` are present. A partial result set
    ///   has fewer entries in `Resources` than the value of
    ///   `totalResults`. §3.4.2 makes the two markers REQUIRED "when
    ///   partial results are returned due to pagination". Pagination is
    ///   the only mechanism §3.4.2 gives for returning fewer resources
    ///   than `totalResults`. For that reason, the check treats a short
    ///   page with no markers as non-conformant. It does not treat that
    ///   page as an unpaginated page of unexplained size.
    ///
    /// The type cannot encode this last invariant. `startIndex` and
    /// `itemsPerPage` are `Option<i64>` so that a wire `0` stays distinct
    /// from an omitted field. This design leaves a
    /// paginated-but-incomplete response expressible. Call this method
    /// before serializing a response that you assembled by hand.
    ///
    /// # Returns
    ///
    /// * `Ok(())`: the response is conformant.
    /// * `Err(ValidationError)`: either `schemas` is empty, or a
    ///   pagination marker is absent from a partial result set. The error
    ///   names the offending attribute by its wire path.
    ///
    /// # Example
    ///
    /// A homogeneous page from `GET /Users` deserializes straight into
    /// `Vec<User<String>>`. The deserializer needs no enum to match
    /// through:
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

    /// This method applies the direction to every resource on the page.
    /// For [`Context::Response`], each resource needs its `id` (RFC 7643
    /// §3.1) and may not carry `password`. A list response is always a
    /// response. The caller still states the context explicitly here, as
    /// everywhere else in this crate.
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
        /// `None` when the operation carried no `value` member at all. An
        /// `add` operation may not omit `value`. RFC 7644 §3.5.2.1 says
        /// "The operation MUST contain a "value" member whose content
        /// specifies the value to be added". This field stays distinct
        /// from `Some(Value::Null)`. [`PatchOp::validate`] can then report
        /// the omission. Otherwise the omission would collapse into an
        /// explicit null on the way in, and the crate would emit that null
        /// on the way out.
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

/// Whether `s` is RFC 7644 §3.10 standard attribute notation. This
/// notation is an `attrPath`, optionally URN-prefixed, with at most one
/// sub-attribute. The crate's own grammar decides the answer, instead of a
/// second guess at the grammar. `<s> pr` must parse to a bare presence
/// test.
#[cfg(feature = "filter")]
fn is_attribute_path(s: &str) -> bool {
    format!("{s} pr")
        .parse::<Filter>()
        .is_ok_and(|f| matches!(f, Filter::Attr(AttrExp::Present(_))))
}

/// The RFC 7644 §3.4.2.3 sort rules. Both query carriers share these
/// rules. `sortOrder` is "the order in which the "sortBy" parameter is
/// applied". On its own, `sortOrder` orders nothing. §3.4.3 also requires
/// that "the "sortBy" attribute MUST be in standard attribute notation
/// (Section 3.10) form".
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

/// The RFC 7644 §3.9 attribute-selection rules. Both query carriers share
/// these rules. §3.9 offers "either of the mutually exclusive URL query
/// parameters "attributes" or "excludedAttributes"". A request that
/// carries both parameters asserts two incompatible projections. §§3.4.2.5
/// and 3.4.3 both say of each parameter that "Attribute names MUST be in
/// standard attribute notation (Section 3.10) form". For that reason, a
/// name that a server cannot resolve to an attribute is a client error,
/// not something to ignore.
///
/// This function takes each selection as its individual names. A wholly
/// empty selection, such as `?attributes=` or an empty `Vec`, counts as
/// absent. Absence is the only reading that asserts nothing. An empty name
/// *within* a list, such as `?attributes=userName,,emails`, is malformed
/// and rejected.
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

/// The individual names in a `ListQuery`'s comma-separated selection. An
/// empty overall selection means no selection at all. An empty entry
/// inside a non-empty list stays in the result. This lets
/// [`validate_attribute_selection`] reject that entry.
#[cfg(feature = "filter")]
fn selection_names(raw: Option<&str>) -> impl Iterator<Item = &str> {
    raw.filter(|s| !s.is_empty())
        .into_iter()
        .flat_map(|s| s.split(','))
        .map(str::trim)
}

#[cfg(feature = "filter")]
impl<F> Validate for ListQuery<F> {
    /// The `GET` half of the rules that [`SearchRequest`] enforces for
    /// `POST /.search`. Those rules are RFC 7644 §3.4.2.3's sort pair and
    /// §3.9's mutually exclusive attribute selection. This method checks
    /// no `schemas` field, because these values are query parameters
    /// rather than a body. RFC 7644 §3.4.2.4 Table 6 has this method
    /// interpret an out-of-range `count` or `startIndex`, instead of
    /// rejecting it. See [`effective_count`](Self::effective_count) and
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
    /// RFC 7644 §3.4.3 requires the body to carry the SearchRequest URN.
    /// §3.4.3 also requires that "the sortBy attribute MUST be in standard
    /// attribute notation (Section 3.10) form". §3.4.2.3 defines
    /// `sortOrder` as "the order in which the sortBy parameter is
    /// applied". Without `sortBy`, `sortOrder` orders nothing, so this
    /// method rejects it instead of ignoring it silently. §3.9 calls
    /// `attributes` and `excludedAttributes` "mutually exclusive URL query
    /// parameters", so this method rejects a request that carries both. A
    /// negative `count` or a `startIndex` below 1 is *not* an error.
    /// §3.4.2.4 Table 6 says each value "SHALL be interpreted" as 0 and 1
    /// respectively. [`effective_count`](Self::effective_count) and
    /// [`effective_start_index`](Self::effective_start_index) perform that
    /// interpretation.
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
    /// RFC 7644 §3.5.2 says the body carries the PatchOp URN and "an array
    /// of one or more PATCH operations". Most of each operation's shape is
    /// enforced by the [`PatchOperation`] type. A `remove` operation needs
    /// a path. A pathless `add` or `replace` operation needs an object.
    /// The type cannot enforce one rule: §3.5.2.1 says "The operation MUST
    /// contain a "value" member whose content specifies the value to be
    /// added". A path-carrying operation may legitimately omit `value`
    /// when the operation is a `remove`.
    ///
    /// A path-carrying `replace` operation follows the same rule. §3.5.2.3
    /// has no single blanket MUST. §3.5.2.3 opens with "The "replace"
    /// operation replaces the value at the target location specified by
    /// the "path"". §3.5.2.3 then requires the `value` member twice. For
    /// the pathless form, §3.5.2.3 says "the "value" attribute SHALL
    /// contain a list of one or more attributes that are to be replaced".
    /// Where the path names a complex attribute, §3.5.2.3 says "a set of
    /// sub-attributes SHALL be specified in the "value" parameter".
    /// Telling a complex target from a single-valued one needs the
    /// resource's schema. This crate does not consult the resource's
    /// schema. No reading of §3.5.2.3 gives a valueless replace a defined
    /// outcome. For that reason, this method checks a `replace` operation
    /// the same way regardless of its target.
    ///
    /// An explicit `"value": null` counts as present. This is why the
    /// field keeps `None` and `Some(Value::Null)` distinct. What a null
    /// value means for the targeted attribute is the server's decision.
    /// What an absent value means is not the server's decision to make.
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
/// feature: [`Resource`], [`ListResponse`] and [`ScimResource`]. These
/// tests still run under `--no-default-features --features models`.
#[cfg(test)]
mod resource_tests;

#[cfg(all(test, feature = "filter"))]
mod tests;
