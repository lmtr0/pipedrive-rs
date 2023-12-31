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
pub struct GetPersonResponse200AllOfAdditionalData {
    /// Dropbox email for the person
    #[serde(rename = "dropbox_email", skip_serializing_if = "Option::is_none")]
    pub dropbox_email: Option<String>,
}

impl GetPersonResponse200AllOfAdditionalData {
    pub fn new() -> GetPersonResponse200AllOfAdditionalData {
        GetPersonResponse200AllOfAdditionalData {
            dropbox_email: None,
        }
    }
}


