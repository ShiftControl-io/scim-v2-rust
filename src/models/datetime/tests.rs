//! The accept/reject surface of RFC 7643 §2.3.5, which defers to XSD 1.1 Part
//! 2 §3.3.7.2 for the lexical space and §3.3.7.1 for the day-of-month rule.

use std::cmp::Ordering;

use super::*;

/// Every shape the published `dateTimeLexicalRep` admits.
#[test]
fn accepts_the_lexical_space() {
    for s in [
        "2010-01-23T04:56:22Z",       // RFC 7643 §3.1's own example spelling
        "2010-01-23T04:56:22",        // no offset: `·timezoneOffset·` is optional
        "2010-01-23T04:56:22.1Z",     // secondFrag fraction, one digit
        "2010-01-23T04:56:22.123456", // and many, with no offset
        "2010-01-23T04:56:22+05:30",  // a half-hour offset
        "2010-01-23T04:56:22-08:00",
        "2010-01-23T04:56:22+14:00", // the ceiling, spelled by its own branch
        "2010-01-23T04:56:22-14:00",
        "2010-01-23T00:00:00+00:00",
        "2010-01-23T24:00:00Z",   // endOfDayFrag
        "2010-01-23T24:00:00.0Z", // which may carry a zero fraction
        "2010-01-23T24:00:00.000",
        "2010-01-23T24:00:00",    // end of day with no offset at all
        "10000-02-29T00:00:00Z",  // five digits, divisible by 400
        "0000-01-01T00:00:00Z",   // year zero is 1 BCE in XSD 1.1
        "-0001-01-01T00:00:00Z",  // and years may be negative
        "12345-01-01T00:00:00Z",  // yearFrag is four digits *or more*
        "-12345-01-01T00:00:00Z", //
        "2000-02-29T00:00:00Z",   // divisible by 400
        "2024-02-29T00:00:00Z",   // divisible by 4, not by 100
        "2010-12-31T23:59:59Z",
    ] {
        assert!(s.parse::<ScimDateTime>().is_ok(), "should accept {s:?}");
    }
}

#[test]
fn rejects_everything_outside_it() {
    for s in [
        "",
        "2010-01-23",                // §2.3.5: "MUST include both a date and a time"
        "04:56:22Z",                 // and both means both
        "2010-01-23 04:56:22Z",      // the separator is T
        "2010-01-23t04:56:22Z",      // and it is upper case
        "2010-01-23T04:56:22z",      // as is the zulu marker
        "999-01-23T04:56:22Z",       // yearFrag is at least four digits
        "01234-01-23T04:56:22Z",     // a leading zero means exactly four
        "2010-00-23T04:56:22Z",      // monthFrag is 01..12
        "2010-13-23T04:56:22Z",      //
        "2010-01-00T04:56:22Z",      // dayFrag is 01..31
        "2010-01-32T04:56:22Z",      //
        "2010-04-31T04:56:22Z",      // 30 days hath April
        "2010-02-30T04:56:22Z",      //
        "1900-02-29T00:00:00Z",      // divisible by 100, not by 400
        "2023-02-29T00:00:00Z",      // not divisible by 4
        "2010-01-23T24:00:01Z",      // endOfDayFrag is exactly 24:00:00
        "2010-01-23T24:30:00Z",      //
        "2010-01-23T24:00:00.1Z",    // whose fraction may only be zeroes
        "2010-01-23T25:00:00Z",      // hourFrag is 00..23 otherwise
        "2010-01-23T04:60:22Z",      // minuteFrag is 00..59
        "2010-01-23T04:56:60Z",      // secondFrag too: XSD has no leap second
        "2010-01-23T04:56:22.Z",     // a fraction needs a digit
        "2010-01-23T04:56:22.",      //
        "2010-01-23T04:56Z",         // seconds are not optional
        "2010-01-23T04:56:22+15:00", // the offset ceiling is ±14:00
        "2010-01-23T04:56:22+14:01", // which 14:00 hits exactly
        "2010-01-23T04:56:22+05:60", // offset minutes are 00..59
        "2010-01-23T04:56:22+5:00",  // and both halves are two digits
        "2010-01-23T04:56:22Z ",     // no trailing anything
        "2010-01-23T04:56:22ZZ",     //
        " 2010-01-23T04:56:22Z",     // no leading anything
        "10100-02-29T00:00:00Z",     // five digits, by 100 and not by 400
        // One non-digit in a two-digit field, positioned so a parser that
        // only rejects when *both* halves are non-digits would read it as a
        // legal value: ';' is '0' + 11.
        "2010-0;-23T04:56:22Z",
        "2010-01-23T0;:56:22Z",
        "2010-01-23T04:0;:22Z",
        "2010-01-23T04:56:0;Z",
        "2010-01-23T04:56:22+0;:00",
        "2010-01-23T04:56:22+05:30:00",
        "yesterday",
    ] {
        assert!(s.parse::<ScimDateTime>().is_err(), "should reject {s:?}");
    }
}

/// The type carries a lexical form, so what went in comes back out. A parser
/// that normalised would make `meta` a lie about what the provider wrote.
#[test]
fn round_trips_byte_for_byte() {
    for s in [
        "2010-01-23T04:56:22Z",
        "2010-01-23T04:56:22",
        "2010-01-23T04:56:22.123456+05:30",
        "-12345-01-01T24:00:00.000-14:00",
    ] {
        let parsed: ScimDateTime = s.parse().expect("valid");
        assert_eq!(parsed.as_str(), s);
        assert_eq!(parsed.to_string(), s);
        let json = serde_json::to_string(&parsed).expect("serializes");
        assert_eq!(json, format!("\"{s}\""));
        let back: ScimDateTime = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(back, parsed);
    }
}

#[test]
fn deserialize_rejects_and_quotes_the_rule() {
    let err = serde_json::from_str::<ScimDateTime>(r#""yesterday""#)
        .expect_err("not a dateTime")
        .to_string();
    assert!(err.contains("2.3.5"), "{err}");
    assert!(err.contains("MUST include both a date and a time"), "{err}");
    assert!(err.contains("yesterday"), "{err}");
}

/// RFC 3339 requires an offset and XSD does not, so a caller handing
/// `as_str` to a date-time library needs to know which it has.
#[test]
fn reports_whether_an_offset_is_present() {
    for (s, expected) in [
        ("2010-01-23T04:56:22Z", true),
        ("2010-01-23T04:56:22+05:30", true),
        ("2010-01-23T04:56:22-08:00", true),
        ("2010-01-23T04:56:22", false),
        ("2010-01-23T04:56:22.123", false),
        ("-0001-01-23T04:56:22", false),
    ] {
        let ts: ScimDateTime = s.parse().expect("valid");
        assert_eq!(ts.has_offset(), expected, "{s}");
    }
}

#[test]
fn constructs_only_through_validating_paths() {
    assert!(ScimDateTime::try_from("2010-01-23T04:56:22Z").is_ok());
    assert!(ScimDateTime::try_from(String::from("2010-01-23T04:56:22Z")).is_ok());
    assert!(ScimDateTime::try_from("nope").is_err());
    let err = ScimDateTime::try_from(String::from("nope")).expect_err("invalid");
    assert_eq!(err.value, "nope");

    // The accessors hand back the value, not a placeholder.
    let ts: ScimDateTime = "2010-01-23T04:56:22Z".parse().expect("valid");
    assert_eq!(AsRef::<str>::as_ref(&ts), "2010-01-23T04:56:22Z");
    assert_eq!(ts.into_string(), "2010-01-23T04:56:22Z");
}

/// Non-ASCII input must not panic on a byte index inside a character.
#[test]
fn handles_multibyte_input_without_panicking() {
    for s in ["2010-01-23T04:56:22Ω", "２０１０-01-23T04:56:22Z", "🕒"] {
        assert!(s.parse::<ScimDateTime>().is_err(), "should reject {s:?}");
    }
}

/// XSD 1.1 §3.3.7.1 orders values by their position on the time line, which
/// is not what `==` does. Both behaviours are deliberate and they disagree.
#[test]
fn equivalence_is_by_instant_where_equality_is_by_text() {
    let same_instant = [
        ("2010-01-23T04:56:22Z", "2010-01-22T20:56:22-08:00"),
        ("2010-01-23T04:56:22Z", "2010-01-23T04:56:22+00:00"),
        ("2010-01-23T04:56:22Z", "2010-01-23T10:26:22+05:30"),
        ("2010-01-23T24:00:00Z", "2010-01-24T00:00:00Z"), // end of day
        ("2010-01-23T04:56:22.1Z", "2010-01-23T04:56:22.100Z"), // trailing zeros
        ("2010-01-23T00:00:00+14:00", "2010-01-22T10:00:00Z"), // the ceiling
        ("2010-01-23T00:00:00-14:00", "2010-01-23T14:00:00Z"),
    ];
    for (a, b) in same_instant {
        let (x, y) = (parse(a), parse(b));
        assert!(x.xsd_equivalent(&y), "{a} and {b} are one instant");
        assert!(y.xsd_equivalent(&x), "equivalence is symmetric");
        assert_eq!(x.xsd_partial_cmp(&y), Some(Ordering::Equal));
        if a != b {
            assert_ne!(x, y, "== stays textual: {a} vs {b}");
        }
    }
}

/// Values that both carry an offset, or both omit one, sit on a single time
/// line and always compare.
#[test]
fn like_values_are_totally_ordered() {
    let ascending = [
        "-0001-12-31T23:59:59Z",
        "0000-01-01T00:00:00Z", // year zero is 1 BCE, and a leap year
        "0000-02-29T00:00:00Z",
        "1969-12-31T23:59:59Z", // either side of the unix epoch
        "1970-01-01T00:00:00Z",
        "2010-01-23T04:56:22.09Z", // fractions order within a second
        "2010-01-23T04:56:22.1Z",
        "2010-01-23T04:56:22.11Z",
        "2010-02-28T23:59:59Z", // and across a month boundary
        "2010-03-01T00:00:00Z",
        "12345-01-01T00:00:00Z",
    ];
    for window in ascending.windows(2) {
        let (earlier, later) = (parse(window[0]), parse(window[1]));
        assert_eq!(
            earlier.xsd_partial_cmp(&later),
            Some(Ordering::Less),
            "{} < {}",
            window[0],
            window[1]
        );
        assert_eq!(later.xsd_partial_cmp(&earlier), Some(Ordering::Greater));
        assert!(!earlier.xsd_equivalent(&later));
    }

    // Offset-less values compare to each other the same way.
    assert_eq!(
        parse("2010-01-23T04:56:22").xsd_partial_cmp(&parse("2010-01-23T04:56:23")),
        Some(Ordering::Less)
    );
}

/// The case that costs `dateTime` its total order: one value anchored, one
/// floating. §3.3.7.1 imputes `+14:00` and `−14:00` to the floating one and
/// calls the pair ·incomparable· when the two imputations disagree.
#[test]
fn an_offset_less_value_compares_only_outside_the_imputed_range() {
    let anchored = parse("2010-01-23T12:00:00Z");
    for (floating, expected) in [
        ("2010-01-23T12:00:00", None),                 // same wall time
        ("2010-01-23T23:59:59", None),                 // inside +14:00
        ("2010-01-24T02:00:00", None),                 // exactly +14:00: bounds touch
        ("2010-01-22T22:00:00", None),                 // exactly -14:00
        ("2010-01-24T02:00:01", Some(Ordering::Less)), // one second past it
        ("2010-01-22T21:59:59", Some(Ordering::Greater)),
        ("2010-01-25T12:00:00", Some(Ordering::Less)),
        ("2010-01-21T12:00:00", Some(Ordering::Greater)),
    ] {
        let other = parse(floating);
        assert_eq!(
            anchored.xsd_partial_cmp(&other),
            expected,
            "{} vs {floating}",
            anchored.as_str()
        );
        assert_eq!(
            other.xsd_partial_cmp(&anchored),
            expected.map(Ordering::reverse),
            "and the reverse"
        );
        assert!(
            !anchored.xsd_equivalent(&other),
            "never equivalent: {floating}"
        );
    }
}

/// The relation's own properties, over generated values rather than a table:
/// reflexive, symmetric under reversal, and never disagreeing with byte
/// equality, which is the invariant that lets both `==` and `xsd_equivalent`
/// exist on one type.
#[test]
fn the_relation_is_reflexive_and_symmetric() {
    let corpus: Vec<ScimDateTime> = [
        "2010-01-23T04:56:22Z",
        "2010-01-23T04:56:22",
        "2010-01-22T20:56:22-08:00",
        "2010-01-23T24:00:00.000+14:00",
        "-0044-03-15T12:00:00",
        "0000-02-29T00:00:00Z",
        "12345-06-07T08:09:10.5-14:00",
        "1970-01-01T00:00:00Z",
    ]
    .iter()
    .map(|s| parse(s))
    .collect();

    for a in &corpus {
        assert_eq!(a.xsd_partial_cmp(a), Some(Ordering::Equal), "reflexive");
        assert!(a.xsd_equivalent(a));
        for b in &corpus {
            assert_eq!(
                a.xsd_partial_cmp(b).map(Ordering::reverse),
                b.xsd_partial_cmp(a),
                "{} vs {}",
                a.as_str(),
                b.as_str()
            );
            if a == b {
                assert!(a.xsd_equivalent(b), "byte-equal implies equivalent");
            }
        }
    }
}

fn parse(s: &str) -> ScimDateTime {
    s.parse().unwrap_or_else(|e| panic!("{s}: {e}"))
}
