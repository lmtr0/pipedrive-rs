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
pub struct GetDealResponse200AdditionalData {
    /// The BCC email of the deal
    #[serde(rename = "dropbox_email", skip_serializing_if = "Option::is_none")]
    pub dropbox_email: Option<String>,
}

impl GetDealResponse200AdditionalData {
    pub fn new() -> GetDealResponse200AdditionalData {
        GetDealResponse200AdditionalData {
            dropbox_email: None,
        }
    }
}


