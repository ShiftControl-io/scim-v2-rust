use crate::models::enterprise_user::EnterpriseUser;
use crate::models::scim_schema::Meta;
use crate::utils::serde::deserialize_optional_lenient_bool;
use crate::utils::validation::{
    Context, Validate, ValidationError, at_most_one_primary, require_schema_urn,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User<T = String> {
    // urn:ietf:params:scim:schemas:core:2.0:User
    pub schemas: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub user_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Lenient for the same reason every `primary` is: providers stringify
    /// booleans, and a User that cannot be parsed because `active` arrived as
    /// `"True"` is a worse outcome than accepting the spelling.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub emails: Vec<Email>,
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub addresses: Vec<Address>,
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub phone_numbers: Vec<PhoneNumber>,
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub ims: Vec<Im>,
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub photos: Vec<Photo>,
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub groups: Vec<Group>,
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub entitlements: Vec<Entitlement>,
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub roles: Vec<Role>,
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub x509_certificates: Vec<X509Certificate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(
        rename = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User",
        skip_serializing_if = "Option::is_none"
    )]
    pub enterprise_user: Option<EnterpriseUser>,
}

impl<T> Default for User<T> {
    fn default() -> Self {
        User {
            schemas: vec![crate::schema_urns::USER.to_string()],
            user_name: "".to_string(),
            id: None,
            external_id: None,
            name: None,
            display_name: None,
            nick_name: None,
            profile_url: None,
            title: None,
            user_type: None,
            preferred_language: None,
            locale: None,
            timezone: None,
            active: None,
            password: None,
            emails: Vec::new(),
            addresses: Vec::new(),
            phone_numbers: Vec::new(),
            ims: Vec::new(),
            photos: Vec::new(),
            groups: Vec::new(),
            entitlements: Vec::new(),
            roles: Vec::new(),
            x509_certificates: Vec::new(),
            meta: None,
            enterprise_user: None,
        }
    }
}

/// The components of the user's name.
///
/// Unassigned sub-attributes are omitted from serialized output rather than
/// emitted as `null`; RFC 7643 §2.5 treats the two as equivalent in resource
/// state. Note the consequence for `PUT`: per RFC 7644 §3.5.1 an omitted
/// `readWrite` attribute is "not asserted by the client" and the server MAY
/// keep the existing value or apply a default — it is *not* a deterministic
/// clear. Callers that need to clear a sub-attribute on `PUT` should use a
/// `PATCH` operation or hand-build a `serde_json::Value` carrying an explicit
/// `null`.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honorific_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honorific_suffix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Email {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// RFC 7643 §2.4 defines `value`, `display` and `primary` as common
    /// sub-attributes of *every* multi-valued attribute, and gives "the
    /// preferred mailing address" as its example of `primary`. §4.1.2's
    /// listing for `addresses` omits all three, which is why they were absent
    /// before 1.0 — but providers do send `primary` here (see
    /// `test_data/provider_samples/jumpcloud_create_user.json`), and it was
    /// silently dropped on the way back out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct PhoneNumber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Im {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Photo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Entitlement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Role {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        default, // required as `deserialize_with` does not set default when field is missing
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct X509Certificate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub primary: Option<bool>,
}

impl<T> Validate for User<T> {
    /// RFC 7643 §4.1 marks `userName` REQUIRED; §3 marks `schemas` REQUIRED on
    /// every resource. Every other User attribute is optional, so those two are
    /// the whole check.
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, crate::schema_urns::USER)?;
        if self.user_name.is_empty() {
            return Err(ValidationError::missing_required("userName"));
        }
        // RFC 7643 §2.4: at most one `primary: true` per multi-valued attribute.
        at_most_one_primary(&self.emails, |e| e.primary, "emails")?;
        at_most_one_primary(&self.phone_numbers, |p| p.primary, "phoneNumbers")?;
        at_most_one_primary(&self.ims, |i| i.primary, "ims")?;
        at_most_one_primary(&self.photos, |p| p.primary, "photos")?;
        at_most_one_primary(&self.addresses, |a| a.primary, "addresses")?;
        at_most_one_primary(&self.entitlements, |e| e.primary, "entitlements")?;
        at_most_one_primary(&self.roles, |r| r.primary, "roles")?;
        at_most_one_primary(&self.x509_certificates, |c| c.primary, "x509Certificates")?;
        Ok(())
    }

    /// RFC 7643 §3.1: a server's representation "MUST include a non-empty
    /// `id`", and `id` "MUST NOT be specified by the client" on create. §4.1:
    /// `password` is `returned: never`, so a response carrying one is
    /// non-conformant however it got there. A replace request tolerates `id`
    /// because RFC 7644 §3.5.1 has the server *ignore* readOnly attributes,
    /// and its own PUT example carries one.
    fn validate_context(&self, ctx: Context) -> Result<(), ValidationError> {
        match ctx {
            Context::CreateRequest => {
                if self.id.is_some() {
                    return Err(ValidationError::invalid_value(
                        "id",
                        "MUST NOT be specified by the client on create (RFC 7643 §3.1)",
                    ));
                }
            }
            Context::Response => {
                // `id` is generic over `T`, so only presence can be checked here;
                // emptiness of a `String` id is a caller concern.
                if self.id.is_none() {
                    return Err(ValidationError::missing_required("id"));
                }
                if self.password.is_some() {
                    return Err(ValidationError::invalid_value(
                        "password",
                        "is returned: never and MUST NOT appear in a response (RFC 7643 §4.1)",
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
