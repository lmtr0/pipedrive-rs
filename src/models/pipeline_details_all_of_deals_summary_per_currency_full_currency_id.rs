/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PipelineDetailsAllOfDealsSummaryPerCurrencyFullCurrencyId : The currency summary. This parameter is dynamic and changes according to `currency_id` value.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineDetailsAllOfDealsSummaryPerCurrencyFullCurrencyId {
    /// Deals count per currency
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// Deals value per currency
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

impl PipelineDetailsAllOfDealsSummaryPerCurrencyFullCurrencyId {
    /// The currency summary. This parameter is dynamic and changes according to `currency_id` value.
    pub fn new() -> PipelineDetailsAllOfDealsSummaryPerCurrencyFullCurrencyId {
        PipelineDetailsAllOfDealsSummaryPerCurrencyFullCurrencyId {
            count: None,
            value: None,
        }
    }
}


