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
pub struct AddWebhookRequest {
    /// A full, valid, publicly accessible URL which determines where to send the notifications. Please note that you cannot use Pipedrive API endpoints as the `subscription_url` and the chosen URL must not redirect to another link.
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    /// The type of action to receive notifications about. Wildcard will match all supported actions.
    #[serde(rename = "event_action")]
    pub event_action: EventAction,
    /// The type of object to receive notifications about. Wildcard will match all supported objects.
    #[serde(rename = "event_object")]
    pub event_object: EventObject,
    /// The ID of the user that this webhook will be authorized with. You have the option to use a different user's `user_id`. If it is not set, the current user's `user_id` will be used. As each webhook event is checked against a user’s permissions, the webhook will only be sent if the user has access to the specified object(s). If you want to receive notifications for all events, please use a top-level admin user’s `user_id`.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// The HTTP basic auth username of the subscription URL endpoint (if required)
    #[serde(rename = "http_auth_user", skip_serializing_if = "Option::is_none")]
    pub http_auth_user: Option<String>,
    /// The HTTP basic auth password of the subscription URL endpoint (if required)
    #[serde(rename = "http_auth_password", skip_serializing_if = "Option::is_none")]
    pub http_auth_password: Option<String>,
    /// The webhook's version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

impl AddWebhookRequest {
    pub fn new(subscription_url: String, event_action: EventAction, event_object: EventObject) -> AddWebhookRequest {
        AddWebhookRequest {
            subscription_url,
            event_action,
            event_object,
            user_id: None,
            http_auth_user: None,
            http_auth_password: None,
            version: None,
        }
    }
}

/// The type of action to receive notifications about. Wildcard will match all supported actions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventAction {
    #[serde(rename = "added")]
    Added,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "merged")]
    Merged,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "*")]
    Star,
}

impl Default for EventAction {
    fn default() -> EventAction {
        Self::Added
    }
}
/// The type of object to receive notifications about. Wildcard will match all supported objects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventObject {
    #[serde(rename = "activity")]
    Activity,
    #[serde(rename = "activityType")]
    ActivityType,
    #[serde(rename = "deal")]
    Deal,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "person")]
    Person,
    #[serde(rename = "pipeline")]
    Pipeline,
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "stage")]
    Stage,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "*")]
    Star,
}

impl Default for EventObject {
    fn default() -> EventObject {
        Self::Activity
    }
}
/// The webhook's version
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "1.0")]
    Variant1Period0,
    #[serde(rename = "2.0")]
    Variant2Period0,
}

impl Default for Version {
    fn default() -> Version {
        Self::Variant1Period0
    }
}

