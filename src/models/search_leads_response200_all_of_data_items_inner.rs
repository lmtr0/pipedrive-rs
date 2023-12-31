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
pub struct SearchLeadsResponse200AllOfDataItemsInner {
    /// Search result relevancy
    #[serde(rename = "result_score", skip_serializing_if = "Option::is_none")]
    pub result_score: Option<f32>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<crate::models::SearchLeadsResponse200AllOfDataItemsInnerItem>>,
}

impl SearchLeadsResponse200AllOfDataItemsInner {
    pub fn new() -> SearchLeadsResponse200AllOfDataItemsInner {
        SearchLeadsResponse200AllOfDataItemsInner {
            result_score: None,
            item: None,
        }
    }
}


