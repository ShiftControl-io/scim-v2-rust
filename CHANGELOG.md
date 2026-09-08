# CHANGELOG

## 1.0.0

First stable release. The API is now covered by SemVer, and
`cargo-semver-checks` runs on every PR to keep it that way.

This release breaks compatibility deliberately, in the one place where doing
so is free, and every break is listed below.

### Breaking Changes

- **Multi-valued attributes are `Vec<T>`, not `Option<Vec<T>>`** — all 18 of
  them: `User`'s `emails`, `addresses`, `phoneNumbers`, `ims`, `photos`,
  `groups`, `entitlements`, `roles` and `x509Certificates`; `Group.members`;
  `ResourceType.schemaExtensions`; `Schema`'s `canonicalValues`,
  `subAttributes` and `referenceTypes`; `SearchRequest`'s `attributes` and
  `excludedAttributes`. RFC 7643 §2.5 makes an unassigned attribute, an
  explicit `null`, and an empty array equivalent in state, so the old type
  offered three representations of one thing and made every caller decide
  whether `None` and `Some(vec![])` differed. Suggested by @travipross while
  reviewing #48. All three wire forms now deserialize to an empty `Vec`,
  including explicit `null` — which needs a `deserialize_with`, since
  `#[serde(default)]` alone rejects `"roles": null`, a form providers do send.

  An unassigned multi-valued attribute serializes as `[]`, not by omission.
  RFC 7643 §2.5 permits omitting it, but RFC 7644 §3.5.1 gives `[]`
  operational meaning that omission does not have: "Clients that want to
  override a server's defaults MAY specify `null` for a single-valued
  attribute, or an empty array `[]` for a multi-valued attribute, to clear all
  values", whereas an omitted attribute is merely "not asserted" and the
  server MAY clear it *or* substitute a default. Since these models are
  request bodies as well as representations, emitting `[]` is what keeps a
  conformant clear-all expressible. `SearchRequest`'s `attributes` and
  `excludedAttributes` are the exception and are still omitted when empty:
  those are §3.9 attribute *selection*, not resource attributes, and an empty
  selection asserts nothing.

- **`ListResponse` is generic over the resource, not the ID type.** `GET
  /Users` is now `ListResponse<User<String>>` and deserializes straight into
  `Vec<User<String>>`; the heterogeneous form RFC 7644 §3.4.3 allows at the
  root `/.search` endpoint is `ListResponse<Resource<String>>`. Previously the
  parameter was the ID type and `Resources` was always a four-way enum a
  caller had to match through even when the endpoint returned one kind. It
  also boxed every variant to keep the enum small — `User<String>` is 992
  bytes against the enum's 16 — so a 100-user page meant 100 separate
  allocations where the typed form is one contiguous `Vec`.

  Reusing an existing type parameter for a new purpose is the dangerous part:
  `ListResponse<String>` used to compile and mean "ids are strings", and would
  have kept compiling under the new meaning while failing at runtime on
  deserialize. `R` is therefore bounded by the new `ScimResource` trait, which
  has a private supertrait, so `ListResponse<String>` is a compile error
  (E0277) rather than a runtime surprise. The sealing also fixes the
  implementing set to this crate's resources, which is the intended scope.
  Note that `cargo-semver-checks` has no lint for this class of change — the
  seal is what catches it. It has no lint for a public field's type changing
  either, which is why the 0.5.0 release below needed its break verified by
  compiling a consumer against the published 0.4.2 rather than by trusting the
  tool.

- **The `serialize()` and `deserialize()` methods are gone** from `User`,
  `Group`, `EnterpriseUser`, `Schema`, `ResourceType` and
  `ServiceProviderConfig`. They wrapped `serde_json::to_string` / `from_str`
  and only remapped the error type, while hiding `to_writer`, `from_slice` and
  `from_value`; an inherent method named `deserialize` beside
  `serde::Deserialize::deserialize` shadowed the trait method. Use
  `serde_json` directly.

- **`validate()` moved to the `Validate` trait** and returns `ValidationError`
  instead of `SCIMError`. Add `use scim_v2::Validate;`. The error carries the
  SCIM **wire** path (`userName`, not `user_name`) and the RFC 7644 §3.12
  `scimType` keyword, and `ValidationError::to_http_error` builds the error
  body a server should return.

- **`Schema`, `ResourceType` and `ServiceProviderConfig` gained a `schemas`
  field.** All three carry it on the wire per RFC 7643 §§5-7, but none
  modelled it, so a present value was silently dropped and never
  round-tripped. `#[serde(default)]` keeps absence legal, which §§6-7 allow.

- **`Address` gained `value`, `display` and `primary`.** RFC 7643 §2.4 defines
  all three as common sub-attributes of every multi-valued attribute, and
  gives "the preferred mailing address" as its example of `primary`; §4.1.2's
  listing for `addresses` simply omits them. JumpCloud sends
  `addresses[].primary`, and it was being dropped.

- **`MemberType` gained an `Other(String)` variant** and is
  `#[non_exhaustive]`. RFC 7643 §7 defines `canonicalValues` as "a
  collection of *suggested* canonical values that MAY be used", so a provider
  may send a `members.type` outside {User, Group} — which previously failed
  deserialization of the entire enclosing payload, losing a whole
  `ListResponse` to one unrecognised member. Unknown labels now round-trip
  verbatim, and matching is case-insensitive per the schema's
  `caseExact: false`. Because the enum now has a non-unit variant, a numeric
  cast like `MemberType as isize` no longer compiles.

- **`SCIMError`, `FilterActionError`, `Resource` and `MemberType` are
  `#[non_exhaustive]`.** After 1.0 an added variant is a breaking change, so
  each public enum was given a deliberate answer. The enums that mirror RFC
  7644's grammar — `Filter`, `AttrExp`, `ValFilter`, `CompareOp`, `CompValue`,
  `PatchPath`, `PatchOperation`, `OperationTarget`, `MaybeFilter` — stay
  exhaustive on purpose, because the RFC closes those sets and an exhaustive
  `match` is worth having. The model *structs* are deliberately not
  `#[non_exhaustive]`, since that would forbid `..Default::default()`
  downstream; `User` was audited against the embedded RFC 7643 §4.1 schema and
  carries every attribute.

- **`SearchRequest::excluded_attributes` is now `pub`.** It never was, so no
  caller could set it.

### Added

- **Feature flags `filter`, `models` and `schemas`, all on by default**, so an
  existing consumer sees no change. Turning off `filter` drops eight crates —
  `lalrpop-util`, `fluent-uri`, `regex-automata`, `regex-syntax`,
  `aho-corasick`, `borrow-or-share`, `ref-cast`, `ref-cast-impl` — taking the
  tree from 22 to 14 and removing a regex engine from the supply chain. For a
  SCIM server that has its own resource model and wants only the grammar:
  `default-features = false, features = ["filter"]`.
- `ScimResource`, a sealed trait naming the resources that may appear in a
  `ListResponse`, with `schema_urn()`.
- `Validate`, `ValidationError` and `ValidationErrorKind`, re-exported at the
  crate root.
- `schema_urns::ERROR`, `SERVICE_PROVIDER_CONFIG`, `BULK_REQUEST` and
  `BULK_RESPONSE`.

### Fixed

- `EnterpriseUser::validate` demanded `employeeNumber`, `costCenter`,
  `organization`, `division`, `department` and `manager`, so it rejected every
  conformant extension that left any of them unset. RFC 7643 §4.3 defines no
  REQUIRED attribute, and all six are `required: false` in the schema this
  crate embeds. The module had no tests at all, which is why nobody noticed.
- `ServiceProviderConfig::validate` failed whenever `patch`, `bulk`, `filter`,
  `changePassword`, `sort` or `etag` reported `supported: false`. §5 makes the
  `supported` *field* required, not its value true: a server without bulk
  support correctly advertises `"bulk": {"supported": false}`. It now checks
  `authenticationSchemes`, which §5 does mark REQUIRED.
- `primary` tolerated a stringified boolean only on `Role`, the one place
  #45 had patched. A provider sending `"primary": "true"` on an email hit the
  identical failure. All nine multi-valued types now share the lenient
  deserializer.
- `Bulk::default()` returned `maxOperations: 1000, maxPayloadSize: 1048576`
  and `Filter::default()` returned `maxResults: 100`, while
  `ServiceProviderConfig::default()` hand-built the same structs with zeros —
  two different defaults for one type depending on the constructor. They now
  delegate, and the limits are zero, since a positive limit beside
  `supported: false` advertises a capacity the server lacks.
- `SCIMError` implements `std::error::Error`. It had a hand-written `Display`
  and no `Error` impl, so it could not be boxed as `dyn Error` or composed
  with `anyhow`. It now derives `thiserror::Error`, like the filter errors
  already did.

### Fixed (MSRV)

- `rust-version` said `1.85`, and the crate has not built on 1.85 since the
  filter parser landed: `lalrpop-util`'s entire 0.23 line declares rustc 1.86,
  and the checked-in parser is generated by 0.23.1, so dropping to 0.22 would
  mean regenerating it. Nothing checked the claim, which is what the new MSRV
  CI job is for — it failed on its first run. Now `1.86`, verified against a
  real 1.86.0 toolchain. The crate's *code* still compiles on 1.85 without
  `filter`, verified likewise, but that is not a usable configuration:
  `rust-version` is a package-wide floor Cargo enforces before compiling
  anything and is not conditional on feature selection, so 1.86 gates every
  configuration. Cargo cannot express a per-feature MSRV.

### Fixed (red-team round 2)

- **`utils::case` corrupted correctly-cased payloads.** `WIRE_NAMES` listed
  both `Resources` and `resources`, and both `Operations` and `operations`;
  the lowercase entries were Rust field names that had leaked into a table of
  wire names. Keyed by lowercase, the pairs collapsed and the lowercase
  spelling won, so the case-insensitive path rewrote the RFC's own
  `"Resources"` to `"resources"` — every page came back **empty with no
  error**, because the field has a default — and `"Operations"` to
  `"operations"`, so a PATCH body byte-identical to RFC 7644's example failed
  to parse. The two entries are gone and a test asserts the table is
  injective under case folding, which the membership test could never catch.
- **Colliding keys were resolved last-write-wins**, with the winner decided
  by `serde_json::Map`'s byte order rather than document order — so
  `{"userName":"alice","USERNAME":"admin"}` and its reverse gave different
  answers, and an attacker could pick the spelling that wins. RFC 7643 §2.1
  makes them one attribute asserted twice with no precedence rule. It is now
  an `AmbiguousKey` error, surfaced through every entry point.
- **Canonicalisation reached into extension namespaces the crate does not
  own.** Keys like `Value` or `TITLE` inside `urn:example:…` were rewritten
  to this crate's spelling, although that namespace is not governed by §2.1
  and its vendor may be case-sensitive; the module doc had claimed nothing
  was lost. A subtree under an unknown URN is now left byte-identical.
- `SerializationError` gained the `#[source]` its sibling already had, so the
  `serde_json` error is reachable through the standard chain walk.
- `to_http_error` takes no status: every `ValidationErrorKind` is a 400, and
  the `u16` parameter accepted `0` or `65535` while the crate's own
  `ScimHttpError::validate` rejected them. A body needing another status is
  built directly and validated.
- The seal's second `compile_fail` guard had been failing for an unrelated
  reason — its impl omitted `declared_schemas` — and so proved nothing about
  the seal; it is complete now, and the inert-on-stable error codes are
  dropped in favour of a comment naming the mutation each snippet catches.
- Pinned, all previously unguarded: the four `ListResponse::validate` bounds,
  every `validate_context` branch on `User` and `Group` including the
  `password` one, `Valid` and `Strict` as unit tests, the three null-collapse
  sites, `at_most_one_primary` on all eight attributes, `Schema::validate`,
  and `MemberType`'s case-insensitive `PartialEq` together with its `Hash`
  through a real `HashSet`. The fixture-inventory check strips block comments
  and matches the `include_str!` invocation rather than a bare path. The
  property strategy now generates all nine multi-valued attributes, and the
  casing property covers `ListResponse` and `PatchOp` — the case that would
  have caught the collision at authoring time.
- CI's matrix now includes the shipped default feature set, which no row had
  exercised: six rows were reductions and the other two both enabled the
  non-default compact posture.

Deferred with reason: R2-I1, `MAX_FILTER_DEPTH` counting terms in a flat
`or` chain as nesting. Pre-existing, and `FilterActionError` is
`#[non_exhaustive]`, so a distinct too-many-terms variant can land in 1.x;
the constant's *value* is the only thing 1.0 freezes.

### Added (resilience and conformance pass)

- **Direction-aware validation.** `Validate` gains `validate_as(Context)` on
  top of `validate()`, with `Context::{CreateRequest, ReplaceRequest,
  Response}`. RFC 7643 §3.1 makes `id` REQUIRED in a server's representation
  and "MUST NOT be specified by the client" on create; §4.1's `password` is
  `returned: never`. Neither can be expressed by a single direction-agnostic
  check, and `validate()` remains that check. A replace request tolerates
  `id`, since RFC 7644 §3.5.1 has the server ignore readOnly attributes and
  its own PUT example carries one.
- **`Valid<T>`.** Obtainable only through `Valid::new(value, Context)`, so a
  handler that takes `Valid<User>` cannot be handed an unvalidated resource:
  forgetting to validate is a compile error, at zero runtime cost. `Deref`
  but no `DerefMut`, since mutation could invalidate it.
- **`Strict<T, M>`.** Deserialize-and-validate in one step for callers who
  want to reject a non-conformant body at the parse boundary, with `M` one of
  the `CreateRequest` / `ReplaceRequest` / `Response` markers. The plain
  models stay lenient so a real provider's payload can always be read.
- **Case-insensitive attribute names**, RFC 7643 §2.1: "Attribute names are
  case insensitive". A payload carrying `"USERNAME"` failed with `missing
  field userName`. `utils::case` canonicalises every known attribute,
  sub-attribute, protocol member and URN before deserializing —
  `CaseInsensitive<T>`, `utils::case::from_str`, `from_value` — leaving
  unknown keys untouched. The Java SCIM SDK gets the same effect from
  Jackson's `ACCEPT_CASE_INSENSITIVE_PROPERTIES`; scim2-models lowercases
  every key citing §2.1. Behind the `case-insensitive` feature, on by default.
- **Wire-behaviour choices are types, not features.** An earlier revision of
  this branch exposed lenient booleans and compact output as Cargo features.
  The second red-team round pointed out what that meant: Cargo unifies
  features across the whole dependency graph, so any transitive crate
  enabling `compact-multi-valued` for its own logging would silently stop
  every other consumer's RFC 7644 §3.5.1 clear-all from reaching the wire,
  with no way for them to opt out. Both were removed. Lenient boolean parsing
  is simply always on — it only widens what is accepted. Compact output is
  `utils::compact::Compact(&value)`, a serialize wrapper the caller chooses at
  the call site, on the same pattern as `CaseInsensitive<T>` and
  `Strict<T, M>`. `case-insensitive` stays a feature because it is genuinely
  additive: it only compiles an opt-in module.
- Every model derives `Clone` and `PartialEq`, and `tests/roundtrip_proptest.rs`
  generates resources and asserts serialize-then-deserialize is the identity.
  Each asymmetry this release fixed by hand was a round-trip failure of the
  kind this now finds mechanically.
- `Schema` gains `Validate`: RFC 7643 §7 says service providers "MUST
  specify" the schema URI as `id`.

### Added (independent RFC review)

- `sortBy` and `sortOrder` on `SearchRequest` and `ListQuery`, per RFC 7644
  §3.4.2.3 and §3.4.3, with a `SortOrder` enum. Neither field existed, so a
  client could not express a sort and a server deserializing a search body
  silently lost one. `SearchRequest::validate` rejects `sortOrder` without
  `sortBy`, which §3.4.2.3 defines as the order in which `sortBy` is applied.
- `ScimType`, the ten RFC 7644 §3.12 keywords as an enum with an `Other`
  catch-all, and `ScimHttpError.scim_type` is now `Option<ScimType>` rather
  than a bare string servers would hand-type. `ValidationError::scim_type()`
  returns it; `scim_type_str()` gives the wire string.
- `Validate` for `SearchRequest`, `PatchOp` (the URN, and §3.5.2's "one or
  more" operations — an empty array previously deserialized and validated as
  nothing) and `ScimHttpError` (the URN, and a numeric status).
- Every `Validate` impl now checks that `schemas` carries the resource's own
  URN. RFC 7643 §3: it "MUST only contain values defined as schema and
  schemaExtensions for the resource's defined resourceType" — a `User` whose
  `schemas` named the Group URN passed before. Additional URNs are still
  accepted, since a resource may carry an extension this crate does not model.
- `User` and `ServiceProviderConfig` validation enforces RFC 7643 §2.4 — "The
  primary attribute value true MUST appear no more than once" — on every
  multi-valued attribute that carries `primary`.
- `AuthenticationScheme` validation enforces §5: `type`, `name` and
  `description` REQUIRED. The red-team finding that led to `type` becoming
  optional cited §8.7's schema representation, which §8 describes as
  non-normative; §5 prose governs. The field stays `Option` so a document
  following the §8.7 text still parses, and `validate` reports the gap by
  index.
- `active` tolerates a stringified boolean, as every `primary` already did.
- Grammar coverage: 36 table-driven cases pinning RFC 7644 §3.4.2.2's ABNF —
  nameChar rules, keyword case, every JSON number form, the `pr`/`valuePath`
  compositions, and the rejections (`pr` with a value, `valuePath.subAttr`
  inside a filter, capitalised literals, unbalanced parens, trailing input) —
  plus all five Figure 8 PATCH paths, every documented `op` spelling, and
  `Resource` dispatch with an unknown URN beside a known one. The parser
  handled all of them already; nothing had pinned them.

### Fixed (red-team triage)

- `AuthenticationScheme.primary` was the ninth `primary` carrier and the one
  without the lenient boolean deserializer, so a provider stringifying
  booleans made the *entire* `/ServiceProviderConfig` document unparseable —
  the one call a client makes before it knows any of a provider's quirks. All
  nine are now covered, and pinned by a test that goes through the real model
  types rather than a synthetic struct.
- `ListResponse.Resources` and `ServiceProviderConfig.authenticationSchemes`
  were missed by the null-collapsing sweep. `"Resources": null` lost an entire
  empty page including `totalResults`, and `"authenticationSchemes": null`
  failed deserialization before `validate()` could report the attribute by
  wire path.
- `AuthenticationScheme.type` and `.spec_uri` are now `Option<String>`. RFC
  7643's schema defines no `type` sub-attribute at all and marks `specUri`
  `required: false`, so a conformant minimal scheme was rejected at the type
  level — including the payload this crate's own `TryFrom` doc example
  presents.
- `ListResponse::validate` was one-sided: it checked only that a *short* page
  carried its pagination markers, so `totalResults: -5`, three resources
  against `totalResults: 1`, and `startIndex: 0` all passed. It now rejects a
  negative `totalResults`, a `totalResults` smaller than the number of
  resources returned, a `startIndex` below 1 (§3.4.2: "the 1-based index of
  the first result"), and a negative `itemsPerPage`.
- `ListResponse::validate` also compares each resource's declared `schemas`
  against the type it was parsed as. `Resource`'s deserializer dispatches on
  that URN "to prevent type confusion", and the typed path bypassed it: a
  Group payload carrying a `userName` deserialized cleanly into
  `ListResponse<User<String>>` with both validators returning `Ok`. This is
  what `ScimResource::schema_urn` is for; it was otherwise vestigial, and
  `declared_schemas` is added alongside it. Absence stays legal per §§6-7.
- `MemberType` compared `Other` byte-exact despite the schema's
  `caseExact: false`, so `Other("serviceaccount") != Other("ServiceAccount")`
  and `Other("User") != MemberType::User` even though both serialize to
  `"User"`. `PartialEq` and `Hash` are now hand-written and route through the
  same folding `From<String>` applies.
- `ValidationError::to_http_error` took the HTTP status as an unvalidated
  `impl Into<String>`, so `"not-a-status"` produced a syntactically valid but
  non-conformant §3.12 body. It now takes `u16` and does the rendering, making
  the wrong value unrepresentable.
- Restored the four `Manager` serialization guards that #48's review had
  specifically asked for and this release's rewrite of the module deleted —
  the rewrite took coverage from 0% to 93% while removing its only defence
  against the defect the module had been fixed for.
- Three empty inherent `impl` blocks left behind by the serde-wrapper removal.

### Security and release pipeline

- `publish.yml` interpolated `${{ inputs.version }}` directly into a `run:`
  body, in the one job that holds `CRATES_TOKEN`. `${{ }}` is substituted
  before bash parses the line, so a version containing a quote could run
  arbitrary commands — after the `crates-io` environment approval had already
  been granted, and before the version, manifest and tag checks. The input now
  arrives through `env:` and its shape is validated first.
- The publish tag guard could not fire in either direction: `actions/checkout`
  defaults to `fetch-tags: false`, so dispatched from a branch it reported
  "tag does not exist" for a tag that did — and following that advice is how a
  tag gets force-moved onto the wrong commit. It now fetches tags and resolves
  against `origin`, so "not pushed" and "not fetched" are distinguishable.
- The publish `Verify` step claimed to re-run everything CI runs and skipped
  the dependency audit, the feature matrix, `cargo doc` and
  `RUSTFLAGS: -D warnings`. With caret requirements and no committed lock, a
  RUSTSEC advisory landing since the last run on main was invisible at publish
  time. All four are now included.
- `cargo-deny` ran `check advisories licenses sources`, so `deny.toml`'s
  `[bans]` section — including `wildcards = "deny"` — was never evaluated.
- The signed-commits check read the API inside a process substitution, which
  `set -e` cannot observe, so a rate limit reported as an unsigned commit with
  an empty summary. The coverage-badge `curl` had no `--fail`, so an expired
  token went green while the badge served a stale value for ever.

### Documentation and tooling

- README rewritten: badges, the feature table, and examples for the 1.0 API.
  Its code blocks are compiled and run as doctests, so it cannot drift. Four
  lines had shipped with escaped backticks, rendering as literal `` \` `` on
  GitHub.
- `CONTRIBUTING.md`, a PR template, and issue templates for bugs, RFC
  conformance gaps and feature requests.
- CI: clippy now runs `--all-targets --all-features` (it previously saw
  neither test code nor gated items), plus new jobs for the six-configuration
  feature matrix, the declared MSRV, `cargo doc -D warnings`,
  `cargo-semver-checks` and `cargo-deny`, and coverage published as a badge.
- A `Signed commits required` check that maps each of GitHub's
  `verification.reason` codes to its specific fix, rather than reporting the
  code alone.
- Publishing to crates.io is manual (`workflow_dispatch`), defaults to a dry
  run, requires the version input to match `Cargo.toml` and a tag on the same
  commit, and runs behind a `crates-io` environment. It was previously
  triggered by any `v*.*.*` tag push.
- Test enforcement tightened after the red team demonstrated it was partly
  vacuous: `tests/fixtures.rs` matched raw source text, so a fixture mentioned
  only in a `//` comment satisfied "read by a test", and provider samples had
  no inventory check at all. Comments are now stripped, the support column is
  resolved against the directory each row's fixture actually lives in, and
  provider samples get their own row check. The three `compile_fail` doctests
  guarding the sealed trait inline their full paths, so a broken `use` line
  cannot make them pass for the wrong reason, and a third case fails the
  moment `mod sealed` becomes public — both verified by mutation. A new
  `tests/public_api.rs` exercises the public surface from a separate crate,
  which is the only place field privacy is enforced: deleting `pub` from
  `SearchRequest::excluded_attributes` previously left the whole suite green.
- A weekly scheduled `Weekly drift` workflow, which is the guard the loose
  caret requirements actually need: consumers get whatever is newest on the day
  they build, and nothing about that is exercised by a PR, since `build.yml`
  only runs when this repo changes and the risk here is something outside it
  changing. It resolves fresh with no cache, runs the suite and the feature
  matrix on stable (plus beta, advisory), re-checks that the advertised MSRV
  still holds — a dependency can raise its own `rust-version` with no commit
  here — re-runs the advisory audit, and files an issue on failure, since
  GitHub does not surface a failed scheduled run the way it does a failed PR.
- Dependency requirements are stated as caret requirements at major.minor
  (`serde = "1.0"`, `thiserror = "2.0"`, `lalrpop-util = "0.23"`,
  `fluent-uri = "0.4"`). Every form is a caret requirement and cargo resolves
  to the newest compatible release either way, so the patch digit only set a
  floor — a claim about the oldest API this crate compiles against — and a
  needlessly precise floor forced consumers to upgrade for no reason.
- Line coverage 88.25% → 95.99%, functions 81.15% → 90.02%, excluding the
  generated parser and measured with CI's exact `cargo llvm-cov` recipe
  (unit and integration tests; doctests are not instrumented on stable). `tests/fixtures.rs` now enforces that every fixture is
  documented and read by a test, which found three dead JumpCloud fixtures
  that now have round-trip tests.
## 0.5.0

Releases the RFC 7644 conformance work from #47 and #48. A minor bump rather
than a patch, because two field types changed and `^0.4` treats that as
breaking.

### Breaking Changes

- `ListResponse.items_per_page` and `ListResponse.start_index` are now
  `Option<i64>`, and so are `SearchRequest.start_index` and
  `SearchRequest.count`. RFC 7644 §3.4.2 makes the two `ListResponse`
  pagination markers REQUIRED only "when partial results are returned due to
  pagination", and §3.4.3 marks both `SearchRequest` fields OPTIONAL — so the
  crate could not deserialize the RFC's own unpaginated `ListResponse`
  example, nor an RFC-legal `SearchRequest` that omitted them. `Option<i64>`
  rather than `#[serde(default)]` to an inert `0`, so a server that really
  sent `0` stays distinguishable from one that omitted the field. Migration is
  mechanical: wrap reads in `Option`, or `.unwrap_or(1)` / `.unwrap_or(0)` at
  the call site.

### Added

- `ListResponse::validate()`, matching the per-model convention. Checks that
  `schemas` is present, and that a partial page — fewer entries in `Resources`
  than `totalResults` — carries both pagination markers, which §3.4.2 requires
  in exactly that case. The `skip_serializing_if` on those fields otherwise
  makes a non-conformant response expressible.
- A corpus of 22 RFC 7644 sample payloads and 5 sanitized provider responses
  under `src/test_data/`, each documented with its RFC section in
  `src/test_data/README.md`, plus round-trip tests over them.

### Fixed

- Unassigned `Option` fields are consistently omitted on serialize rather than
  emitted as `null`: `Name.formatted`, and four fields on `EnterpriseUser`.
  RFC 7643 §2.5 treats `null` and omitted as equivalent in state and permits
  the compact form, and the rest of the crate already did this. A fully-unset
  `Name` now serializes to `{}` rather than `{"formatted":null}`. Thanks to
  @travipross for both #47 and #48.

## 0.4.2

### Fixed
- `Role.primary` may again be omitted on the wire and deserializes to `None`. The lenient bool deserializer added in 0.4 suppressed serde's implicit `Option` default, so a `Role` without a `primary` key (as sent by GitHub Enterprise, e.g. `{"value": "enterprise_owner"}`) failed with a `missing field` error. Pairing `deserialize_with` with `#[serde(default)]` restores the 0.3 behaviour while keeping the stringified-bool leniency Entra requires.

## 0.4.1

### Fixed
- `ListResponse.Resources` may now be omitted on the wire and deserializes to an empty list. Per RFC 7644 §3.4.2, `Resources` is REQUIRED only when `totalResults` is non-zero, so responses for empty result sets no longer fail to deserialize.

## 0.4

### Security
- Reject SCIM filter and PATCH-path strings whose parsed AST exceeds `filter::MAX_FILTER_DEPTH` (64). Previously, deeply nested inputs like `not (not (… (title pr) …))` or long `and`/`or` chains would produce an AST that overflowed the stack on subsequent `Display` / `==` / `{:?}` / serde / drop, allowing a remote DoS. Over-deep inputs now return `ParseError::User(FilterActionError::DepthExceeded(_))`.

### Breaking Changes
- Refactor `PatchOperation` into a tagged enum (`Add`, `Remove`, `Replace`) with `OperationTarget` variants (`WithPath`, `WithoutPath`), replacing the old `PatchOperations` struct. Paths are now parsed as `PatchPath` filter expressions instead of raw strings.
- Parameterize ID types on `User`, `Group`, `Member`, `Resource`, and `ListResponse` (default type parameter is `String`, so unparameterized usage is unchanged)
- `Member.type` changed from `Option<String>` to `Option<MemberType>` enum
- `SearchRequest` and `ListQuery` are now generic over the filter field type (`SearchRequest<F = Filter>` / `ListQuery<F = Filter>`). Unparameterized usage is source-compatible; explicit turbofish on a bare generic name (e.g. `ListQuery::<_>::default()`) may need adjustment.
- Remove `TryFrom<&str>` impls on `User` and `Group`; use `serde_json::from_str` directly
- Remove `Default` impls on `Group`, `ListResponse`, and `PatchOp`

### Added
- SCIM filter expression parser (`filter` module, RFC 7644 §3.4.2.2); `SearchRequest` and `ListQuery` now include an optional `filter` field
- `filter::MaybeFilter` deserialize-only wrapper plus `TolerantListQuery` / `TolerantSearchRequest` aliases (= `ListQuery<MaybeFilter>` / `SearchRequest<MaybeFilter>`) so servers can recover malformed filter input and return an RFC 7644 §3.12 `invalidFilter` response without losing `start_index`, `count`, and other fields
- Case-insensitive deserialization of `PatchOperation` ops and `MemberType` for Entra compatibility
- Lenient `bool` deserialization (accepts `"true"`/`"false"` strings) on fields like `Role.primary`

## 0.3

- Add `externalId` to user and group entities
- **Breaking** - Uses raw identifiers, converting `type_` to `r#type`
