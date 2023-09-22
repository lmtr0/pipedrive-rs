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
pub struct DeleteMailThreadResponse200AllOfData {
    /// The ID of the deleted mail thread
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl DeleteMailThreadResponse200AllOfData {
    pub fn new() -> DeleteMailThreadResponse200AllOfData {
        DeleteMailThreadResponse200AllOfData {
            id: None,
        }
    }
}


