use super::*;
use crate::utils::validation::ValidationErrorKind;
use test_case::test_case;

/// The four `ListResponse::validate` bounds, each pinned
/// with the wire path it reports.
#[test_case(-5, 0, Some(1), Some(1), "totalResults", "must not be negative" ; "negative_total")]
#[test_case(-5, 3, Some(1), Some(3), "totalResults", "must not be negative" ; "negative_total_with_a_page")]
#[test_case(1, 2, Some(1), Some(2), "totalResults", "resources returned but totalResults is" ; "more_returned_than_total")]
#[test_case(3, 1, Some(0), Some(1), "startIndex", "" ; "zero_based_start_index")]
#[test_case(3, 1, Some(1), Some(-1), "itemsPerPage", "" ; "negative_items_per_page")]
fn validate_rejects_impossible_pagination(
    total: i64,
    returned: usize,
    start: Option<i64>,
    per_page: Option<i64>,
    path: &str,
    detail: &str,
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
    // `path` alone could not tell the negative-total branch from the
    // more-returned-than-total one, so the detail text pins each to its own.
    let err = list.validate().expect_err(path);
    assert_eq!(err.path(), path);
    assert!(err.to_string().contains(detail), "{err}");
}

/// An explicit `"Resources": null` is a form providers send, and it
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
        let patch: PatchOp = serde_json::from_str(&body).unwrap_or_else(|e| panic!("{op}: {e}"));
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

    // And the RFC's own example is conformant.
    let add_member: PatchOp = serde_json::from_str(include_str!(
        "../../test_data/rfc7644/s3.5.2.1_add_member.json"
    ))
    .unwrap();
    assert_eq!(add_member.validate(), Ok(()));
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
    let body =
        format!(r#"{{"schemas":["{schema}"],"totalResults":0,"startIndex":1,"itemsPerPage":0}}"#);
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
    let body =
        format!(r#"{{"schemas":["{schema}"],"totalResults":0,"startIndex":1,"itemsPerPage":0}}"#);
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
fn validate_list_response_accepts_count_zero_page_with_markers() {
    // RFC 7644 §3.4.2.4 Table 6: `count=0` "indicates that no resource
    // results are to be returned except for totalResults" — a paginated
    // page of size zero, which is conformant with its markers present and
    // not without them.
    let mut list: ListResponse<Resource<String>> = ListResponse {
        schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
        total_results: 5,
        items_per_page: Some(0),
        start_index: Some(1),
        resources: vec![],
    };
    assert_eq!(list.validate(), Ok(()));
    list.items_per_page = None;
    assert_eq!(list.validate().unwrap_err().path(), "itemsPerPage");
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
    let raw = include_str!("../../test_data/rfc7644/s3.4.2_list_response.json");
    let list: ListResponse<Resource<String>> =
        serde_json::from_str(raw).expect("RFC 7644 §3.4.2 list response must deserialize");
    assert_eq!(list.total_results, 2);
    assert_eq!(list.start_index, None);
    assert_eq!(list.items_per_page, None);
    assert_eq!(list.resources.len(), 2);
    assert!(matches!(list.resources[0], Resource::User(_)));

    let reserialized: Value = serde_json::from_str(&serde_json::to_string(&list).unwrap()).unwrap();
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
    let raw = include_str!("../../test_data/rfc7644/s3.4.2.4_fig3_pagination_response.json");
    let list: ListResponse<Resource<String>> =
        serde_json::from_str(raw).expect("RFC 7644 Figure 3 must deserialize");
    assert_eq!(list.total_results, 100);
    assert_eq!(list.items_per_page, Some(10));
    assert_eq!(list.start_index, Some(1));
    assert_eq!(list.resources.len(), 1);
    assert!(matches!(list.resources[0], Resource::User(_)));

    let reserialized: Value = serde_json::from_str(&serde_json::to_string(&list).unwrap()).unwrap();
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
    let raw = include_str!("../../test_data/rfc7644/s3.4.3_fig5_post_query_response.json");
    let list: ListResponse<Resource<String>> =
        serde_json::from_str(raw).expect("RFC 7644 Figure 5 must deserialize");
    assert_eq!(list.total_results, 100);
    assert_eq!(list.items_per_page, Some(10));
    assert_eq!(list.start_index, Some(1));
    assert!(matches!(list.resources[0], Resource::User(_)));
    assert!(matches!(list.resources[1], Resource::Group(_)));

    let reserialized: Value = serde_json::from_str(&serde_json::to_string(&list).unwrap()).unwrap();
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
    let raw = include_str!("../../test_data/rfc7644/s3.4.3_fig4_search_request.json");
    let req: SearchRequest = serde_json::from_str(raw).expect("RFC 7644 Figure 4 must deserialize");
    assert_eq!(req.start_index, Some(1));
    assert_eq!(req.count, Some(10));
    assert_eq!(
        req.attributes,
        vec!["displayName".to_string(), "userName".to_string()]
    );
    assert!(req.filter.is_some());

    // The parsed filter must serialize back to the RFC's filter string.
    let reserialized: Value = serde_json::from_str(&serde_json::to_string(&req).unwrap()).unwrap();
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
        "../../test_data/rfc7644/s3.5.2.1_add_member.json"
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
        "../../test_data/rfc7644/s3.5.2.1_add_user_attributes.json"
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
        "../../test_data/rfc7644/s3.5.2.2_remove_member_by_filter.json"
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
        "../../test_data/rfc7644/s3.5.2.2_remove_all_members.json"
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
        "../../test_data/rfc7644/s3.5.2.2_remove_complex_attribute.json"
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
                    filter: ValFilter::And(items),
                    sub_attr: None,
                }),
            value: None,
        } if attr_name == "emails" => {
            let [left, right] = items.as_slice() else {
                panic!("expected two And operands, got {items:?}");
            };
            match left {
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
            match right {
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
        "../../test_data/rfc7644/s3.5.2.2_remove_by_filter_then_add_member.json"
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
        "../../test_data/rfc7644/s3.5.2.2_replace_all_members.json"
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
        "../../test_data/rfc7644/s3.5.2.3_replace_members_single_op.json"
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
        "../../test_data/rfc7644/s3.5.2.3_replace_work_address.json"
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
        "../../test_data/rfc7644/s3.5.2.3_replace_street_address_via_filter.json"
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
            assert_eq!(
                value.as_ref().and_then(Value::as_str),
                Some("1010 Broadway Ave")
            );
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
        "../../test_data/rfc7644/s3.5.2.3_replace_multiple_attributes.json"
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
    let q: TolerantListQuery = serde_json::from_str(json).expect("deserialization must succeed");
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

    let round: StrictSearchRequest =
        serde_json::from_str(&serde_json::to_string(&req).expect("serialize full SearchRequest"))
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

/// The deep-nesting abort through the protocol message a server actually deserializes: a deep
/// `not (` chain with a trailing token inside `SearchRequest.filter`, on a
/// 2 MiB thread (tokio's default worker stack), must be an error and not an
/// abort. Both the strict `Filter` field and the tolerant `MaybeFilter` one.
#[test]
fn deep_not_chain_in_a_search_request_is_rejected_not_fatal() {
    std::thread::Builder::new()
        .stack_size(2 * 1024 * 1024)
        .spawn(|| {
            let deep = format!(
                "{}title pr{} and",
                "not (".repeat(40_000),
                ")".repeat(40_000)
            );
            let body = serde_json::json!({
                "schemas": [schema_urns::SEARCH_REQUEST],
                "filter": deep,
            })
            .to_string();
            assert!(serde_json::from_str::<SearchRequest<Filter>>(&body).is_err());
            let tolerant: SearchRequest<MaybeFilter> = serde_json::from_str(&body).unwrap();
            assert!(matches!(tolerant.filter, Some(MaybeFilter::Invalid(_))));
        })
        .unwrap()
        .join()
        .expect("must return, not abort");
}

/// RFC 7644 §3.4.2.4 Table 6 says a negative `count`
/// "SHALL be interpreted as 0" and a `startIndex` below 1 as 1, so neither is
/// a validation error; the effective accessors apply the table.
#[test]
fn search_request_out_of_range_pagination_is_interpreted_not_rejected() {
    let req = SearchRequest::<Filter> {
        count: Some(-7),
        start_index: Some(0),
        ..Default::default()
    };
    assert_eq!(req.validate(), Ok(()));
    assert_eq!(req.effective_count(), Some(0));
    assert_eq!(req.effective_start_index(), 1);
    let req = SearchRequest::<Filter> {
        count: None,
        start_index: None,
        ..Default::default()
    };
    assert_eq!(req.effective_count(), None);
    assert_eq!(req.effective_start_index(), 1);
    let req = SearchRequest::<Filter> {
        count: Some(10),
        start_index: Some(5),
        ..Default::default()
    };
    assert_eq!(
        (req.effective_count(), req.effective_start_index()),
        (Some(10), 5)
    );
    let q = ListQuery::<Filter> {
        count: Some(-1),
        start_index: Some(-3),
        ..Default::default()
    };
    assert_eq!(
        (q.effective_count(), q.effective_start_index()),
        (Some(0), 1)
    );
}

/// RFC 7644 §3.9 makes `attributes` and `excludedAttributes` "mutually
/// exclusive"; §3.4.3 says `sortBy` "MUST be in standard attribute notation".
#[test]
fn search_request_rejects_both_selections_and_a_malformed_sort_by() {
    let both = SearchRequest::<Filter> {
        attributes: vec!["userName".to_string()],
        excluded_attributes: vec!["emails".to_string()],
        ..Default::default()
    };
    assert_eq!(both.validate().unwrap_err().path(), "excludedAttributes");

    for good in [
        "userName",
        "name.familyName",
        "urn:ietf:params:scim:schemas:core:2.0:User:userName",
        "emails.value",
    ] {
        let req = SearchRequest::<Filter> {
            sort_by: Some(good.to_string()),
            ..Default::default()
        };
        assert_eq!(req.validate(), Ok(()), "{good}");
    }
    for bad in [
        "",
        "user Name",
        "name.",
        "emails[type eq \"work\"]",
        "userName eq \"x\"",
        "1abc",
    ] {
        let req = SearchRequest::<Filter> {
            sort_by: Some(bad.to_string()),
            ..Default::default()
        };
        assert_eq!(req.validate().unwrap_err().path(), "sortBy", "{bad:?}");
    }
}

/// The Table 6 accessors pass in-range values through unchanged.
#[test]
fn list_query_effective_accessors_pass_in_range_values_through() {
    let q = ListQuery::<Filter> {
        count: Some(25),
        start_index: Some(4),
        ..Default::default()
    };
    assert_eq!(
        (q.effective_count(), q.effective_start_index()),
        (Some(25), 4)
    );
    let q = ListQuery::<Filter> {
        count: None,
        start_index: None,
        ..Default::default()
    };
    assert_eq!((q.effective_count(), q.effective_start_index()), (None, 1));
}

/// Either selection alone is fine; only the combination is the §3.9 error.
#[test]
fn search_request_accepts_either_selection_alone() {
    let only_attributes = SearchRequest::<Filter> {
        attributes: vec!["userName".to_string()],
        ..Default::default()
    };
    assert_eq!(only_attributes.validate(), Ok(()));
    let only_excluded = SearchRequest::<Filter> {
        excluded_attributes: vec!["emails".to_string()],
        ..Default::default()
    };
    assert_eq!(only_excluded.validate(), Ok(()));
}

/// RFC 7644 §3.5.2.1: "The operation MUST contain a "value" member whose
/// content specifies the value to be added". An omitted member and an
/// explicit `null` are different things, and only the omission is the error.
#[test]
fn patch_op_validate_requires_a_value_on_add() {
    let op = |json: &str| -> PatchOp {
        serde_json::from_str(&format!(
            r#"{{"schemas":["{}"],"Operations":[{json}]}}"#,
            schema_urns::PATCH_OP
        ))
        .unwrap_or_else(|e| panic!("{json}: {e}"))
    };

    let err = op(r#"{"op":"add","path":"nickName"}"#)
        .validate()
        .expect_err("an add with no value member");
    assert_eq!(err.path(), "Operations[0].value");
    assert_eq!(err.kind(), &ValidationErrorKind::MissingRequiredAttribute);

    // The index is the operation's own, so a server can point at it.
    let err = op(r#"{"op":"remove","path":"nickName"},{"op":"add","path":"nickName"}"#)
        .validate()
        .unwrap_err();
    assert_eq!(err.path(), "Operations[1].value");

    // Present, including an explicit null, satisfies the MUST.
    assert_eq!(
        op(r#"{"op":"add","path":"nickName","value":"bj"}"#).validate(),
        Ok(())
    );
    assert_eq!(
        op(r#"{"op":"add","path":"nickName","value":null}"#).validate(),
        Ok(())
    );
    // A `remove` needs no value, and §3.5.2.3 states its requirement only for
    // the pathless form, which the type already enforces.
    assert_eq!(
        op(r#"{"op":"remove","path":"nickName"}"#).validate(),
        Ok(())
    );
    assert_eq!(
        op(r#"{"op":"replace","path":"nickName"}"#).validate(),
        Ok(())
    );
    // A pathless add without a value never becomes a `PatchOp` at all.
    assert!(
        serde_json::from_str::<PatchOp>(&format!(
            r#"{{"schemas":["{}"],"Operations":[{{"op":"add"}}]}}"#,
            schema_urns::PATCH_OP
        ))
        .is_err()
    );
}

/// An omitted `value` stays omitted on the way out, and an explicit `null`
/// stays null, so a PATCH body round-trips rather than gaining a member.
#[test]
fn patch_operation_value_presence_round_trips() {
    for (raw, expected) in [
        (r#"{"op":"replace","path":"nickName"}"#, None),
        (
            r#"{"op":"replace","path":"nickName","value":null}"#,
            Some(Value::Null),
        ),
        (
            r#"{"op":"replace","path":"nickName","value":"x"}"#,
            Some(Value::String("x".into())),
        ),
    ] {
        let body = format!(
            r#"{{"schemas":["{}"],"Operations":[{raw}]}}"#,
            schema_urns::PATCH_OP
        );
        let patch: PatchOp = serde_json::from_str(&body).unwrap();
        let PatchOperation::Replace(OperationTarget::WithPath { value, .. }) = &patch.operations[0]
        else {
            panic!("{raw}");
        };
        assert_eq!(value, &expected, "{raw}");
        let back: PatchOp = serde_json::from_str(&serde_json::to_string(&patch).unwrap()).unwrap();
        assert_eq!(back, patch, "{raw}");
        let json = serde_json::to_value(&patch).unwrap();
        assert_eq!(
            json["Operations"][0].get("value").is_some(),
            expected.is_some(),
            "{raw}"
        );
    }
}

/// The `GET` query carrier enforces the same RFC 7644 §3.4.2.3 sort rules and
/// §3.9 selection rule as the `POST /.search` body, so the two endpoints do
/// not disagree about what a client may ask for.
#[test]
fn list_query_validate_mirrors_the_search_request_rules() {
    let base = ListQuery::<Filter>::default;

    assert_eq!(base().validate(), Ok(()), "the default query is conformant");

    let sort_order_alone = ListQuery::<Filter> {
        sort_order: Some(SortOrder::Descending),
        ..base()
    };
    assert_eq!(sort_order_alone.validate().unwrap_err().path(), "sortOrder");

    for good in [
        "userName",
        "name.familyName",
        "urn:ietf:params:scim:schemas:core:2.0:User:userName",
    ] {
        let q = ListQuery::<Filter> {
            sort_by: Some(good.to_string()),
            sort_order: Some(SortOrder::Ascending),
            ..base()
        };
        assert_eq!(q.validate(), Ok(()), "{good}");
    }
    for bad in ["", "user Name", "name.", "userName eq \"x\"", "1abc"] {
        let q = ListQuery::<Filter> {
            sort_by: Some(bad.to_string()),
            ..base()
        };
        assert_eq!(
            q.validate().map_err(|e| e.path().to_string()),
            Err("sortBy".to_string()),
            "{bad:?}"
        );
    }

    let both = ListQuery::<Filter> {
        attributes: Some("userName".to_string()),
        excluded_attributes: Some("emails".to_string()),
        ..base()
    };
    assert_eq!(both.validate().unwrap_err().path(), "excludedAttributes");
    for (attributes, excluded) in [
        (Some("userName".to_string()), None),
        (None, Some("emails".to_string())),
        (Some(String::new()), Some("emails".to_string())),
    ] {
        let q = ListQuery::<Filter> {
            attributes,
            excluded_attributes: excluded,
            ..base()
        };
        assert_eq!(q.validate(), Ok(()));
    }

    // Out-of-range pagination is interpreted, not rejected (§3.4.2.4 Table 6).
    let q = ListQuery::<Filter> {
        count: Some(-1),
        start_index: Some(0),
        ..base()
    };
    assert_eq!(q.validate(), Ok(()));

    // And the same rules reach a tolerant query, before and after conversion.
    let tolerant: TolerantListQuery =
        serde_json::from_str(r#"{"sortOrder":"descending","filter":"userName eq \"x\""}"#).unwrap();
    assert_eq!(tolerant.validate().unwrap_err().path(), "sortOrder");
    let strict: StrictListQuery = TolerantListQuery {
        sort_by: Some("name.".to_string()),
        ..serde_json::from_str(r#"{"filter":"userName eq \"x\""}"#).unwrap()
    }
    .try_into()
    .expect("the filter parses");
    assert_eq!(strict.validate().unwrap_err().path(), "sortBy");
}

/// RFC 7644 §3.9 makes `attributes` and `excludedAttributes` mutually
/// exclusive, so the default query carries neither rather than both as empty
/// selectors that would serialize into the query string.
#[test]
fn list_query_default_omits_both_attribute_selectors() {
    let json = serde_json::to_value(ListQuery::<Filter>::default()).unwrap();
    assert!(json.get("attributes").is_none(), "{json}");
    assert!(json.get("excludedAttributes").is_none(), "{json}");
    assert_eq!(json["startIndex"], 1);
    assert_eq!(json["count"], 100);
    let back: ListQuery<Filter> = serde_json::from_value(json).unwrap();
    assert_eq!(back, ListQuery::default());
}
