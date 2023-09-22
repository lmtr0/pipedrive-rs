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
pub struct AddUserRequest {
    /// The email of the user
    #[serde(rename = "email")]
    pub email: String,
    /// The access given to the user. Each item in the array represents access to a specific app. Optionally may include either admin flag or permission set ID to specify which access to give within the app. If both are omitted, the default access for the corresponding app will be used. It requires structure as follows: `[{ app: 'sales', permission_set_id: '62cc4d7f-4038-4352-abf3-a8c1c822b631' }, { app: 'global', admin: true }, { app: 'account_settings' }]` 
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<crate::models::AddUserRequestAccessInner>>,
    /// Whether the user is active or not. `false` = Not activated, `true` = Activated
    #[serde(rename = "active_flag", skip_serializing_if = "Option::is_none")]
    pub active_flag: Option<bool>,
}

impl AddUserRequest {
    pub fn new(email: String) -> AddUserRequest {
        AddUserRequest {
            email,
            access: None,
            active_flag: None,
        }
    }
}


