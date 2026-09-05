# Test fixtures

Two groups, one per subdirectory:

- **`rfc7644/`** — sample payloads lifted from [RFC 7644](https://www.rfc-editor.org/rfc/rfc7644).
- **`provider_samples/`** — sanitized real responses from SCIM providers.

## `rfc7644/` — RFC 7644 sample payloads

Each file is one JSON payload from RFC 7644. Filenames follow
`s<section>_<statement>.json`, or `s<section>_fig<N>_<statement>.json` when the
payload is one of the RFC's numbered figures — so the RFC location is
self-documenting. The matching tests are named `rfc7644_s<section>_<statement>`
(dots in the section become underscores).

**RFC 7644 has only nine numbered figures** (Figures 1-9); most of the payloads
below are *unnumbered* inline examples, so most filenames carry no `fig<N>`
segment. Only three do: Figures 3, 4 and 5. The **RFC 7644 location** column
below gives the full citation: a section number, plus either the figure number
or the sentence in the RFC that introduces the example. Whitespace is normalized
from the RFC's printed form; content is otherwise verbatim **except** as noted
under "Deviations" below.

The **Support** column reflects whether this crate currently models the payload:

- `SUPPORTED` — a model exists and a test in `src/models/` deserializes (and,
  where lossless, round-trips) this fixture.
- `NOT SUPPORTED` — no model yet. The fixture is checked in so a test can be
  added when support lands; nothing references it today.

| File | RFC 7644 location | Payload | Support |
| --- | --- | --- | --- |
| `s3.3_user_create_request.json` | §3.3, example: "a client sends a POST request containing a `User` to the `/Users` endpoint" | `User` — POST /Users request | SUPPORTED |
| `s3.3_user_create_response.json` | §3.3, example: the 201 Created body ("returns a representation of the resource created") | `User` — creation response | SUPPORTED |
| `s3.4.1_user_retrieval_response.json` | §3.4.1, example: "The example below retrieves a single User via the `/Users` endpoint" | `User` — GET /Users/{id} response | SUPPORTED |
| `s3.4.2_list_response.json` | §3.4.2, example: "The following is an example response to the query above" (`GET /Users?attributes=userName`) | `ListResponse` — no pagination fields | SUPPORTED |
| `s3.4.2.4_fig3_pagination_response.json` | §3.4.2.4, **Figure 3** ("ListResponse Format for Returning Multiple Resources") | `ListResponse` — paginated | SUPPORTED |
| `s3.4.3_fig4_search_request.json` | §3.4.3, **Figure 4** ("Example POST Query Request") | `SearchRequest` — POST /.search body | SUPPORTED |
| `s3.4.3_fig5_post_query_response.json` | §3.4.3, **Figure 5** ("Example POST Query Response") | `ListResponse` — paginated, heterogeneous | SUPPORTED |
| `s3.5.1_user_put_request.json` | §3.5.1, example following "a successful PUT operation returns … the entire resource … For example" | `User` — PUT request (empty `roles`) | SUPPORTED |
| `s3.5.1_user_put_response.json` | §3.5.1, example: "The service responds with the entire updated User" | `User` — PUT response | SUPPORTED |
| `s3.5.2.1_add_member.json` | §3.5.2.1, example: "how to add a member to a group" (rfc7644.txt L2033-2048; **not** Figure 6) | `PatchOp` — `add` member with path | SUPPORTED |
| `s3.5.2.1_add_user_attributes.json` | §3.5.2.1, example: "how to add one or more attributes to a User resource without using a `path` attribute" | `PatchOp` — `add` attributes, no path | SUPPORTED |
| `s3.5.2.2_remove_member_by_filter.json` | §3.5.2.2, example: "Remove a single member from a group" | `PatchOp` — `remove` member by filter | SUPPORTED |
| `s3.5.2.2_remove_all_members.json` | §3.5.2.2, example: "Remove all members of a group" | `PatchOp` — `remove` all members | SUPPORTED |
| `s3.5.2.2_remove_complex_attribute.json` | §3.5.2.2, example: "Removal of a value from a complex multi-valued attribute" | `PatchOp` — `remove` by compound filter | SUPPORTED |
| `s3.5.2.2_remove_by_filter_then_add_member.json` | §3.5.2.2, example: "Example request to remove and add a member" | `PatchOp` — `remove` by filter + `add` | SUPPORTED |
| `s3.5.2.2_replace_all_members.json` | §3.5.2.2, example: "how to replace all of the members of a group with a different members list" | `PatchOp` — `remove` + `add` members | SUPPORTED |
| `s3.5.2.3_replace_members_single_op.json` | §3.5.2.3, example: "… with a different members list in a single replace operation" | `PatchOp` — `replace` members list | SUPPORTED |
| `s3.5.2.3_replace_work_address.json` | §3.5.2.3, example: "how to change a User's entire `work` address, using a `valuePath` filter" | `PatchOp` — `replace` filtered entry | SUPPORTED |
| `s3.5.2.3_replace_street_address_via_filter.json` | §3.5.2.3, example: "how to change a specific sub-attribute `streetAddress` … selected by a `valuePath` filter" | `PatchOp` — `replace` sub-attribute | SUPPORTED |
| `s3.5.2.3_replace_multiple_attributes.json` | §3.5.2.3, example: "how to replace all values of one or more specific attributes of a User resource" | `PatchOp` — `replace` attributes, no path | SUPPORTED |
| `s3.6_error_response.json` | §3.6, example: the 404 body from "Client's attempt to retrieve the previously deleted User" | `Error` — 404 on DELETE | SUPPORTED |
| `s3.7.1_bulk_request_circular_reference.json` | §3.7.1, example: "The following example exhibits the potential conflict" | `BulkRequest` — circular group refs | NOT SUPPORTED |

### Deviations from the printed RFC text

- **`s3.4.2_list_response`, `s3.4.2.4_fig3_pagination_response`,
  `s3.4.3_fig5_post_query_response`** — the RFC abbreviates the embedded
  resources (to `id` + `userName`, or to a bare `{...}`) and omits their
  `schemas` array. This crate's `Resource` deserializer requires a
  resource-type discriminator and will not guess, so each embedded resource
  here carries a concrete body with a `schemas` array.
  `s3.4.3_fig5_post_query_response` additionally drops the trailing `...` that
  truncates the RFC's `Resources` list.
- **`s3.5.2.1_add_member`, `s3.5.2.2_remove_member_by_filter`,
  `s3.5.2.2_remove_by_filter_then_add_member`, `s3.5.2.2_replace_all_members`,
  `s3.5.2.3_replace_members_single_op`** — retain the RFC's truncated `$ref` /
  `value` UUIDs (e.g. `2819c223...413861904646`). The filter parser treats
  these as opaque strings, so they exercise the same code paths.
- **`s3.5.2.2_remove_by_filter_then_add_member`** — the RFC prints the remove
  path as `members[value eq"..."]` with no space after `eq`; a space was added
  so the filter expression parses. (The truncated `2819c223...` / `08e1d05d...`
  UUIDs are kept as printed, per the bullet above.)
- **`s3.5.2.1_add_user_attributes`, `s3.5.2.3_replace_multiple_attributes`** —
  the RFC prints both `Operations` bodies with the operation object's closing
  brace missing (`… "nickname": "Babs" }]` where `}}]` is required), so neither
  parses as JSON. The brace is restored here; nothing else is changed. For the
  §3.5.2.1 example this is [RFC 7644 Errata ID 8096](https://www.rfc-editor.org/errata/eid8096)
  ("missing one closing curly bracket", Verified 2025-10-28); the §3.5.2.3
  example carries the identical typo.

### Not included

- The RFC's **Figure 2** (§3.4.2.2, "Example Filters") and **Figure 1** (§3.4.2.2,
  "ABNF Specification of SCIM Filters") are filter-expression grammar/strings,
  not JSON bodies — covered by the `filter` module's own tests.
- The RFC's **Figure 7** ("SCIM PATCH PATH Rule"), **Figure 8** ("Example Path
  Values") and **Figure 9** (§4, "Example Resource Type JSON Representation") are
  not checked in here as standalone fixtures.
- The remaining §3.7 bulk examples (bulk response body, `bulkId` temporary
  identifiers, error-in-response, `maxOperations` exceeded) are not checked in
  yet; add them alongside a `BulkRequest` / `BulkResponse` model.

## `provider_samples/`

Sanitized real responses, kept for regression coverage of
non-conformant-but-common provider behavior:

| File | Source |
| --- | --- |
| `entra_user_creation_test.json` | Microsoft Entra (via scimvalidator.microsoft.com) |
| `github_enterprise_user_list_test.json` | GitHub Enterprise `/Users` |
| `jumpcloud_create_user.json` | JumpCloud user creation |
| `jumpcloud_put_user_full.json` | JumpCloud PUT (full) |
| `jumpcloud_put_user_minimal.json` | JumpCloud PUT (minimal) |
