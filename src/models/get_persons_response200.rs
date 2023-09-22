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
pub struct GetPersonsResponse200 {
    /// If the response is successful or not
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// The array of persons
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::PersonItem>>,
    #[serde(rename = "additional_data", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<Box<crate::models::GetActivitiesResponse200AdditionalData>>,
    #[serde(rename = "related_objects", skip_serializing_if = "Option::is_none")]
    pub related_objects: Option<Box<crate::models::GetOrganizationsResponse200AllOfRelatedObjects>>,
}

impl GetPersonsResponse200 {
    pub fn new() -> GetPersonsResponse200 {
        GetPersonsResponse200 {
            success: None,
            data: None,
            additional_data: None,
            related_objects: None,
        }
    }
}


