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
pub struct AddTeamUserRequest {
    /// The list of user IDs
    #[serde(rename = "users")]
    pub users: Vec<i32>,
}

impl AddTeamUserRequest {
    pub fn new(users: Vec<i32>) -> AddTeamUserRequest {
        AddTeamUserRequest {
            users,
        }
    }
}


