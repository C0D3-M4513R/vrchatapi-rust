/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

#[allow(unused_imports)] use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`add_favorite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddFavoriteError {
    Status400(crate::models::Error),
    Status403(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`clear_favorite_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClearFavoriteGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_favorite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFavoriteError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_favorite_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFavoriteGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_favorite_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFavoriteGroupsError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_favorites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFavoritesError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_favorite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveFavoriteError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_favorite_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFavoriteGroupError {
    UnknownValue(serde_json::Value),
}


/// Add a new favorite.  Friend groups are named `group_0` through `group_3`. Avatar and World groups are named `avatars1` to `avatars4` and `worlds1` to `worlds4`.  You cannot add people whom you are not friends with to your friends list. Destroying a friendship removes the person as favorite on both sides.
pub async fn add_favorite(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, add_favorite_request: Option<crate::models::AddFavoriteRequest>) -> Result<crate::models::Favorite, Error<AddFavoriteError>> {
    let local_var_uri_str = format!("{}/favorites", configuration.base_path);
    #[allow(unused_mut)] let mut local_var_req_builder = configuration.client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    crate::request(configuration, local_var_req_builder, Some(add_favorite_request)).await
}

/// Clear ALL contents of a specific favorite group.
pub async fn clear_favorite_group(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, favorite_group_type: &str, favorite_group_name: &str, user_id: &str) -> Result<crate::models::Success, Error<ClearFavoriteGroupError>> {
    let local_var_uri_str = format!("{}/favorite/group/{favoriteGroupType}/{favoriteGroupName}/{userId}", configuration.base_path, favoriteGroupType=crate::apis::urlencode(favorite_group_type), favoriteGroupName=crate::apis::urlencode(favorite_group_name), userId=crate::apis::urlencode(user_id));
    #[allow(unused_mut)] let mut local_var_req_builder = configuration.client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());
    crate::request(configuration, local_var_req_builder, None::<()>).await
}

/// Return information about a specific Favorite.
pub async fn get_favorite(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, favorite_id: &str) -> Result<crate::models::Favorite, Error<GetFavoriteError>> {
    let local_var_uri_str = format!("{}/favorites/{favoriteId}", configuration.base_path, favoriteId=crate::apis::urlencode(favorite_id));
    #[allow(unused_mut)] let mut local_var_req_builder = configuration.client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    crate::request(configuration, local_var_req_builder, None::<()>).await
}

/// Fetch information about a specific favorite group.
pub async fn get_favorite_group(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, favorite_group_type: &str, favorite_group_name: &str, user_id: &str) -> Result<crate::models::FavoriteGroup, Error<GetFavoriteGroupError>> {
    let local_var_uri_str = format!("{}/favorite/group/{favoriteGroupType}/{favoriteGroupName}/{userId}", configuration.base_path, favoriteGroupType=crate::apis::urlencode(favorite_group_type), favoriteGroupName=crate::apis::urlencode(favorite_group_name), userId=crate::apis::urlencode(user_id));
    #[allow(unused_mut)] let mut local_var_req_builder = configuration.client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    crate::request(configuration, local_var_req_builder, None::<()>).await
}

/// Return a list of favorite groups owned by a user. Returns the same information as `getFavoriteGroups`.
pub async fn get_favorite_groups(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, n: Option<i32>, offset: Option<i32>, owner_id: Option<&str>) -> Result<Vec<crate::models::FavoriteGroup>, Error<GetFavoriteGroupsError>> {
    let local_var_uri_str = format!("{}/favorite/groups", configuration.base_path);
    #[allow(unused_mut)] let mut local_var_req_builder = configuration.client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = n {
        local_var_req_builder = local_var_req_builder.query(&[("n", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = owner_id {
        local_var_req_builder = local_var_req_builder.query(&[("ownerId", &local_var_str.to_string())]);
    }
    crate::request(configuration, local_var_req_builder, None::<()>).await
}

/// Returns a list of favorites.
pub async fn get_favorites(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, n: Option<i32>, offset: Option<i32>, r#type: Option<&str>, tag: Option<&str>) -> Result<Vec<crate::models::Favorite>, Error<GetFavoritesError>> {
    let local_var_uri_str = format!("{}/favorites", configuration.base_path);
    #[allow(unused_mut)] let mut local_var_req_builder = configuration.client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = n {
        local_var_req_builder = local_var_req_builder.query(&[("n", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = r#type {
        local_var_req_builder = local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tag {
        local_var_req_builder = local_var_req_builder.query(&[("tag", &local_var_str.to_string())]);
    }
    crate::request(configuration, local_var_req_builder, None::<()>).await
}

/// Remove a favorite from your favorites list.
pub async fn remove_favorite(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, favorite_id: &str) -> Result<crate::models::Success, Error<RemoveFavoriteError>> {
    let local_var_uri_str = format!("{}/favorites/{favoriteId}", configuration.base_path, favoriteId=crate::apis::urlencode(favorite_id));
    #[allow(unused_mut)] let mut local_var_req_builder = configuration.client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());
    crate::request(configuration, local_var_req_builder, None::<()>).await
}

/// Update information about a specific favorite group.
pub async fn update_favorite_group(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, favorite_group_type: &str, favorite_group_name: &str, user_id: &str, update_favorite_group_request: Option<crate::models::UpdateFavoriteGroupRequest>) -> Result<(), Error<UpdateFavoriteGroupError>> {
    let local_var_uri_str = format!("{}/favorite/group/{favoriteGroupType}/{favoriteGroupName}/{userId}", configuration.base_path, favoriteGroupType=crate::apis::urlencode(favorite_group_type), favoriteGroupName=crate::apis::urlencode(favorite_group_name), userId=crate::apis::urlencode(user_id));
    #[allow(unused_mut)] let mut local_var_req_builder = configuration.client.request(reqwest::Method::PUT, local_var_uri_str.as_str());
    crate::request_ok(configuration, local_var_req_builder, Some(update_favorite_group_request)).await
}

