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
pub struct GetRolesResponse200 {
    /// If the response is successful or not
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// The array of roles
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::FullRole>>,
    #[serde(rename = "additional_data", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<Box<crate::models::GetRolesResponse200AllOfAdditionalData>>,
}

impl GetRolesResponse200 {
    pub fn new() -> GetRolesResponse200 {
        GetRolesResponse200 {
            success: None,
            data: None,
            additional_data: None,
        }
    }
}


