/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldPublishStatus {
    #[serde(rename = "canPublish")]
    pub can_publish: bool,
}

impl WorldPublishStatus {
    pub fn new(can_publish: bool) -> WorldPublishStatus {
        WorldPublishStatus {
            can_publish,
        }
    }
}


