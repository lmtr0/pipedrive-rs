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
pub struct AddRoleAssignmentRequest {
    /// The ID of the user
    #[serde(rename = "user_id")]
    pub user_id: i32,
}

impl AddRoleAssignmentRequest {
    pub fn new(user_id: i32) -> AddRoleAssignmentRequest {
        AddRoleAssignmentRequest {
            user_id,
        }
    }
}


