/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupInstance {
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "world")]
    pub world: Box<crate::models::World>,
    #[serde(rename = "memberCount")]
    pub member_count: i32,
}

impl GroupInstance {
    pub fn new(instance_id: String, location: String, world: crate::models::World, member_count: i32) -> GroupInstance {
        GroupInstance {
            instance_id,
            location,
            world: Box::new(world),
            member_count,
        }
    }
}


