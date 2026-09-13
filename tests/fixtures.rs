//! Keeps `src/test_data/` honest.
//!
//! `src/test_data/README.md` documents 22 RFC 7644 fixtures and hand-maintains
//! a Support column describing compile-time facts. Nothing enforced any of it,
//! so a fixture could be added without a row, deleted with its row surviving,
//! or gain support while the row still read NOT SUPPORTED. These tests close
//! that gap — the "optional" I-1 finding from the review of #47.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fixture_names(dir: &str) -> BTreeSet<String> {
    fs::read_dir(repo_root().join("src/test_data").join(dir))
        .expect("fixture directory must exist")
        .map(|e| e.expect("readable dir entry").file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".json"))
        .collect()
}

/// Every `.rs` file under `src/`, concatenated, with `//` line comments
/// stripped. Used to ask whether a fixture is referenced by any
/// `include_str!`. Unit tests live in `src/<module>/tests.rs`, one level
/// below the module they test, so a fixture path from there is
/// `../../test_data/…`; that exact prefix is what the checks below match.
///
/// The comment stripping is load-bearing: matching raw text meant a fixture
/// mentioned only in a `//` comment or a TODO satisfied the claim that a test
/// reads it. Verified — an undocumented fixture named only in a trailing
/// comment passed every test in this file before this change.
fn all_sources() -> String {
    fn walk(dir: &Path, out: &mut String) {
        for entry in fs::read_dir(dir).expect("readable source dir") {
            let path = entry.expect("readable dir entry").path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = fs::read_to_string(&path).expect("readable source file");
                out.push_str(&strip_comments(&text));
            }
        }
    }
    let mut out = String::new();
    walk(&repo_root().join("src"), &mut out);
    out
}

/// Source text with block comments, line comments and all whitespace removed,
/// so a fixture path mentioned in a comment cannot satisfy a "a test reads
/// this" claim and a rustfmt-wrapped `include_str!` matches its one-line form.
/// Crude against comment markers inside string literals, which only ever
/// under-counts.
fn strip_comments(text: &str) -> String {
    let mut rest = text;
    let mut no_blocks = String::with_capacity(text.len());
    while let Some(start) = rest.find("/*") {
        no_blocks.push_str(&rest[..start]);
        match rest[start + 2..].find("*/") {
            Some(end) => rest = &rest[start + 2 + end + 2..],
            None => rest = "",
        }
    }
    no_blocks.push_str(rest);
    let mut out = String::with_capacity(no_blocks.len());
    for line in no_blocks.lines() {
        let code = line.split_once("//").map_or(line, |(before, _)| before);
        out.extend(code.chars().filter(|c| !c.is_whitespace()));
    }
    out
}

/// Whether `sources` (as produced by [`strip_comments`]) contains the actual
/// `include_str!` invocation for `dir/name`, so a bare path in a string
/// literal or a `format!` cannot satisfy the claim.
fn is_included(sources: &str, dir: &str, name: &str) -> bool {
    sources.contains(&format!(r#"include_str!("../../test_data/{dir}/{name}")"#))
}

/// The two properties that make the inventory search trustworthy have
/// a negative control each, so reverting either fails here rather than
/// silently returning the failure mode they prevent.
#[test]
fn comment_stripping_and_the_include_needle_are_load_bearing() {
    let needle = r#"include_str!("../../test_data/rfc7644/x.json")"#;
    assert_eq!(strip_comments(&format!("/* {needle} */")), "");
    assert_eq!(
        strip_comments(&format!("code // {needle}\nmore")),
        "codemore"
    );
    assert_eq!(
        strip_comments("a /* multi\nline\n*/ b // tail\nc"),
        "abc",
        "block comments spanning lines are removed too"
    );
    assert!(
        !is_included("../../test_data/rfc7644/x.json", "rfc7644", "x.json"),
        "a bare path must not count"
    );
    assert!(!is_included(
        r#"format!("../../test_data/rfc7644/x.json")"#,
        "rfc7644",
        "x.json"
    ));
    assert!(is_included(needle, "rfc7644", "x.json"));
    let wrapped = r#"include_str!(
    "../../test_data/rfc7644/x.json"
)"#;
    assert!(
        is_included(&strip_comments(wrapped), "rfc7644", "x.json"),
        "a rustfmt-wrapped invocation matches"
    );
    assert!(
        !is_included(
            &strip_comments(&format!("// {needle}")),
            "rfc7644",
            "x.json"
        ),
        "a commented-out invocation does not"
    );
}

fn readme() -> String {
    fs::read_to_string(repo_root().join("src/test_data/README.md")).expect("README must exist")
}

/// Each RFC fixture is documented exactly once. A duplicate row is as
/// misleading as a missing one, since the two can disagree.
#[test]
fn every_rfc_fixture_has_exactly_one_readme_row() {
    let readme = readme();
    for name in fixture_names("rfc7644") {
        let rows = readme.matches(&format!("`{name}`")).count();
        assert_eq!(
            rows, 1,
            "`{name}` appears in {rows} README rows; expected exactly 1"
        );
    }
}

/// No row outlives its fixture. Deleting a file and leaving the row behind
/// leaves the README describing a payload nobody can look at.
#[test]
fn every_readme_row_names_a_fixture_that_exists() {
    let mut present = fixture_names("rfc7644");
    present.extend(fixture_names("provider_samples"));
    for line in readme().lines().filter(|l| l.starts_with("| `")) {
        let name = line
            .split('`')
            .nth(1)
            .expect("a row's first backticked span is its filename");
        assert!(
            present.contains(name),
            "README documents `{name}`, which is not in src/test_data/rfc7644/"
        );
    }
}

/// The Support column states a compile-time fact: SUPPORTED means some test
/// pulls the fixture in with `include_str!`. Check the claim rather than
/// trusting it.
#[test]
fn the_support_column_matches_actual_include_str_usage() {
    let sources = all_sources();
    let rfc = fixture_names("rfc7644");
    let provider = fixture_names("provider_samples");
    for line in readme().lines().filter(|l| l.starts_with("| `")) {
        let name = line.split('`').nth(1).expect("row filename");
        if !name.ends_with(".json") {
            continue;
        }
        // Only rows that carry a Support column make a claim to check. The
        // `provider_samples/` table has two columns and asserts nothing about
        // support; those fixtures are covered by
        // `every_provider_sample_is_exercised_by_a_test` instead.
        if !line.contains("| SUPPORTED |") && !line.contains("| NOT SUPPORTED |") {
            continue;
        }
        // Resolve against the directory the row's fixture actually lives in.
        // Keying every row off `rfc7644/` meant a provider row gaining a
        // SUPPORTED column would have failed for the wrong reason.
        let dir = if rfc.contains(name) {
            "rfc7644"
        } else if provider.contains(name) {
            "provider_samples"
        } else {
            panic!("README documents `{name}`, which is in neither fixture directory");
        };
        let claimed_supported = line.contains("| SUPPORTED |");
        // The `include_str!` invocation itself, so a bare path in a string
        // literal or a `format!` cannot satisfy the claim.
        let actually_used = is_included(&sources, dir, name);

        assert_eq!(
            claimed_supported,
            actually_used,
            "`{name}`: README says {}, but include_str! usage says {}",
            if claimed_supported {
                "SUPPORTED"
            } else {
                "NOT SUPPORTED"
            },
            if actually_used {
                "SUPPORTED"
            } else {
                "NOT SUPPORTED"
            },
        );
    }
}

/// A provider sample that no test reads is dead weight: it looks like
/// coverage of a real provider's quirks without providing any.
#[test]
fn every_provider_sample_is_exercised_by_a_test() {
    let sources = all_sources();
    for name in fixture_names("provider_samples") {
        assert!(
            is_included(&sources, "provider_samples", &name),
            "provider sample `{name}` is not read by any test"
        );
    }
}

/// Provider samples need a README row too. `every_rfc_fixture_has_exactly_one_readme_row`
/// covered only `rfc7644/`, so the inventory claim held for 22 of 27 fixtures
/// — and the five it missed are the JumpCloud and Entra payloads that
/// motivated the null-collapsing work.
#[test]
fn every_provider_sample_has_exactly_one_readme_row() {
    let readme = readme();
    for name in fixture_names("provider_samples") {
        let rows = readme.matches(&format!("`{name}`")).count();
        assert_eq!(
            rows, 1,
            "`{name}` appears in {rows} README rows; expected exactly 1"
        );
    }
}
