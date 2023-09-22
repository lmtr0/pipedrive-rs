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
pub struct AddGoalRequest {
    /// The title of the goal
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Who this goal is assigned to. It requires the following JSON structure: `{ \"id\": \"1\", \"type\": \"person\" }`. `type` can be either `person`, `company` or `team`. ID of the assignee person, company or team.
    #[serde(rename = "assignee")]
    pub assignee: serde_json::Value,
    /// The type of the goal. It requires the following JSON structure: `{ \"name\": \"deals_started\", \"params\": { \"pipeline_id\": [1, 2], \"activity_type_id\": [9] } }`. Type can be one of: `deals_won`, `deals_progressed`, `activities_completed`, `activities_added`, `deals_started` or `revenue_forecast`. `params` can include `pipeline_id`, `stage_id` or `activity_type_id`. `stage_id` is related to only `deals_progressed` type of goals and `activity_type_id` to `activities_completed` or `activities_added` types of goals. The `pipeline_id` and `activity_type_id` need to be given as an array of integers. To track the goal in all pipelines, set `pipeline_id` as `null` and similarly, to track the goal for all activities, set `activity_type_id` as `null`.”
    #[serde(rename = "type")]
    pub r#type: serde_json::Value,
    /// The expected outcome of the goal. Expected outcome can be tracked either by `quantity` or by `sum`. It requires the following JSON structure: `{ \"target\": \"50\", \"tracking_metric\": \"quantity\" }` or `{ \"target\": \"50\", \"tracking_metric\": \"sum\", \"currency_id\": 1 }`. `currency_id` should only be added to `sum` type of goals.
    #[serde(rename = "expected_outcome")]
    pub expected_outcome: serde_json::Value,
    /// The date when the goal starts and ends. It requires the following JSON structure: `{ \"start\": \"2019-01-01\", \"end\": \"2022-12-31\" }`. Date in format of YYYY-MM-DD. \"end\" can be set to `null` for an infinite, open-ended goal.
    #[serde(rename = "duration")]
    pub duration: serde_json::Value,
    /// The interval of the goal
    #[serde(rename = "interval")]
    pub interval: Interval,
}

impl AddGoalRequest {
    pub fn new(assignee: serde_json::Value, r#type: serde_json::Value, expected_outcome: serde_json::Value, duration: serde_json::Value, interval: Interval) -> AddGoalRequest {
        AddGoalRequest {
            title: None,
            assignee,
            r#type,
            expected_outcome,
            duration,
            interval,
        }
    }
}

/// The interval of the goal
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Interval {
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "quarterly")]
    Quarterly,
    #[serde(rename = "yearly")]
    Yearly,
}

impl Default for Interval {
    fn default() -> Interval {
        Self::Weekly
    }
}

