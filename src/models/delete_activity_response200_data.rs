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
pub struct DeleteActivityResponse200Data {
    /// The ID of the activity that was deleted
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl DeleteActivityResponse200Data {
    pub fn new() -> DeleteActivityResponse200Data {
        DeleteActivityResponse200Data {
            id: None,
        }
    }
}


