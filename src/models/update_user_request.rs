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
pub struct UpdateUserRequest {
    /// Whether the user is active or not. `false` = Not activated, `true` = Activated
    #[serde(rename = "active_flag")]
    pub active_flag: bool,
}

impl UpdateUserRequest {
    pub fn new(active_flag: bool) -> UpdateUserRequest {
        UpdateUserRequest {
            active_flag,
        }
    }
}


