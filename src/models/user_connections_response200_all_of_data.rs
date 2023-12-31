/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserConnectionsResponse200AllOfData : The object of UserConnections



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConnectionsResponse200AllOfData {
    /// The third party ID or false in case the ID is not found
    #[serde(rename = "google", skip_serializing_if = "Option::is_none")]
    pub google: Option<String>,
}

impl UserConnectionsResponse200AllOfData {
    /// The object of UserConnections
    pub fn new() -> UserConnectionsResponse200AllOfData {
        UserConnectionsResponse200AllOfData {
            google: None,
        }
    }
}


