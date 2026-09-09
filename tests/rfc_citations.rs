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
//! against every RFC mentioned within 240 characters of it, and when a `§`
//! section is named nearby, only against that section and its subsections —
//! so a real sentence filed under the wrong section fails too.

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

/// One RFC: its whole text, and each numbered section (with any tables read
/// column by column) so a citation can be checked against the section it
/// names.
struct Rfc {
    whole: String,
    sections: Vec<(String, String)>,
}

/// The section number of an RFC header line such as `3.1.  Common Attributes`
/// (column 0, number, period, two spaces), or `None`.
fn header_number(line: &str) -> Option<String> {
    let digits: String = line
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let number = digits.strip_suffix('.')?;
    if number.is_empty() || !number.chars().next()?.is_ascii_digit() || number.contains("..") {
        return None;
    }
    let rest = &line[digits.len()..];
    rest.starts_with("  ")
        .then(|| rest.trim_start().chars().next())
        .flatten()
        .filter(char::is_ascii_uppercase)
        .map(|_| number.to_string())
}

/// Lines joined, followed by every table column read as continuous text, so a
/// sentence the RFC wraps inside a `|` cell matches too.
fn flatten(lines: &[&str]) -> String {
    let mut text = lines.join(" ");
    let mut columns: Vec<String> = Vec::new();
    for line in lines.iter().chain(std::iter::once(&"")) {
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
    text
}

fn load_rfc(number: &str) -> Rfc {
    let raw = fs::read_to_string(repo_root().join(format!("docs/rfcs/rfc{number}.txt")))
        .unwrap_or_else(|e| panic!("docs/rfcs/rfc{number}.txt: {e}"));
    let body: Vec<&str> = raw
        .lines()
        .filter(|l| !l.starts_with("Hunt, et al.") && !l.starts_with("RFC 764"))
        .filter(|l| !l.contains('\u{c}'))
        .collect();
    let mut sections: Vec<(String, Vec<&str>)> = vec![("0".to_string(), Vec::new())];
    for line in &body {
        match header_number(line) {
            Some(n) => sections.push((n, vec![line])),
            None => sections.last_mut().unwrap().1.push(line),
        }
    }
    Rfc {
        whole: normalize(&flatten(&body)),
        sections: sections
            .into_iter()
            .map(|(n, lines)| (n, normalize(&flatten(&lines))))
            .collect(),
    }
}

/// Whether `quote` is verbatim in one of `numbers`' RFCs — inside one of
/// `sections` (or a subsection of it) when any are named, anywhere in the
/// document otherwise.
fn quotation_is_verbatim(
    rfcs: &[(&str, Rfc)],
    numbers: &[String],
    sections: &[String],
    quote: &str,
) -> bool {
    let fragments: Vec<String> = quote
        .replace("...", "…")
        .split('…')
        .map(normalize)
        .filter(|f| f.len() >= 8)
        .collect();
    if fragments.is_empty() {
        return false;
    }
    let contains_all = |text: &str| fragments.iter().all(|f| text.contains(f.as_str()));
    numbers.iter().any(|number| {
        let Some((_, rfc)) = rfcs.iter().find(|(n, _)| n == number) else {
            return false;
        };
        if sections.is_empty() {
            return contains_all(&rfc.whole);
        }
        sections.iter().any(|sec| {
            let prefix = format!("{sec}.");
            let text: String = rfc
                .sections
                .iter()
                .filter(|(n, _)| n == sec || n.starts_with(&prefix))
                .map(|(_, t)| t.as_str())
                .collect::<Vec<_>>()
                .join(" \n ");
            !text.is_empty() && contains_all(&text)
        })
    })
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
fn citations(text: &str) -> Vec<(Vec<String>, Vec<String>, String)> {
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
            // Section numbers named beside the quote: `§3.1`, `§3.4.2.4`.
            let mut sections: Vec<String> = Vec::new();
            for window in [before, after] {
                let mut rest = window;
                while let Some(j) = rest.find('§') {
                    let tail = &rest[j + '§'.len_utf8()..];
                    let num: String = tail
                        .trim_start()
                        .chars()
                        .take_while(|c| c.is_ascii_digit() || *c == '.')
                        .collect();
                    let num = num.trim_end_matches('.').to_string();
                    if !num.is_empty()
                        && num.chars().next().is_some_and(|c| c.is_ascii_digit())
                        && !sections.contains(&num)
                    {
                        sections.push(num);
                    }
                    rest = tail;
                }
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
                out.push((numbers, sections, quote.to_string()));
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
    let rfcs = [
        ("7642", load_rfc("7642")),
        ("7643", load_rfc("7643")),
        ("7644", load_rfc("7644")),
    ];
    let mut files = Vec::new();
    source_files(&repo_root().join("src"), &mut files);
    source_files(&repo_root().join("tests"), &mut files);

    let mut checked = 0;
    let mut failures = Vec::new();
    for file in &files {
        for (numbers, sections, quote) in citations(&comment_text(file)) {
            checked += 1;
            if !quotation_is_verbatim(&rfcs, &numbers, &sections, &quote) {
                let where_ = if sections.is_empty() {
                    String::new()
                } else {
                    format!(" §{}", sections.join(" / §"))
                };
                failures.push(format!(
                    "{}: RFC {}{where_} does not contain {quote:?}",
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
        "{} of {checked} RFC quotations are not verbatim in the cited place:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn the_matcher_itself_is_load_bearing() {
    let rfcs = [("7643", load_rfc("7643")), ("7644", load_rfc("7644"))];
    let n = |s: &str| vec![s.to_string()];
    let ok = |numbers: Vec<String>, sections: Vec<String>, q: &str| {
        quotation_is_verbatim(&rfcs, &numbers, &sections, q)
    };
    // Verbatim text in the section that holds it, and under its parent.
    assert!(ok(n("7643"), n("3.1"), "MUST include a non-empty id value"));
    assert!(ok(n("7643"), n("3"), "MUST include a non-empty id value"));
    assert!(ok(n("7643"), vec![], "MUST include a non-empty id value"));
    // Real text filed under the wrong section, a nonexistent section, or the
    // wrong RFC fails — the failure mode the "§§6-7" citation had.
    assert!(!ok(
        n("7643"),
        n("4.1"),
        "MUST include a non-empty id value"
    ));
    assert!(!ok(
        n("7643"),
        n("99.99"),
        "MUST include a non-empty id value"
    ));
    assert!(!ok(
        n("7644"),
        n("3.1"),
        "MUST include a non-empty id value"
    ));
    assert!(!ok(
        n("7643"),
        n("3.1"),
        "every id is a purple elephant on Tuesdays"
    ));
    // A table cell's sentence reads as one line, inside its section.
    assert!(ok(
        n("7644"),
        n("3.4.2.4"),
        "A negative value SHALL be interpreted as 0"
    ));
    assert!(ok(
        n("7644"),
        n("3.4.2.4"),
        "indicates that no resource results are to be returned except for totalResults"
    ));
    assert!(!ok(
        n("7644"),
        n("3.12"),
        "A negative value SHALL be interpreted as 0"
    ));
    // The RFC's own quotation marks and case are ignored.
    assert!(ok(
        n("7643"),
        n("3"),
        "duplicate values MUST NOT be included"
    ));

    // Section headers are found where they are, and the section list is real.
    assert_eq!(
        header_number("3.1.  Common Attributes"),
        Some("3.1".to_string())
    );
    assert_eq!(
        header_number("   3.1. Common Attributes .......... 13"),
        None,
        "table of contents"
    );
    assert_eq!(header_number("2.  SCIM Schema"), Some("2".to_string()));
    assert_eq!(header_number("RFC 7643 text"), None);
    assert!(
        rfcs[0].1.sections.len() > 40,
        "{}",
        rfcs[0].1.sections.len()
    );

    // Citation extraction finds the RFC and the section beside a quote.
    let text = comment_text_from(
        "/// RFC 7643 §3.1 says \"every representation MUST include a non-empty id\" here.",
    );
    let (numbers, sections, quote) = &citations(&text)[0];
    assert_eq!(numbers, &["7643".to_string()]);
    assert_eq!(sections, &["3.1".to_string()]);
    assert!(quote.starts_with("every representation"));
    assert!(
        citations(&comment_text_from(
            "/// plain prose with \"a long quoted phrase and no citation\"."
        ))
        .is_empty()
    );
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
