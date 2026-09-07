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

/// Every `.rs` file under `src/`, concatenated. Used to ask whether a fixture
/// is referenced by any `include_str!`.
fn all_sources() -> String {
    fn walk(dir: &Path, out: &mut String) {
        for entry in fs::read_dir(dir).expect("readable source dir") {
            let path = entry.expect("readable dir entry").path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push_str(&fs::read_to_string(&path).expect("readable source file"));
            }
        }
    }
    let mut out = String::new();
    walk(&repo_root().join("src"), &mut out);
    out
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
    for line in readme().lines().filter(|l| l.starts_with("| `")) {
        let name = line.split('`').nth(1).expect("row filename");
        if !name.ends_with(".json") {
            continue;
        }
        let claimed_supported = line.contains("| SUPPORTED |");
        let actually_used = sources.contains(&format!("test_data/rfc7644/{name}"));

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
            sources.contains(&format!("test_data/provider_samples/{name}")),
            "provider sample `{name}` is not read by any test"
        );
    }
}
