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
pub struct SearchDealsResponse200AllOfDataItemsInnerItemOrganization {
    /// The ID of the organization the deal is associated with
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The name of the organization the deal is associated with
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SearchDealsResponse200AllOfDataItemsInnerItemOrganization {
    pub fn new() -> SearchDealsResponse200AllOfDataItemsInnerItemOrganization {
        SearchDealsResponse200AllOfDataItemsInnerItemOrganization {
            id: None,
            name: None,
        }
    }
}


