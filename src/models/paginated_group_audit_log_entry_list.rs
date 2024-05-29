/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedGroupAuditLogEntryList {
    ///  
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::GroupAuditLogEntry>>,
    /// The total number of results that the query would return if there were no pagination.
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// Whether there are more results after this page.
    #[serde(rename = "hasNext", skip_serializing_if = "Option::is_none")]
    pub has_next: Option<bool>,
}

impl PaginatedGroupAuditLogEntryList {
    pub fn new() -> PaginatedGroupAuditLogEntryList {
        PaginatedGroupAuditLogEntryList {
            results: None,
            total_count: None,
            has_next: None,
        }
    }
}


