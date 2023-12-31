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
pub struct GetOrganizationResponse200 {
    /// If the response is successful or not
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::OrganizationItem>>,
    #[serde(rename = "additional_data", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<Box<crate::models::GetOrganizationResponse200AllOfAdditionalData>>,
    #[serde(rename = "related_objects", skip_serializing_if = "Option::is_none")]
    pub related_objects: Option<Box<crate::models::GetOrganizationsResponse200AllOfRelatedObjects>>,
}

impl GetOrganizationResponse200 {
    pub fn new() -> GetOrganizationResponse200 {
        GetOrganizationResponse200 {
            success: None,
            data: None,
            additional_data: None,
            related_objects: None,
        }
    }
}


