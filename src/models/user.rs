use crate::models::enterprise_user::EnterpriseUser;
use crate::models::scim_schema::Meta;
use crate::multi::Multi;
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
    /// This field is lenient for the same reason every `primary` field is
    /// lenient. A provider may stringify a boolean value, for example
    /// sending `active` as `"True"`. This crate accepts that spelling
    /// instead of rejecting the whole `User`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default = "Multi::absent", skip_serializing_if = "Multi::is_absent")]
    pub emails: Multi<Email>,
    #[serde(default = "Multi::absent", skip_serializing_if = "Multi::is_absent")]
    pub addresses: Multi<Address>,
    #[serde(default = "Multi::absent", skip_serializing_if = "Multi::is_absent")]
    pub phone_numbers: Multi<PhoneNumber>,
    #[serde(default = "Multi::absent", skip_serializing_if = "Multi::is_absent")]
    pub ims: Multi<Im>,
    #[serde(default = "Multi::absent", skip_serializing_if = "Multi::is_absent")]
    pub photos: Multi<Photo>,
    #[serde(default = "Multi::absent", skip_serializing_if = "Multi::is_absent")]
    pub groups: Multi<Group>,
    #[serde(default = "Multi::absent", skip_serializing_if = "Multi::is_absent")]
    pub entitlements: Multi<Entitlement>,
    #[serde(default = "Multi::absent", skip_serializing_if = "Multi::is_absent")]
    pub roles: Multi<Role>,
    #[serde(default = "Multi::absent", skip_serializing_if = "Multi::is_absent")]
    pub x509_certificates: Multi<X509Certificate>,
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
            emails: Multi::absent(),
            addresses: Multi::absent(),
            phone_numbers: Multi::absent(),
            ims: Multi::absent(),
            photos: Multi::absent(),
            groups: Multi::absent(),
            entitlements: Multi::absent(),
            roles: Multi::absent(),
            x509_certificates: Multi::absent(),
            meta: None,
            enterprise_user: None,
        }
    }
}

/// The components of the user's name.
///
/// This struct omits an unassigned sub-attribute from serialized output
/// instead of writing `null`. RFC 7643 §2.5 treats the two forms as
/// equivalent in resource state.
///
/// This omission has a consequence for `PUT`. Per RFC 7644 §3.5.1, an
/// omitted `readWrite` attribute is "not asserted by the client". The
/// server MAY assume that the existing values are to be cleared. The
/// server MAY assign a default value instead. An omission is
/// therefore *not* a deterministic clear. A caller that needs to clear a
/// sub-attribute on `PUT` should use a `PATCH` operation instead. A
/// caller can also hand-build a `serde_json::Value` that carries an
/// explicit `null`.
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
    /// sub-attributes of *every* multi-valued attribute. RFC 7643 §2.4
    /// gives "the preferred mailing address" as its example of `primary`.
    /// RFC 7643 §4.1.2's listing for `addresses` omits all three
    /// sub-attributes. This omission is why this struct lacked all three
    /// before version 1.0. Providers do send `primary` here, though (see
    /// `test_data/provider_samples/jumpcloud_create_user.json`). Before
    /// this fix, this struct silently dropped that value on
    /// serialization.
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

/// This impl requires `T: Display`. This requirement lets a check test a
/// response's `id` against RFC 7643 §3.1's "non-empty" requirement, not
/// only against presence. Every identifier type in practice (`String`,
/// `uuid::Uuid`, the integers) prints itself.
impl<T: std::fmt::Display> Validate for User<T> {
    /// RFC 7643 §4.1 marks `userName` REQUIRED. RFC 7643 §3 marks
    /// `schemas` REQUIRED on every resource. RFC 7643 §3 says `schemas`
    /// names "the namespaces of the SCIM schemas that define the
    /// attributes present". An enterprise extension body therefore needs
    /// its URN declared. The other direction stays lenient. RFC 7643
    /// §3.3 calls an extension URI an indication that its attributes
    /// *may* exist. A declared URN with no body is therefore fine. Every
    /// other User attribute is optional.
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, crate::schema_urns::USER)?;
        if self.user_name.is_empty() {
            return Err(ValidationError::missing_required("userName"));
        }
        if self.enterprise_user.is_some()
            && !self
                .schemas
                .iter()
                .any(|s| s == crate::schema_urns::ENTERPRISE_USER)
        {
            return Err(ValidationError::invalid_value(
                "schemas",
                format!(
                    "enterprise extension attributes are present but {} is not declared (RFC 7643 §3)",
                    crate::schema_urns::ENTERPRISE_USER
                ),
            ));
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

    /// RFC 7643 §3.1 says a server's representation "MUST include a
    /// non-empty `id`". RFC 7643 §3.1 also says `id` "MUST NOT be
    /// specified by the client" on create. RFC 7643 §4.1 marks `password`
    /// as `returned: never`. A response that carries a password is
    /// therefore non-conformant, however the password got there. A
    /// replace request tolerates `id`, though. RFC 7644 §3.5.1 has the
    /// server *ignore* readOnly attributes. RFC 7644's own PUT example
    /// also carries an `id`.
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
                // §3.1: "MUST include a non-empty id value". Emptiness is judged
                // through `Display`, which is what lets `T` stay generic.
                if self.id.as_ref().is_none_or(|id| id.to_string().is_empty()) {
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
