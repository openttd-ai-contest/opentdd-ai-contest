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
pub struct NewPackagePost201Response {
    #[serde(rename = "upload-token", skip_serializing_if = "Option::is_none")]
    pub upload_token: Option<String>,
}

impl NewPackagePost201Response {
    pub fn new() -> NewPackagePost201Response {
        NewPackagePost201Response {
            upload_token: None,
        }
    }
}

