// -----------------------------------------------------------------------
// RFC 7644 §3.4.2.2 grammar permutations. Each row is a case the parser
// handled correctly when probed by hand; without these, a grammar
// regression on any of them would pass the suite.
// -----------------------------------------------------------------------

/// ABNF: `ATTRNAME = ALPHA *(nameChar)`, `nameChar = "-" / "_" / DIGIT / ALPHA`.
/// Keywords and operators are case-insensitive (§3.4.2.2). `compValue` is
/// JSON: `false / null / true / number / string`, so a JSON number in any
/// form is legal. Whitespace between tokens is any run of SP or HTAB.
#[test_case(r#"x509Certificates.value eq "abc""# ; "digit_inside_attrname")]
#[test_case(r#"my-attr eq "x""# ; "hyphen_in_attrname")]
#[test_case(r#"my_attr eq "x""# ; "underscore_in_attrname")]
#[test_case(r#"USERNAME EQ "x""# ; "attr_and_operator_uppercase")]
#[test_case("title PR" ; "pr_uppercase")]
#[test_case(r#"a eq "1" AND b eq "2""# ; "and_uppercase")]
#[test_case(r#"a eq "1" Or b eq "2""# ; "or_mixed_case")]
#[test_case("NOT (a pr)" ; "not_uppercase")]
#[test_case("age gt -1" ; "negative_integer")]
#[test_case("score eq 1.5" ; "decimal")]
#[test_case("n eq 1e3" ; "exponent")]
#[test_case("n eq 1.5e-3" ; "negative_exponent")]
#[test_case("userName eq null" ; "null_value")]
#[test_case("active eq true" ; "true_value")]
#[test_case("active eq false" ; "false_value")]
#[test_case("emails[primary pr]" ; "pr_inside_value_path")]
#[test_case(r#"not (emails[type eq "work"])"# ; "not_on_value_path")]
#[test_case(r#"urn:ietf:params:scim:schemas:extension:enterprise:2.0:User:manager.value eq "x""# ; "urn_prefix_with_sub_attr")]
#[test_case(r#"userName    eq     "x""# ; "multiple_spaces")]
#[test_case("userName\teq\t\"x\"" ; "tabs_as_whitespace")]
#[test_case("not (not (title pr))" ; "nested_not")]
#[test_case(r#"emails[type eq "work" or type eq "home"]"# ; "value_path_with_or")]
fn grammar_accepts(s: &str) {
    let f: Filter = s
        .parse()
        .unwrap_or_else(|e| panic!("{s:?} must parse: {e}"));
    let back: Filter = f.to_string().parse().expect("display output must re-parse");
    assert_eq!(f, back, "{s:?} must round-trip through Display");
}

/// The rejections the ABNF requires. `pr` takes no value; a `FILTER` may
/// contain a `valuePath` but not `valuePath "." subAttr` (that form exists
/// only in the PATCH `PATH` rule); JSON literals are lowercase; an
/// attribute name starts with ALPHA.
#[test_case(r#"1abc eq "x""# ; "attrname_leading_digit")]
#[test_case(r#"_abc eq "x""# ; "attrname_leading_underscore")]
#[test_case("active eq True" ; "capitalised_boolean_literal")]
#[test_case("userName eq bjensen" ; "bare_word_value")]
#[test_case(r#"title pr "x""# ; "pr_with_a_value")]
#[test_case("userName eq" ; "missing_comp_value")]
#[test_case(r#"(userName eq "x""# ; "unbalanced_open_paren")]
#[test_case(r#"userName eq "x")"# ; "unbalanced_close_paren")]
#[test_case("" ; "empty")]
#[test_case("   " ; "whitespace_only")]
#[test_case(r#"userName eq "x" garbage"# ; "trailing_garbage")]
#[test_case(r#"userName eq "x"# ; "unterminated_string")]
#[test_case(r#"emails[type eq "work"].value eq "x""# ; "value_path_sub_attr_in_filter")]
#[test_case(r#"emails[type eq "work"][primary eq true]"# ; "double_value_path")]
fn grammar_rejects(s: &str) {
    assert!(s.parse::<Filter>().is_err(), "{s:?} must be rejected");
}

/// RFC 7644 Figure 8 ("Example Path Values"), all five, plus the
/// URN-prefixed forms the `PATH` rule admits.
#[test_case("members" ; "fig8_attr")]
#[test_case("name.familyName" ; "fig8_sub_attr")]
#[test_case(r#"addresses[type eq "work"]"# ; "fig8_value_path")]
#[test_case(r#"members[value eq "2819c223-7f76-453a-919d-413861904646"]"# ; "fig8_value_path_by_id")]
#[test_case(r#"members[value eq "2819c223-7f76-453a-919d-413861904646"].displayName"# ; "fig8_value_path_sub_attr")]
#[test_case(r#"emails[type eq "work" and primary eq true].value"# ; "value_path_and_sub_attr")]
#[test_case("urn:ietf:params:scim:schemas:extension:enterprise:2.0:User:manager" ; "urn_prefixed_attr")]
#[test_case("urn:ietf:params:scim:schemas:extension:enterprise:2.0:User:manager.value" ; "urn_prefixed_sub_attr")]
fn patch_path_accepts(s: &str) {
    let p: PatchPath = s
        .parse()
        .unwrap_or_else(|e| panic!("{s:?} must parse: {e}"));
    let back: PatchPath = p.to_string().parse().expect("display output must re-parse");
    assert_eq!(p, back, "{s:?} must round-trip through Display");
}

#[test_case(r#"emails[type eq "work"][primary eq true]"# ; "double_value_path")]
#[test_case("" ; "empty")]
fn patch_path_rejects(s: &str) {
    assert!(s.parse::<PatchPath>().is_err(), "{s:?} must be rejected");
}

/// Every comparison operator and every `compValue` kind must survive
/// parse -> Display -> parse. This is the cheapest way to cover the
/// `Display` arms exhaustively; before it, `ew`, `false`, `not (...)` and
/// value-path `or` had no test touching their formatting at all.
#[test]
fn display_round_trips_every_operator_and_value_kind() {
    let exprs = [
        r#"userName eq "bjensen""#,
        r#"userName ne "bjensen""#,
        r#"userName co "jen""#,
        r#"userName sw "b""#,
        r#"userName ew "n""#,
        "title pr",
        r#"meta.lastModified gt "2011-05-13T04:42:34Z""#,
        r#"meta.lastModified lt "2011-05-13T04:42:34Z""#,
        r#"meta.lastModified ge "2011-05-13T04:42:34Z""#,
        r#"meta.lastModified le "2011-05-13T04:42:34Z""#,
        "active eq true",
        "active eq false",
        "active eq null",
        "loginCount eq 42",
        "score eq 3.14",
        r#"not (userName eq "bjensen")"#,
        r#"userName eq "a" and title pr"#,
        r#"userName eq "a" or title pr"#,
        r#"emails[type eq "work" and value co "@example.com"]"#,
        r#"emails[type eq "work" or type eq "home"]"#,
        r#"emails[not (type eq "work")]"#,
        r#"urn:ietf:params:scim:schemas:core:2.0:User:userName eq "bjensen""#,
    ];

    for raw in exprs {
        let parsed: Filter = raw.parse().unwrap_or_else(|e| panic!("{raw:?}: {e}"));
        let rendered = parsed.to_string();
        let reparsed: Filter = rendered
            .parse()
            .unwrap_or_else(|e| panic!("re-parsing {rendered:?} from {raw:?}: {e}"));
        assert_eq!(
            parsed, reparsed,
            "{raw:?} did not survive Display -> parse (rendered as {rendered:?})"
        );
    }
}

/// `AttrPath::with_name` is the programmatic constructor, the path callers
/// use when building a filter rather than parsing one.
#[test]
fn attr_path_can_be_built_without_parsing() {
    let path = AttrPath::with_name("userName");
    assert_eq!(path.name, "userName");
    assert_eq!(path.uri, None);
    assert_eq!(path.sub_attr, None);
    assert_eq!(path.to_string(), "userName");
}

/// A bare attribute name has no URI prefix to detect, which is the
/// `rsplit_once(':')` miss branch in `parse_attr_path`.
#[test]
fn attr_path_without_a_colon_has_no_uri() {
    let path = parse_attr_path("displayName").expect("a bare name is a valid attrPath");
    assert_eq!(path.uri, None);
    assert_eq!(path.name, "displayName");
}

/// `CompValue` converts from both owned and borrowed strings, so callers
/// building filters programmatically need no ceremony.
#[test]
fn comp_value_converts_from_strings() {
    assert_eq!(
        CompValue::from("bjensen".to_string()),
        CompValue::Str("bjensen".to_string())
    );
    assert_eq!(
        CompValue::from("bjensen"),
        CompValue::Str("bjensen".to_string())
    );
    // Display emits a JSON string literal, so an embedded quote is
    // escaped and the whole value is wrapped.
    assert_eq!(CompValue::from("a\"b").to_string(), "\"a\\\"b\"");
}

/// A `PatchPath` serializes back to the wire form a PATCH body carries.
#[test]
fn patch_path_serializes_to_its_wire_form() {
    for raw in [
        "members",
        "name.familyName",
        r#"members[value eq "2819c223"]"#,
        r#"addresses[type eq "work"].streetAddress"#,
    ] {
        let path: PatchPath = raw.parse().unwrap_or_else(|e| panic!("{raw:?}: {e}"));
        let json = serde_json::to_value(&path).expect("PatchPath must serialize");
        let reparsed: PatchPath =
            serde_json::from_value(json.clone()).expect("and deserialize back");
        assert_eq!(path, reparsed, "{raw:?} round-trip via {json}");
    }
}
use super::*;
use test_case::test_case;

#[test]
fn test_scim_filter_simple_eq() {
    let f = crate::filter_parser::FilterParser::new()
        .parse(&ParseBudget::default(), r#"userName eq "bjensen""#)
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
        .parse(&ParseBudget::default(), r#"userName Eq "john""#)
        .unwrap();
    let f2 = crate::filter_parser::FilterParser::new()
        .parse(&ParseBudget::default(), r#"userName eq "john""#)
        .unwrap();
    assert_eq!(f1, f2);
}

#[test]
fn test_scim_filter_pr() {
    let f = crate::filter_parser::FilterParser::new()
        .parse(&ParseBudget::default(), "title pr")
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
        .parse(&ParseBudget::default(), r#"name.familyName co "O'Malley""#)
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
        .parse(
            &ParseBudget::default(),
            r#"urn:ietf:params:scim:schemas:core:2.0:User:userName sw "J""#,
        )
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
        .parse(
            &ParseBudget::default(),
            r#"title pr and userType eq "Employee""#,
        )
        .unwrap();
    assert_eq!(
        f,
        Filter::And(Operands::pair(
            Filter::Attr(AttrExp::Present(AttrPath {
                uri: None,
                name: "title".into(),
                sub_attr: None,
            })),
            Filter::Attr(AttrExp::Comparison(
                AttrPath {
                    uri: None,
                    name: "userType".into(),
                    sub_attr: None
                },
                CompareOp::Eq,
                CompValue::Str("Employee".into()),
            )),
        ))
    );
}

#[test]
fn test_scim_filter_or() {
    let f = crate::filter_parser::FilterParser::new()
        .parse(
            &ParseBudget::default(),
            r#"title pr or userType eq "Intern""#,
        )
        .unwrap();
    assert_eq!(
        f,
        Filter::Or(Operands::pair(
            Filter::Attr(AttrExp::Present(AttrPath {
                uri: None,
                name: "title".into(),
                sub_attr: None,
            })),
            Filter::Attr(AttrExp::Comparison(
                AttrPath {
                    uri: None,
                    name: "userType".into(),
                    sub_attr: None
                },
                CompareOp::Eq,
                CompValue::Str("Intern".into()),
            )),
        ))
    );
}

#[test]
fn test_scim_filter_and_precedence_over_or() {
    // "A and B or C" should parse as "(A and B) or C"
    let f = crate::filter_parser::FilterParser::new()
        .parse(
            &ParseBudget::default(),
            r#"title pr and userType eq "Employee" or emails pr"#,
        )
        .unwrap();

    let Filter::Or(items) = f else {
        panic!("expected Or, got {f:?}");
    };
    assert_eq!(items.len(), 2);
    assert!(matches!(&items[0], Filter::And(inner) if inner.len() == 2));
    assert!(matches!(&items[1], Filter::Attr(_)));
}

#[test]
fn test_scim_filter_not() {
    let f = crate::filter_parser::FilterParser::new()
        .parse(&ParseBudget::default(), r#"not (emails co "example.com")"#)
        .unwrap();
    assert!(matches!(f, Filter::Not(_)));
}

#[test]
fn test_scim_filter_grouping() {
    // Parens change precedence: "A or (B and C)" — inner group is And
    let f = crate::filter_parser::FilterParser::new()
            .parse(&ParseBudget::default(), r#"userType eq "Employee" and (emails co "example.com" or emails.value co "example.org")"#)
            .unwrap();
    let Filter::And(items) = f else {
        panic!("expected And, got {f:?}");
    };
    assert_eq!(items.len(), 2);
    assert!(matches!(&items[1], Filter::Or(inner) if inner.len() == 2));
}

#[test]
fn test_scim_filter_value_path() {
    let f = crate::filter_parser::FilterParser::new()
        .parse(
            &ParseBudget::default(),
            r#"emails[type eq "work" and value co "@example.com"]"#,
        )
        .unwrap();
    assert!(matches!(f, Filter::ValuePath(_)));
    if let Filter::ValuePath(vp) = f {
        assert_eq!(vp.attr.name, "emails");
        assert!(matches!(&*vp.filter, ValFilter::And(items) if items.len() == 2));
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
        .parse(&ParseBudget::default(), "userName")
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
        .parse(&ParseBudget::default(), "name.familyName")
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
        .parse(&ParseBudget::default(), r#"emails[type eq "work"]"#)
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
        .parse(&ParseBudget::default(), r#"emails[type eq "work"].value"#)
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
    assert!(
        crate::filter_parser::PathParser::new()
            .parse(&ParseBudget::default(), s)
            .is_err()
    );
}

// Filter strings with invalid comp-values that must be rejected.
#[test_case(r#"userName eq "\q""# ; "unknown_escape")]
#[test_case(r#"userName eq "\uGHIJ""# ; "invalid_unicode_escape")]
fn filter_comp_value_rejected(s: &str) {
    assert!(
        crate::filter_parser::FilterParser::new()
            .parse(&ParseBudget::default(), s)
            .is_err()
    );
}

// Invalid attrPaths that do not conform to RFC 7644 Figure 1.
#[test_case("name." ; "empty_sub_attr")]
#[test_case("name.family.given pr" ; "too_many_sub_attrs")]
#[test_case(r#"emails[name.family.given eq "x"]"# ; "value_path_inner_too_many_sub_attrs")]
fn invalid_attr_path_rejected(s: &str) {
    assert!(
        crate::filter_parser::FilterParser::new()
            .parse(&ParseBudget::default(), s)
            .is_err()
    );
}

// The RFC ABNF requires SP separators around compare/logical operators.
#[test_case(r#"userName eq"bjensen""# ; "missing_space_before_comp_value")]
#[test_case(r#"emails[type eq"work"]"# ; "missing_space_in_value_path")]
#[test_case(r#"title prand userType eq "Employee""# ; "missing_space_before_and")]
fn filter_requires_required_spaces(s: &str) {
    assert!(
        crate::filter_parser::FilterParser::new()
            .parse(&ParseBudget::default(), s)
            .is_err()
    );
}

#[test]
fn test_comp_value_surrogate_pair_handled() {
    // \uD800\uDC00 is a surrogate pair representing U+10000
    let f = crate::filter_parser::FilterParser::new()
        .parse(&ParseBudget::default(), r#"userName eq "\uD800\uDC00""#)
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
        .parse(
            &ParseBudget::default(),
            r#"emails[type eq "work" and value co "@example.com"].value"#,
        )
        .unwrap();
    let PatchPath::Value(vp) = p else {
        panic!("expected PatchPath::Value");
    };
    let ValFilter::And(items) = &vp.filter else {
        panic!("expected ValFilter::And, got {:?}", vp.filter);
    };
    assert_eq!(items.len(), 2);
    assert_eq!(vp.sub_attr.as_deref(), Some("value"));
}

#[test]
fn test_val_filter_parenthesized_grouping() {
    // Parenthesized grouping inside [...] should be accepted
    let f = crate::filter_parser::FilterParser::new()
        .parse(
            &ParseBudget::default(),
            r#"emails[(type eq "work") and value pr]"#,
        )
        .unwrap();
    assert!(matches!(f, Filter::ValuePath(_)));
    if let Filter::ValuePath(vp) = f {
        assert!(matches!(&*vp.filter, ValFilter::And(items) if items.len() == 2));
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
// Depth- and term-limit enforcement (DoS hardening against pathological
// filters), enforced while parsing
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

fn chain(op: &str, terms: usize) -> String {
    assert!(terms >= 1);
    let mut s = String::from("title pr");
    for _ in 1..terms {
        s.push(' ');
        s.push_str(op);
        s.push_str(" title pr");
    }
    s
}

fn assert_too_many_terms<T: std::fmt::Debug>(res: Result<T, ParseError>, expected: usize) {
    match res {
        Err(LalrParseError::User {
            error: FilterActionError::TooManyTerms(n),
        }) => assert_eq!(n, expected, "reported term count"),
        Err(other) => panic!("expected ParseError::User(TooManyTerms), got {other:?}"),
        Ok(ok) => panic!("expected rejection, but parse succeeded: {ok:?}"),
    }
}

// A same-operator chain is one n-ary node: its depth is 2 regardless of
// length, so the depth cap never fires on it and the term cap is what
// bounds it.
#[test_case("and" ; "and_chain")]
#[test_case("or" ; "or_chain")]
fn chain_at_term_limit_parses_flat(op: &str) {
    let f: Filter = chain(op, MAX_FILTER_TERMS)
        .parse()
        .unwrap_or_else(|e| panic!("expected success with {MAX_FILTER_TERMS} terms: {e:?}"));
    let items = match &f {
        Filter::And(items) | Filter::Or(items) => items,
        other => panic!("expected a chain node, got {other:?}"),
    };
    assert_eq!(items.len(), MAX_FILTER_TERMS);
    assert!(
        filter_depth_exceeds(&f, 2).is_none(),
        "flat chain must have depth 2"
    );
    assert_eq!(filter_depth_exceeds(&f, 1), Some(2));
}

#[test_case("and" ; "and_chain")]
#[test_case("or" ; "or_chain")]
fn chain_exceeding_term_limit_rejected(op: &str) {
    let n = MAX_FILTER_TERMS + 1;
    assert_too_many_terms(chain(op, n).parse::<Filter>(), n);
}

// R2-I1: the case that motivated the n-ary AST. A client resolving a
// batch of ids in one `or` chain is an ordinary request; 100 terms is
// well past the old depth cap of 64 and must parse.
#[test]
fn hundred_term_or_chain_parses_and_round_trips() {
    let s = (0..100)
        .map(|i| format!(r#"id eq "{i:032x}""#))
        .collect::<Vec<_>>()
        .join(" or ");
    let f: Filter = s.parse().expect("100-term or chain parses");
    let Filter::Or(items) = &f else {
        panic!("expected Or, got {f:?}");
    };
    assert_eq!(items.len(), 100);
    assert!(items.iter().all(|i| matches!(i, Filter::Attr(_))));
    assert_eq!(f.to_string(), s);
    let reparsed: Filter = f.to_string().parse().unwrap();
    assert_eq!(f, reparsed);
}

// Same-operator grouping flattens: `and`/`or` are associative, so the
// parenthesised and unparenthesised forms produce identical trees.
#[test_case("a pr and (b pr and c pr)", "a pr and b pr and c pr" ; "and_right_grouped")]
#[test_case("(a pr and b pr) and c pr", "a pr and b pr and c pr" ; "and_left_grouped")]
#[test_case("a pr or (b pr or c pr)", "a pr or b pr or c pr" ; "or_right_grouped")]
#[test_case("(a pr or b pr) or c pr", "a pr or b pr or c pr" ; "or_left_grouped")]
#[test_case("(a pr and b pr) and (c pr and d pr)", "a pr and b pr and c pr and d pr" ; "and_both_grouped")]
fn same_operator_grouping_flattens(grouped: &str, flat: &str) {
    let g: Filter = grouped.parse().unwrap();
    let f: Filter = flat.parse().unwrap();
    assert_eq!(g, f);
    let (Filter::And(items) | Filter::Or(items)) = &g else {
        panic!("expected a chain node, got {g:?}");
    };
    assert_eq!(items.len(), flat.matches(" pr").count());
    assert_eq!(g.to_string(), flat);
}

// Mixed operators keep their structure: an `or` under an `and` is real
// nesting, and Display puts the parens back so the string round-trips.
#[test_case("(a pr or b pr) and c pr", &["Or", "Attr"] ; "or_under_and_keeps_parens")]
#[test_case("a pr and b pr or c pr", &["And", "Attr"] ; "and_under_or_no_parens_needed")]
#[test_case("a pr and (b pr or c pr) and d pr", &["Attr", "Or", "Attr"] ; "or_in_middle_of_and")]
fn mixed_operator_structure_preserved(src: &str, shape: &[&str]) {
    let f: Filter = src.parse().unwrap();
    let (Filter::And(items) | Filter::Or(items)) = &f else {
        panic!("expected a chain node, got {f:?}");
    };
    let got: Vec<&str> = items
        .iter()
        .map(|i| match i {
            Filter::Attr(_) => "Attr",
            Filter::And(_) => "And",
            Filter::Or(_) => "Or",
            Filter::Not(_) => "Not",
            Filter::ValuePath(_) => "ValuePath",
        })
        .collect();
    assert_eq!(got, shape);
    assert_eq!(f.to_string(), src);
    assert_eq!(f.to_string().parse::<Filter>().unwrap(), f);
}

#[test]
fn programmatic_constructors_flatten_like_the_parser() {
    let leaf = |n: &str| {
        Filter::Attr(AttrExp::Present(AttrPath {
            uri: None,
            name: n.into(),
            sub_attr: None,
        }))
    };
    let a = Filter::and(Filter::and(leaf("a"), leaf("b")), leaf("c"));
    let b = Filter::and(leaf("a"), Filter::and(leaf("b"), leaf("c")));
    let parsed: Filter = "a pr and b pr and c pr".parse().unwrap();
    assert_eq!(a, parsed);
    assert_eq!(b, parsed);

    let mixed = Filter::and(Filter::or(leaf("a"), leaf("b")), leaf("c"));
    let parsed: Filter = "(a pr or b pr) and c pr".parse().unwrap();
    assert_eq!(mixed, parsed);

    let v = |n: &str| {
        ValFilter::Attr(AttrExp::Present(AttrPath {
            uri: None,
            name: n.into(),
            sub_attr: None,
        }))
    };
    let va = ValFilter::or(ValFilter::or(v("a"), v("b")), ValFilter::or(v("c"), v("d")));
    let parsed: Filter = "emails[a pr or b pr or c pr or d pr]".parse().unwrap();
    let Filter::ValuePath(vp) = parsed else {
        panic!("expected ValuePath");
    };
    assert_eq!(*vp.filter, va);
}

// The term budget spans value-path inner filters too, since every
// attribute expression costs the same to hold: 600 inside the brackets
// plus 425 outside is 1025, one over the limit, while 600 + 424 parses.
#[test]
fn term_budget_spans_value_path_terms() {
    let inner = chain("or", 600);
    let outer = chain("and", MAX_FILTER_TERMS - 600 + 1);
    assert_too_many_terms(
        format!("emails[{inner}] and {outer}").parse::<Filter>(),
        MAX_FILTER_TERMS + 1,
    );
    let outer = chain("and", MAX_FILTER_TERMS - 600);
    format!("emails[{inner}] and {outer}")
        .parse::<Filter>()
        .expect("exactly MAX_FILTER_TERMS terms across a value path parses");
}

// Devin SEC-1: the limits are enforced while parsing, not on the finished
// tree. Observable without an allocator hook: garbage *after* the point
// where a limit is crossed is never reached, so the error is the limit
// error rather than a syntax error. Post-parse enforcement would have to
// finish the parse first and would report the syntax error instead.
#[test]
fn term_limit_fires_before_the_parser_reaches_the_rest_of_the_input() {
    let s = format!("{} or ((((", chain("or", MAX_FILTER_TERMS + 1));
    assert_too_many_terms(s.parse::<Filter>(), MAX_FILTER_TERMS + 1);
    let s = format!("emails[{}] or ((((", chain("or", MAX_FILTER_TERMS + 1));
    assert_too_many_terms(s.parse::<PatchPath>(), MAX_FILTER_TERMS + 1);
}

#[test]
fn depth_limit_fires_before_the_parser_reaches_the_rest_of_the_input() {
    let s = format!("{}title pr ))))", "not (".repeat(MAX_FILTER_DEPTH + 1));
    match s.parse::<Filter>() {
        Err(LalrParseError::User {
            error: FilterActionError::DepthExceeded(d),
        }) => assert_eq!(
            d,
            MAX_FILTER_DEPTH + 1,
            "reported at the first excess bracket"
        ),
        other => panic!("expected DepthExceeded from the live check, got {other:?}"),
    }
    // A well-formed input at exactly the syntactic limit still parses.
    let s = not_chain(MAX_FILTER_DEPTH);
    s.parse::<Filter>()
        .expect("MAX_FILTER_DEPTH nested nots parse");
}

// Redundant parentheses are nesting too: the red team's round-2 report
// noted 5,000 of them parsed while a 65-term chain did not. Both limits
// now mean what they say.
#[test]
fn redundant_parentheses_count_as_nesting() {
    let ok = format!(
        "{}title pr{}",
        "(".repeat(MAX_FILTER_DEPTH),
        ")".repeat(MAX_FILTER_DEPTH)
    );
    ok.parse::<Filter>()
        .expect("MAX_FILTER_DEPTH redundant parens parse");
    let over = format!(
        "{}title pr{}",
        "(".repeat(MAX_FILTER_DEPTH + 1),
        ")".repeat(MAX_FILTER_DEPTH + 1)
    );
    assert_depth_exceeded(over.parse::<Filter>());
    let deep = format!("{}title pr{}", "(".repeat(5_000), ")".repeat(5_000));
    assert_depth_exceeded(deep.parse::<Filter>());
}

// Devin BUG-4: the type rules out the arities the grammar cannot write.
#[test]
fn operands_refuse_fewer_than_two() {
    let leaf = |n: &str| Filter::Attr(AttrExp::Present(AttrPath::with_name(n)));
    assert_eq!(Operands::<Filter>::new(vec![]), Err(TooFewOperands(0)));
    assert_eq!(Operands::new(vec![leaf("a")]), Err(TooFewOperands(1)));
    let two = Operands::new(vec![leaf("a"), leaf("b")]).unwrap();
    assert_eq!(two.len(), 2);
    let mut three = two.clone();
    three.push(leaf("c"));
    assert_eq!(three.len(), 3);
    assert_eq!(Filter::And(three).to_string(), "a pr and b pr and c pr");
    assert_eq!(
        TooFewOperands(1).to_string(),
        "a logical operator needs at least two operands, got 1"
    );
}

// Every hand-built tree the type permits survives Display and re-parse.
#[test]
fn hand_built_operands_round_trip() {
    let leaf = |n: &str| Filter::Attr(AttrExp::Present(AttrPath::with_name(n)));
    let f = Filter::Or(Operands::new(vec![leaf("a"), leaf("b"), leaf("c")]).unwrap());
    assert_eq!(f.to_string().parse::<Filter>().unwrap(), f);
    let f = Filter::And(Operands::pair(
        Filter::Or(Operands::pair(leaf("a"), leaf("b"))),
        Filter::Not(Box::new(leaf("c"))),
    ));
    assert_eq!(f.to_string(), "(a pr or b pr) and not (c pr)");
    assert_eq!(f.to_string().parse::<Filter>().unwrap(), f);
    let json = serde_json::to_string(&f).unwrap();
    assert_eq!(serde_json::from_str::<Filter>(&json).unwrap(), f);
}

#[test]
fn value_path_chain_exceeding_term_limit_rejected() {
    let n = MAX_FILTER_TERMS + 1;
    let inner = chain("or", n);
    assert_too_many_terms(format!("emails[{inner}]").parse::<Filter>(), n);
    assert_too_many_terms(format!("emails[{inner}].value").parse::<PatchPath>(), n);
}

#[test]
fn value_path_chain_at_term_limit_parses() {
    let inner = chain("or", MAX_FILTER_TERMS);
    let f: Filter = format!("emails[{inner}]").parse().unwrap();
    let Filter::ValuePath(vp) = &f else {
        panic!("expected ValuePath");
    };
    assert!(matches!(&*vp.filter, ValFilter::Or(items) if items.len() == MAX_FILTER_TERMS));
    let p: PatchPath = format!("emails[{inner}].value").parse().unwrap();
    assert!(matches!(p, PatchPath::Value(_)));
}

// The point of the n-ary shape: every recursive impl runs in depth-2
// recursion on a flat chain, so a chain at the term limit is safe on a
// stack far smaller than the test harness default (2 MiB), where the old
// binary tree overflowed `Display`/`PartialEq` in debug builds between
// 1_024 and 4_096 terms.
#[test]
fn flat_chain_at_term_limit_is_safe_on_a_small_stack() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let s = chain("or", MAX_FILTER_TERMS);
            let f: Filter = s.parse().unwrap();
            assert_eq!(f.to_string(), s);
            let reparsed: Filter = f.to_string().parse().unwrap();
            assert_eq!(f, reparsed);
            let _ = format!("{f:?}");
            let json = serde_json::to_string(&f).unwrap();
            let back: Filter = serde_json::from_str(&json).unwrap();
            assert_eq!(back, f);
            drop(f);
            drop(reparsed);
            drop(back);
        })
        .unwrap()
        .join()
        .expect("flat chain at the term limit must not overflow a 256 KiB stack");
}

// Regression for R2-I1's hygiene half: a 100_000-term chain is rejected
// with the term error rather than parsed, and neither the parse, the
// count, nor the drop of the rejected AST recurses over the chain.
#[test]
fn extremely_long_chain_rejected_without_panic() {
    let n = 100_000;
    assert_too_many_terms(chain("or", n).parse::<Filter>(), MAX_FILTER_TERMS + 1);
}

#[test]
fn deserialize_rejects_too_many_terms() {
    let raw = chain("and", MAX_FILTER_TERMS + 1);
    let json = serde_json::to_string(&raw).unwrap();
    let err = serde_json::from_str::<Filter>(&json).unwrap_err();
    assert!(
        err.to_string().contains("too many terms"),
        "expected term error, got: {err}"
    );
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
