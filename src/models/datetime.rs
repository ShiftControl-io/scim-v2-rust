//! A SCIM `dateTime` attribute value, validated on construction.
//!
//! RFC 7643 §2.3.5 says the value "MUST be encoded as a valid xsd:dateTime as
//! specified in Section 3.3.7 of [XML-Schema] and MUST include both a date and
//! a time". [`ScimDateTime`] is the only way to put one in a model, and it
//! cannot hold anything else: a malformed string is rejected by `FromStr`,
//! `TryFrom` and `Deserialize` alike, so a service provider filling in
//! `meta.created` by hand gets an error rather than a peer that cannot parse
//! its responses.
//!
//! # What this type deliberately does not do
//!
//! It validates and carries a lexical form. It does not represent an instant.
//! There is no arithmetic, no ordering, no comparison across time zone
//! offsets, no conversion to UTC, and no normalisation to a canonical
//! spelling. `PartialEq` compares the text, so `2010-01-23T04:56:22Z` and
//! `2010-01-22T20:56:22-08:00` are the same instant and are *not* equal here.
//! [`Ord`] is intentionally not implemented: XSD 1.1 §3.3.7.1 notes that a
//! value carrying an offset and one without are ordered by imputing both
//! `+14:00` and `−14:00` to the latter, so "many such combinations will be
//! ·incomparable·" and dateTime has only a partial order.
//!
//! Those are the things a date-time library would give you, and this crate
//! does not depend on one. The reason is fit, not weight.
//!
//! XSD 1.1 §3.3.7.1 leaves `·timezoneOffset·` **optional**, and none of
//! `time`, `chrono` or `jiff` has a single type that spans that. Each splits
//! the two cases apart — `OffsetDateTime` beside `PrimitiveDateTime`,
//! `DateTime<Tz>` beside `NaiveDateTime`, `Timestamp` beside
//! `civil::DateTime` — so a field typed as any one of them either rejects a
//! conformant offset-less value or invents an offset the wire never sent, and
//! a faithful mapping needs a sum type across two of them. Extended years and
//! the `24:00:00` end-of-day form narrow the fit further. A type that models
//! the value space badly is worse than a string.
//!
//! Two smaller reasons stack on top. The value is a lexical form and this
//! crate round-trips it byte for byte; parsing to an instant and formatting
//! back would rewrite a provider's `meta`. And there is no consensus library
//! to pick — putting one in `Meta`'s signature picks it for every consumer,
//! including those already using another.
//!
//! So the conversion is yours to make, which for an offset-bearing value is
//! one line:
//!
//! ```
//! # use scim_v2::ScimDateTime;
//! # use std::str::FromStr;
//! let ts = ScimDateTime::from_str("2010-01-23T04:56:22Z").unwrap();
//! // time:   OffsetDateTime::parse(ts.as_str(), &Rfc3339)
//! // chrono: DateTime::parse_from_rfc3339(ts.as_str())
//! assert_eq!(ts.as_str(), "2010-01-23T04:56:22Z");
//! ```
//!
//! It is not one line for the rest. `as_str` may return a value with no
//! offset, which every RFC 3339 parser rejects, so check
//! [`ScimDateTime::has_offset`] and decide what your caller should do with a
//! timestamp that names no instant — the crate will not guess for you.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// A string that is not a valid RFC 7643 §2.3.5 dateTime.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "{value:?} is not a valid dateTime; RFC 7643 §2.3.5: the value \"MUST be encoded as a valid xsd:dateTime as specified in Section 3.3.7 of [XML-Schema] and MUST include both a date and a time\""
)]
pub struct ParseScimDateTimeError {
    /// The rejected input.
    pub value: String,
}

/// An RFC 7643 §2.3.5 dateTime, guaranteed well-formed.
///
/// See the [module docs](self) for the deliberate limits of this type and for
/// how to convert to a real date-time library.
///
/// ```
/// # use scim_v2::ScimDateTime;
/// # use std::str::FromStr;
/// assert!(ScimDateTime::from_str("2010-01-23T04:56:22Z").is_ok());
/// assert!(ScimDateTime::from_str("2000-02-29T00:00:00Z").is_ok()); // leap year
/// assert!(ScimDateTime::from_str("1900-02-29T00:00:00Z").is_err()); // not one
/// assert!(ScimDateTime::from_str("2010-01-23").is_err()); // §2.3.5: date and time
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScimDateTime(String);

impl ScimDateTime {
    /// The value as it was written. Byte-identical to the accepted input.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The value as it was written, consuming self.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Whether the value carries a time zone offset.
    ///
    /// XSD 1.1 §3.3.7.1 leaves `·timezoneOffset·` optional, so a conformant
    /// value need not have one and is then not an unambiguous instant. RFC
    /// 3339 parsers require an offset; check this before handing [`as_str`]
    /// to one.
    ///
    /// [`as_str`]: Self::as_str
    pub fn has_offset(&self) -> bool {
        let Some(time) = self.0.rsplit('T').next() else {
            return false;
        };
        time.ends_with('Z') || time.contains('+') || time.contains('-')
    }
}

impl fmt::Display for ScimDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for ScimDateTime {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for ScimDateTime {
    type Err = ParseScimDateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_date_time(s) {
            Ok(Self(s.to_owned()))
        } else {
            Err(ParseScimDateTimeError {
                value: s.to_owned(),
            })
        }
    }
}

impl TryFrom<&str> for ScimDateTime {
    type Error = ParseScimDateTimeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<String> for ScimDateTime {
    type Error = ParseScimDateTimeError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if is_date_time(&s) {
            Ok(Self(s))
        } else {
            Err(ParseScimDateTimeError { value: s })
        }
    }
}

impl Serialize for ScimDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ScimDateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s).map_err(serde::de::Error::custom)
    }
}

/// The XSD 1.1 Part 2 §3.3.7.2 `dateTimeLexicalRep` production, which that
/// section publishes as the regular expression below, plus the separate
/// "Constraint: Day-of-month Values" the same section notes the expression
/// does not enforce.
///
/// ```text
/// -?([1-9][0-9]{3,}|0[0-9]{3})
/// -(0[1-9]|1[0-2])
/// -(0[1-9]|[12][0-9]|3[01])
/// T(([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9](\.[0-9]+)?|(24:00:00(\.0+)?))
/// (Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?
/// ```
///
/// Hand-written rather than compiled, so the crate keeps its `regex`-free
/// dependency graph; the expression alone would not settle February anyway.
fn is_date_time(s: &str) -> bool {
    let b = s.as_bytes();
    let mut i = 0;

    let negative = b.first() == Some(&b'-');
    if negative {
        i += 1;
    }

    // yearFrag: `[1-9][0-9]{3,}` or `0[0-9]{3}` — four digits at least, and a
    // leading zero is only allowed in the exactly-four-digit form.
    let year_start = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    let year = &b[year_start..i];
    match year.first() {
        Some(b'0') if year.len() == 4 => {}
        Some(d) if *d != b'0' && year.len() >= 4 => {}
        _ => return false,
    }

    if !eat(b, &mut i, b'-') {
        return false;
    }
    let Some(month) = two_digits(b, &mut i, 1, 12) else {
        return false;
    };
    if !eat(b, &mut i, b'-') {
        return false;
    }
    let Some(day) = two_digits(b, &mut i, 1, 31) else {
        return false;
    };
    if day > days_in_month(year, month) {
        return false;
    }

    if !eat(b, &mut i, b'T') {
        return false;
    }

    // endOfDayFrag `24:00:00(\.0+)?`, or hour/minute/second with an optional
    // fraction. `secondFrag` caps at 59: XSD has no leap seconds.
    if b[i..].starts_with(b"24:00:00") {
        i += 8;
        if i < b.len() && b[i] == b'.' {
            i += 1;
            if !eat_while(b, &mut i, |c| c == b'0') {
                return false;
            }
        }
    } else {
        if two_digits(b, &mut i, 0, 23).is_none()
            || !eat(b, &mut i, b':')
            || two_digits(b, &mut i, 0, 59).is_none()
            || !eat(b, &mut i, b':')
            || two_digits(b, &mut i, 0, 59).is_none()
        {
            return false;
        }
        if i < b.len() && b[i] == b'.' {
            i += 1;
            if !eat_while(b, &mut i, |c| c.is_ascii_digit()) {
                return false;
            }
        }
    }

    // timezoneFrag, optional: `Z`, or a signed offset in ±14:00.
    if i < b.len() {
        if b[i] == b'Z' {
            i += 1;
        } else {
            if b[i] != b'+' && b[i] != b'-' {
                return false;
            }
            i += 1;
            if b[i..].starts_with(b"14:00") {
                i += 5;
            } else {
                if two_digits(b, &mut i, 0, 13).is_none()
                    || !eat(b, &mut i, b':')
                    || two_digits(b, &mut i, 0, 59).is_none()
                {
                    return false;
                }
            }
        }
    }

    i == b.len()
}

/// Days in `month`, applying the XSD 1.1 §3.3.7.1 "Constraint: Day-of-month
/// Values": 30 for months 4, 6, 9 and 11, and February by the proleptic
/// Gregorian leap rule. The year arrives as digits because `yearFrag` admits
/// arbitrarily many of them; only its residue mod 400 matters here, and that
/// residue is the same for a year and its negation.
fn days_in_month(year: &[u8], month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => {
            let y = year
                .iter()
                .fold(0u32, |acc, d| (acc * 10 + u32::from(d - b'0')) % 400);
            if y % 400 == 0 || (y % 4 == 0 && y % 100 != 0) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}

fn eat(b: &[u8], i: &mut usize, want: u8) -> bool {
    if b.get(*i) == Some(&want) {
        *i += 1;
        true
    } else {
        false
    }
}

fn eat_while(b: &[u8], i: &mut usize, f: impl Fn(u8) -> bool) -> bool {
    let start = *i;
    while *i < b.len() && f(b[*i]) {
        *i += 1;
    }
    *i > start
}

/// Exactly two ASCII digits whose value falls in `min..=max`.
fn two_digits(b: &[u8], i: &mut usize, min: u32, max: u32) -> Option<u32> {
    let d = b.get(*i..*i + 2)?;
    if !d[0].is_ascii_digit() || !d[1].is_ascii_digit() {
        return None;
    }
    let v = u32::from(d[0] - b'0') * 10 + u32::from(d[1] - b'0');
    (min..=max).contains(&v).then(|| {
        *i += 2;
        v
    })
}

#[cfg(test)]
mod tests;
