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
pub struct AddRoleResponse200 {
    /// If the response is successful or not
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::AddRoleResponse200AllOfData>>,
}

impl AddRoleResponse200 {
    pub fn new() -> AddRoleResponse200 {
        AddRoleResponse200 {
            success: None,
            data: None,
        }
    }
}


