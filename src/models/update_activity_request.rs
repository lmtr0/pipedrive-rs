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
pub struct UpdateActivityRequest {
    /// The due date of the activity. Format: YYYY-MM-DD
    #[serde(rename = "due_date", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// The due time of the activity in UTC. Format: HH:MM
    #[serde(rename = "due_time", skip_serializing_if = "Option::is_none")]
    pub due_time: Option<String>,
    /// The duration of the activity. Format: HH:MM
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// The ID of the deal this activity is associated with
    #[serde(rename = "deal_id", skip_serializing_if = "Option::is_none")]
    pub deal_id: Option<i32>,
    /// The ID of the lead in the UUID format this activity is associated with
    #[serde(rename = "lead_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lead_id: Option<Option<uuid::Uuid>>,
    /// The ID of the person this activity is associated with
    #[serde(rename = "person_id", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<i32>,
    /// The ID of the project this activity is associated with
    #[serde(rename = "project_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Option<i32>>,
    /// The ID of the organization this activity is associated with
    #[serde(rename = "org_id", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i32>,
    /// The address of the activity. Pipedrive will automatically check if the location matches a geo-location on Google maps.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Additional details about the activity that is synced to your external calendar. Unlike the note added to the activity, the description is publicly visible to any guests added to the activity.
    #[serde(rename = "public_description", skip_serializing_if = "Option::is_none")]
    pub public_description: Option<String>,
    /// The note of the activity (HTML format)
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The subject of the activity
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The type of the activity. This is in correlation with the `key_string` parameter of ActivityTypes.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The ID of the user whom the activity is assigned to
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// List of multiple persons (participants) this activity is associated with. It requires a structure as follows: `[{\"person_id\":1,\"primary_flag\":true}]`
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<serde_json::Value>>,
    /// Set the activity as 'Busy' or 'Free'. If the flag is set to `true`, your customers will not be able to book that time slot through any Scheduler links. The flag can also be unset by never setting it or overriding it with `null`. When the value of the flag is unset (`null`), the flag defaults to 'Busy' if it has a time set, and 'Free' if it is an all-day event without specified time.
    #[serde(rename = "busy_flag", skip_serializing_if = "Option::is_none")]
    pub busy_flag: Option<bool>,
    /// The attendees of the activity. This can be either your existing Pipedrive contacts or an external email address. It requires a structure as follows: `[{\"email_address\":\"mail@example.org\"}]` or `[{\"person_id\":1, \"email_address\":\"mail@example.org\"}]`
    #[serde(rename = "attendees", skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<serde_json::Value>>,
    #[serde(rename = "done", skip_serializing_if = "Option::is_none")]
    pub done: Option<Done>,
}

impl UpdateActivityRequest {
    pub fn new() -> UpdateActivityRequest {
        UpdateActivityRequest {
            due_date: None,
            due_time: None,
            duration: None,
            deal_id: None,
            lead_id: None,
            person_id: None,
            project_id: None,
            org_id: None,
            location: None,
            public_description: None,
            note: None,
            subject: None,
            r#type: None,
            user_id: None,
            participants: None,
            busy_flag: None,
            attendees: None,
            done: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Done {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for Done {
    fn default() -> Done {
        Self::Variant0
    }
}

