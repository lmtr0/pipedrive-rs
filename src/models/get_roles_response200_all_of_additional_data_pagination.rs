/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetRolesResponse200AllOfAdditionalDataPagination : The pagination details in the role list



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetRolesResponse200AllOfAdditionalDataPagination {
    /// Pagination start
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Items shown per page
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Whether there are more list items in the collection than displayed
    #[serde(rename = "more_items_in_collection", skip_serializing_if = "Option::is_none")]
    pub more_items_in_collection: Option<bool>,
}

impl GetRolesResponse200AllOfAdditionalDataPagination {
    /// The pagination details in the role list
    pub fn new() -> GetRolesResponse200AllOfAdditionalDataPagination {
        GetRolesResponse200AllOfAdditionalDataPagination {
            start: None,
            limit: None,
            more_items_in_collection: None,
        }
    }
}


