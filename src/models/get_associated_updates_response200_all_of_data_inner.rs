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
pub struct GetAssociatedUpdatesResponse200AllOfDataInner {
    /// The type of the person update. (Possible object types - organizationChange, dealChange, file, activity)
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// The creation date and time of the update
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The data related to the update
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl GetAssociatedUpdatesResponse200AllOfDataInner {
    pub fn new() -> GetAssociatedUpdatesResponse200AllOfDataInner {
        GetAssociatedUpdatesResponse200AllOfDataInner {
            object: None,
            timestamp: None,
            data: None,
        }
    }
}


