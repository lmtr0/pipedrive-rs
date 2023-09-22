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
pub struct DeleteActivitiesResponse200Data {
    /// An array of the IDs of activities that were deleted
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<i32>>,
}

impl DeleteActivitiesResponse200Data {
    pub fn new() -> DeleteActivitiesResponse200Data {
        DeleteActivitiesResponse200Data {
            id: None,
        }
    }
}


