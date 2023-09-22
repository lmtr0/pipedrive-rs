# MailMessageResponse200AllOfData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | ID of the mail message. | [optional]
**from** | Option<[**Vec<crate::models::MailMessageItemForListAllOfFromInner>**](mailMessageItemForList_allOf_from_inner.md)> | The array of mail message sender (object) | [optional]
**to** | Option<[**Vec<crate::models::MailMessageItemForListAllOfFromInner>**](mailMessageItemForList_allOf_from_inner.md)> | The array of mail message receiver (object) | [optional]
**cc** | Option<[**Vec<crate::models::MailMessageItemForListAllOfFromInner>**](mailMessageItemForList_allOf_from_inner.md)> | The array of mail message copies (object) | [optional]
**bcc** | Option<[**Vec<crate::models::MailMessageItemForListAllOfFromInner>**](mailMessageItemForList_allOf_from_inner.md)> | The array of mail message blind copies (object) | [optional]
**body_url** | Option<**String**> | The mail message body URL | [optional]
**account_id** | Option<**String**> | The connection account ID | [optional]
**user_id** | Option<**i32**> | ID of the user whom mail message will be assigned to | [optional]
**mail_thread_id** | Option<**i32**> | ID of the mail message thread | [optional]
**subject** | Option<**String**> | The subject of mail message | [optional]
**snippet** | Option<**String**> | The snippet of mail message. Snippet length is up to 225 characters. | [optional]
**mail_tracking_status** | Option<**String**> | The status of tracking mail message. Value is `null` if tracking is not enabled. | [optional]
**mail_link_tracking_enabled_flag** | Option<**f32**> |  | [optional][default to Variant0]
**read_flag** | Option<**f32**> |  | [optional][default to Variant0]
**draft** | Option<**String**> | If the mail message has a draft status then the value is the mail message object as JSON formatted string, otherwise `null`. | [optional]
**draft_flag** | Option<**f32**> |  | [optional][default to Variant0]
**synced_flag** | Option<**f32**> |  | [optional][default to Variant0]
**deleted_flag** | Option<**f32**> |  | [optional][default to Variant0]
**has_body_flag** | Option<**f32**> |  | [optional][default to Variant0]
**sent_flag** | Option<**f32**> |  | [optional][default to Variant0]
**sent_from_pipedrive_flag** | Option<**f32**> |  | [optional][default to Variant0]
**smart_bcc_flag** | Option<**f32**> |  | [optional][default to Variant0]
**message_time** | Option<**String**> | Creation or receival time of the mail message | [optional]
**add_time** | Option<**String**> | The insertion into the database time of the mail message | [optional]
**update_time** | Option<**String**> | The updating time in the database of the mail message | [optional]
**has_attachments_flag** | Option<**f32**> |  | [optional][default to Variant0]
**has_inline_attachments_flag** | Option<**f32**> |  | [optional][default to Variant0]
**has_real_attachments_flag** | Option<**f32**> |  | [optional][default to Variant0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


