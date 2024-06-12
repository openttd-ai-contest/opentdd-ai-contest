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
pub struct Author {
    #[serde(rename = "display-name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl Author {
    pub fn new() -> Author {
        Author {
            display_name: None,
        }
    }
}

