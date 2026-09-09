//! Every quotation of an RFC in the source is verbatim.
//!
//! A rule in this crate is only as good as the sentence it enforces, and a
//! sentence recalled from memory is how a check citing "§§6-7" for a rule
//! those sections do not contain shipped through two review rounds. So a
//! doc comment that cites an RFC and quotes it — `RFC 7643 §3.1: "MUST
//! include a non-empty id value"` — is checked here against the text under
//! `docs/rfcs/`. Paraphrases go in plain words; quotation marks mean
//! quotation.
//!
//! Matching is case- and whitespace-insensitive, ignores the RFC's own
//! quotation marks, page furniture and table rules, strips inline code and
//! emphasis markers, reads each column of an RFC table as continuous text,
//! and treats `…` or `...` inside a quote as a gap. A quotation is checked
//! against every RFC mentioned within 240 characters of it, before or after.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

/// Lowercased, whitespace collapsed, curly quotes straightened, and double
/// quotes, backticks and asterisks dropped (the RFC quotes attribute names;
/// comments mark them as code or emphasis).
fn normalize(s: &str) -> String {
    let s: String = s
        .chars()
        .map(|c| match c {
            '\u{2018}' | '\u{2019}' => '\'',
            '\u{201c}' | '\u{201d}' => '"',
            c => c,
        })
        .filter(|c| !matches!(c, '"' | '`' | '*'))
        .flat_map(char::to_lowercase)
        .collect();
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The RFC text with page headers and footers removed, so a quotation that
/// spans a page break still matches, followed by every table column read as
/// continuous text, so a sentence the RFC wraps inside a `|` cell matches
/// too.
fn rfc_text(number: &str) -> String {
    let raw = fs::read_to_string(repo_root().join(format!("docs/rfcs/rfc{number}.txt")))
        .unwrap_or_else(|e| panic!("docs/rfcs/rfc{number}.txt: {e}"));
    let body: Vec<&str> = raw
        .lines()
        .filter(|l| !l.starts_with("Hunt, et al.") && !l.starts_with("RFC 764"))
        .filter(|l| !l.contains('\u{c}'))
        .collect();
    let mut text = body.join(" ");
    // Table columns: consecutive `|`-delimited lines, each column joined.
    let mut columns: Vec<String> = Vec::new();
    for line in body.iter().chain(std::iter::once(&"")) {
        let t = line.trim();
        if t.starts_with('|') && t.ends_with('|') {
            for (i, cell) in t.trim_matches('|').split('|').enumerate() {
                if columns.len() <= i {
                    columns.push(String::new());
                }
                columns[i].push(' ');
                columns[i].push_str(cell.trim());
            }
        } else if !columns.is_empty() {
            text.push(' ');
            text.push_str(&columns.join(" \n "));
            columns.clear();
        }
    }
    normalize(&text)
}

/// Comment text of one source file, one line per comment, with the markers
/// removed. Non-comment lines become blank lines so a quote cannot bridge two
/// unrelated comments.
fn comment_text(path: &Path) -> String {
    let src = fs::read_to_string(path).expect("readable source file");
    let mut out = String::with_capacity(src.len());
    for line in src.lines() {
        let t = line.trim_start();
        let body = ["///", "//!", "//"].iter().find_map(|m| t.strip_prefix(m));
        match body {
            Some(b) => {
                out.push_str(b.trim());
                out.push(' ');
            }
            None => out.push('\n'),
        }
    }
    out
}

/// Every `(rfc numbers, quoted text)` pair: a quotation of at least twenty
/// characters and two spaces with an `RFC 764x` mention within 240 characters
/// before or after it in the same comment block. All RFCs mentioned in that
/// span are candidates, since a comment may cite one RFC and then quote the
/// other.
fn citations(text: &str) -> Vec<(Vec<String>, String)> {
    let mut out = Vec::new();
    for block in text.split('\n') {
        let mut i = 0;
        while let Some(open_rel) = block[i..].find('"') {
            let open = i + open_rel;
            let Some(close_rel) = block[open + 1..].find('"') else {
                break;
            };
            let close = open + 1 + close_rel;
            let quote = &block[open + 1..close];
            i = close + 1;
            if quote.len() < 20 || quote.matches(' ').count() < 2 {
                continue;
            }
            // Only quotations presented *as* citations: an RFC or section
            // mention shortly before the opening quote, or an `(RFC …` shortly
            // after the closing one. Ordinary quoted prose near a citation is
            // left alone.
            let before = &block[floor(block, open.saturating_sub(80))..open];
            let after = &block[close..ceil(block, close + 40)];
            let cited =
                before.contains("RFC 764") || before.contains('§') || after.contains("RFC 764");
            if !cited {
                continue;
            }
            let lo = floor(block, open.saturating_sub(240));
            let hi = ceil(block, close + 240);
            let span = &block[lo..hi];
            let mut numbers: Vec<String> = Vec::new();
            let mut from = 0;
            while let Some(j) = span[from..].find("RFC 764") {
                let n = &span[from + j + 4..(from + j + 8).min(span.len())];
                if ["7642", "7643", "7644"].contains(&n) && !numbers.iter().any(|x| x == n) {
                    numbers.push(n.to_string());
                }
                from += j + 7;
            }
            if !numbers.is_empty() {
                out.push((numbers, quote.to_string()));
            }
        }
    }
    out
}

/// Nearest char boundary at or below `i`.
fn floor(s: &str, mut i: usize) -> usize {
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

/// Nearest char boundary at or above `i`, capped at the end.
fn ceil(s: &str, mut i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    while !s.is_char_boundary(i) {
        i += 1;
    }
    i
}

fn source_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("readable dir") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            source_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs")
            && path.file_name().is_none_or(|n| n != "filter_parser.rs")
        {
            out.push(path);
        }
    }
}

#[test]
fn every_rfc_quotation_in_the_source_is_verbatim() {
    let texts = [
        ("7642", rfc_text("7642")),
        ("7643", rfc_text("7643")),
        ("7644", rfc_text("7644")),
    ];
    let mut files = Vec::new();
    source_files(&repo_root().join("src"), &mut files);
    source_files(&repo_root().join("tests"), &mut files);

    let mut checked = 0;
    let mut failures = Vec::new();
    for file in &files {
        for (numbers, quote) in citations(&comment_text(file)) {
            checked += 1;
            let fragments: Vec<String> = quote
                .replace("...", "…")
                .split('…')
                .map(normalize)
                .filter(|f| f.len() >= 8)
                .collect();
            let found = !fragments.is_empty()
                && numbers.iter().any(|number| {
                    let rfc = &texts.iter().find(|(n, _)| *n == number).unwrap().1;
                    fragments.iter().all(|f| rfc.contains(f.as_str()))
                });
            if !found {
                failures.push(format!(
                    "{}: RFC {} does not contain {quote:?}",
                    file.strip_prefix(repo_root()).unwrap().display(),
                    numbers.join("/")
                ));
            }
        }
    }
    assert!(
        checked >= 20,
        "expected to find RFC quotations to check, found {checked}"
    );
    assert!(
        failures.is_empty(),
        "{} of {checked} RFC quotations are not verbatim:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn the_matcher_itself_is_load_bearing() {
    let rfc = rfc_text("7643");
    assert!(
        rfc.contains("must include a non-empty id value"),
        "quotes inside the RFC are ignored"
    );
    assert!(rfc.contains("duplicate values must not be included"));
    let text = comment_text(&repo_root().join("src/models/user.rs"));
    let found = citations(&text);
    assert!(!found.is_empty(), "user.rs cites the RFC");
    let fake = "/// RFC 7643 §3.1 says \"every id is a purple elephant on Tuesdays\".";
    let (n, q) = &citations(&comment_text_from(fake))[0];
    assert_eq!(n, &["7643".to_string()]);
    assert!(!rfc.contains(normalize(q).as_str()));
    // A table cell's sentence reads as one line (RFC 7644 §3.4.2.4 Table 6).
    let rfc7644 = rfc_text("7644");
    assert!(rfc7644.contains("a negative value shall be interpreted as 0"));
    assert!(rfc7644.contains(&normalize(
        "indicates that no resource results are to be returned except for totalResults"
    )));
}

fn comment_text_from(src: &str) -> String {
    let mut out = String::new();
    for line in src.lines() {
        if let Some(b) = line.trim_start().strip_prefix("///") {
            out.push_str(b.trim());
            out.push(' ');
        }
    }
    out
}
