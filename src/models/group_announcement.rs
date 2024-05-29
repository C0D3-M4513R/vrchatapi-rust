/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAnnouncement {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<std::sync::Arc<str>>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<std::sync::Arc<str>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "authorId", skip_serializing_if = "Option::is_none")]
    pub author_id: Option<std::sync::Arc<str>>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<std::sync::Arc<str>>>,
    #[serde(rename = "text", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub text: Option<Option<std::sync::Arc<str>>>,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<std::sync::Arc<str>>,
    #[serde(rename = "imageUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<Option<std::sync::Arc<str>>>,
    #[serde(rename = "createdAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<std::sync::Arc<str>>>,
    #[serde(rename = "updatedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Option<std::sync::Arc<str>>>,
}

impl GroupAnnouncement {
    pub fn new() -> GroupAnnouncement {
        GroupAnnouncement {
            id: None,
            group_id: None,
            author_id: None,
            title: None,
            text: None,
            image_id: None,
            image_url: None,
            created_at: None,
            updated_at: None,
        }
    }
}


