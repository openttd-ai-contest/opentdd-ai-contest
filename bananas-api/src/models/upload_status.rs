/*
 * OpenTTD Content API
 *
 * OpenTTD Content API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadStatus {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "license", skip_serializing_if = "Option::is_none")]
    pub license: Option<Box<crate::models::VersionMinimizedAllOfLicense>>,
    #[serde(rename = "upload-date")]
    pub upload_date: String,
    #[serde(rename = "md5sum-partial", skip_serializing_if = "Option::is_none")]
    pub md5sum_partial: Option<String>,
    #[serde(rename = "filesize", skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i32>,
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<crate::models::Dependency>>,
    #[serde(rename = "compatibility", skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<Vec<crate::models::Compatibility>>,
    #[serde(rename = "content-type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<crate::models::ContentType>,
    #[serde(rename = "unique-id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<crate::models::UploadStatusAllOfFiles>>,
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl UploadStatus {
    pub fn new(upload_date: String) -> UploadStatus {
        UploadStatus {
            name: None,
            description: None,
            url: None,
            tags: None,
            version: None,
            license: None,
            upload_date,
            md5sum_partial: None,
            filesize: None,
            availability: None,
            dependencies: None,
            compatibility: None,
            content_type: None,
            unique_id: None,
            files: None,
            warnings: None,
            errors: None,
            status: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Availability {
    #[serde(rename = "savegames-only")]
    SavegamesOnly,
    #[serde(rename = "new-games")]
    NewGames,
}

impl Default for Availability {
    fn default() -> Availability {
        Self::SavegamesOnly
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "Warnings")]
    Warnings,
    #[serde(rename = "Errors")]
    Errors,
}

impl Default for Status {
    fn default() -> Status {
        Self::Ok
    }
}

