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
pub struct DeleteOrganizationFollowerResponse200Data {
    /// The ID of the follower that was deleted from the organization
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl DeleteOrganizationFollowerResponse200Data {
    pub fn new() -> DeleteOrganizationFollowerResponse200Data {
        DeleteOrganizationFollowerResponse200Data {
            id: None,
        }
    }
}


