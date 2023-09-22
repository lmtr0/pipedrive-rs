/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetPipelineConversionStatisticsResponse200AllOfData : The pipeline object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPipelineConversionStatisticsResponse200AllOfData {
    /// The stage conversions
    #[serde(rename = "stage_conversions", skip_serializing_if = "Option::is_none")]
    pub stage_conversions: Option<Vec<crate::models::GetPipelineConversionStatisticsResponse200AllOfDataStageConversionsInner>>,
    /// The won conversion
    #[serde(rename = "won_conversion", skip_serializing_if = "Option::is_none")]
    pub won_conversion: Option<i32>,
    /// The lost conversion
    #[serde(rename = "lost_conversion", skip_serializing_if = "Option::is_none")]
    pub lost_conversion: Option<i32>,
}

impl GetPipelineConversionStatisticsResponse200AllOfData {
    /// The pipeline object
    pub fn new() -> GetPipelineConversionStatisticsResponse200AllOfData {
        GetPipelineConversionStatisticsResponse200AllOfData {
            stage_conversions: None,
            won_conversion: None,
            lost_conversion: None,
        }
    }
}


