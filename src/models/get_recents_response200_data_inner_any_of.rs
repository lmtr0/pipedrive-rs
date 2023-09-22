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
pub struct GetRecentsResponse200DataInnerAnyOf {
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::ActivityResponseObject>>,
}

impl GetRecentsResponse200DataInnerAnyOf {
    pub fn new() -> GetRecentsResponse200DataInnerAnyOf {
        GetRecentsResponse200DataInnerAnyOf {
            item: None,
            id: None,
            data: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Item {
    #[serde(rename = "activity")]
    Activity,
}

impl Default for Item {
    fn default() -> Item {
        Self::Activity
    }
}

