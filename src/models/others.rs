use serde::de::{DeserializeOwned, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::filter::{Filter, InvalidFilterError, MaybeFilter, PatchPath};
use crate::models::group::Group;
use crate::models::resource_types::ResourceType;
use crate::models::scim_schema::Schema;
use crate::models::user::User;
use crate::schema_urns;

/// Server-side variant of [`ListQuery`] that tolerates malformed filter
/// expressions so the handler can produce an RFC 7644 §3.12 `invalidFilter`
/// error response instead of aborting deserialization of the whole query.
pub type TolerantListQuery = ListQuery<MaybeFilter>;

/// Server-side variant of [`SearchRequest`] with the same tolerant filter
/// behavior as [`TolerantListQuery`].
pub type TolerantSearchRequest = SearchRequest<MaybeFilter>;

/// [`ListQuery`] with a fully-parsed [`Filter`]. Equivalent to `ListQuery` with
/// its default type parameter; provided as a named alias for symmetry with
/// [`TolerantListQuery`] and as the `Ok` type of
/// [`ListQuery::<MaybeFilter>::into_strict`].
pub type StrictListQuery = ListQuery<Filter>;

/// [`SearchRequest`] with a fully-parsed [`Filter`]. Equivalent to
/// `SearchRequest` with its default type parameter; provided as a named alias
/// for symmetry with [`TolerantSearchRequest`] and as the `Ok` type of
/// [`SearchRequest::<MaybeFilter>::into_strict`].
pub type StrictSearchRequest = SearchRequest<Filter>;

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
            start_index: q.start_index,
            count: q.count,
            attributes: q.attributes,
            excluded_attributes: q.excluded_attributes,
        })
    }
}

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
            start_index: r.start_index,
            count: r.count,
        })
    }
}

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest<F = Filter> {
    pub schemas: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<F>,
    pub start_index: i64,
    pub count: i64,
}

impl<F> Default for SearchRequest<F> {
    fn default() -> Self {
        SearchRequest {
            schemas: vec![schema_urns::SEARCH_REQUEST.to_string()],
            attributes: None,
            excluded_attributes: None,
            filter: None,
            start_index: 1,
            count: 100,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery<F = Filter> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<F>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_attributes: Option<String>,
}

impl<F> Default for ListQuery<F> {
    fn default() -> Self {
        ListQuery {
            filter: None,
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
#[derive(Serialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", bound(deserialize = "T: DeserializeOwned"))]
pub struct ListResponse<T> {
    pub items_per_page: i64,
    pub total_results: i64,
    pub start_index: i64,
    pub schemas: Vec<String>,
    #[serde(rename = "Resources")]
    pub resources: Vec<Resource<T>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatchOp {
    pub schemas: Vec<String>,
    #[serde(rename = "Operations")]
    pub operations: Vec<PatchOperation>,
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::{
        AttrExp, AttrPath, CompValue, CompareOp, PatchPath, PatchValuePath, ValFilter,
    };
    use pretty_assertions::assert_eq;

    const PATCH_OP_SCHEMA: &str = schema_urns::PATCH_OP;

    #[test]
    fn test_patch_op_01_add_with_path() {
        let ops: PatchOp = serde_json::from_str(include_str!("../test_data/operations_01.json"))
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

    #[test]
    fn test_patch_op_02_add_without_path() {
        let ops: PatchOp = serde_json::from_str(include_str!("../test_data/operations_02.json"))
            .expect("Failed to deserialize patch operations");
        assert_eq!(ops.schemas, vec![PATCH_OP_SCHEMA]);
        assert_eq!(ops.operations.len(), 1);
        assert!(matches!(
            &ops.operations[0],
            PatchOperation::Add(OperationTarget::WithoutPath { .. })
        ));
    }

    #[test]
    fn test_patch_op_03_remove_member_by_filter() {
        let ops: PatchOp = serde_json::from_str(include_str!("../test_data/operations_03.json"))
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

    #[test]
    fn test_patch_op_04_remove_all_members() {
        let ops: PatchOp = serde_json::from_str(include_str!("../test_data/operations_04.json"))
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

    #[test]
    fn test_patch_op_05_remove_emails_compound_filter() {
        let ops: PatchOp = serde_json::from_str(include_str!("../test_data/operations_05.json"))
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

    #[test]
    fn test_patch_op_06_remove_then_add_members() {
        let ops: PatchOp = serde_json::from_str(include_str!("../test_data/operations_06.json"))
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

    #[test]
    fn test_patch_op_07_replace_members_list() {
        let ops: PatchOp = serde_json::from_str(include_str!("../test_data/operations_07.json"))
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

    #[test]
    fn test_patch_op_08_replace_work_address() {
        let ops: PatchOp = serde_json::from_str(include_str!("../test_data/operations_08.json"))
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

    #[test]
    fn test_patch_op_09_replace_street_address_via_filter() {
        let ops: PatchOp = serde_json::from_str(include_str!("../test_data/operations_09.json"))
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

    #[test]
    fn test_patch_op_10_replace_without_path() {
        let ops: PatchOp = serde_json::from_str(include_str!("../test_data/operations_10.json"))
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
        assert_eq!(req.start_index, 3);
        assert_eq!(req.count, 25);
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
                members: None,
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
            let list: ListResponse<String> = serde_json::from_str(&json).unwrap();
            assert_eq!(list.resources.len(), 3);
            assert!(matches!(list.resources[0], Resource::User(_)));
            assert!(matches!(list.resources[1], Resource::Group(_)));
            assert!(matches!(list.resources[2], Resource::Schema(_)));
        }
    }
}
