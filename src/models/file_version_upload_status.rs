/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

/// FileVersionUploadStatus : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileVersionUploadStatus {
    #[serde(rename = "uploadId")]
    pub upload_id: std::sync::Arc<str>,
    #[serde(rename = "fileName")]
    pub file_name: std::sync::Arc<str>,
    #[serde(rename = "nextPartNumber")]
    pub next_part_number: f32,
    #[serde(rename = "maxParts")]
    pub max_parts: f32,
    #[serde(rename = "parts")]
    pub parts: Vec<serde_json::Value>,
    /// Unknown
    #[serde(rename = "etags")]
    pub etags: Vec<serde_json::Value>,
}

impl FileVersionUploadStatus {
    /// 
    pub fn new(upload_id: std::sync::Arc<str>, file_name: std::sync::Arc<str>, next_part_number: f32, max_parts: f32, parts: Vec<serde_json::Value>, etags: Vec<serde_json::Value>) -> FileVersionUploadStatus {
        FileVersionUploadStatus {
            upload_id,
            file_name,
            next_part_number,
            max_parts,
            parts,
            etags,
        }
    }
}


