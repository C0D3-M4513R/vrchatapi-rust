use std::fmt::{Display, Formatter};
use std::sync::Arc;
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
    ///User Tag
    ///Instance Tag
    Language(LanguageTags),
    ///User Tag
    Show(ShowTags),
    ///Avatar Tag
    ///World Tag
    Content(ContentTags),
    ///User Tag
    Admin(AdminTags),
    ///World Tag
    Author(AuthorTags),
    ///World Debugging has been enabled by author, allowing to see state of triggers
    ///World Tag
    DebugAllowed,
    //Favorite Group Start
    Group(String),
    World(String),
    Avatars(String),
    //Favorite Group end
    ///User Tag
    System(SystemTags),
    ///other (mostly fun or undocumented) tags, such as: "system_neuralink_beta", "ඞ", "'; DROP DATABASE tags; --", "we've been trying to reach you about your car's extended warranty" and others.
    Other(Arc<str>)
}
impl Tags{
    pub fn get_tag(&self) -> String {
        match self {
            Tags::Language(language) => format!("language_{}", language.get_tag()),
            Tags::Show(show) => format!("show_{}", show.get_tag()),
            Tags::Content(content) => format!("content_{}", content.get_tag()),
            Tags::Admin(admin) => format!("admin_{}", admin.get_tag()),
            Tags::Author(author) => format!("author_{}", author.get_tag()),
            Tags::DebugAllowed => "debug_allowed".to_string(),
            Tags::Group(group) => format!("group_{group}"),
            Tags::World(world) => format!("worlds{world}"),
            Tags::Avatars(avatar) => format!("avatars{avatar}"),
            Tags::System(system) => format!("system_{}", system.get_tag()),
            Tags::Other(other) => other.to_string()
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
            Self::DebugAllowed => write!(f, "World Debugging Allowed"),
            Self::World(i) => write!(f, "World: {i}"),
            Self::Avatars(i) => write!(f, "Avatars: {i}"),
            Self::System(system) => write!(f, "System: {system}"),
            Tags::Other(other) => write!(f, "Other: {other}"),
        }
    }
}

impl<'a> core::convert::From<&'a str> for Tags{
    fn from(value: &'a str) -> Self {
        if let Some(lang) = value.strip_prefix("language_"){
            Self::Language(LanguageTags::from(lang))
        } else if let Some(system) = value.strip_prefix("system_"){
            match core::convert::TryFrom::try_from(system) {
                Ok(tag) => Self::System(tag),
                Err(_) => Self::Other(Arc::from(value)),
            }
        } else if let Some(show) = value.strip_prefix("show_"){
            match core::convert::TryFrom::try_from(show) {
                Ok(tag) => Self::Show(tag),
                Err(_) => Self::Other(Arc::from(value)),
            }
        } else if let Some(content) = value.strip_prefix("content_"){
            match core::convert::TryFrom::try_from(content) {
                Ok(tag) => Self::Content(tag),
                Err(_) => Self::Other(Arc::from(value)),
            }
        }else if let Some(admin) = value.strip_prefix("admin_"){
            Self::Admin(AdminTags::from(admin))
        }else if let Some(author) = value.strip_prefix("author_"){
            Self::Author(AuthorTags::from(author))
        }else if let Some(group) = value.strip_prefix("group_"){
            Self::Group(group.to_string())
        }else if let Some(world) = value.strip_prefix("worlds"){
            Self::World(world.to_string())
        }else if let Some(avatar) = value.strip_prefix("avatars"){
            Self::Avatars(avatar.to_string())
        } else {
            match value{
                "debug_allowed" => Self::DebugAllowed,
                other => {
                    log::error!("New or Fun Tag: {other}");
                    Self::Other(Arc::from(other))
                }
            }
        }
    }
}
impl_into_str!(Tags, String);

#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(from = "&str", into = "String")]
pub enum LanguageTags{
    ///English / English
    English,
    ///한국어 / Korean
    Korean,
    ///Русский / Russian
    Russian,
    ///Español / Spanish
    Spanish,
    ///Português / Portuguese
    Portuguese,
    ///中文 / Chinese
    Chinese,
    ///Deutsch / German
    German,
    ///日本語 / Japanese
    Japanese,
    ///Français / French
    French,
    ///Svenska / Swedish
    Swedish,
    ///Nederlands / Dutch
    Dutch,
    ///Polski / Polish
    Polish,
    ///Dansk / Danish
    Danish,
    ///Norsk / Norwegian
    Norwegian,
    ///Italiano / Italian
    Italian,
    ///ภาษาไทย / Thai
    Thai,
    ///Suomi / Finnish
    Finnish,
    ///Magyar / Hungarian
    Hungarian,
    ///Čeština / Czech
    Czech,
    ///Türkçe / Turkish
    Turkish,
    ///العربية / Arabic
    Arabic,
    ///Română / Romanian
    Romanian,
    ///Tiếng Việt / Vietnamese
    Vietnamese,
    ///American Sign Language
    AmericanSignLanguage,
    ///British Sign Language
    BritishSignLanguage,
    ///Dutch Sign Language
    DutchSignLanguage,
    ///French Sign Language
    FrenchSignLanguage,
    ///Korean Sign Language
    KoreanSignLanguage,
    ///Other Undocumented Languages
    Other(Arc<str>),
}

impl LanguageTags{
    pub fn get_tag(&self) -> String {
        match self {
            Self::English => "eng".to_string(),
            Self::Korean => "kor".to_string(),
            Self::Russian => "rus".to_string(),
            Self::Spanish => "spa".to_string(),
            Self::Portuguese => "por".to_string(),
            Self::Chinese => "zho".to_string(),
            Self::German => "deu".to_string(),
            Self::Japanese => "jpn".to_string(),
            Self::French => "fra".to_string(),
            Self::Swedish => "swe".to_string(),
            Self::Dutch => "nld".to_string(),
            Self::Polish => "pol".to_string(),
            Self::Danish => "dan".to_string(),
            Self::Norwegian => "nor".to_string(),
            Self::Italian => "ita".to_string(),
            Self::Thai => "tha".to_string(),
            Self::Finnish => "fin".to_string(),
            Self::Hungarian => "hun".to_string(),
            Self::Czech => "ces".to_string(),
            Self::Turkish => "tur".to_string(),
            Self::Arabic => "ara".to_string(),
            Self::Romanian => "ron".to_string(),
            Self::Vietnamese => "vie".to_string(),
            Self::AmericanSignLanguage => "ase".to_string(),
            Self::BritishSignLanguage => "bfi".to_string(),
            Self::DutchSignLanguage => "dse".to_string(),
            Self::FrenchSignLanguage => "fsl".to_string(),
            Self::KoreanSignLanguage => "kvk".to_string(),
            Self::Other(other) => other.to_string(),
        }
    }
}

impl Display for LanguageTags{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::English => write!(f, "English / English"),
            Self::Korean => write!(f, "한국어 / Korean"),
            Self::Russian => write!(f, "Русский / Russian"),
            Self::Spanish => write!(f, "Español / Spanish"),
            Self::Portuguese => write!(f, "Português / Portuguese"),
            Self::Chinese => write!(f, "中文 / Chinese"),
            Self::German => write!(f, "Deutsch / German"),
            Self::Japanese => write!(f, "日本語 / Japanese"),
            Self::French => write!(f, "Français / French"),
            Self::Swedish => write!(f, "Svenska / Swedish"),
            Self::Dutch => write!(f, "Nederlands / Dutch"),
            Self::Polish => write!(f, "Polski / Polish"),
            Self::Danish => write!(f, "Dansk / Danish"),
            Self::Norwegian => write!(f, "Norsk / Norwegian"),
            Self::Italian => write!(f, "Italiano / Italian"),
            Self::Thai => write!(f, "ภาษาไทย / Thai"),
            Self::Finnish => write!(f, "Suomi / Finnish"),
            Self::Hungarian => write!(f, "Magyar / Hungarian"),
            Self::Czech => write!(f, "Čeština / Czech"),
            Self::Turkish => write!(f, "Türkçe / Turkish"),
            Self::Arabic => write!(f, "العربية / Arabic"),
            Self::Romanian => write!(f, "Română / Romanian"),
            Self::Vietnamese => write!(f, "Tiếng Việt / Vietnamese"),
            Self::AmericanSignLanguage => write!(f, "American Sign Language"),
            Self::BritishSignLanguage => write!(f, "British Sign Language"),
            Self::DutchSignLanguage => write!(f, "Dutch Sign Language"),
            Self::FrenchSignLanguage => write!(f, "French Sign Language"),
            Self::KoreanSignLanguage => write!(f, "Korean Sign Language"),
            Self::Other(other) => write!(f, "Other: {other}"),
        }
    }
}
impl<'a> core::convert::From<&'a str> for LanguageTags{
    fn from(value: &'a str) -> Self {
        match value {
            "eng" => Self::English,
            "kor" => Self::Korean,
            "rus" => Self::Russian,
            "spa" => Self::Spanish,
            "por" => Self::Portuguese,
            "zho" => Self::Chinese,
            "deu" => Self::German,
            "jpn" => Self::Japanese,
            "fra" => Self::French,
            "swe" => Self::Swedish,
            "nld" => Self::Dutch,
            "pol" => Self::Polish,
            "dan" => Self::Danish,
            "nor" => Self::Norwegian,
            "ita" => Self::Italian,
            "tha" => Self::Thai,
            "fin" => Self::Finnish,
            "hun" => Self::Hungarian,
            "ces" => Self::Czech,
            "tur" => Self::Turkish,
            "ara" => Self::Arabic,
            "ron" => Self::Romanian,
            "vie" => Self::Vietnamese,
            "ase" => Self::AmericanSignLanguage,
            "bfi" => Self::BritishSignLanguage,
            "dse" => Self::DutchSignLanguage,
            "fsl" => Self::FrenchSignLanguage,
            "kvk" => Self::KoreanSignLanguage,
            other => Self::Other(Arc::from(other)),
        }
    }
}
impl_into_str!(LanguageTags, String);
#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "&str")]
pub enum ShowTags{
    ///Toggle whether to show the user’s real social rank
    /// (Deprecated: This is now a registry key and sent over Photon)
    ///User Tag
    #[deprecated]
    SocialRank,
    ///Toggle whether to show the Red Staff nameplate
    ///User Tag
    ModTag,
}

impl ShowTags{
    pub const fn get_tag(&self) -> &'static str {
        match self {
            #[allow(deprecated)]
            Self::SocialRank => "social_rank",
            Self::ModTag => "mod_tag",
        }
    }
}

impl Display for ShowTags{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            #[allow(deprecated)]
            Self::SocialRank => write!(f, "Show Social Rank"),
            Self::ModTag => write!(f, "Show Mod Tag"),
        }
    }
}
impl<'a> core::convert::TryFrom<&'a str> for ShowTags{
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            #[allow(deprecated)]
            "social_rank" => Ok(Self::SocialRank),
            "mod_tag" => Ok(Self::ModTag),
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
    ///World Tag
    Sex,
    Adult,
    ///World Tag
    Violence,
    ///World Tag
    Gore,
    Horror,
    ///World Tag
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
#[serde(from = "&str", into = "String")]
pub enum AdminTags{
    ///User Tag
    Access(AdminAccess),
    ///User can give out licenses to other users
    ///User Tag
    CanGrantLicenses,
    ///User’s tags have been locked and cannot be edited by the user
    ///User Tag
    LockTags,
    ///User’s Trust rank has been locked and can no longer be changed automatically
    ///User Tag
    LockLevel,
    ///User is part of the VRChat Staff team
    ///User Tag
    Moderator,
    ///Replaces the users profile picture with the VRChat logo
    ///User Tag
    OfficialThumbnail,
    ///World was manually been selected by staff as avatar world (Deprecated)
    /// World Tag
    #[deprecated]
    AvatarWorld,
    ///World has been manually selected by staff to appear in the “Featured” world category
    ///World Tag
    Featured,
    ///World has been manually approved by staff
    ///World Tag
    Approved,
    ///World has been manually selected by staff to appear in “Spotlight” row
    ///World Tag
    CommunitySpotlight,
    ///World will always be hidden from search
    ///World Tag
    Hidden,
    ///World will be hidden from “Active” row
    ///World Tag
    HideActive,
    ///World will be hidden from “Recently Updated Worlds” row when updated
    ///World Tag
    HideNew,
    ///World will be hidden from “Popular Worlds” row
    ///World Tag
    HidePopular,
    ///Hides both online members and total number of members in the group. Used for the VRCHAT.0000 staff group.
    ///Group Tag
    HideMemberCount,
    ContentReviewed,
    QuestFallbackExtended,
    FeaturedCategory(FeaturedTags),
    ///Worlds can also have a range of different event-specific tags such as admin_vket2021 for the Vket row, admin_muzzfesst for the MUZZFEST event, or admin_halloween_2019 for the 2019 Halloween row. These are not listed because it would be impossible to accurately keep such list up-to-date, and each tag is of little use outside of that specific event.
    ///World Tag
    Other(Arc<str>)
}

impl AdminTags{
    pub fn get_tag(&self) -> String {
        match self {
            Self::ContentReviewed => String::from("content_reviewed"),
            Self::QuestFallbackExtended => String::from("quest_fallback_extended"),
            Self::FeaturedCategory(feature) => format!("featured_{}", feature.get_tag()),
            Self::Access(access) => format!("{}_access", access.get_tag()),
            Self::CanGrantLicenses => "can_grant_licenses".to_string(),
            Self::LockTags => "lock_tags".to_string(),
            Self::LockLevel => "lock_level".to_string(),
            Self::Moderator => "moderator".to_string(),
            Self::OfficialThumbnail => "official_thumbnail".to_string(),
            #[allow(deprecated)]
            Self::AvatarWorld => "avatar_world".to_string(),
            Self::Featured => "featured".to_string(),
            Self::Approved => "approved".to_string(),
            Self::CommunitySpotlight => "community_spotlight".to_string(),
            Self::Hidden => "hidden".to_string(),
            Self::HideActive => "hide_active".to_string(),
            Self::HideNew => "hide_new".to_string(),
            Self::HidePopular => "hide_popular".to_string(),
            Self::HideMemberCount => "hide_member_count".to_string(),
            Self::Other(other) => other.to_string(),
        }
    }
}

impl<'a> core::convert::From<&'a str> for AdminTags {
    fn from(value: &'a str) -> Self {
        if let Some(featured) = value.strip_prefix("featured_"){
            match core::convert::TryFrom::try_from(featured) {
                Ok(tag) => Self::FeaturedCategory(tag),
                Err(_) => Self::Other(Arc::from(value)),
            }
        } else if let Some(access) = value.strip_suffix("_access"){
            match core::convert::TryFrom::try_from(access) {
                Ok(tag) => Self::Access(tag),
                Err(_) => Self::Other(Arc::from(value)),
            }
        } else{
            match value {
                "content_reviewed" => Self::ContentReviewed,
                "quest_fallback_extended" => Self::QuestFallbackExtended,
                "can_grant_licenses" => Self::CanGrantLicenses,
                "lock_tags" => Self::LockTags,
                "lock_level" => Self::LockLevel,
                "moderator" => Self::Moderator,
                "official_thumbnail" => Self::OfficialThumbnail,
                #[allow(deprecated)]
                "avatar_world" => Self::AvatarWorld,
                "featured" => Self::Featured,
                "approved" => Self::Approved,
                "community_spotlight" => Self::CommunitySpotlight,
                "hidden" => Self::Hidden,
                "hide_active" => Self::HideActive,
                "hide_new" => Self::HideNew,
                "hide_popular" => Self::HidePopular,
                "hide_member_count" => Self::HideMemberCount,
                other => Self::Other(Arc::from(other))
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
            Self::FeaturedCategory(featured) => write!(f, "Featured: {featured}"),
            Self::Access(access) => write!(f, "Access: {access}"),
            Self::CanGrantLicenses => write!(f, "Can Grant Licenses"),
            Self::LockTags => write!(f, "Locked Tags"),
            Self::LockLevel => write!(f, "Locked Level"),
            Self::Moderator => write!(f, "Moderator"),
            Self::OfficialThumbnail => write!(f, "Official Thumbnail"),
            #[allow(deprecated)]
            Self::AvatarWorld => write!(f, "Avatar World"),
            Self::Featured => write!(f, "Manually Featured"),
            Self::Approved => write!(f, "Manually Approved"),
            Self::CommunitySpotlight => write!(f, "Manually Community Spotlight"),
            Self::Hidden => write!(f, "Hidden from Search"),
            Self::HideActive => write!(f, "Hidden from Active"),
            Self::HideNew => write!(f, "Hidden from Recently Updated Worlds"),
            Self::HidePopular => write!(f, "Hidden from Popular Worlds"),
            Self::HideMemberCount => write!(f, "Hidden Group Member Count"),
            Self::Other(other) => write!(f, "Other: {other}"),
        }
    }
}
#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "&str")]
pub enum AdminAccess{
    ///User can upload Avatars without needing the neccesary trust rank
    Avatar,
    ///User can access Canny without needing the neccesary trust rank
    Canny,
    ///User can upload user-made scripts (Deprecated)
    #[deprecated]
    Scripting,
    ///User can upload Worlds without needing the neccesary trust rank
    WorldAccess,
}

impl AdminAccess{
    pub const fn get_tag(&self) -> &'static str {
        match self {
            Self::Avatar => "avatar",
            Self::Canny => "canny",
            #[allow(deprecated)]
            Self::Scripting => "scripting",
            Self::WorldAccess => "world",
        }
    }
}

impl<'a> core::convert::TryFrom<&'a str> for AdminAccess {
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "avatar" => Ok(Self::Avatar),
            "canny" => Ok(Self::Canny),
            #[allow(deprecated)]
            "scripting" => Ok(Self::Scripting),
            "world" => Ok(Self::WorldAccess),
            other => Err(other),
        }
    }
}
impl_into_str!(AdminAccess, &'static str);

impl Display for AdminAccess{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Avatar => write!(f, "Avatar Upload Access without Trust Rank"),
            Self::Canny => write!(f, "Canny Access without Trust Rank"),
            #[allow(deprecated)]
            Self::Scripting => write!(f, "Can upload custom User Scripts"),
            Self::WorldAccess => write!(f, "World Upload Access without Trust Rank"),
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
#[serde(from = "&str", into = "String")]
pub enum AuthorTags {
    QuestFallback,
    ///World will show up in “Avatar Worlds” row
    ///World Tag
    TagAvatar,
    ///World will show up in the “Games” world row
    ///World Tag
    TagGame,
    ///World Tag
    Other(Arc<str>)
}


impl AuthorTags{
    pub fn get_tag(&self) -> String {
        match self {
            Self::QuestFallback => "quest_fallback".to_string(),
            Self::TagAvatar => "tag_avatar".to_string(),
            Self::TagGame => "tag_game".to_string(),
            Self::Other(other) => other.to_string(),
        }
    }
}
impl<'a> core::convert::From<&'a str> for AuthorTags {
    fn from(value: &'a str) -> Self {
        match value {
            "quest_fallback" => Self::QuestFallback,
            "tag_avatar" => Self::TagAvatar,
            "tag_game" => Self::TagGame,
            other => Self::Other(Arc::from(other))
        }
    }
}
impl_into_str!(AuthorTags, String);

impl Display for AuthorTags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QuestFallback => write!(f, "Quest Fallback"),
            Self::TagAvatar => write!(f, "Avatar World"),
            Self::TagGame => write!(f, "Game World"),
            Self::Other(other) => write!(f, "Custom Tag: {other}"),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "String")]
pub enum SystemTags {
    ///User Tag
    Access(SystemAccess),
    ///User Tag
    Trust(SystemTrust),
    Jam(String),
    ///User can upload user-made scripts (Deprecated)
    ///User Tag
    #[deprecated]
    ScriptingAccess,
    ///User bought VRC+ in the early period of it’s launch, around December 2020
    ///User Tag
    EarlyAdopter,
    ///User has been reported multiple times and is (probably) a troll
    ///User Tag
    ProbableTroll,
    ///User is a confirmed troll
    ///User Tag
    Troll,
    ///User has an active VRC+ subscription
    ///User Tag
    Supporter,
    ///User is an Experienced player and was active during the Summer of 2018
    /// (Tag removed in 2022-05-05)
    ///User Tag
    Legend,
    ///World has been automatically approved through the Community Labs
    ///World Tag
    Approved,
    ///World was recently created
    ///World Tag
    CreatedRecently,
    ///World has been submitted to Community Labs
    /// World Tag
    Labs,
    ///World has been recently updated and will show up in “Updated Recently” worlds row if also system approved
    /// World Tag
    UpdatedRecently,
    NoCaptcha,
}
impl SystemTags{
    pub fn get_tag(&self) -> String {
        match self {
            Self::Access(access) => format!("{}_access", access.get_tag()),
            Self::Trust(trust) => format!("trust_{}", trust.get_tag()),
            Self::Jam(id) => format!("jam{id}"),
            #[allow(deprecated)]
            Self::ScriptingAccess => "scripting_access".to_string(),
            Self::ProbableTroll => "probable_troll".to_string(),
            Self::Troll => "troll".to_string(),
            Self::EarlyAdopter => "early_adopter".to_string(),
            Self::Supporter => "supporter".to_string(),
            Self::Legend => "legend".to_string(),
            Self::NoCaptcha => "no_captcha".to_string(),
            Self::Approved => "approved".to_string(),
            Self::CreatedRecently => "created_recently".to_string(),
            Self::Labs => "labs".to_string(),
            Self::UpdatedRecently => "updated_recently".to_string(),
        }
    }
}
impl Display for SystemTags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Access(access) => write!(f, "{access}"),
            Self::Trust(trust) => write!(f, "Trust: {trust}"),
            Self::Jam(jam) => write!(f, "Jam: {jam}"),
            #[allow(deprecated)]
            Self::ScriptingAccess => write!(f, "Scripting Access"),
            Self::ProbableTroll => write!(f, "Probable Troll"),
            Self::Troll => write!(f, "Troll"),
            Self::EarlyAdopter => write!(f, "EarlyAdopter"),
            Self::Supporter => write!(f, "Supporter"),
            Self::Legend => write!(f, "Legend"),
            Self::NoCaptcha => write!(f, "NoCaptcha"),
            Self::Approved => write!(f, "Approved"),
            Self::CreatedRecently => write!(f, "Created Recently"),
            Self::Labs => write!(f, "Labs"),
            Self::UpdatedRecently => write!(f, "Updated Recently")
        }
    }
}

impl<'a> core::convert::TryFrom<&'a str> for SystemTags{
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some(access) = value.strip_suffix("_access") {
            match core::convert::TryFrom::try_from(access) {
                Ok(tag) => Ok(Self::Access(tag)),
                Err(_) => Err(value),
            }
        } else if let Some(trust) = value.strip_prefix("trust_") {
            match core::convert::TryFrom::try_from(trust) {
                Ok(tag) => Ok(Self::Trust(tag)),
                Err(_) => Err(value),
            }
        }else if let Some(jam) = value.strip_prefix("jam") {
            Ok(Self::Jam(jam.to_string()))
        } else {
            match value{
                #[allow(deprecated)]
                "scripting_access" => Ok(Self::ScriptingAccess),
                "probable_troll" => Ok(Self::ProbableTroll),
                "troll" => Ok(Self::Troll),
                "early_adopter" => Ok(Self::EarlyAdopter),
                "supporter" => Ok(Self::Supporter),
                "legend" => Ok(Self::Legend),
                "no_captcha" => Ok(Self::NoCaptcha),
                "approved" => Ok(Self::Approved),
                "created_recently" => Ok(Self::CreatedRecently),
                "labs" => Ok(Self::Labs),
                "updated_recently" => Ok(Self::UpdatedRecently),
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
    ///User is “Veteran User” (gold color) Trust rank
    /// (Role was removed in Sep 2018. Tag removed in 2022-05-05)
    #[deprecated]
    Ledgend,
    ///Unknown
    /// (Tag removed in 2022-05-05)
    #[deprecated]
    Advanced,
    ///Unknown
    /// (Tag removed in 2022-05-05)
    #[deprecated]
    Intermediate,
    ///User is “Trusted User” (purple) Trust rank
    Veteran,
    ///User is “Known User” (orange) Trust rank
    Trusted,
    ///	User is “User” (green) Trust rank
    Known,
    ///User is “New User” (blue) Trust rank
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
            #[allow(deprecated)]
            Self::Intermediate => "intermediate",
            #[allow(deprecated)]
            Self::Advanced => "advanced",
            #[allow(deprecated)]
            Self::Ledgend => "legend",
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
            #[allow(deprecated)]
            Self::Intermediate => write!(f, "Intermediate User"),
            #[allow(deprecated)]
            Self::Advanced => write!(f,"Advanced User"),
            #[allow(deprecated)]
            Self::Ledgend => write!(f, "Legend User"),
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
    ///User can upload and publish Avatars
    Avatar,
    ///User can upload and publish Worlds
    World,
    ///User can send Feedback
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


#[cfg(test)]
mod test{
    use std::sync::Arc;

    #[test]
    #[allow(deprecated)]
    fn test(){
        for (tag, expected) in [
            //User Tags
            ("admin_avatar_access", super::Tags::Admin(super::AdminTags::Access(super::AdminAccess::Avatar))),
            ("admin_canny_access", super::Tags::Admin(super::AdminTags::Access(super::AdminAccess::Canny))),
            ("admin_scripting_access", super::Tags::Admin(super::AdminTags::Access(super::AdminAccess::Scripting))),
            ("admin_world_access", super::Tags::Admin(super::AdminTags::Access(super::AdminAccess::WorldAccess))),
            ("admin_can_grant_licenses", super::Tags::Admin(super::AdminTags::CanGrantLicenses)),
            ("admin_lock_tags", super::Tags::Admin(super::AdminTags::LockTags)),
            ("admin_lock_level", super::Tags::Admin(super::AdminTags::LockLevel)),
            ("admin_moderator", super::Tags::Admin(super::AdminTags::Moderator)),
            ("admin_official_thumbnail", super::Tags::Admin(super::AdminTags::OfficialThumbnail)),
            ("system_scripting_access", super::Tags::System(super::SystemTags::ScriptingAccess)),
            ("show_social_rank", super::Tags::Show(super::ShowTags::SocialRank)),
            ("show_mod_tag", super::Tags::Show(super::ShowTags::ModTag)),
            ("system_early_adopter", super::Tags::System(super::SystemTags::EarlyAdopter)),
            ("system_avatar_access", super::Tags::System(super::SystemTags::Access(super::SystemAccess::Avatar))),
            ("system_feedback_access", super::Tags::System(super::SystemTags::Access(super::SystemAccess::Feedback))),
            ("system_world_access", super::Tags::System(super::SystemTags::Access(super::SystemAccess::World))),
            ("system_probable_troll", super::Tags::System(super::SystemTags::ProbableTroll)),
            ("system_troll", super::Tags::System(super::SystemTags::Troll)),
            ("system_supporter", super::Tags::System(super::SystemTags::Supporter)),
            ("system_legend", super::Tags::System(super::SystemTags::Legend)),
            ("system_trust_basic", super::Tags::System(super::SystemTags::Trust(super::SystemTrust::Basic))),
            //World Tags
            ("author_tag_", super::Tags::Author(super::AuthorTags::Other(Arc::from("tag_")))),
            ("author_tag_*", super::Tags::Author(super::AuthorTags::Other(Arc::from("tag_*")))),
            ("author_tag_avatar", super::Tags::Author(super::AuthorTags::TagAvatar)),
            ("author_tag_game", super::Tags::Author(super::AuthorTags::TagGame)),
            ("admin_featured", super::Tags::Admin(super::AdminTags::Featured)),
            ("admin_approved", super::Tags::Admin(super::AdminTags::Approved)),
            ("admin_avatar_world", super::Tags::Admin(super::AdminTags::AvatarWorld)),
            ("admin_community_spotlight", super::Tags::Admin(super::AdminTags::CommunitySpotlight)),
            ("admin_hide_popular", super::Tags::Admin(super::AdminTags::HidePopular)),
            ("debug_allowed", super::Tags::DebugAllowed),
            ("system_approved", super::Tags::System(super::SystemTags::Approved)),
            ("system_created_recently", super::Tags::System(super::SystemTags::CreatedRecently)),
            ("system_labs", super::Tags::System(super::SystemTags::Labs)),
            ("system_updated_recently", super::Tags::System(super::SystemTags::UpdatedRecently)),
            ("content_sex", super::Tags::Content(super::ContentTags::Sex)),
            ("content_adult", super::Tags::Content(super::ContentTags::Adult)),
            ("content_violence", super::Tags::Content(super::ContentTags::Violence)),
            ("content_gore", super::Tags::Content(super::ContentTags::Gore)),
            ("content_horror", super::Tags::Content(super::ContentTags::Horror)),
            ("content_other", super::Tags::Content(super::ContentTags::Other)),
            //Languages
            ("language_eng", super::Tags::Language(super::LanguageTags::English)),
            ("language_kor", super::Tags::Language(super::LanguageTags::Korean)),
            ("language_rus", super::Tags::Language(super::LanguageTags::Russian)),
            ("language_spa", super::Tags::Language(super::LanguageTags::Spanish)),
            ("language_por", super::Tags::Language(super::LanguageTags::Portuguese)),
            ("language_zho", super::Tags::Language(super::LanguageTags::Chinese)),
            ("language_deu", super::Tags::Language(super::LanguageTags::German)),
            ("language_jpn", super::Tags::Language(super::LanguageTags::Japanese)),
            ("language_fra", super::Tags::Language(super::LanguageTags::French)),
            ("language_swe", super::Tags::Language(super::LanguageTags::Swedish)),
            ("language_nld", super::Tags::Language(super::LanguageTags::Dutch)),
            ("language_pol", super::Tags::Language(super::LanguageTags::Polish)),
            ("language_dan", super::Tags::Language(super::LanguageTags::Danish)),
            ("language_nor", super::Tags::Language(super::LanguageTags::Norwegian)),
            ("language_ita", super::Tags::Language(super::LanguageTags::Italian)),
            ("language_tha", super::Tags::Language(super::LanguageTags::Thai)),
            ("language_fin", super::Tags::Language(super::LanguageTags::Finnish)),
            ("language_hun", super::Tags::Language(super::LanguageTags::Hungarian)),
            ("language_ces", super::Tags::Language(super::LanguageTags::Czech)),
            ("language_tur", super::Tags::Language(super::LanguageTags::Turkish)),
            ("language_ara", super::Tags::Language(super::LanguageTags::Arabic)),
            ("language_ron", super::Tags::Language(super::LanguageTags::Romanian)),
            ("language_vie", super::Tags::Language(super::LanguageTags::Vietnamese)),
            ("language_ase", super::Tags::Language(super::LanguageTags::AmericanSignLanguage)),
            ("language_bfi", super::Tags::Language(super::LanguageTags::BritishSignLanguage)),
            ("language_dse", super::Tags::Language(super::LanguageTags::DutchSignLanguage)),
            ("language_fsl", super::Tags::Language(super::LanguageTags::FrenchSignLanguage)),
            ("language_kvk", super::Tags::Language(super::LanguageTags::KoreanSignLanguage)),
            ("language_other", super::Tags::Language(super::LanguageTags::Other(Arc::from("other")))),
            //Group Tags
            ("admin_hide_member_count", super::Tags::Admin(super::AdminTags::HideMemberCount)),
            //Fun Tags
            (r#"system_neuralink_beta"#, super::Tags::Other(Arc::from(r#"system_neuralink_beta"#))),
            (r#"system_extremely_cool_guy"#, super::Tags::Other(Arc::from(r#"system_extremely_cool_guy"#))),
            (r#"system_stop_being_nosy"#, super::Tags::Other(Arc::from(r#"system_stop_being_nosy"#))),
            (r#"system_notamod"#, super::Tags::Other(Arc::from(r#"system_notamod"#))),
            (r#"system_no_seriously_im_not_a_mod_how_many_times_do_i_have_to_tell_people"#, super::Tags::Other(Arc::from(r#"system_no_seriously_im_not_a_mod_how_many_times_do_i_have_to_tell_people"#))),
            (r#"system_the_tag_is_just_named_that"#, super::Tags::Other(Arc::from(r#"system_the_tag_is_just_named_that"#))),
            (r#"system_haha_you_have_to_document_this_one_too"#, super::Tags::Other(Arc::from(r#"system_haha_you_have_to_document_this_one_too"#))),
            (r#"system_legen_wait_for_it_dary"#, super::Tags::Other(Arc::from(r#"system_legen_wait_for_it_dary"#))),
            (r#"system_cute_robot"#, super::Tags::Other(Arc::from(r#"system_cute_robot"#))),
            (r#"Never gonna give you up"#, super::Tags::Other(Arc::from(r#"Never gonna give you up"#))),
            (r#"system_not_cute"#, super::Tags::Other(Arc::from(r#"system_not_cute"#))),
            (r#"'; DROP DATABASE tags; --"#, super::Tags::Other(Arc::from(r#"'; DROP DATABASE tags; --"#))),
            (r#""'; DROP TABLE tags; --"#, super::Tags::Other(Arc::from(r#""'; DROP TABLE tags; --"#))),
            (r#""¯\_(ツ)_/¯"#, super::Tags::Other(Arc::from(r#""¯\_(ツ)_/¯"#))),
            (r#"ඞ"#, super::Tags::Other(Arc::from(r#"ඞ"#))),
            (r#"we've been trying to reach you about your car's extended warranty"#, super::Tags::Other(Arc::from(r#"we've been trying to reach you about your car's extended warranty"#))),
            (r#""<script>alert(document.cookie);</script>"#, super::Tags::Other(Arc::from(r#""<script>alert(document.cookie);</script>"#))),
            (r#"eeeeeeeeee"#, super::Tags::Other(Arc::from(r#"eeeeeeeeee"#))),
            (r#"if you're reading this, you've been in a coma for almost 20 years now. we're trying a new technique. we don't know where this message will end up in your dream, but we hope we're getting through. please wake up-"#, super::Tags::Other(Arc::from(r#"if you're reading this, you've been in a coma for almost 20 years now. we're trying a new technique. we don't know where this message will end up in your dream, but we hope we're getting through. please wake up-"#))),
            (r#"system_smart_fridge_beta"#, super::Tags::Other(Arc::from(r#"system_smart_fridge_beta"#))),
            (r#"system_hey_mom_look_im_in_a_git_commit"#, super::Tags::Other(Arc::from(r#"system_hey_mom_look_im_in_a_git_commit"#))),
            (r#"system_trust_sussy"#, super::Tags::Other(Arc::from(r#"system_trust_sussy"#))),
            (r#"system_lizard"#, super::Tags::Other(Arc::from(r#"system_lizard"#))),
            (r#"system_me_I_read_them"#, super::Tags::Other(Arc::from(r#"system_me_I_read_them"#))),
            (r#"system_slug"#, super::Tags::Other(Arc::from(r#"system_slug"#))),
        ]{
            assert_eq!(super::Tags::from(tag).get_tag(), expected.get_tag(), "Failed to verify tag {tag} against {:#?}.", expected.to_string());
            assert_eq!(tag, String::from(expected.clone()), "Failed to verify tag {tag} against {:#?}.", expected.to_string());
        }
    }
}