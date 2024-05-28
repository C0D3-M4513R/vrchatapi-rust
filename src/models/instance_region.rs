/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

/// InstanceRegion : Instance region

/// Instance region
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceRegion {
    #[serde(rename = "us")]
    Us,
    #[serde(rename = "use")]
    Use,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "jp")]
    Jp,

}

impl ToString for InstanceRegion {
    fn to_string(&self) -> String {
        match self {
            Self::Us => String::from("us"),
            Self::Use => String::from("use"),
            Self::Eu => String::from("eu"),
            Self::Jp => String::from("jp"),
        }
    }
}

impl Default for InstanceRegion {
    fn default() -> InstanceRegion {
        Self::Us
    }
}




