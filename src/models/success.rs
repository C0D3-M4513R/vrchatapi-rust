/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Success {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<Box<models::Response>>,
}

impl Success {
    pub fn new() -> Success {
        Success { success: None }
    }
}
