# UpdatePersonRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the person | [optional]
**owner_id** | Option<**i32**> | The ID of the user who will be marked as the owner of this person. When omitted, the authorized user ID will be used. | [optional]
**org_id** | Option<**i32**> | The ID of the organization this person will belong to | [optional]
**email** | Option<[**Vec<crate::models::BasicPersonRequestEmailInner>**](basicPersonRequest_email_inner.md)> | An email address as a string or an array of email objects related to the person. The structure of the array is as follows: `[{ \"value\": \"mail@example.com\", \"primary\": \"true\", \"label\": \"main\" }]`. Please note that only `value` is required. | [optional]
**phone** | Option<[**Vec<crate::models::PersonItemAllOfPhoneInner>**](personItem_allOf_phone_inner.md)> | A phone number supplied as a string or an array of phone objects related to the person. The structure of the array is as follows: `[{ \"value\": \"12345\", \"primary\": \"true\", \"label\": \"mobile\" }]`. Please note that only `value` is required. | [optional]
**visible_to** | Option<**String**> |  | [optional]
**marketing_status** | Option<**String**> |  | [optional]
**add_time** | Option<**String**> | The optional creation date & time of the person in UTC. Requires admin user API token. Format: YYYY-MM-DD HH:MM:SS | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


