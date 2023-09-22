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
pub struct PersonItemAllOfPhoneInner {
    /// The phone number
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Boolean that indicates if phone number is primary for the person or not
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    /// The label that indicates the type of the phone number. (Possible values - work, home, mobile or other)
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl PersonItemAllOfPhoneInner {
    pub fn new() -> PersonItemAllOfPhoneInner {
        PersonItemAllOfPhoneInner {
            value: None,
            primary: None,
            label: None,
        }
    }
}


