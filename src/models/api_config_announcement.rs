/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ApiConfigAnnouncement : Public Announcement
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigAnnouncement {
    /// Announcement name
    #[serde(rename = "name")]
    pub name: String,
    /// Announcement text
    #[serde(rename = "text")]
    pub text: String,
}

impl ApiConfigAnnouncement {
    /// Public Announcement
    pub fn new(name: String, text: String) -> ApiConfigAnnouncement {
        ApiConfigAnnouncement { name, text }
    }
}
