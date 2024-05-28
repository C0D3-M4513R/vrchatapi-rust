/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

/// LimitedWorld : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LimitedWorld {
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorName")]
    pub author_name: String,
    #[serde(rename = "capacity")]
    pub capacity: i32,
    #[serde(rename = "recommendedCapacity", skip_serializing_if = "Option::is_none")]
    pub recommended_capacity: Option<i32>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "favorites")]
    pub favorites: i32,
    #[serde(rename = "visits", skip_serializing_if = "Option::is_none")]
    pub visits: Option<i32>,
    #[serde(rename = "heat")]
    pub heat: i32,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "labsPublicationDate")]
    pub labs_publication_date: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "occupants")]
    pub occupants: i32,
    #[serde(rename = "organization")]
    pub organization: String,
    #[serde(rename = "popularity")]
    pub popularity: i32,
    #[serde(rename = "previewYoutubeId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub preview_youtube_id: Option<Option<String>>,
    #[serde(rename = "publicationDate")]
    pub publication_date: String,
    #[serde(rename = "releaseStatus")]
    pub release_status: crate::models::ReleaseStatus,
    ///  
    #[serde(rename = "tags")]
    pub tags: Vec<crate::models::tags::Tags>,
    #[serde(rename = "thumbnailImageUrl")]
    pub thumbnail_image_url: String,
    ///  
    #[serde(rename = "unityPackages")]
    pub unity_packages: Vec<crate::models::LimitedUnityPackage>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "udonProducts", skip_serializing_if = "Option::is_none")]
    pub udon_products: Option<Vec<String>>,
}

impl LimitedWorld {
    /// 
    pub fn new(author_id: String, author_name: String, capacity: i32, created_at: String, favorites: i32, heat: i32, id: String, image_url: String, labs_publication_date: String, name: String, occupants: i32, organization: String, popularity: i32, publication_date: String, release_status: crate::models::ReleaseStatus, tags: Vec<crate::models::tags::Tags>, thumbnail_image_url: String, unity_packages: Vec<crate::models::LimitedUnityPackage>, updated_at: String) -> LimitedWorld {
        LimitedWorld {
            author_id,
            author_name,
            capacity,
            recommended_capacity: None,
            created_at,
            favorites,
            visits: None,
            heat,
            id,
            image_url,
            labs_publication_date,
            name,
            occupants,
            organization,
            popularity,
            preview_youtube_id: None,
            publication_date,
            release_status,
            tags,
            thumbnail_image_url,
            unity_packages,
            updated_at,
            udon_products: None,
        }
    }
}


