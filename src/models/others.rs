use serde::de::{DeserializeOwned, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(feature = "filter")]
use crate::filter::{Filter, InvalidFilterError, MaybeFilter, PatchPath};
use crate::models::group::Group;
use crate::models::resource_types::ResourceType;
use crate::models::scim_schema::Schema;
use crate::models::user::User;
use crate::schema_urns;
use crate::utils::validation::{Validate, ValidationError, require_schema_urn};

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
    /// omitted rather than sent as `[]`. §3.5.1's "empty array to clear all
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
    /// omitted rather than sent as `[]`. §3.5.1's "empty array to clear all
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_attributes: Option<String>,
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
            attributes: Some("".to_string()),
            excluded_attributes: Some("".to_string()),
        }
    }
}

/// Heterogeneous SCIM resource type used inside [`ListResponse`].
///
/// Deserialization dispatches on the SCIM schema URN carried in the payload's
/// `schemas` attribute (RFC 7643 §3). `Schema` and `ResourceType` resources,
/// which per RFC 7643 §§6-7 are often served without a `schemas` field, are
/// disambiguated by structural markers: the `attributes` array (Schema) or
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

        // No "schemas" field. Per RFC 7643 §§6-7, Schema and ResourceType
        // resources may appear without one. User and Group MUST carry their
        // URN and are not eligible for structural fallback.
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
pub trait ScimResource: sealed::Sealed {
    /// The RFC 7643 schema URN for this resource's type.
    fn schema_urn(&self) -> &'static str;

    /// The `schemas` attribute this particular instance carries on the wire.
    ///
    /// Empty when the payload omitted it, which RFC 7643 §§6-7 permit for the
    /// discovery resources. [`ListResponse::validate`] compares this against
    /// [`schema_urn`](ScimResource::schema_urn) so the typed path performs the
    /// same discriminator check the [`Resource`] deserializer does.
    fn declared_schemas(&self) -> &[String];
}

impl<T> sealed::Sealed for User<T> {}
impl<T> ScimResource for User<T> {
    fn declared_schemas(&self) -> &[String] {
        &self.schemas
    }

    fn schema_urn(&self) -> &'static str {
        schema_urns::USER
    }
}

impl<T> sealed::Sealed for Group<T> {}
impl<T> ScimResource for Group<T> {
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
impl<T> ScimResource for Resource<T> {
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(
    rename_all = "camelCase",
    bound(deserialize = "R: ScimResource + DeserializeOwned")
)]
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
    // RFC 7644 section 3.4.2: `Resources` is REQUIRED only if `totalResults` is
    // non-zero, so a query returning no matches may omit it on the wire.
    #[serde(
        rename = "Resources",
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub resources: Vec<R>,
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
    /// * When the response carries a *partial* result set — fewer entries in
    ///   `Resources` than `totalResults` — both `startIndex` and `itemsPerPage`
    ///   are present. §3.4.2 makes them REQUIRED "when partial results are
    ///   returned due to pagination".
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
        // makes both pagination markers REQUIRED.
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
        // and both validators returned Ok. Absence stays legal, per §§6-7.
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
        value: Value,
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
            let value = map.remove("value").unwrap_or(Value::Null);
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

#[cfg(feature = "filter")]
impl<F> Validate for SearchRequest<F> {
    /// RFC 7644 §3.4.3: the body carries the SearchRequest URN. §3.4.2.3:
    /// `sortOrder` without `sortBy` orders nothing, so it is rejected rather
    /// than silently ignored.
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, schema_urns::SEARCH_REQUEST)?;
        if self.sort_order.is_some() && self.sort_by.is_none() {
            return Err(ValidationError::invalid_value(
                "sortOrder",
                "present without sortBy; RFC 7644 §3.4.2.3 defines it as the order in which sortBy is applied",
            ));
        }
        if let Some(count) = self.count.filter(|c| *c < 0) {
            return Err(ValidationError::invalid_value(
                "count",
                format!("must not be negative, got {count}"),
            ));
        }
        Ok(())
    }
}

#[cfg(feature = "filter")]
impl Validate for PatchOp {
    /// RFC 7644 §3.5.2: the body carries the PatchOp URN and "an array of one
    /// or more PATCH operations". Each operation's own shape — `remove` needs
    /// a path, pathless `add`/`replace` need an object — is enforced by the
    /// [`PatchOperation`] type, so this only has to check the envelope.
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, schema_urns::PATCH_OP)?;
        if self.operations.is_empty() {
            return Err(ValidationError::invalid_value(
                "Operations",
                "must contain at least one operation (RFC 7644 §3.5.2)",
            ));
        }
        Ok(())
    }
}

/// Tests for the parts of this module that do not depend on the `filter`
/// feature — [`Resource`], [`ListResponse`] and [`ScimResource`] — so they
/// still run under `--no-default-features --features models`.
#[cfg(test)]
mod resource_tests {
    use super::*;
    use crate::models::user::User;
    use crate::schema_urns;

    /// The headline of the 1.0 `ListResponse` change: a homogeneous page
    /// deserializes into the concrete resource, so a caller that already knows
    /// `GET /Users` returns users does not match through a four-way enum to
    /// prove it.
    #[test]
    fn typed_list_response_deserializes_without_matching_an_enum() {
        let schema = schema_urns::LIST_RESPONSE;
        let user = schema_urns::USER;
        let body = format!(
            r#"{{"schemas":["{schema}"],"totalResults":2,"startIndex":1,"itemsPerPage":2,
                 "Resources":[
                   {{"schemas":["{user}"],"userName":"bjensen"}},
                   {{"schemas":["{user}"],"userName":"jsmith"}}]}}"#
        );

        let list: ListResponse<User<String>> =
            serde_json::from_str(&body).expect("typed ListResponse must deserialize");
        assert!(list.validate().is_ok());
        let names: Vec<&str> = list
            .resources
            .iter()
            .map(|u| u.user_name.as_str())
            .collect();
        assert_eq!(names, ["bjensen", "jsmith"]);
    }

    /// L-2: the typed path chooses `R` at the call site, so before this the
    /// payload's declared `schemas` was never compared against the type it was
    /// parsed as. A Group payload carrying a `userName` deserialized cleanly
    /// into `ListResponse<User<String>>` and both validators returned `Ok`.
    #[test]
    fn typed_list_response_validate_rejects_a_schema_mismatch() {
        let schema = schema_urns::LIST_RESPONSE;
        let group = schema_urns::GROUP;
        let body = format!(
            r#"{{"schemas":["{schema}"],"totalResults":1,"startIndex":1,"itemsPerPage":1,
                 "Resources":[{{"schemas":["{group}"],"userName":"admin","displayName":"Tour Guides"}}]}}"#
        );

        // It still deserializes — `R` decides the shape, and every attribute
        // `User` requires is present.
        let list: ListResponse<User<String>> =
            serde_json::from_str(&body).expect("shape matches User");
        assert_eq!(list.resources[0].user_name, "admin");

        let err = list
            .validate()
            .expect_err("a Group URN in a ListResponse<User> is a mismatch");
        assert_eq!(err.path(), "Resources[0].schemas");
        assert_eq!(err.scim_type_str(), "invalidValue");
    }

    /// RFC 7643 §§6-7 let a discovery resource omit `schemas` entirely, so an
    /// absent discriminator must stay legal rather than becoming a new failure.
    #[test]
    fn typed_list_response_validate_tolerates_an_absent_discriminator() {
        let list: ListResponse<User<String>> = ListResponse {
            schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
            total_results: 1,
            start_index: Some(1),
            items_per_page: Some(1),
            resources: vec![User::<String> {
                schemas: Vec::new(),
                user_name: "bjensen".to_string(),
                ..Default::default()
            }],
        };
        assert!(
            list.validate().is_ok(),
            "an omitted `schemas` is legal per §§6-7"
        );
    }

    /// The `schemas` array may carry URNs this crate does not model — a
    /// custom extension, say — alongside one it does. Dispatch follows the
    /// recognised URN rather than failing on the unrecognised one.
    #[test]
    fn resource_dispatch_ignores_an_unknown_urn_beside_a_known_one() {
        let body = format!(
            r#"{{"schemas":["urn:example:custom:Thing","{}"],"userName":"bjensen"}}"#,
            schema_urns::USER
        );
        let r: Resource<String> =
            serde_json::from_str(&body).expect("must dispatch on the User URN");
        assert!(matches!(r, Resource::User(_)));
    }

    /// The heterogeneous form still works, which RFC 7644 §3.4.3 needs for a
    /// query against the root `/.search` endpoint.
    #[test]
    fn heterogeneous_list_response_still_dispatches_per_resource() {
        let schema = schema_urns::LIST_RESPONSE;
        let user = schema_urns::USER;
        let group = schema_urns::GROUP;
        let body = format!(
            r#"{{"schemas":["{schema}"],"totalResults":2,"startIndex":1,"itemsPerPage":2,
                 "Resources":[
                   {{"schemas":["{user}"],"userName":"bjensen"}},
                   {{"schemas":["{group}"],"displayName":"Tour Guides"}}]}}"#
        );

        let list: ListResponse<Resource<String>> =
            serde_json::from_str(&body).expect("heterogeneous ListResponse must deserialize");
        let urns: Vec<&str> = list
            .resources
            .iter()
            .map(ScimResource::schema_urn)
            .collect();
        assert_eq!(urns, [schema_urns::USER, schema_urns::GROUP]);
    }

    /// A typed `ListResponse` rejects a payload whose resources are a
    /// different type, rather than silently accepting it. The `schemas` URN in
    /// the body does not select the type here — `R` does — so the mismatch has
    /// to surface as a deserialization failure.
    #[test]
    fn typed_list_response_rejects_a_mismatched_resource() {
        let schema = schema_urns::LIST_RESPONSE;
        let group = schema_urns::GROUP;
        let body = format!(
            r#"{{"schemas":["{schema}"],"totalResults":1,"startIndex":1,"itemsPerPage":1,
                 "Resources":[{{"schemas":["{group}"],"displayName":"Tour Guides"}}]}}"#
        );

        // `Group` has no `userName`, which `User` requires.
        let parsed: Result<ListResponse<User<String>>, _> = serde_json::from_str(&body);
        assert!(
            parsed.is_err(),
            "a Group payload must not deserialize as ListResponse<User>"
        );
    }
}

#[cfg(all(test, feature = "filter"))]
mod tests {
    use super::*;
    use test_case::test_case;

    /// R2-M3: the four bounds added to `validate` in round 1, each pinned
    /// with the wire path it reports.
    #[test_case(-5, 0, Some(1), Some(1), "totalResults" ; "negative_total")]
    #[test_case(1, 2, Some(1), Some(2), "totalResults" ; "more_returned_than_total")]
    #[test_case(3, 1, Some(0), Some(1), "startIndex" ; "zero_based_start_index")]
    #[test_case(3, 1, Some(1), Some(-1), "itemsPerPage" ; "negative_items_per_page")]
    fn validate_rejects_impossible_pagination(
        total: i64,
        returned: usize,
        start: Option<i64>,
        per_page: Option<i64>,
        path: &str,
    ) {
        let user = || {
            Resource::User(Box::new(User::<String> {
                schemas: vec![schema_urns::USER.to_string()],
                user_name: "u".to_string(),
                id: Some("1".to_string()),
                ..Default::default()
            }))
        };
        let list = ListResponse::<Resource<String>> {
            schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
            total_results: total,
            start_index: start,
            items_per_page: per_page,
            resources: (0..returned).map(|_| user()).collect(),
        };
        assert_eq!(list.validate().expect_err(path).path(), path);
    }

    /// R2-M6: an explicit `"Resources": null` is a form providers send, and it
    /// must land as an empty page *with the envelope intact*, not as an error
    /// or a silently defaulted struct.
    #[test]
    fn resources_null_collapses_to_an_empty_page_and_keeps_total_results() {
        let body = format!(
            r#"{{"schemas":["{}"],"totalResults":7,"startIndex":1,"itemsPerPage":0,"Resources":null}}"#,
            schema_urns::LIST_RESPONSE
        );
        let list: ListResponse<User<String>> = serde_json::from_str(&body).expect("null Resources");
        assert!(list.resources.is_empty());
        assert_eq!(list.total_results, 7);
        assert_eq!(list.items_per_page, Some(0));
    }

    /// RFC 7644 §3.5.2 spells the ops lowercase; Entra and others send them
    /// capitalised. Each accepted spelling must map to the same variant.
    #[test_case("add", "Add", "ADD" ; "add")]
    #[test_case("remove", "Remove", "REMOVE" ; "remove")]
    #[test_case("replace", "Replace", "REPLACE" ; "replace")]
    fn patch_op_accepts_every_documented_spelling(lower: &str, title: &str, upper: &str) {
        for op in [lower, title, upper] {
            let body = format!(
                r#"{{"schemas":["{}"],"Operations":[{{"op":"{op}","path":"nickName","value":"x"}}]}}"#,
                schema_urns::PATCH_OP
            );
            let patch: PatchOp =
                serde_json::from_str(&body).unwrap_or_else(|e| panic!("{op}: {e}"));
            let as_lower = serde_json::to_value(&patch).unwrap()["Operations"][0]["op"].clone();
            assert_eq!(
                as_lower, lower,
                "{op} must serialize back as the RFC's lowercase form"
            );
        }
    }

    /// §3.5.2: "an array of one or more PATCH operations". Deserialization
    /// accepts the envelope; `validate` is where the requirement lives.
    #[test]
    fn patch_op_validate_requires_at_least_one_operation_and_the_urn() {
        let empty: PatchOp = serde_json::from_str(&format!(
            r#"{{"schemas":["{}"],"Operations":[]}}"#,
            schema_urns::PATCH_OP
        ))
        .expect("an empty Operations array deserializes");
        let err = empty.validate().expect_err("but is not conformant");
        assert_eq!(err.path(), "Operations");

        let wrong_urn: PatchOp = serde_json::from_str(&format!(
            r#"{{"schemas":["{}"],"Operations":[{{"op":"remove","path":"nickName"}}]}}"#,
            schema_urns::USER
        ))
        .unwrap();
        assert_eq!(
            wrong_urn
                .validate()
                .expect_err("URN must be PatchOp")
                .path(),
            "schemas"
        );
    }

    /// RFC 7644 §3.4.2.3 / §3.4.3: `sortBy` and `sortOrder` are part of the
    /// search body and must survive a round-trip. Before 1.0 both were
    /// silently dropped on deserialize.
    #[test]
    fn search_request_round_trips_sort_parameters() {
        let body = format!(
            r#"{{"schemas":["{}"],"sortBy":"name.familyName","sortOrder":"descending","startIndex":1,"count":10}}"#,
            schema_urns::SEARCH_REQUEST
        );
        let req: SearchRequest<Filter> = serde_json::from_str(&body).unwrap();
        assert_eq!(req.sort_by.as_deref(), Some("name.familyName"));
        assert_eq!(req.sort_order, Some(SortOrder::Descending));
        assert!(req.validate().is_ok());

        let back = serde_json::to_value(&req).unwrap();
        assert_eq!(back["sortBy"], "name.familyName");
        assert_eq!(back["sortOrder"], "descending");

        // Provider spelling is tolerated; the RFC's lowercase is what goes out.
        let caps: ListQuery<Filter> =
            serde_json::from_str(r#"{"sortBy":"userName","sortOrder":"Ascending"}"#).unwrap();
        assert_eq!(caps.sort_order, Some(SortOrder::Ascending));
        assert_eq!(
            serde_json::to_value(&caps).unwrap()["sortOrder"],
            "ascending"
        );
    }

    /// §3.4.2.3 defines `sortOrder` as "the order in which the sortBy parameter
    /// is applied", so on its own it orders nothing.
    #[test]
    fn search_request_validate_rejects_sort_order_without_sort_by() {
        let req = SearchRequest::<Filter> {
            sort_order: Some(SortOrder::Descending),
            ..Default::default()
        };
        assert_eq!(
            req.validate().expect_err("orphan sortOrder").path(),
            "sortOrder"
        );
        assert!(SearchRequest::<Filter>::default().validate().is_ok());
    }

    /// RFC 7643 §3 makes `schemas` the discriminator, and the ListResponse
    /// envelope has one fixed URN.
    #[test]
    fn list_response_validate_requires_its_own_urn() {
        let list = ListResponse::<Resource<String>> {
            schemas: vec![schema_urns::USER.to_string()],
            total_results: 0,
            items_per_page: None,
            start_index: None,
            resources: vec![],
        };
        let err = list
            .validate()
            .expect_err("a User URN is not the ListResponse URN");
        assert_eq!(err.path(), "schemas");
    }
    use crate::filter::{
        AttrExp, AttrPath, CompValue, CompareOp, PatchPath, PatchValuePath, ValFilter,
    };
    use crate::utils::serde::drop_unassigned;
    use pretty_assertions::assert_eq;

    const PATCH_OP_SCHEMA: &str = schema_urns::PATCH_OP;

    #[test]
    fn test_list_response_without_resources() {
        let schema = schema_urns::LIST_RESPONSE;
        let body = format!(
            r#"{{"schemas":["{schema}"],"totalResults":0,"startIndex":1,"itemsPerPage":0}}"#
        );
        let list: ListResponse<Resource<String>> =
            serde_json::from_str(&body).expect("Failed to deserialize empty list response");
        assert_eq!(list.total_results, 0);
        assert!(list.resources.is_empty());
    }

    #[test]
    fn test_list_response_without_pagination_fields() {
        // RFC 7644 §3.4.2: `startIndex` and `itemsPerPage` are REQUIRED only
        // when partial results are returned due to pagination. A full,
        // unpaginated response may omit them, and some providers do; that
        // must deserialize rather than fail with a `missing field` error.
        let schema = schema_urns::LIST_RESPONSE;
        let body = format!(r#"{{"schemas":["{schema}"],"totalResults":0}}"#);
        let list: ListResponse<Resource<String>> = serde_json::from_str(&body)
            .expect("ListResponse without startIndex/itemsPerPage must deserialize");
        assert_eq!(list.total_results, 0);
        assert_eq!(list.start_index, None);
        assert_eq!(list.items_per_page, None);
        assert!(list.resources.is_empty());
    }

    #[test]
    fn test_list_response_preserves_zero_pagination_fields() {
        // A wire value of 0 must survive as `Some(0)`, distinct from an
        // omitted field (`None`) — the reason these are `Option<i64>` rather
        // than `#[serde(default)]` to an inert 0.
        let schema = schema_urns::LIST_RESPONSE;
        let body = format!(
            r#"{{"schemas":["{schema}"],"totalResults":0,"startIndex":1,"itemsPerPage":0}}"#
        );
        let list: ListResponse<Resource<String>> =
            serde_json::from_str(&body).expect("Failed to deserialize list response");
        assert_eq!(list.start_index, Some(1));
        assert_eq!(list.items_per_page, Some(0));
    }

    #[test]
    fn test_list_response_omits_none_pagination_fields_on_serialize() {
        // `None` must be omitted from the wire, not emitted as `null` —
        // `"itemsPerPage": null` is not valid SCIM.
        let list: ListResponse<Resource<String>> = ListResponse {
            items_per_page: None,
            total_results: 0,
            start_index: None,
            schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
            resources: vec![],
        };
        let json = serde_json::to_string(&list).expect("serialize ListResponse");
        assert!(
            !json.contains("itemsPerPage"),
            "omitted itemsPerPage must not appear on the wire: {json}"
        );
        assert!(
            !json.contains("startIndex"),
            "omitted startIndex must not appear on the wire: {json}"
        );
        assert!(
            !json.contains("null"),
            "no field should serialize as null: {json}"
        );
    }

    #[test]
    fn test_list_response_pagination_fields_round_trip() {
        let list: ListResponse<Resource<String>> = ListResponse {
            items_per_page: Some(20),
            total_results: 137,
            start_index: Some(41),
            schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
            resources: vec![],
        };
        let json = serde_json::to_string(&list).expect("serialize ListResponse");
        let back: ListResponse<Resource<String>> = serde_json::from_str(&json).expect("round-trip");
        assert_eq!(back.items_per_page, Some(20));
        assert_eq!(back.start_index, Some(41));
        assert_eq!(back.total_results, 137);
    }

    #[test]
    fn test_list_response_serializes_and_round_trips_zero_pagination_fields() {
        // The serialize-side guarantee behind `Option<i64>`: `Some(0)` is a
        // real SCIM value (a zero-result page) and MUST reach the wire — only
        // `None` may be dropped. Guards against a `skip_serializing_if`
        // predicate that also swallows `Some(0)`.
        let list: ListResponse<Resource<String>> = ListResponse {
            items_per_page: Some(0),
            total_results: 0,
            start_index: Some(0),
            schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
            resources: vec![],
        };
        let json = serde_json::to_string(&list).expect("serialize ListResponse");
        assert!(
            json.contains(r#""itemsPerPage":0"#),
            "Some(0) itemsPerPage must be emitted, not skipped: {json}"
        );
        assert!(
            json.contains(r#""startIndex":0"#),
            "Some(0) startIndex must be emitted, not skipped: {json}"
        );

        let back: ListResponse<Resource<String>> = serde_json::from_str(&json).expect("round-trip");
        assert_eq!(back.items_per_page, Some(0));
        assert_eq!(back.start_index, Some(0));
    }

    #[test]
    fn validate_list_response_accepts_complete_unpaginated_response() {
        // `Resources` count == `totalResults`: not a partial page, so the
        // pagination markers are legitimately absent (RFC 7644 §3.4.2).
        let list: ListResponse<Resource<String>> = ListResponse {
            schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
            total_results: 0,
            items_per_page: None,
            start_index: None,
            resources: vec![],
        };
        assert!(list.validate().is_ok());
    }

    #[test]
    fn validate_list_response_accepts_partial_page_with_pagination_markers() {
        let list: ListResponse<Resource<String>> = ListResponse {
            schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
            total_results: 100,
            items_per_page: Some(10),
            start_index: Some(1),
            resources: vec![],
        };
        assert!(list.validate().is_ok());
    }

    #[test]
    fn validate_list_response_rejects_partial_page_missing_start_index() {
        let list: ListResponse<Resource<String>> = ListResponse {
            schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
            total_results: 100,
            items_per_page: Some(10),
            start_index: None,
            resources: vec![],
        };
        let err = list.validate().expect_err("must fail validation");
        assert_eq!(err.path(), "startIndex");
        assert_eq!(err.scim_type_str(), "invalidValue");
    }

    #[test]
    fn validate_list_response_rejects_partial_page_missing_items_per_page() {
        let list: ListResponse<Resource<String>> = ListResponse {
            schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
            total_results: 100,
            items_per_page: None,
            start_index: Some(1),
            resources: vec![],
        };
        let err = list.validate().expect_err("must fail validation");
        assert_eq!(err.path(), "itemsPerPage");
        assert_eq!(err.scim_type_str(), "invalidValue");
    }

    #[test]
    fn validate_list_response_rejects_empty_schemas() {
        let list: ListResponse<Resource<String>> = ListResponse {
            schemas: vec![],
            total_results: 0,
            items_per_page: None,
            start_index: None,
            resources: vec![],
        };
        let err = list.validate().expect_err("must fail validation");
        assert_eq!(err.path(), "schemas");
        assert_eq!(err.scim_type_str(), "invalidValue");
    }

    // ---- RFC 7644 sample payloads (verbatim, except as noted) ----

    /// RFC 7644 §3.4.2 — the (unnumbered) response to `GET
    /// /Users?attributes=userName`, introduced by "The following is an example
    /// response to the query above". A query response with no pagination fields.
    ///
    /// The RFC prints the embedded resources abbreviated to `id` + `userName`
    /// with no `schemas`; a `schemas` array is added to each here because this
    /// crate's [`Resource`] deserializer requires a resource-type
    /// discriminator (see the module docs) and will not guess. The
    /// `ListResponse` envelope — `totalResults` present, `startIndex` and
    /// `itemsPerPage` absent — is untouched and is the point of the test.
    #[test]
    fn rfc7644_s3_4_2_list_response() {
        let raw = include_str!("../test_data/rfc7644/s3.4.2_list_response.json");
        let list: ListResponse<Resource<String>> =
            serde_json::from_str(raw).expect("RFC 7644 §3.4.2 list response must deserialize");
        assert_eq!(list.total_results, 2);
        assert_eq!(list.start_index, None);
        assert_eq!(list.items_per_page, None);
        assert_eq!(list.resources.len(), 2);
        assert!(matches!(list.resources[0], Resource::User(_)));

        let reserialized: Value =
            serde_json::from_str(&serde_json::to_string(&list).unwrap()).unwrap();
        let original: Value = serde_json::from_str(raw).unwrap();
        assert_eq!(
            drop_unassigned(reserialized),
            drop_unassigned(original),
            "every assigned attribute of the RFC payload must survive the round-trip"
        );
    }

    /// RFC 7644 §3.4.2.4, Figure 3 ("ListResponse Format for Returning Multiple
    /// Resources") — the pagination example. The RFC prints its single resource
    /// abbreviated to `{...}`; that placeholder is replaced here with one
    /// concrete User (see `test_data/README.md`). `totalResults` (100)
    /// intentionally exceeds the page size (10).
    #[test]
    fn rfc7644_s3_4_2_4_fig3_pagination_response() {
        let raw = include_str!("../test_data/rfc7644/s3.4.2.4_fig3_pagination_response.json");
        let list: ListResponse<Resource<String>> =
            serde_json::from_str(raw).expect("RFC 7644 Figure 3 must deserialize");
        assert_eq!(list.total_results, 100);
        assert_eq!(list.items_per_page, Some(10));
        assert_eq!(list.start_index, Some(1));
        assert_eq!(list.resources.len(), 1);
        assert!(matches!(list.resources[0], Resource::User(_)));

        let reserialized: Value =
            serde_json::from_str(&serde_json::to_string(&list).unwrap()).unwrap();
        let original: Value = serde_json::from_str(raw).unwrap();
        assert_eq!(
            drop_unassigned(reserialized),
            drop_unassigned(original),
            "every assigned attribute of the RFC payload must survive the round-trip"
        );
    }

    /// RFC 7644 §3.4.3, Figure 5 ("Example POST Query Response") — POST
    /// `/.search` response with pagination fields present. The RFC truncates the
    /// resource list with a trailing
    /// `...`; that placeholder is dropped here and a `schemas` array is added
    /// to each resource (see [`rfc7644_s3_4_2_list_response`]). The second
    /// resource is a Group, so this also covers a heterogeneous list.
    #[test]
    fn rfc7644_s3_4_3_fig5_post_query_response() {
        let raw = include_str!("../test_data/rfc7644/s3.4.3_fig5_post_query_response.json");
        let list: ListResponse<Resource<String>> =
            serde_json::from_str(raw).expect("RFC 7644 Figure 5 must deserialize");
        assert_eq!(list.total_results, 100);
        assert_eq!(list.items_per_page, Some(10));
        assert_eq!(list.start_index, Some(1));
        assert!(matches!(list.resources[0], Resource::User(_)));
        assert!(matches!(list.resources[1], Resource::Group(_)));

        let reserialized: Value =
            serde_json::from_str(&serde_json::to_string(&list).unwrap()).unwrap();
        let original: Value = serde_json::from_str(raw).unwrap();
        assert_eq!(
            drop_unassigned(reserialized),
            drop_unassigned(original),
            "every assigned attribute of the RFC payload must survive the round-trip"
        );
    }

    /// RFC 7644 §3.4.3, Figure 4 ("Example POST Query Request") — POST
    /// `/.search` request body. Verbatim.
    #[test]
    fn rfc7644_s3_4_3_fig4_search_request() {
        let raw = include_str!("../test_data/rfc7644/s3.4.3_fig4_search_request.json");
        let req: SearchRequest =
            serde_json::from_str(raw).expect("RFC 7644 Figure 4 must deserialize");
        assert_eq!(req.start_index, Some(1));
        assert_eq!(req.count, Some(10));
        assert_eq!(
            req.attributes,
            vec!["displayName".to_string(), "userName".to_string()]
        );
        assert!(req.filter.is_some());

        // The parsed filter must serialize back to the RFC's filter string.
        let reserialized: Value =
            serde_json::from_str(&serde_json::to_string(&req).unwrap()).unwrap();
        let original: Value = serde_json::from_str(raw).unwrap();
        assert_eq!(reserialized, original);
    }

    /// RFC 7644 §3.5.2.1 — unnumbered example, "how to add a member to a group"
    /// (the block at rfc7644.txt lines 2033-2048; *not* Figure 6, which carries
    /// a `... + additional operations if needed ...` placeholder). `add` a group
    /// member via an explicit `members` path.
    #[test]
    fn rfc7644_s3_5_2_1_patch_add_member() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.1_add_member.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 1);
        assert!(matches!(
            &ops.operations[0],
            PatchOperation::Add(OperationTarget::WithPath {
                path: PatchPath::Attr(AttrPath { uri: None, name, sub_attr: None }),
                ..
            }) if name == "members"
        ));
    }

    /// RFC 7644 §3.5.2.1 — unnumbered example, "how to add one or more
    /// attributes to a User resource without using a `path` attribute". `add`
    /// several user attributes at once with no `path` (the value is a partial
    /// resource object).
    #[test]
    fn rfc7644_s3_5_2_1_patch_add_user_attributes() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.1_add_user_attributes.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 1);
        assert!(matches!(
            &ops.operations[0],
            PatchOperation::Add(OperationTarget::WithoutPath { .. })
        ));
    }

    /// RFC 7644 §3.5.2.2 — unnumbered example, "Remove a single member from a
    /// group". `remove` a group member selected by a value filter
    /// (`members[value eq "..."]`).
    #[test]
    fn rfc7644_s3_5_2_2_patch_remove_member_by_filter() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.2_remove_member_by_filter.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 1);
        match &ops.operations[0] {
            PatchOperation::Remove {
                path:
                    PatchPath::Value(PatchValuePath {
                        attr:
                            AttrPath {
                                uri: None,
                                name: attr_name,
                                sub_attr: None,
                            },
                        filter:
                            ValFilter::Attr(AttrExp::Comparison(
                                AttrPath {
                                    uri: None,
                                    name: inner_name,
                                    sub_attr: None,
                                },
                                CompareOp::Eq,
                                CompValue::Str(v),
                            )),
                        sub_attr: None,
                    }),
                value: None,
            } if attr_name == "members"
                && inner_name == "value"
                && v == "2819c223-7f76-...413861904646" => {}
            other => panic!("unexpected operation: {other:?}"),
        }
    }

    /// RFC 7644 §3.5.2.2 — unnumbered example, "Remove all members of a group".
    /// `remove` the entire `members` attribute (bare path, no filter).
    #[test]
    fn rfc7644_s3_5_2_2_patch_remove_all_members() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.2_remove_all_members.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 1);
        assert!(matches!(
            &ops.operations[0],
            PatchOperation::Remove {
                path: PatchPath::Attr(AttrPath { uri: None, name, sub_attr: None }),
                value: None,
            } if name == "members"
        ));
    }

    /// RFC 7644 §3.5.2.2 — unnumbered example, "Removal of a value from a
    /// complex multi-valued attribute". `remove` an entry selected by a compound
    /// `and` filter (`emails[type eq "work" and value ew "example.com"]`).
    #[test]
    fn rfc7644_s3_5_2_2_patch_remove_complex_attribute() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.2_remove_complex_attribute.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 1);
        match &ops.operations[0] {
            PatchOperation::Remove {
                path:
                    PatchPath::Value(PatchValuePath {
                        attr:
                            AttrPath {
                                uri: None,
                                name: attr_name,
                                sub_attr: None,
                            },
                        filter: ValFilter::And(left, right),
                        sub_attr: None,
                    }),
                value: None,
            } if attr_name == "emails" => {
                match left.as_ref() {
                    ValFilter::Attr(AttrExp::Comparison(
                        AttrPath {
                            uri: None,
                            name: n,
                            sub_attr: None,
                        },
                        CompareOp::Eq,
                        CompValue::Str(v),
                    )) if n == "type" && v == "work" => {}
                    other => panic!("unexpected left filter: {other:?}"),
                }
                match right.as_ref() {
                    ValFilter::Attr(AttrExp::Comparison(
                        AttrPath {
                            uri: None,
                            name: n,
                            sub_attr: None,
                        },
                        CompareOp::Ew,
                        CompValue::Str(v),
                    )) if n == "value" && v == "example.com" => {}
                    other => panic!("unexpected right filter: {other:?}"),
                }
            }
            other => panic!("unexpected operation: {other:?}"),
        }
    }

    /// RFC 7644 §3.5.2.2 — unnumbered example, "Example request to remove and
    /// add a member". A two-op `PatchOp` that `remove`s one member by value
    /// filter, then `add`s a different member. The RFC prints the remove path as
    /// `members[value eq"..."]` with no space after `eq` and truncates both
    /// UUIDs with `...`; the space is normalized here so the filter parses (see
    /// `test_data/README.md`).
    #[test]
    fn rfc7644_s3_5_2_2_patch_remove_by_filter_then_add_member() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.2_remove_by_filter_then_add_member.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 2);
        match &ops.operations[0] {
            PatchOperation::Remove {
                path:
                    PatchPath::Value(PatchValuePath {
                        attr:
                            AttrPath {
                                uri: None,
                                name: attr_name,
                                sub_attr: None,
                            },
                        filter:
                            ValFilter::Attr(AttrExp::Comparison(
                                AttrPath {
                                    uri: None,
                                    name: inner_name,
                                    sub_attr: None,
                                },
                                CompareOp::Eq,
                                CompValue::Str(_),
                            )),
                        sub_attr: None,
                    }),
                value: None,
            } if attr_name == "members" && inner_name == "value" => {}
            other => panic!("unexpected remove operation: {other:?}"),
        }
        assert!(matches!(
            &ops.operations[1],
            PatchOperation::Add(OperationTarget::WithPath {
                path: PatchPath::Attr(AttrPath { uri: None, name, sub_attr: None }),
                ..
            }) if name == "members"
        ));
    }

    /// RFC 7644 §3.5.2.2 — unnumbered example, "how to replace all of the
    /// members of a group with a different members list". Modeled as a `remove`
    /// of `members` followed by an `add` of `members` in one `PatchOp`.
    #[test]
    fn rfc7644_s3_5_2_2_patch_replace_all_members() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.2_replace_all_members.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 2);
        assert!(matches!(
            &ops.operations[0],
            PatchOperation::Remove {
                path: PatchPath::Attr(AttrPath { uri: None, name, sub_attr: None }),
                value: None,
            } if name == "members"
        ));
        assert!(matches!(
            &ops.operations[1],
            PatchOperation::Add(OperationTarget::WithPath {
                path: PatchPath::Attr(AttrPath { uri: None, name, sub_attr: None }),
                ..
            }) if name == "members"
        ));
    }

    /// RFC 7644 §3.5.2.3 — unnumbered example, "how to replace all of the
    /// members of a group with a different members list in a single replace
    /// operation". `replace` the entire `members` list in one operation.
    #[test]
    fn rfc7644_s3_5_2_3_patch_replace_members_single_op() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.3_replace_members_single_op.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 1);
        assert!(matches!(
            &ops.operations[0],
            PatchOperation::Replace(OperationTarget::WithPath {
                path: PatchPath::Attr(AttrPath { uri: None, name, sub_attr: None }),
                ..
            }) if name == "members"
        ));
    }

    /// RFC 7644 §3.5.2.3 — unnumbered example, "how to change a User's entire
    /// `work` address, using a `valuePath` filter". `replace` the entry selected
    /// by `addresses[type eq "work"]` with a full complex value.
    #[test]
    fn rfc7644_s3_5_2_3_patch_replace_work_address() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.3_replace_work_address.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 1);
        match &ops.operations[0] {
            PatchOperation::Replace(OperationTarget::WithPath {
                path:
                    PatchPath::Value(PatchValuePath {
                        attr:
                            AttrPath {
                                uri: None,
                                name: attr_name,
                                sub_attr: None,
                            },
                        filter:
                            ValFilter::Attr(AttrExp::Comparison(
                                AttrPath {
                                    uri: None,
                                    name: n,
                                    sub_attr: None,
                                },
                                CompareOp::Eq,
                                CompValue::Str(v),
                            )),
                        sub_attr: None,
                    }),
                ..
            }) if attr_name == "addresses" && n == "type" && v == "work" => {}
            other => panic!("unexpected operation: {other:?}"),
        }
    }

    /// RFC 7644 §3.5.2.3 — unnumbered example, "how to change a specific
    /// sub-attribute `streetAddress` ... selected by a `valuePath` filter".
    /// `replace` a single sub-attribute of a filtered entry
    /// (`addresses[type eq "work"].streetAddress`).
    #[test]
    fn rfc7644_s3_5_2_3_patch_replace_street_address_via_filter() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.3_replace_street_address_via_filter.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 1);
        match &ops.operations[0] {
            PatchOperation::Replace(OperationTarget::WithPath {
                path:
                    PatchPath::Value(PatchValuePath {
                        attr:
                            AttrPath {
                                uri: None,
                                name: attr_name,
                                sub_attr: None,
                            },
                        filter:
                            ValFilter::Attr(AttrExp::Comparison(
                                AttrPath {
                                    uri: None,
                                    name: n,
                                    sub_attr: None,
                                },
                                CompareOp::Eq,
                                CompValue::Str(v),
                            )),
                        sub_attr: Some(sub_attr),
                    }),
                value,
            }) if attr_name == "addresses"
                && sub_attr == "streetAddress"
                && n == "type"
                && v == "work" =>
            {
                assert_eq!(value.as_str(), Some("1010 Broadway Ave"));
            }
            _ => panic!("Expected Replace WithPath for addresses[type eq \"work\"].streetAddress"),
        }
    }

    /// RFC 7644 §3.5.2.3 — unnumbered example, "how to replace all values of one
    /// or more specific attributes of a User resource". `replace` multiple
    /// attributes at once with no `path` (the value is a partial resource
    /// object).
    #[test]
    fn rfc7644_s3_5_2_3_patch_replace_multiple_attributes() {
        let ops: PatchOp = serde_json::from_str(include_str!(
            "../test_data/rfc7644/s3.5.2.3_replace_multiple_attributes.json"
        ))
        .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 1);
        assert!(matches!(
            &ops.operations[0],
            PatchOperation::Replace(OperationTarget::WithoutPath { .. })
        ));
    }

    // ---- Negative tests: malformed path must NOT silently fallthrough ----

    #[test]
    fn test_malformed_path_returns_error() {
        let json = r#"{
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:PatchOp"],
            "Operations": [{
                "op": "replace",
                "path": "emails[broken!!!filter",
                "value": {"displayName": "pwned"}
            }]
        }"#;
        let result: Result<PatchOp, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "malformed path must produce an error, not silently fallthrough"
        );
    }

    #[test]
    fn test_invalid_op_returns_error() {
        let json = r#"{
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:PatchOp"],
            "Operations": [{"op": "delete", "path": "members"}]
        }"#;
        let result: Result<PatchOp, _> = serde_json::from_str(json);
        assert!(result.is_err(), "invalid op 'delete' must produce an error");
    }

    // ---- Tolerant filter deserialization (RFC 7644 §3.12) ----

    #[test]
    fn test_tolerant_list_query_valid_filter() {
        let json = r#"{
            "filter": "userName eq \"alice\"",
            "count": 50
        }"#;
        let q: TolerantListQuery =
            serde_json::from_str(json).expect("deserialization must succeed");
        assert_eq!(q.count, Some(50));
        match q.filter {
            Some(MaybeFilter::Valid(_)) => {}
            other => panic!("expected Valid filter, got {other:?}"),
        }
    }

    #[test]
    fn test_tolerant_list_query_invalid_filter_preserves_other_fields() {
        // Unterminated string literal — parser must reject this filter but
        // the surrounding query should still deserialize so the handler can
        // return a proper 400 invalidFilter response.
        let json = r#"{
            "filter": "userName eq \"alice",
            "count": 50,
            "startIndex": 2
        }"#;
        let q: TolerantListQuery =
            serde_json::from_str(json).expect("tolerant deserialization must succeed");
        assert_eq!(q.count, Some(50));
        assert_eq!(q.start_index, Some(2));
        match q.filter {
            Some(MaybeFilter::Invalid(err)) => {
                assert_eq!(err.raw, r#"userName eq "alice"#);
            }
            other => panic!("expected Invalid filter, got {other:?}"),
        }
    }

    #[test]
    fn test_strict_list_query_rejects_invalid_filter() {
        // The non-tolerant variant must still hard-fail on a malformed
        // filter, preserving the existing strict contract for callers that
        // opt in to the default.
        let json = r#"{
            "filter": "userName eq \"alice",
            "count": 50
        }"#;
        let result: Result<ListQuery, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "strict ListQuery<Filter> must reject malformed filters"
        );
    }

    #[test]
    fn test_tolerant_search_request_invalid_filter_preserves_other_fields() {
        let json = r#"{
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:SearchRequest"],
            "filter": "emails[type eq",
            "startIndex": 3,
            "count": 25
        }"#;
        let req: TolerantSearchRequest =
            serde_json::from_str(json).expect("tolerant deserialization must succeed");
        assert_eq!(req.start_index, Some(3));
        assert_eq!(req.count, Some(25));
        assert!(matches!(req.filter, Some(MaybeFilter::Invalid(_))));
    }

    #[test]
    fn test_into_strict_list_query_valid_filter() {
        let json = r#"{ "filter": "userName eq \"alice\"", "count": 5 }"#;
        let tolerant: TolerantListQuery = serde_json::from_str(json).unwrap();
        let strict: StrictListQuery = tolerant
            .into_strict()
            .expect("valid filter must convert cleanly");
        assert_eq!(strict.count, Some(5));
        assert!(matches!(strict.filter, Some(Filter::Attr(_))));
    }

    #[test]
    fn test_into_strict_list_query_invalid_filter_returns_error() {
        let json = r#"{ "filter": "userName eq \"alice", "count": 5 }"#;
        let tolerant: TolerantListQuery = serde_json::from_str(json).unwrap();
        let err = tolerant
            .into_strict()
            .expect_err("malformed filter must produce InvalidFilterError");
        assert_eq!(err.raw, r#"userName eq "alice"#);
    }

    #[test]
    fn test_into_strict_search_request_invalid_filter_returns_error() {
        let json = r#"{
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:SearchRequest"],
            "filter": "emails[type eq",
            "startIndex": 2,
            "count": 15
        }"#;
        let tolerant: TolerantSearchRequest = serde_json::from_str(json).unwrap();
        let err = tolerant
            .into_strict()
            .expect_err("malformed filter must produce InvalidFilterError");
        assert_eq!(err.raw, "emails[type eq");
    }

    #[test]
    fn test_into_strict_passes_through_none_filter() {
        let tolerant: TolerantListQuery = serde_json::from_str(r#"{ "count": 7 }"#).unwrap();
        let strict = tolerant.into_strict().expect("no filter must succeed");
        assert!(strict.filter.is_none());
        assert_eq!(strict.count, Some(7));
    }

    #[test]
    fn test_strict_list_query_round_trip() {
        // The documented client-side build-and-serialize path still works
        // unchanged under the default F = Filter.
        let filter: Filter = r#"userName eq "alice""#.parse().unwrap();
        let q = ListQuery {
            filter: Some(filter),
            count: Some(10),
            ..ListQuery::default()
        };
        let json = serde_json::to_string(&q).expect("serialize ListQuery<Filter>");
        assert!(json.contains(r#""filter":"userName eq \"alice\"""#));
        let round: ListQuery = serde_json::from_str(&json).expect("round-trip");
        assert_eq!(round.count, Some(10));
        assert!(matches!(round.filter, Some(Filter::Attr(_))));
    }

    // ---- SearchRequest: every field except `schemas` is optional ----

    #[test]
    fn test_search_request_deserializes_with_only_schemas() {
        // A payload carrying nothing but the mandatory `schemas` member must
        // deserialize, leaving all of `attributes`, `excludedAttributes`,
        // `filter`, `startIndex` and `count` unset (RFC 7644 §3.4.3 lists them
        // all as optional).
        let json = r#"{
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:SearchRequest"]
        }"#;
        let req: StrictSearchRequest =
            serde_json::from_str(json).expect("minimal SearchRequest must deserialize");
        assert_eq!(req.schemas, vec![schema_urns::SEARCH_REQUEST.to_string()]);
        assert!(req.attributes.is_empty());
        assert!(req.excluded_attributes.is_empty());
        assert!(req.filter.is_none());
        assert!(req.start_index.is_none());
        assert!(req.count.is_none());
    }

    #[test]
    fn test_search_request_tolerant_deserializes_with_only_schemas() {
        let json = r#"{
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:SearchRequest"]
        }"#;
        let req: TolerantSearchRequest =
            serde_json::from_str(json).expect("minimal tolerant SearchRequest must deserialize");
        assert!(req.filter.is_none());
        assert!(req.start_index.is_none());
        assert!(req.count.is_none());
    }

    #[test]
    fn test_search_request_omits_unset_optional_fields_when_serialized() {
        // Mirror image of the deserialization case: a request with only
        // `schemas` set must serialize to just that key, so servers are not
        // sent `null`s or defaulted pagination values the caller never chose.
        let req = SearchRequest::<Filter> {
            schemas: vec![schema_urns::SEARCH_REQUEST.to_string()],
            attributes: Vec::new(),
            excluded_attributes: Vec::new(),
            filter: None,
            sort_by: None,
            sort_order: None,
            start_index: None,
            count: None,
        };
        let json = serde_json::to_value(&req).expect("serialize SearchRequest");
        assert_eq!(
            json,
            serde_json::json!({
                "schemas": ["urn:ietf:params:scim:api:messages:2.0:SearchRequest"]
            })
        );
    }

    #[test]
    fn test_search_request_partial_pagination_fields() {
        // `startIndex` and `count` are independently optional: supplying one
        // must not force the other to be present.
        let json = r#"{
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:SearchRequest"],
            "count": 10
        }"#;
        let req: StrictSearchRequest =
            serde_json::from_str(json).expect("SearchRequest with only `count` must deserialize");
        assert!(req.start_index.is_none());
        assert_eq!(req.count, Some(10));
    }

    #[test]
    fn test_into_strict_search_request_with_only_schemas() {
        // The tolerant -> strict conversion must pass through cleanly when no
        // filter is present, regardless of the pagination fields being unset.
        let json = r#"{
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:SearchRequest"]
        }"#;
        let tolerant: TolerantSearchRequest = serde_json::from_str(json).unwrap();
        let strict = tolerant
            .into_strict()
            .expect("no filter must convert without error");
        assert!(strict.filter.is_none());
        assert!(strict.start_index.is_none());
        assert!(strict.count.is_none());
    }

    #[test]
    fn test_search_request_all_fields_round_trip() {
        let json = r#"{
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:SearchRequest"],
            "attributes": ["userName"],
            "excludedAttributes": ["password"],
            "filter": "userName eq \"alice\"",
            "startIndex": 5,
            "count": 20
        }"#;
        let req: StrictSearchRequest =
            serde_json::from_str(json).expect("full SearchRequest must deserialize");
        assert_eq!(req.attributes, vec!["userName".to_string()]);
        assert_eq!(req.excluded_attributes, vec!["password".to_string()]);
        assert!(matches!(req.filter, Some(Filter::Attr(_))));
        assert_eq!(req.start_index, Some(5));
        assert_eq!(req.count, Some(20));

        let round: StrictSearchRequest = serde_json::from_str(
            &serde_json::to_string(&req).expect("serialize full SearchRequest"),
        )
        .expect("round-trip");
        assert_eq!(round.start_index, Some(5));
        assert_eq!(round.count, Some(20));
    }

    // ---- Resource<T> URN-based dispatch (RFC 7643 §§3-4) ----

    mod resource_dispatch {
        use super::*;
        use crate::models::group::Group;
        use crate::models::resource_types::ResourceType;
        use crate::models::scim_schema::Schema;
        use crate::models::user::User;
        use pretty_assertions::assert_eq;

        fn user_json() -> String {
            format!(
                r#"{{
                    "schemas": ["{user}"],
                    "id": "u-1",
                    "userName": "alice@example.com"
                }}"#,
                user = schema_urns::USER
            )
        }

        fn group_json() -> String {
            format!(
                r#"{{
                    "schemas": ["{group}"],
                    "id": "g-1",
                    "displayName": "Admins"
                }}"#,
                group = schema_urns::GROUP
            )
        }

        fn schema_json() -> String {
            format!(
                r#"{{
                    "schemas": ["{schema}"],
                    "id": "{user}",
                    "name": "User",
                    "description": "SCIM User schema",
                    "attributes": [],
                    "meta": {{}}
                }}"#,
                schema = schema_urns::SCHEMA,
                user = schema_urns::USER
            )
        }

        fn resource_type_json() -> String {
            format!(
                r#"{{
                    "schemas": ["{rt}"],
                    "id": "User",
                    "name": "User",
                    "endpoint": "/Users",
                    "schema": "{user}"
                }}"#,
                rt = schema_urns::RESOURCE_TYPE,
                user = schema_urns::USER
            )
        }

        #[test]
        fn dispatches_user_by_urn() {
            let parsed: Resource<String> = serde_json::from_str(&user_json()).unwrap();
            match parsed {
                Resource::User(u) => {
                    assert_eq!(u.user_name, "alice@example.com");
                    assert_eq!(u.id.as_deref(), Some("u-1"));
                }
                other => panic!("expected User, got {other:?}"),
            }
        }

        #[test]
        fn dispatches_group_by_urn() {
            let parsed: Resource<String> = serde_json::from_str(&group_json()).unwrap();
            match parsed {
                Resource::Group(g) => {
                    assert_eq!(g.display_name, "Admins");
                    assert_eq!(g.id.as_deref(), Some("g-1"));
                }
                other => panic!("expected Group, got {other:?}"),
            }
        }

        #[test]
        fn dispatches_user_with_enterprise_extension() {
            let json = format!(
                r#"{{
                    "schemas": ["{user}", "{ent}"],
                    "id": "u-1",
                    "userName": "bob@example.com"
                }}"#,
                user = schema_urns::USER,
                ent = schema_urns::ENTERPRISE_USER
            );
            let parsed: Resource<String> = serde_json::from_str(&json).unwrap();
            assert!(matches!(parsed, Resource::User(_)));
        }

        #[test]
        fn dispatches_enterprise_user_only_as_user() {
            // Real-world non-conformant providers sometimes ship only the
            // enterprise extension URN. Treat it as a User signal.
            let json = format!(
                r#"{{
                    "schemas": ["{ent}"],
                    "userName": "carol@example.com"
                }}"#,
                ent = schema_urns::ENTERPRISE_USER
            );
            let parsed: Resource<String> = serde_json::from_str(&json).unwrap();
            assert!(matches!(parsed, Resource::User(_)));
        }

        #[test]
        fn dispatches_schema_by_urn() {
            let parsed: Resource<String> = serde_json::from_str(&schema_json()).unwrap();
            assert!(matches!(parsed, Resource::Schema(_)));
        }

        #[test]
        fn dispatches_schema_without_schemas_field_via_attributes() {
            // /Schemas responses often omit a top-level `schemas` field; the
            // `attributes` array is the structural discriminator per RFC 7643 §7.
            let json = format!(
                r#"{{
                    "id": "{user}",
                    "name": "User",
                    "description": "SCIM User schema",
                    "attributes": [],
                    "meta": {{}}
                }}"#,
                user = schema_urns::USER
            );
            let parsed: Resource<String> = serde_json::from_str(&json).unwrap();
            assert!(matches!(parsed, Resource::Schema(_)));
        }

        #[test]
        fn dispatches_resource_type_by_urn() {
            let parsed: Resource<String> = serde_json::from_str(&resource_type_json()).unwrap();
            assert!(matches!(parsed, Resource::ResourceType(_)));
        }

        #[test]
        fn dispatches_resource_type_without_schemas_field() {
            let json = format!(
                r#"{{
                    "id": "User",
                    "name": "User",
                    "endpoint": "/Users",
                    "schema": "{user}"
                }}"#,
                user = schema_urns::USER
            );
            let parsed: Resource<String> = serde_json::from_str(&json).unwrap();
            assert!(matches!(parsed, Resource::ResourceType(_)));
        }

        #[test]
        fn rejects_ambiguous_user_and_group_urns() {
            // The confusion attack: overlapping fields plus both URNs.
            let json = format!(
                r#"{{
                    "schemas": ["{user}", "{group}"],
                    "id": "x-1",
                    "userName": "mallory@example.com",
                    "displayName": "Mallory"
                }}"#,
                user = schema_urns::USER,
                group = schema_urns::GROUP
            );
            let err = serde_json::from_str::<Resource<String>>(&json).unwrap_err();
            assert!(
                err.to_string().to_lowercase().contains("ambiguous"),
                "expected ambiguity error, got: {err}"
            );
        }

        #[test]
        fn rejects_empty_schemas() {
            let json = r#"{
                "schemas": [],
                "userName": "dave@example.com"
            }"#;
            let err = serde_json::from_str::<Resource<String>>(json).unwrap_err();
            assert!(err.to_string().contains("empty"), "got: {err}");
        }

        #[test]
        fn rejects_unknown_urn_with_structural_overlap() {
            // A payload that today would silently deserialize as User (userName
            // present, schemas listed) must now be rejected because the URN is
            // not recognized. Attacker-crafted shape.
            let json = r#"{
                "schemas": ["urn:evil:fake"],
                "userName": "eve@example.com",
                "displayName": "Eve"
            }"#;
            let err = serde_json::from_str::<Resource<String>>(json).unwrap_err();
            assert!(
                err.to_string().contains("no recognized"),
                "expected URN-recognition error, got: {err}"
            );
        }

        #[test]
        fn rejects_missing_discriminators() {
            let json = r#"{
                "id": "x-1",
                "externalId": "ext-1"
            }"#;
            let err = serde_json::from_str::<Resource<String>>(json).unwrap_err();
            assert!(err.to_string().contains("cannot determine"), "got: {err}");
        }

        #[test]
        fn group_payload_is_not_misclassified_as_user() {
            // Exact attack shape from the review. A Group with a User URN
            // should not be silently accepted; but a Group with the correct
            // Group URN must always land as Group even if User-ish fields
            // could structurally parse.
            let parsed: Resource<String> = serde_json::from_str(&group_json()).unwrap();
            match parsed {
                Resource::Group(g) => assert_eq!(g.display_name, "Admins"),
                other => panic!("expected Group, got {other:?}"),
            }
        }

        #[test]
        fn user_urn_with_malformed_user_surfaces_inner_error() {
            // schemas asserts this IS a User. Missing `userName` must surface
            // as an error — NOT fall through to the Group variant.
            let json = format!(
                r#"{{
                    "schemas": ["{user}"],
                    "id": "u-1"
                }}"#,
                user = schema_urns::USER
            );
            let err = serde_json::from_str::<Resource<String>>(&json).unwrap_err();
            let msg = err.to_string();
            assert!(
                msg.contains("userName") || msg.contains("user_name"),
                "expected inner User error mentioning userName, got: {msg}"
            );
        }

        #[test]
        fn round_trip_user() {
            let u: User = User::default();
            let r = Resource::User(Box::new(u));
            let json = serde_json::to_string(&r).unwrap();
            let back: Resource<String> = serde_json::from_str(&json).unwrap();
            assert!(matches!(back, Resource::User(_)));
        }

        #[test]
        fn round_trip_group() {
            let g: Group = Group {
                schemas: vec![schema_urns::GROUP.to_string()],
                id: Some("g-1".to_string()),
                external_id: None,
                display_name: "Admins".to_string(),
                members: Vec::new(),
                meta: None,
            };
            let r = Resource::Group(Box::new(g));
            let json = serde_json::to_string(&r).unwrap();
            let back: Resource<String> = serde_json::from_str(&json).unwrap();
            match back {
                Resource::Group(g) => assert_eq!(g.display_name, "Admins"),
                other => panic!("expected Group after round-trip, got {other:?}"),
            }
        }

        #[test]
        fn round_trip_schema() {
            let s: Schema = serde_json::from_str(&schema_json()).unwrap();
            let r: Resource<String> = Resource::Schema(Box::new(s));
            let json = serde_json::to_string(&r).unwrap();
            let back: Resource<String> = serde_json::from_str(&json).unwrap();
            assert!(matches!(back, Resource::Schema(_)));
        }

        #[test]
        fn round_trip_resource_type() {
            let rt: ResourceType = serde_json::from_str(&resource_type_json()).unwrap();
            let r: Resource<String> = Resource::ResourceType(Box::new(rt));
            let json = serde_json::to_string(&r).unwrap();
            let back: Resource<String> = serde_json::from_str(&json).unwrap();
            assert!(matches!(back, Resource::ResourceType(_)));
        }

        #[test]
        fn list_response_mixed_resources() {
            let json = format!(
                r#"{{
                    "itemsPerPage": 3,
                    "totalResults": 3,
                    "startIndex": 1,
                    "schemas": ["{lr}"],
                    "Resources": [{user}, {group}, {schema}]
                }}"#,
                lr = schema_urns::LIST_RESPONSE,
                user = user_json(),
                group = group_json(),
                schema = schema_json()
            );
            let list: ListResponse<Resource<String>> = serde_json::from_str(&json).unwrap();
            assert_eq!(list.resources.len(), 3);
            assert!(matches!(list.resources[0], Resource::User(_)));
            assert!(matches!(list.resources[1], Resource::Group(_)));
            assert!(matches!(list.resources[2], Resource::Schema(_)));
        }
    }
}
