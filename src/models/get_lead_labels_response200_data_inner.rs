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
pub struct GetLeadLabelsResponse200DataInner {
    /// The unique ID of the lead label
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The name of the lead label
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The color of the label. Only a subset of colors can be used.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// The date and time of when the lead label was created. In ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
    #[serde(rename = "add_time", skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
    /// The date and time of when the lead label was last updated. In ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl GetLeadLabelsResponse200DataInner {
    pub fn new() -> GetLeadLabelsResponse200DataInner {
        GetLeadLabelsResponse200DataInner {
            id: None,
            name: None,
            color: None,
            add_time: None,
            update_time: None,
        }
    }
}

/// The color of the label. Only a subset of colors can be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "gray")]
    Gray,
}

impl Default for Color {
    fn default() -> Color {
        Self::Green
    }
}

