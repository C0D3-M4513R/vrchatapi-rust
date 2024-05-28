use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
macro_rules! impl_into_str {
    ($type_from:ty, $type_to:ty) => {
impl From<&$type_from> for $type_to{
    fn from(value: &$type_from) -> Self {
        value.get_tag()
    }
}
impl From<$type_from> for $type_to{
    fn from(value: $type_from) -> Self {
        value.get_tag()
    }
}
    };
}
#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "String")]
pub enum Tags{
    Language(String),
    Show(ShowTags),
    //Avatar Tags
    Content(ContentTags),
    Admin(AdminTags),
    //Avatar Tags?
    Author(AuthorTags),
    //Favorite Group Start
    Group(String),
    World(String),
    Avatars(String),
    //Favorite Group end
    System(SystemTags),
}

impl Tags{
    pub fn get_tag(&self) -> String {
        match self {
            Tags::Language(language) => format!("language_{language}"),
            Tags::Show(show) => format!("show_{}", show.get_tag()),
            Tags::Content(content) => format!("content_{}", content.get_tag()),
            Tags::Admin(admin) => format!("admin_{}", admin.get_tag()),
            Tags::Author(author) => format!("author_{}", author.get_tag()),
            Tags::Group(group) => format!("group_{group}"),
            Tags::World(world) => format!("worlds{world}"),
            Tags::Avatars(avatar) => format!("avatars{avatar}"),
            Tags::System(system) => format!("system_{}", system.get_tag()),
        }
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Language(language) => write!(f, "Language: {language}"),
            Self::Show(show) => write!(f, "Show: {show}"),
            Self::Content(content) => write!(f, "Content Warning: {content}"),
            Self::Admin(admin) => write!(f, "Admin: {admin}"),
            Self::Author(author) => write!(f, "Author: {author}"),
            Self::Group(i) => write!(f, "User: {i}"),
            Self::World(i) => write!(f, "World: {i}"),
            Self::Avatars(i) => write!(f, "Avatars: {i}"),
            Self::System(system) => write!(f, "System: {system}"),
        }
    }
}

impl<'a> core::convert::TryFrom<&'a str> for Tags{
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some(lang) = value.strip_prefix("language_"){
            Ok(Self::Language(lang.to_string()))
        } else if let Some(system) = value.strip_prefix("system_"){
            Ok(Self::System(SystemTags::try_from(system)?))
        } else if let Some(show) = value.strip_prefix("show_"){
            Ok(Self::Show(ShowTags::try_from(show)?))
        } else if let Some(show) = value.strip_prefix("content_"){
            Ok(Self::Content(ContentTags::try_from(show)?))
        }else if let Some(show) = value.strip_prefix("admin_"){
            Ok(Self::Admin(AdminTags::try_from(show)?))
        }else if let Some(show) = value.strip_prefix("author_"){
            Ok(Self::Author(AuthorTags::try_from(show)?))
        }else if let Some(group) = value.strip_prefix("group_"){
            Ok(Self::Group(group.to_string()))
        }else if let Some(world) = value.strip_prefix("worlds"){
            Ok(Self::World(world.to_string()))
        }else if let Some(avatar) = value.strip_prefix("avatars"){
            Ok(Self::Avatars(avatar.to_string()))
        } else {
            log::error!("NEW UNKNOWN TAG: {}", value);
            Err(value)
        }
    }
}
impl_into_str!(Tags, String);

#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "&str")]
pub enum ShowTags{
    SocialRank,
}

impl ShowTags{
    pub const fn get_tag(&self) -> &'static str {
        match self {
            Self::SocialRank => "social_rank",
        }
    }
}

impl Display for ShowTags{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::SocialRank => write!(f, "Show Social Rank"),
        }
    }
}
impl<'a> core::convert::TryFrom<&'a str> for ShowTags{
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "social_rank" => Ok(Self::SocialRank),
            other => {
                log::error!("NEW UNKNOWN TAG: show_{}", value);
                Err(other)
            }
        }
    }
}
impl_into_str!(ShowTags, &'static str);

#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "&str")]
pub enum ContentTags{
    Sex,
    Adult,
    Violence,
    Gore,
    Horror,
    Other,
}

impl ContentTags{
    pub const fn get_tag(&self) -> &'static str {
        match self {
            Self::Sex => "sex",
            Self::Adult => "adult",
            Self::Violence => "violence",
            Self::Gore => "gore",
            Self::Horror => "horror",
            Self::Other => "other",
        }
    }
}

impl Display for ContentTags{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Sex => write!(f, "Sexually Suggestive"),
            Self::Adult => write!(f, "Adult Language and Themes"),
            Self::Violence => write!(f, "Graphic Violence"),
            Self::Gore => write!(f, "Excessive Gore"),
            Self::Horror => write!(f, "Extreme Horror"),
            Self::Other => write!(f, "Other Content Warning"),
        }
    }
}
impl<'a> core::convert::TryFrom<&'a str> for ContentTags{
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "sex" => Ok(Self::Sex),
            "adult" => Ok(Self::Adult),
            "violence" => Ok(Self::Violence),
            "gore" => Ok(Self::Gore),
            "horror" => Ok(Self::Horror),
            "other" => Ok(Self::Other),
            other => {
                log::error!("NEW UNKNOWN TAG: content_{}", value);
                Err(other)
            }
        }
    }
}
impl_into_str!(ContentTags, &'static str);

#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "String")]
pub enum AdminTags{
    ContentReviewed,
    QuestFallbackExtended,
    Featured(FeaturedTags),
}

impl AdminTags{
    pub fn get_tag(&self) -> String {
        match self {
            Self::ContentReviewed => String::from("content_reviewed"),
            Self::QuestFallbackExtended => String::from("quest_fallback_extended"),
            Self::Featured(feature) => format!("featured_{}", feature.get_tag()),
        }
    }
}

impl<'a> core::convert::TryFrom<&'a str> for AdminTags {
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some(featured) = value.strip_prefix("featured_"){
            Ok(Self::Featured(FeaturedTags::try_from(featured)?))
        } else{
            match value {
                "content_reviewed" => Ok(Self::ContentReviewed),
                "quest_fallback_extended" => Ok(Self::QuestFallbackExtended),
                other => {
                    log::error!("NEW UNKNOWN TAG: admin_{other}");
                    Err(other)
                }
            }
        }
    }
}
impl_into_str!(AdminTags, String);

impl Display for AdminTags{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::ContentReviewed => write!(f, "Content Reviewed"),
            Self::QuestFallbackExtended => write!(f, "Extended Quest Fallback"),
            Self::Featured(featured) => write!(f, "Featured: {featured}"),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "&str")]
pub enum FeaturedTags {
    LegacyContent,
    Quest,
}

impl FeaturedTags{
    pub const fn get_tag(&self) -> &'static str {
        match self {
            Self::LegacyContent => "legacy",
            Self::Quest => "quest",
        }
    }
}

impl<'a> core::convert::TryFrom<&'a str> for FeaturedTags{
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "legacy" => Ok(Self::LegacyContent),
            "quest" => Ok(Self::Quest),
            other => {
                log::error!("NEW UNKNOWN TAG: admin_featured_{value}");
                Err(other)
            }
        }
    }
}
impl_into_str!(FeaturedTags, &'static str);

impl Display for FeaturedTags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LegacyContent => write!(f, "Legacy Content"),
            Self::Quest => write!(f, "Quest"),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "&str")]
pub enum AuthorTags {
    QuestFallback,
}
impl AuthorTags{
    pub const fn get_tag(&self) -> &'static str {
        match self {
            Self::QuestFallback => "quest_fallback",
        }
    }
}
impl<'a> core::convert::TryFrom<&'a str> for AuthorTags {
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "quest_fallback" => Ok(Self::QuestFallback),
            other => {
                log::error!("NEW UNKNOWN TAG: author_{value}");
                Err(other)
            }
        }
    }
}
impl_into_str!(AuthorTags, &'static str);

impl Display for AuthorTags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QuestFallback => write!(f, "Quest Fallback"),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "String")]
pub enum SystemTags {
    Access(SystemAccess),
    Trust(SystemTrust),
    Jam(String),
    EarlyAdopter,
    Supporter,
    NoCaptcha,
}
impl SystemTags{
    pub fn get_tag(&self) -> String {
        match self {
            Self::Access(access) => format!("{}_access", access.get_tag()),
            Self::Trust(trust) => format!("trust_{}", trust.get_tag()),
            Self::Jam(id) => format!("jam{id}"),
            Self::EarlyAdopter => "early_adopter".to_string(),
            Self::Supporter => "supporter".to_string(),
            Self::NoCaptcha => "no_captcha".to_string(),
        }
    }
}
impl Display for SystemTags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Access(access) => write!(f, "{access}"),
            Self::Trust(trust) => write!(f, "Trust: {trust}"),
            Self::Jam(jam) => write!(f, "Jam: {jam}"),
            Self::EarlyAdopter => write!(f, "EarlyAdopter"),
            Self::Supporter => write!(f, "Supporter"),
            Self::NoCaptcha => write!(f, "NoCaptcha"),
        }
    }
}

impl<'a> core::convert::TryFrom<&'a str> for SystemTags{
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some(access) = value.strip_suffix("_access") {
            Ok(Self::Access(SystemAccess::try_from(access)?))
        } else if let Some(trust) = value.strip_prefix("trust_") {
            Ok(Self::Trust(SystemTrust::try_from(trust)?))
        }else if let Some(jam) = value.strip_prefix("jam") {
            Ok(Self::Jam(jam.to_string()))
        } else {
            match value{
                "early_adopter" => Ok(Self::EarlyAdopter),
                "supporter" => Ok(Self::Supporter),
                "no_captcha" => Ok(Self::NoCaptcha),
                value=> {
                    log::error!("NEW UNKNOWN TAG: system_{}", value);
                    Err(value)
                },
            }
        }
    }
}
impl_into_str!(SystemTags, String);

#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "&str")]
pub enum SystemTrust {
    Veteran,
    Trusted,
    Known,
    Basic,
    //Visitors have no Trust tag
}
impl SystemTrust{
    pub const fn get_tag(&self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Known => "known",
            Self::Trusted => "trusted",
            Self::Veteran => "veteran",
        }
    }
}
impl Display for SystemTrust {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "New User"),
            Self::Known => write!(f, "User"),
            Self::Trusted => write!(f, "Known User"),
            Self::Veteran => write!(f, "Trusted User"),
        }
    }
}

impl<'a> core::convert::TryFrom<&'a str> for SystemTrust {
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "basic" => Ok(Self::Basic),
            "known" => Ok(Self::Known),
            "trusted" => Ok(Self::Trusted),
            "veteran" => Ok(Self::Veteran),
            other => {
                log::error!("NEW UNKNOWN TAG: system_trust_{}", value);
                Err(other)
            }
        }
    }
}
impl_into_str!(SystemTrust, &'static str);

#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "&str")]
pub enum SystemAccess {
    Avatar,
    World,
    Feedback,
}
impl SystemAccess{
    pub const fn get_tag(&self) -> &'static str {
        match self {
            Self::Avatar => "avatar",
            Self::World => "world",
            Self::Feedback => "feedback",
        }
    }
}
impl Display for SystemAccess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Avatar => write!(f, "Avatar Access"),
            Self::World => write!(f, "World Access"),
            Self::Feedback => write!(f, "Feedback Access"),
        }
    }
}

impl<'a> core::convert::TryFrom<&'a str> for SystemAccess{
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "avatar" => Ok(Self::Avatar),
            "world" => Ok(Self::World),
            "feedback" => Ok(Self::Feedback),
            other => {
                log::error!("NEW UNKNOWN TAG: system_{}_access", value);
                Err(other)
            }
        }
    }
}
impl_into_str!(SystemAccess, &'static str);