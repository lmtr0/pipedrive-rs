/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetPipelineMovementStatisticsResponse200AllOfDataNewDealsFormattedValues : The formatted values of the deals



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPipelineMovementStatisticsResponse200AllOfDataNewDealsFormattedValues {
    /// The formatted values of the deals
    #[serde(rename = "CURRENCY_ID", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
}

impl GetPipelineMovementStatisticsResponse200AllOfDataNewDealsFormattedValues {
    /// The formatted values of the deals
    pub fn new() -> GetPipelineMovementStatisticsResponse200AllOfDataNewDealsFormattedValues {
        GetPipelineMovementStatisticsResponse200AllOfDataNewDealsFormattedValues {
            currency_id: None,
        }
    }
}


