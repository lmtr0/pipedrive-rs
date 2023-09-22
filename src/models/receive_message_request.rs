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
pub struct ReceiveMessageRequest {
    /// The ID of the message
    #[serde(rename = "id")]
    pub id: String,
    /// The channel ID as in the provider
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// The ID of the provider's user that sent the message
    #[serde(rename = "sender_id")]
    pub sender_id: String,
    /// The ID of the conversation
    #[serde(rename = "conversation_id")]
    pub conversation_id: String,
    /// The body of the message
    #[serde(rename = "message")]
    pub message: String,
    /// The status of the message
    #[serde(rename = "status")]
    pub status: Status,
    /// The date and time when the message was created in the provider, in UTC. Format: YYYY-MM-DD HH:MM
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date and time when the message can no longer receive a reply, in UTC. Format: YYYY-MM-DD HH:MM
    #[serde(rename = "reply_by", skip_serializing_if = "Option::is_none")]
    pub reply_by: Option<String>,
    /// A URL that can open the conversation in the provider's side
    #[serde(rename = "conversation_link", skip_serializing_if = "Option::is_none")]
    pub conversation_link: Option<String>,
    /// The list of attachments available in the message
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<crate::models::ReceiveMessageRequestAttachmentsInner>>,
}

impl ReceiveMessageRequest {
    pub fn new(id: String, channel_id: String, sender_id: String, conversation_id: String, message: String, status: Status, created_at: String) -> ReceiveMessageRequest {
        ReceiveMessageRequest {
            id,
            channel_id,
            sender_id,
            conversation_id,
            message,
            status,
            created_at,
            reply_by: None,
            conversation_link: None,
            attachments: None,
        }
    }
}

/// The status of the message
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Sent
    }
}

