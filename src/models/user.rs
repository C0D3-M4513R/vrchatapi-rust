/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "allowAvatarCopying")]
    pub allow_avatar_copying: bool,
    ///  
    #[serde(rename = "badges", skip_serializing_if = "Option::is_none")]
    pub badges: Option<Vec<crate::models::Badge>>,
    #[serde(rename = "bio")]
    pub bio: std::sync::Arc<str>,
    #[serde(rename = "bioLinks")]
    pub bio_links: Vec<std::sync::Arc<str>>,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarImageUrl")]
    pub current_avatar_image_url: std::sync::Arc<str>,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarThumbnailImageUrl")]
    pub current_avatar_thumbnail_image_url: std::sync::Arc<str>,
    #[serde(rename = "currentAvatarTags")]
    pub current_avatar_tags: Vec<crate::models::tags::Tags>,
    #[serde(rename = "date_joined")]
    pub date_joined: std::sync::Arc<str>,
    #[serde(rename = "developerType")]
    pub developer_type: crate::models::DeveloperType,
    /// A users visual display name. This is what shows up in-game, and can different from their `username`. Changing display name is restricted to a cooldown period.
    #[serde(rename = "displayName")]
    pub display_name: std::sync::Arc<str>,
    #[serde(rename = "friendKey")]
    pub friend_key: std::sync::Arc<str>,
    #[serde(rename = "friendRequestStatus", skip_serializing_if = "Option::is_none")]
    pub friend_request_status: Option<std::sync::Arc<str>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id")]
    pub id: std::sync::Arc<str>,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<std::sync::Arc<str>>,
    /// Either their `friendKey`, or empty string if you are not friends. Unknown usage.
    #[serde(rename = "isFriend")]
    pub is_friend: bool,
    /// Either a date-time or empty string.
    #[serde(rename = "last_activity")]
    pub last_activity: std::sync::Arc<str>,
    /// Either a date-time or empty string.
    #[serde(rename = "last_login")]
    pub last_login: std::sync::Arc<str>,
    /// This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "last_platform")]
    pub last_platform: std::sync::Arc<str>,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<std::sync::Arc<str>>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<std::sync::Arc<str>>,
    #[serde(rename = "profilePicOverride")]
    pub profile_pic_override: std::sync::Arc<str>,
    #[serde(rename = "pronouns")]
    pub pronouns: std::sync::Arc<str>,
    #[serde(rename = "state")]
    pub state: crate::models::UserState,
    #[serde(rename = "status")]
    pub status: crate::models::UserStatus,
    #[serde(rename = "statusDescription")]
    pub status_description: std::sync::Arc<str>,
    ///  
    #[serde(rename = "tags")]
    pub tags: Vec<crate::models::tags::Tags>,
    #[serde(rename = "travelingToInstance", skip_serializing_if = "Option::is_none")]
    pub traveling_to_instance: Option<std::sync::Arc<str>>,
    #[serde(rename = "travelingToLocation", skip_serializing_if = "Option::is_none")]
    pub traveling_to_location: Option<std::sync::Arc<str>>,
    #[serde(rename = "travelingToWorld", skip_serializing_if = "Option::is_none")]
    pub traveling_to_world: Option<std::sync::Arc<str>>,
    #[serde(rename = "userIcon")]
    pub user_icon: std::sync::Arc<str>,
    /// -| A users unique name, used during login. This is different from `displayName` which is what shows up in-game. A users `username` can never be changed.' **DEPRECATED:** VRChat API no longer return usernames of other users. [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429).
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<std::sync::Arc<str>>,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId", skip_serializing_if = "Option::is_none")]
    pub world_id: Option<std::sync::Arc<str>>,
}

impl User {
    pub fn new(allow_avatar_copying: bool, bio: std::sync::Arc<str>, bio_links: Vec<std::sync::Arc<str>>, current_avatar_image_url: std::sync::Arc<str>, current_avatar_thumbnail_image_url: std::sync::Arc<str>, current_avatar_tags: Vec<crate::models::tags::Tags>, date_joined: std::sync::Arc<str>, developer_type: crate::models::DeveloperType, display_name: std::sync::Arc<str>, friend_key: std::sync::Arc<str>, id: std::sync::Arc<str>, is_friend: bool, last_activity: std::sync::Arc<str>, last_login: std::sync::Arc<str>, last_platform: std::sync::Arc<str>, profile_pic_override: std::sync::Arc<str>, pronouns: std::sync::Arc<str>, state: crate::models::UserState, status: crate::models::UserStatus, status_description: std::sync::Arc<str>, tags: Vec<crate::models::tags::Tags>, user_icon: std::sync::Arc<str>) -> User {
        User {
            allow_avatar_copying,
            badges: None,
            bio,
            bio_links,
            current_avatar_image_url,
            current_avatar_thumbnail_image_url,
            current_avatar_tags,
            date_joined,
            developer_type,
            display_name,
            friend_key,
            friend_request_status: None,
            id,
            instance_id: None,
            is_friend,
            last_activity,
            last_login,
            last_platform,
            location: None,
            note: None,
            profile_pic_override,
            pronouns,
            state,
            status,
            status_description,
            tags,
            traveling_to_instance: None,
            traveling_to_location: None,
            traveling_to_world: None,
            user_icon,
            username: None,
            world_id: None,
        }
    }
}


