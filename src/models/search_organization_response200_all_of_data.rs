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
pub struct SearchOrganizationResponse200AllOfData {
    /// The array of found items
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::SearchOrganizationResponse200AllOfDataItemsInner>>,
}

impl SearchOrganizationResponse200AllOfData {
    pub fn new() -> SearchOrganizationResponse200AllOfData {
        SearchOrganizationResponse200AllOfData {
            items: None,
        }
    }
}


