/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateLeadRequestValue : The potential value of the lead



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLeadRequestValue {
    #[serde(rename = "amount")]
    pub amount: f32,
    #[serde(rename = "currency")]
    pub currency: String,
}

impl UpdateLeadRequestValue {
    /// The potential value of the lead
    pub fn new(amount: f32, currency: String) -> UpdateLeadRequestValue {
        UpdateLeadRequestValue {
            amount,
            currency,
        }
    }
}


