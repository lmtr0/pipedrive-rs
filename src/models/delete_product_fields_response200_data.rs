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
pub struct DeleteProductFieldsResponse200Data {
    /// Array of all the IDs of the deleted product fields
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<i32>>,
}

impl DeleteProductFieldsResponse200Data {
    pub fn new() -> DeleteProductFieldsResponse200Data {
        DeleteProductFieldsResponse200Data {
            id: None,
        }
    }
}


