/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetActivitiesResponse200RelatedObjectsOrganizationOrganizationId : The ID of the organization associated with the item



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActivitiesResponse200RelatedObjectsOrganizationOrganizationId {
    /// The ID of the organization associated with the item
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The name of the organization associated with the item
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of people connected with the organization that is associated with the item
    #[serde(rename = "people_count", skip_serializing_if = "Option::is_none")]
    pub people_count: Option<i32>,
    /// The ID of the owner of the organization that is associated with the item
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i32>,
    /// The address of the organization
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The BCC email of the organization associated with the item
    #[serde(rename = "cc_email", skip_serializing_if = "Option::is_none")]
    pub cc_email: Option<String>,
}

impl GetActivitiesResponse200RelatedObjectsOrganizationOrganizationId {
    /// The ID of the organization associated with the item
    pub fn new() -> GetActivitiesResponse200RelatedObjectsOrganizationOrganizationId {
        GetActivitiesResponse200RelatedObjectsOrganizationOrganizationId {
            id: None,
            name: None,
            people_count: None,
            owner_id: None,
            address: None,
            cc_email: None,
        }
    }
}


