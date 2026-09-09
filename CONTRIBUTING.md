# Contributing to scim_v2

Thanks for your interest. This crate models the SCIM 2.0 wire format and
parses the grammars RFC 7644 defines. Contributions that improve RFC
conformance, close a coverage gap, or fix a real provider interop problem are
especially welcome.

## Commit signing — required

**All commits must be signed**, and GitHub must be able to *verify* the
signature. The `Signed commits required` workflow checks every commit in a PR
and will tell you exactly which of the failure modes below you hit.

Set up SSH signing once:

```bash
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519.pub
git config --global commit.gpgsign true
git config --global tag.gpgsign true
```

Then add that public key on GitHub under **Settings → SSH and GPG keys → New
SSH key**, and choose **Signing Key** as the type. An Authentication Key is
not enough.

Verify locally:

```bash
git log --show-signature -1
```

Four things go wrong in practice, and they need different fixes:

| CI reason | What it means | Fix |
|-----------|---------------|-----|
| `unsigned` | no signature at all | enable signing as above, then `git rebase --exec 'git commit --amend --no-edit -S' origin/main` |
| `unverified_email` | signature is valid, but the commit's author email is not a verified email on the signing account | add that address under Settings → Emails, or set `git config user.email` to one already verified |
| `unknown_signature_type` | GitHub will not verify this signature type (X.509/S-MIME needs an uploaded certificate) | re-sign with SSH or GPG |
| `not_signing_key` / `unknown_key` | the key is not registered as a *Signing* Key | re-add it with the Signing Key type |

After fixing, `git push --force-with-lease`.

GPG and S/MIME work too — see
[GitHub's signing guide](https://docs.github.com/en/authentication/managing-commit-signature-verification).

## Before you open a PR

Run what CI runs:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Two things people miss:

- **`--all-targets --all-features`.** Without `--all-targets` clippy never
  sees test code; without `--all-features` it never sees feature-gated items.
  CI uses both.
- **The feature matrix.** Nine configurations are built in CI. If you touch
  anything under a `#[cfg(feature = ...)]`, check at least
  `cargo test --no-default-features --features filter` and
  `--no-default-features --features models` locally.

If you edit `src/filter_parser.lalrpop`, regenerate the parser and commit both
files together:

```bash
cargo install lalrpop --version 0.23.1
lalrpop src/filter_parser.lalrpop
```

## Where tests live

Unit tests for `src/foo.rs` are in `src/foo/tests.rs`, declared at the bottom
of `foo.rs` as `#[cfg(test)] mod tests;` (a module with a feature gate keeps
it, e.g. `#[cfg(all(test, feature = "filter"))]`). They are child modules, so
`use super::*;` sees the parent's private items exactly as an inline module
would; the split only keeps the source files readable. Fixtures are pulled in
with `include_str!("../../test_data/…")` from there, and `tests/fixtures.rs`
matches that exact prefix, so a fixture read from anywhere else will show up as
unread. Cross-crate behaviour — the public API, property round-trips, the
allocation bound — lives in `tests/`.

## RFC claims quote the RFC

The RFC texts are checked in under `docs/rfcs/`. Any check that rejects or
normalises input traces to a MUST, MUST NOT, SHALL or REQUIRED in that text,
and the doc comment on the check quotes the sentence with its section:
`RFC 7643 §3.1: "MUST include a non-empty id value"`. `tests/rfc_citations.rs`
extracts every such quotation from the source and fails if it is not verbatim
in the RFC, so a paraphrase goes in plain words and quotation marks mean
quotation. Section numbers are read from the file, never recalled: this crate
briefly shipped a rule citing "§§6-7" that those sections do not contain.
"SHALL be interpreted as" is normalisation, not a reason to reject (RFC 7644
§3.4.2.4 Table 6 on `count` and `startIndex`).

Two things the RFC does that surprise people: `canonicalValues` are
*suggestions* (RFC 7643 §7), and the per-attribute listings are not always
complete — §4.1.2 omits the `primary` sub-attribute that §2.4 defines for
every multi-valued attribute.

## Changing the shape of a public type

Before changing a public type's representation, write down what the old shape
made unrepresentable. Each of those invariants is either still impossible in
the new shape or has a test that rejects it, and the PR says which.
`And(Box<_>, Box<_>)` becoming `And(Vec<_>)` silently allowed zero and one
operand; `Operands<T>` is the shape this rule would have produced first.

## Guards and limits

A limit is verified by the hostile input it exists for, on every entry point
(`Filter`, `PatchPath`, `Deserialize`, and the protocol message that carries
it), on the failure path as well as the success path (the same input with a
trailing syntax error), and by measurement where the claim is about a
resource — `tests/filter_budget_alloc.rs` counts bytes rather than reasoning
about them. A guard tested only on the path its author was looking at was
found unreachable on the one next to it.

## Error paths on in-place mutation

A function that takes `&mut` and can fail states what the value holds on
`Err`, and a test asserts it (`v == v_before`). Check first, mutate second:
`canonicalize_keys` is the two-phase example.

## Mutation-test your diff

Before opening or updating a PR:

```bash
cargo mutants --in-diff <(git diff origin/main...HEAD -- src) \
  -e filter_parser.rs -e tests.rs -e '*_tests.rs' -- --all-features
```

A surviving mutant in code you added is a missing test, not noise. Line
coverage on this crate read 96% while seven mutants in new code survived.
Mutation testing is a local step, not a CI job: it is slow and its output
needs a reader.

## Comments are for readers of the code, not of the PR

No review-round tags, finding numbers or reviewer names in code comments;
they mean nothing once the PR closes. Keep the reason and the RFC citation.
A comment explaining why something is *not* checked is a gap, not a
resolution: fix it, or list it in the PR under known gaps.

## Test fixtures

`src/test_data/` holds RFC payloads and sanitized real provider responses, and
`src/test_data/README.md` documents every one. `tests/fixtures.rs` enforces
that: a new fixture needs a README row, and the Support column has to match
whether a test actually reads the file. A fixture no test reads will fail CI.

Real provider payloads are the most valuable thing you can contribute. Strip
identifiers, keep the structure.

## Coverage

Line coverage is measured on every push to `main` and is currently ~96%
(excluding the generated parser). A PR that lowers it should say why.

```bash
cargo llvm-cov --all-features --ignore-filename-regex 'filter_parser\.rs'
```

## Dependencies

Requirements in `Cargo.toml` are caret requirements at major.minor:
`serde = "1.0"`, `thiserror = "2.0"`, `lalrpop-util = "0.23"`. Every form Cargo
accepts there is a caret requirement — `"1.0.228"` means `>=1.0.228, <2.0.0`,
not a pin — and Cargo always resolves to the newest compatible release, so a
patch digit changes nothing about what a consumer gets. All it sets is the
floor, and a floor is a claim about the oldest API this crate compiles against;
stating it at major.minor keeps that claim honest and generous. A library
cannot pin in any case: `=1.0.228` would refuse to unify with any other crate
in the graph wanting a different patch, and a library's `Cargo.lock` is ignored
by its consumers, which is why none is committed here.

Because consumers get whatever is newest on the day they build, no PR can
exercise that. The `Weekly drift` workflow (`.github/workflows/weekly.yml`)
resolves fresh with no cache, runs the suite and feature matrix on stable and
beta, re-checks the MSRV, re-runs the advisory audit, and files an issue on
failure. When it does, the fix is usually a floor bump here plus a CHANGELOG
line.

## Releases

`cargo-semver-checks` runs on every PR, so an accidental breaking change is
caught before merge. Publishing is manual: the `Publish to crates.io` workflow
is `workflow_dispatch` only, defaults to a dry run, and requires the version
input to match `Cargo.toml` and an existing tag on the same commit.

## Questions

[Open an issue](https://github.com/ShiftControl-io/scim-v2-rust/issues).
