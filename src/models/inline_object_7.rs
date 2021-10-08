/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject7 {
    #[serde(rename = "assetUrl", skip_serializing_if = "Option::is_none")]
    pub asset_url: Option<String>,
    #[serde(rename = "assetVersion", skip_serializing_if = "Option::is_none")]
    pub asset_version: Option<String>,
    #[serde(rename = "authorId", skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(rename = "authorName", skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "releaseStatus", skip_serializing_if = "Option::is_none")]
    pub release_status: Option<crate::models::ReleaseStatus>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "unityPackageUrl", skip_serializing_if = "Option::is_none")]
    pub unity_package_url: Option<String>,
    #[serde(rename = "unityVersion", skip_serializing_if = "Option::is_none")]
    pub unity_version: Option<String>,
}

impl InlineObject7 {
    pub fn new() -> InlineObject7 {
        InlineObject7 {
            asset_url: None,
            asset_version: None,
            author_id: None,
            author_name: None,
            capacity: None,
            description: None,
            id: None,
            image_url: None,
            name: None,
            platform: None,
            release_status: None,
            tags: None,
            unity_package_url: None,
            unity_version: None,
        }
    }
}


