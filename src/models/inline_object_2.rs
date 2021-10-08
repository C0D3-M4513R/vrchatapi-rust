/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject2 {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "birthday", skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(rename = "acceptedTOSVersion", skip_serializing_if = "Option::is_none")]
    pub accepted_tos_version: Option<f32>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::UserStatus>,
    #[serde(rename = "statusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "bio", skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(rename = "bioLinks", skip_serializing_if = "Option::is_none")]
    pub bio_links: Option<Vec<String>>,
    /// MUST be a valid VRChat /file/ url.
    #[serde(rename = "userIcon", skip_serializing_if = "Option::is_none")]
    pub user_icon: Option<String>,
}

impl InlineObject2 {
    pub fn new() -> InlineObject2 {
        InlineObject2 {
            email: None,
            birthday: None,
            accepted_tos_version: None,
            tags: None,
            status: None,
            status_description: None,
            bio: None,
            bio_links: None,
            user_icon: None,
        }
    }
}


