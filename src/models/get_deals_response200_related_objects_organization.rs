/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetDealsResponse200RelatedObjectsOrganization : The organization which is associated with the deal



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDealsResponse200RelatedObjectsOrganization {
    /// The name of the organization associated with the deal
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of people connected with the organization that is associated with the deal
    #[serde(rename = "people_count", skip_serializing_if = "Option::is_none")]
    pub people_count: Option<i32>,
    /// The ID of the owner of the organization that is associated with the deal
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i32>,
    /// The address of the organization that is associated with the deal
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Whether the associated organization is active or not
    #[serde(rename = "active_flag", skip_serializing_if = "Option::is_none")]
    pub active_flag: Option<bool>,
    /// The BCC email of the organization associated with the deal
    #[serde(rename = "cc_email", skip_serializing_if = "Option::is_none")]
    pub cc_email: Option<String>,
}

impl GetDealsResponse200RelatedObjectsOrganization {
    /// The organization which is associated with the deal
    pub fn new() -> GetDealsResponse200RelatedObjectsOrganization {
        GetDealsResponse200RelatedObjectsOrganization {
            name: None,
            people_count: None,
            owner_id: None,
            address: None,
            active_flag: None,
            cc_email: None,
        }
    }
}


