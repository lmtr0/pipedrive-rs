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
pub struct GetNoteFieldsResponse200AllOfDataInnerOptionsInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl GetNoteFieldsResponse200AllOfDataInnerOptionsInner {
    pub fn new() -> GetNoteFieldsResponse200AllOfDataInnerOptionsInner {
        GetNoteFieldsResponse200AllOfDataInnerOptionsInner {
            id: None,
            label: None,
        }
    }
}


