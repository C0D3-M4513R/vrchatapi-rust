/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "allowAvatarCopying")]
    pub allow_avatar_copying: bool,
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "bioLinks")]
    pub bio_links: Vec<String>,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarImageUrl")]
    pub current_avatar_image_url: String,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarThumbnailImageUrl")]
    pub current_avatar_thumbnail_image_url: String,
    #[serde(rename = "date_joined")]
    pub date_joined: String,
    #[serde(rename = "developerType")]
    pub developer_type: crate::models::DeveloperType,
    /// A users visual display name. This is what shows up in-game, and can different from their `username`. Changing display name is restricted to a cooldown period.
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "friendKey")]
    pub friend_key: String,
    #[serde(rename = "friendRequestStatus", skip_serializing_if = "Option::is_none")]
    pub friend_request_status: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id")]
    pub id: String,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Either their `friendKey`, or empty string if you are not friends. Unknown usage.
    #[serde(rename = "isFriend")]
    pub is_friend: bool,
    /// Either a date-time or empty string.
    #[serde(rename = "last_activity")]
    pub last_activity: String,
    /// Either a date-time or empty string.
    #[serde(rename = "last_login")]
    pub last_login: String,
    /// This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "last_platform")]
    pub last_platform: String,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "profilePicOverride")]
    pub profile_pic_override: String,
    #[serde(rename = "state")]
    pub state: crate::models::UserState,
    #[serde(rename = "status")]
    pub status: crate::models::UserStatus,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "travelingToInstance", skip_serializing_if = "Option::is_none")]
    pub traveling_to_instance: Option<String>,
    #[serde(rename = "travelingToLocation", skip_serializing_if = "Option::is_none")]
    pub traveling_to_location: Option<String>,
    #[serde(rename = "travelingToWorld", skip_serializing_if = "Option::is_none")]
    pub traveling_to_world: Option<String>,
    #[serde(rename = "userIcon")]
    pub user_icon: String,
    /// -| A users unique name, used during login. This is different from `displayName` which is what shows up in-game. A users `username` can never be changed.' **DEPRECATED:** VRChat API no longer return usernames of other users. [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429).
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId", skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
}

impl User {
    pub fn new(allow_avatar_copying: bool, bio: String, bio_links: Vec<String>, current_avatar_image_url: String, current_avatar_thumbnail_image_url: String, date_joined: String, developer_type: crate::models::DeveloperType, display_name: String, friend_key: String, id: String, is_friend: bool, last_activity: String, last_login: String, last_platform: String, profile_pic_override: String, state: crate::models::UserState, status: crate::models::UserStatus, status_description: String, tags: Vec<String>, user_icon: String) -> User {
        User {
            allow_avatar_copying,
            bio,
            bio_links,
            current_avatar_image_url,
            current_avatar_thumbnail_image_url,
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


