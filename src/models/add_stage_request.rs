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
pub struct AddStageRequest {
    /// The name of the stage
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the pipeline to add stage to
    #[serde(rename = "pipeline_id")]
    pub pipeline_id: i32,
    /// The success probability percentage of the deal. Used/shown when deal weighted values are used.
    #[serde(rename = "deal_probability", skip_serializing_if = "Option::is_none")]
    pub deal_probability: Option<i32>,
    /// Whether deals in this stage can become rotten
    #[serde(rename = "rotten_flag", skip_serializing_if = "Option::is_none")]
    pub rotten_flag: Option<bool>,
    /// The number of days the deals not updated in this stage would become rotten. Applies only if the `rotten_flag` is set.
    #[serde(rename = "rotten_days", skip_serializing_if = "Option::is_none")]
    pub rotten_days: Option<i32>,
}

impl AddStageRequest {
    pub fn new(name: String, pipeline_id: i32) -> AddStageRequest {
        AddStageRequest {
            name,
            pipeline_id,
            deal_probability: None,
            rotten_flag: None,
            rotten_days: None,
        }
    }
}


