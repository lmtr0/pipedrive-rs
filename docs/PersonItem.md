# PersonItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the person | [optional]
**company_id** | Option<**i32**> | The ID of the company related to the person | [optional]
**active_flag** | Option<**bool**> | Whether the person is active or not | [optional]
**phone** | Option<[**Vec<crate::models::PersonItemAllOfPhoneInner>**](personItem_allOf_phone_inner.md)> | A phone number supplied as a string or an array of phone objects related to the person. The structure of the array is as follows: `[{ \"value\": \"12345\", \"primary\": \"true\", \"label\": \"mobile\" }]`. Please note that only `value` is required. | [optional]
**email** | Option<[**Vec<crate::models::PersonItemAllOfEmailInner>**](personItem_allOf_email_inner.md)> | An email address as a string or an array of email objects related to the person. The structure of the array is as follows: `[{ \"value\": \"mail@example.com\", \"primary\": \"true\", \"label\": \"main\" } ]`. Please note that only `value` is required. | [optional]
**first_char** | Option<**String**> | The first letter of the name of the person | [optional]
**add_time** | Option<**String**> | The date and time when the person was added/created. Format: YYYY-MM-DD HH:MM:SS | [optional]
**update_time** | Option<**String**> | The last updated date and time of the person. Format: YYYY-MM-DD HH:MM:SS | [optional]
**visible_to** | Option<**String**> | The visibility group ID of who can see the person | [optional]
**picture_id** | Option<[**crate::models::PersonItemAllOfPictureId**](personItem_allOf_picture_id.md)> |  | [optional]
**label** | Option<**i32**> | The label assigned to the person | [optional]
**org_name** | Option<**String**> | The name of the organization associated with the person | [optional]
**owner_name** | Option<**String**> | The name of the owner associated with the person | [optional]
**cc_email** | Option<**String**> | The BCC email associated with the person | [optional]
**owner_id** | Option<[**crate::models::Owner**](owner.md)> |  | [optional]
**org_id** | Option<[**crate::models::RelationshipOrganizationInfoItemWithActiveFlag**](relationshipOrganizationInfoItemWithActiveFlag.md)> |  | [optional]
**name** | Option<**String**> | The name of the person | [optional]
**first_name** | Option<**String**> | The first name of the person | [optional]
**last_name** | Option<**String**> | The last name of the person | [optional]
**email_messages_count** | Option<**i32**> | The count of email messages related to the person | [optional]
**activities_count** | Option<**i32**> | The count of activities related to the person | [optional]
**done_activities_count** | Option<**i32**> | The count of done activities related to the person | [optional]
**undone_activities_count** | Option<**i32**> | The count of undone activities related to the person | [optional]
**files_count** | Option<**i32**> | The count of files related to the person | [optional]
**notes_count** | Option<**i32**> | The count of notes related to the person | [optional]
**followers_count** | Option<**i32**> | The count of followers related to the person | [optional]
**last_incoming_mail_time** | Option<**String**> | The date and time of the last incoming email associated with the person | [optional]
**last_outgoing_mail_time** | Option<**String**> | The date and time of the last outgoing email associated with the person | [optional]
**open_deals_count** | Option<**i32**> | The count of open deals related with the item | [optional]
**related_open_deals_count** | Option<**i32**> | The count of related open deals related with the item | [optional]
**closed_deals_count** | Option<**i32**> | The count of closed deals related with the item | [optional]
**related_closed_deals_count** | Option<**i32**> | The count of related closed deals related with the item | [optional]
**won_deals_count** | Option<**i32**> | The count of won deals related with the item | [optional]
**related_won_deals_count** | Option<**i32**> | The count of related won deals related with the item | [optional]
**lost_deals_count** | Option<**i32**> | The count of lost deals related with the item | [optional]
**related_lost_deals_count** | Option<**i32**> | The count of related lost deals related with the item | [optional]
**next_activity_date** | Option<**String**> | The date of the next activity associated with the deal | [optional]
**next_activity_time** | Option<**String**> | The time of the next activity associated with the deal | [optional]
**next_activity_id** | Option<**i32**> | The ID of the next activity associated with the deal | [optional]
**last_activity_id** | Option<**i32**> | The ID of the last activity associated with the deal | [optional]
**last_activity_date** | Option<**String**> | The date of the last activity associated with the deal | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


