//! Property-based round-trips: for any well-formed resource the crate can
//! build, serialize-then-deserialize must give back an equal value.
//!
//! Every asymmetry this release fixed by hand — `Address.primary` dropped,
//! `[]` versus omitted, an `AuthenticationScheme` that could not be read
//! back — is a round-trip failure, and a generator finds that class
//! mechanically instead of one fixture at a time.

#![cfg(all(feature = "models", feature = "filter"))]

use proptest::prelude::*;
use scim_v2::ScimDateTime;
use scim_v2::models::enterprise_user::{EnterpriseUser, Manager};
use scim_v2::models::group::{Group, Member, MemberType};
use scim_v2::models::others::{ListResponse, Resource};
use scim_v2::models::others::{OperationTarget, PatchOp, PatchOperation};
use scim_v2::models::scim_schema::Meta;
use scim_v2::models::user::{
    Address, Email, Entitlement, Group as UserGroup, Im, Name, PhoneNumber, Photo, Role, User,
    X509Certificate,
};
use scim_v2::schema_urns;

fn opt_str() -> impl Strategy<Value = Option<String>> {
    proptest::option::of("[a-zA-Z0-9 @._-]{0,24}")
}
// `meta` timestamps are `ScimDateTime`, so the generator builds them from
// their parts rather than from arbitrary text: an invalid one is
// unconstructable, and a round-trip can only be checked on values that exist.
// The day stops at 28 so no month length or leap rule is in play here; those
// live in the parser's own tests.
prop_compose! {
    fn date_time()(year in 1u32..=9999, month in 1u32..=12, day in 1u32..=28,
                   hour in 0u32..=23, minute in 0u32..=59, second in 0u32..=59,
                   offset in prop::option::of(-14i32..=14)) -> ScimDateTime {
        let zone = match offset {
            None => String::new(),
            Some(0) => "Z".to_string(),
            Some(h) if h < 0 => format!("-{:02}:00", -h),
            Some(h) => format!("+{h:02}:00"),
        };
        format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}{zone}")
            .parse()
            .expect("built from the grammar")
    }
}
fn opt_date_time() -> impl Strategy<Value = Option<ScimDateTime>> {
    proptest::option::of(date_time())
}
fn opt_bool() -> impl Strategy<Value = Option<bool>> {
    proptest::option::of(any::<bool>())
}

prop_compose! {
    fn email()(value in opt_str(), display in opt_str(), t in opt_str(), primary in opt_bool()) -> Email {
        Email { value, display, r#type: t, primary }
    }
}
prop_compose! {
    fn phone()(value in opt_str(), display in opt_str(), t in opt_str(), primary in opt_bool()) -> PhoneNumber {
        PhoneNumber { value, display, r#type: t, primary }
    }
}
prop_compose! {
    fn address()(formatted in opt_str(), street_address in opt_str(), locality in opt_str(),
                 region in opt_str(), postal_code in opt_str(), country in opt_str(),
                 t in opt_str(), value in opt_str(), display in opt_str(), primary in opt_bool()) -> Address {
        Address { formatted, street_address, locality, region, postal_code, country, r#type: t, value, display, primary }
    }
}
prop_compose! {
    fn role()(value in opt_str(), display in opt_str(), t in opt_str(), primary in opt_bool()) -> Role {
        Role { value, display, r#type: t, primary }
    }
}
prop_compose! {
    fn im()(value in opt_str(), display in opt_str(), t in opt_str(), primary in opt_bool()) -> Im {
        Im { value, display, r#type: t, primary }
    }
}
prop_compose! {
    fn photo()(value in opt_str(), display in opt_str(), t in opt_str(), primary in opt_bool()) -> Photo {
        Photo { value, display, r#type: t, primary }
    }
}
prop_compose! {
    fn entitlement()(value in opt_str(), display in opt_str(), t in opt_str(), primary in opt_bool()) -> Entitlement {
        Entitlement { value, display, r#type: t, primary }
    }
}
prop_compose! {
    fn cert()(value in opt_str(), display in opt_str(), t in opt_str(), primary in opt_bool()) -> X509Certificate {
        X509Certificate { value, display, r#type: t, primary }
    }
}
prop_compose! {
    fn user_group()(value in opt_str(), r in opt_str(), display in opt_str(), t in opt_str()) -> UserGroup {
        UserGroup { value, r#ref: r, display, r#type: t }
    }
}
prop_compose! {
    fn name()(formatted in opt_str(), family_name in opt_str(), given_name in opt_str(),
              middle_name in opt_str(), honorific_prefix in opt_str(), honorific_suffix in opt_str()) -> Name {
        Name { formatted, family_name, given_name, middle_name, honorific_prefix, honorific_suffix }
    }
}
prop_compose! {
    fn meta()(resource_type in opt_str(), created in opt_date_time(),
              last_modified in opt_date_time(),
              version in opt_str(), location in opt_str()) -> Meta {
        Meta { resource_type, created, last_modified, version, location }
    }
}
prop_compose! {
    fn manager()(value in opt_str(), r in opt_str(), display_name in opt_str()) -> Manager {
        Manager { value, r#ref: r, display_name }
    }
}
prop_compose! {
    fn enterprise()(employee_number in opt_str(), cost_center in opt_str(), organization in opt_str(),
                    division in opt_str(), department in opt_str(),
                    manager in proptest::option::of(manager())) -> EnterpriseUser {
        EnterpriseUser { employee_number, cost_center, organization, division, department, manager }
    }
}
prop_compose! {
    fn user()(id in opt_str(), external_id in opt_str(), user_name in "[a-z][a-z0-9.@-]{0,20}",
              name in proptest::option::of(name()), display_name in opt_str(), nick_name in opt_str(),
              title in opt_str(), user_type in opt_str(), preferred_language in opt_str(),
              locale in opt_str(), timezone in opt_str(), active in opt_bool(), password in opt_str(),
              emails in proptest::collection::vec(email(), 0..3),
              phone_numbers in proptest::collection::vec(phone(), 0..3),
              addresses in proptest::collection::vec(address(), 0..2),
              roles in proptest::collection::vec(role(), 0..2),
              ims in proptest::collection::vec(im(), 0..2),
              photos in proptest::collection::vec(photo(), 0..2),
              groups in proptest::collection::vec(user_group(), 0..2),
              entitlements in proptest::collection::vec(entitlement(), 0..2),
              x509_certificates in proptest::collection::vec(cert(), 0..2),
              meta in proptest::option::of(meta()),
              enterprise_user in proptest::option::of(enterprise())) -> User<String> {
        User {
            schemas: vec![schema_urns::USER.to_string()],
            id, external_id, user_name, name, display_name, nick_name, title, user_type,
            preferred_language, locale, timezone, active, password,
            emails, phone_numbers, addresses, roles, ims, photos, groups, entitlements,
            x509_certificates, meta, enterprise_user,
            ..Default::default()
        }
    }
}
prop_compose! {
    fn member()(value in opt_str(), r in opt_str(), display in opt_str(),
                t in proptest::option::of(prop_oneof![
                    Just(MemberType::User), Just(MemberType::Group),
                    "[A-Za-z]{1,12}".prop_map(MemberType::from)
                ])) -> Member<String> {
        Member { value, r#ref: r, r#type: t, display }
    }
}
prop_compose! {
    fn group()(id in opt_str(), external_id in opt_str(), display_name in "[A-Za-z ]{1,20}",
               members in proptest::collection::vec(member(), 0..4),
               meta in proptest::option::of(meta())) -> Group<String> {
        Group { schemas: vec![schema_urns::GROUP.to_string()], id, external_id, display_name, members, meta }
    }
}

/// `User<String>` is ~1 KB and the composed strategies nest several `Option`s
/// and `Vec`s deep. In a debug build, proptest's value-tree generation and
/// shrinking for one case can overflow the 2 MB default test-thread stack —
/// it did, non-deterministically and in a different property each run. The
/// overflow is in the *driver*, not in the round-trip itself, so the whole
/// `proptest!` invocation runs on a thread with an explicit 64 MB stack. This
/// keeps the test independent of `RUST_MIN_STACK` and of the runner.
fn on_big_stack<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(f)
        .expect("spawn")
        .join()
        .unwrap_or_else(|e| std::panic::resume_unwind(e));
}

fn round_trip<T>(v: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(v).expect("serialize");
    let back: T = serde_json::from_str(&json).unwrap_or_else(|e| panic!("deserialize {json}: {e}"));
    assert_eq!(
        &back, v,
        "round-trip changed the value; wire form was {json}"
    );
}

fn config() -> ProptestConfig {
    ProptestConfig::with_cases(256)
}

#[test]
fn user_round_trips() {
    on_big_stack(|| proptest!(config(), |(u in user())| { round_trip(&u); }));
}

#[test]
fn group_round_trips() {
    on_big_stack(|| proptest!(config(), |(g in group())| { round_trip(&g); }));
}

#[test]
fn list_response_of_users_round_trips() {
    on_big_stack(|| {
        proptest!(config(), |(users in proptest::collection::vec(user(), 0..4))| {
            let total = users.len() as i64;
            let list = ListResponse::<User<String>> {
                schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
                total_results: total,
                start_index: Some(1),
                items_per_page: Some(total),
                resources: users,
            };
            round_trip(&list);
        })
    });
}

#[test]
fn heterogeneous_list_response_round_trips() {
    on_big_stack(|| {
        proptest!(config(), |(users in proptest::collection::vec(user(), 0..2),
                              groups in proptest::collection::vec(group(), 0..2))| {
            let mut resources: Vec<Resource<String>> =
                users.into_iter().map(|u| Resource::User(Box::new(u))).collect();
            resources.extend(groups.into_iter().map(|g| Resource::Group(Box::new(g))));
            let total = resources.len() as i64;
            let list = ListResponse::<Resource<String>> {
                schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
                total_results: total,
                start_index: Some(1),
                items_per_page: Some(total),
                resources,
            };
            round_trip(&list);
        })
    });
}

/// RFC 7643 §2.1: any casing of any known key deserializes to the same value
/// as the canonical casing.
#[test]
fn any_key_casing_deserializes_identically() {
    fn recase(v: &serde_json::Value, upper: bool) -> serde_json::Value {
        match v {
            serde_json::Value::Object(m) => serde_json::Value::Object(
                m.iter()
                    .map(|(k, v)| {
                        let k = if upper {
                            k.to_uppercase()
                        } else {
                            k.to_lowercase()
                        };
                        (k, recase(v, upper))
                    })
                    .collect(),
            ),
            serde_json::Value::Array(a) => {
                serde_json::Value::Array(a.iter().map(|x| recase(x, upper)).collect())
            }
            other => other.clone(),
        }
    }
    on_big_stack(|| {
        proptest!(config(), |(users in proptest::collection::vec(user(), 1..3), upper in any::<bool>())| {
            // A resource.
            let u = users[0].clone();
            let canonical = serde_json::to_value(&u).unwrap();
            let back: User<String> = scim_v2::case_insensitive::from_value(recase(&canonical, upper))
                .expect("recased User must deserialize");
            prop_assert_eq!(back, u);

            // A list envelope — `Resources` is the capitalised member whose
            // collision in the table shipped green.
            let total = users.len() as i64;
            let list = ListResponse::<User<String>> {
                schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
                total_results: total, start_index: Some(1), items_per_page: Some(total),
                resources: users,
            };
            let canonical = serde_json::to_value(&list).unwrap();
            let back: ListResponse<User<String>> =
                scim_v2::case_insensitive::from_value(recase(&canonical, upper)).expect("recased ListResponse");
            prop_assert_eq!(back.resources.len(), list.resources.len());
            prop_assert_eq!(back, list);

            // A PATCH envelope — `Operations` is the other one.
            let patch = PatchOp {
                schemas: vec![schema_urns::PATCH_OP.to_string()],
                operations: vec![PatchOperation::Replace(OperationTarget::WithPath {
                    path: "nickName".parse().unwrap(),
                    value: Some(serde_json::json!("x")),
                })],
            };
            let canonical = serde_json::to_value(&patch).unwrap();
            let back: PatchOp = scim_v2::case_insensitive::from_value(recase(&canonical, upper)).expect("recased PatchOp");
            prop_assert_eq!(back.operations.len(), 1);
        })
    });
}
