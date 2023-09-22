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
pub struct AddChannel400Response {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// The error description
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "error_info", skip_serializing_if = "Option::is_none")]
    pub error_info: Option<String>,
    #[serde(rename = "additional_data", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<Box<crate::models::AddChannel400ResponseAdditionalData>>,
}

impl AddChannel400Response {
    pub fn new() -> AddChannel400Response {
        AddChannel400Response {
            success: None,
            error: None,
            error_info: None,
            additional_data: None,
        }
    }
}


