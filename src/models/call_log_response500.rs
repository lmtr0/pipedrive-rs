/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallLogResponse500 {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// The description of the error
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// A message describing how to solve the problem
    #[serde(rename = "error_info", skip_serializing_if = "Option::is_none")]
    pub error_info: Option<String>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<serde_json::Value>>,
    #[serde(rename = "additional_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<Option<serde_json::Value>>,
}

impl CallLogResponse500 {
    pub fn new() -> CallLogResponse500 {
        CallLogResponse500 {
            success: None,
            error: None,
            error_info: None,
            data: None,
            additional_data: None,
        }
    }
}


