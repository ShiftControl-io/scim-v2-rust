//! Canonical SCIM schema URNs.
//!
//! RFC 7643 and RFC 7644 define these URNs. A server or a client writes the
//! URN into the `schemas` attribute. The URN declares the type of the
//! resource or the message.

pub const USER: &str = "urn:ietf:params:scim:schemas:core:2.0:User";
pub const GROUP: &str = "urn:ietf:params:scim:schemas:core:2.0:Group";
pub const SCHEMA: &str = "urn:ietf:params:scim:schemas:core:2.0:Schema";
pub const RESOURCE_TYPE: &str = "urn:ietf:params:scim:schemas:core:2.0:ResourceType";
pub const ENTERPRISE_USER: &str = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User";
pub const SERVICE_PROVIDER_CONFIG: &str =
    "urn:ietf:params:scim:schemas:core:2.0:ServiceProviderConfig";

pub const LIST_RESPONSE: &str = "urn:ietf:params:scim:api:messages:2.0:ListResponse";
pub const PATCH_OP: &str = "urn:ietf:params:scim:api:messages:2.0:PatchOp";
pub const SEARCH_REQUEST: &str = "urn:ietf:params:scim:api:messages:2.0:SearchRequest";
/// RFC 7644 §3.12 error response.
pub const ERROR: &str = "urn:ietf:params:scim:api:messages:2.0:Error";
pub const BULK_REQUEST: &str = "urn:ietf:params:scim:api:messages:2.0:BulkRequest";
pub const BULK_RESPONSE: &str = "urn:ietf:params:scim:api:messages:2.0:BulkResponse";
