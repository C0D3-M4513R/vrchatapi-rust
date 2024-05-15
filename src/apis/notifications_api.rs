/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`accept_friend_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AcceptFriendRequestError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`clear_notifications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClearNotificationsError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_notification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNotificationError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_notifications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNotificationsError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mark_notification_as_read`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkNotificationAsReadError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Accept a friend request by notification `frq_` ID. Friend requests can be found using the NotificationsAPI `getNotifications` by filtering of type `friendRequest`.
pub fn accept_friend_request(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, notification_id: &str) -> Result<crate::models::Success, Error<AcceptFriendRequestError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/user/notifications/{notificationId}/accept", local_var_configuration.base_path, notificationId=crate::apis::urlencode(notification_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AcceptFriendRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Clear **all** notifications.
pub fn clear_notifications(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, ) -> Result<crate::models::Success, Error<ClearNotificationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/user/notifications/clear", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ClearNotificationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a notification.
pub fn delete_notification(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, notification_id: &str) -> Result<crate::models::Notification, Error<DeleteNotificationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/user/notifications/{notificationId}/hide", local_var_configuration.base_path, notificationId=crate::apis::urlencode(notification_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteNotificationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve all of the current user's notifications.
pub fn get_notifications(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, r#type: Option<&str>, sent: Option<bool>, hidden: Option<bool>, after: Option<&str>, n: Option<i32>, offset: Option<i32>) -> Result<Vec<crate::models::Notification>, Error<GetNotificationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/user/notifications", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = r#type {
        local_var_req_builder = local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sent {
        local_var_req_builder = local_var_req_builder.query(&[("sent", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = hidden {
        local_var_req_builder = local_var_req_builder.query(&[("hidden", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = after {
        local_var_req_builder = local_var_req_builder.query(&[("after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = n {
        local_var_req_builder = local_var_req_builder.query(&[("n", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetNotificationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Mark a notification as seen.
pub fn mark_notification_as_read(configuration: &configuration::Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>, notification_id: &str) -> Result<crate::models::Notification, Error<MarkNotificationAsReadError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/auth/user/notifications/{notificationId}/see", local_var_configuration.base_path, notificationId=crate::apis::urlencode(notification_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarkNotificationAsReadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

