## Summary

<!-- What this changes and why. If it's an RFC conformance fix, cite the
     section — the RFC texts are in docs/rfcs/. -->

## Type of change

- [ ] Bug fix (no API change)
- [ ] New feature (no API change)
- [ ] Breaking change (requires a major version bump)
- [ ] Test coverage only
- [ ] Documentation only
- [ ] Tooling / CI / repo config

## Checklist

- [ ] My commits are signed and GitHub verifies them (`git log --show-signature`)
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --all-features` passes
- [ ] If I touched anything feature-gated, I checked the reduced configurations
      (`--no-default-features --features filter`, and `--features models`)
- [ ] If I edited `filter_parser.lalrpop`, I regenerated and committed
      `filter_parser.rs`
- [ ] If I added a test fixture, it has a row in `src/test_data/README.md` and
      a test that reads it
- [ ] Any claim about SCIM cites an RFC section I checked against `docs/rfcs/`,
      and every new rejection or normalisation quotes the sentence it enforces
      (`tests/rfc_citations.rs` passes)
- [ ] If a public type's shape changed, the PR lists what the old shape made
      unrepresentable and how the new one still forbids it
- [ ] Any new limit or guard is tested with the hostile input it exists for,
      on every entry point and on the failure path
- [ ] `cargo mutants --in-diff` over my diff: no surviving mutants in added
      code, or each one is explained in the PR
- [ ] No review-round tags or reviewer names in code comments

## Test plan

<!-- What you ran, and what a reviewer should look at. For a conformance fix,
     the most useful thing is a test that fails before the change. -->
