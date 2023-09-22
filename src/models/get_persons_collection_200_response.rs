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
pub struct GetPersonsCollection200Response {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::PersonsCollectionResponseObject>>,
    #[serde(rename = "additional_data", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<Box<crate::models::GetActivitiesCollectionResponse200AdditionalData>>,
}

impl GetPersonsCollection200Response {
    pub fn new() -> GetPersonsCollection200Response {
        GetPersonsCollection200Response {
            success: None,
            data: None,
            additional_data: None,
        }
    }
}


