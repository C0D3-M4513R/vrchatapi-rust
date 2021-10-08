/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FavoriteType {
    #[serde(rename = "world")]
    World,
    #[serde(rename = "friend")]
    Friend,
    #[serde(rename = "avatar")]
    Avatar,

}

impl ToString for FavoriteType {
    fn to_string(&self) -> String {
        match self {
            Self::World => String::from("world"),
            Self::Friend => String::from("friend"),
            Self::Avatar => String::from("avatar"),
        }
    }
}




