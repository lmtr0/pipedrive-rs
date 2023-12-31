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
pub struct GetPipelineConversionStatisticsResponse200AllOfDataStageConversionsInner {
    /// The stage ID from where conversion starts
    #[serde(rename = "from_stage_id", skip_serializing_if = "Option::is_none")]
    pub from_stage_id: Option<i32>,
    /// The stage ID to where conversion ends
    #[serde(rename = "to_stage_id", skip_serializing_if = "Option::is_none")]
    pub to_stage_id: Option<i32>,
    /// The conversion rate
    #[serde(rename = "conversion_rate", skip_serializing_if = "Option::is_none")]
    pub conversion_rate: Option<i32>,
}

impl GetPipelineConversionStatisticsResponse200AllOfDataStageConversionsInner {
    pub fn new() -> GetPipelineConversionStatisticsResponse200AllOfDataStageConversionsInner {
        GetPipelineConversionStatisticsResponse200AllOfDataStageConversionsInner {
            from_stage_id: None,
            to_stage_id: None,
            conversion_rate: None,
        }
    }
}


