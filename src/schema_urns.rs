//! Canonical SCIM schema URNs defined by RFC 7643 and RFC 7644.
//!
//! These are the protocol-level identifiers servers and clients use in the
//! `schemas` attribute to declare the type of a resource or message.

pub const USER: &str = "urn:ietf:params:scim:schemas:core:2.0:User";
pub const GROUP: &str = "urn:ietf:params:scim:schemas:core:2.0:Group";
pub const SCHEMA: &str = "urn:ietf:params:scim:schemas:core:2.0:Schema";
pub const RESOURCE_TYPE: &str = "urn:ietf:params:scim:schemas:core:2.0:ResourceType";
pub const ENTERPRISE_USER: &str = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User";

pub const LIST_RESPONSE: &str = "urn:ietf:params:scim:api:messages:2.0:ListResponse";
pub const PATCH_OP: &str = "urn:ietf:params:scim:api:messages:2.0:PatchOp";
pub const SEARCH_REQUEST: &str = "urn:ietf:params:scim:api:messages:2.0:SearchRequest";
