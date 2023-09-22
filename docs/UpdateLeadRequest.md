# UpdateLeadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The name of the lead | [optional]
**owner_id** | Option<**i32**> | The ID of the user which will be the owner of the created lead. If not provided, the user making the request will be used. | [optional]
**label_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | The IDs of the lead labels which will be associated with the lead | [optional]
**person_id** | Option<**i32**> | The ID of a person which this lead will be linked to. If the person does not exist yet, it needs to be created first. A lead always has to be linked to a person or organization or both.  | [optional]
**organization_id** | Option<**i32**> | The ID of an organization which this lead will be linked to. If the organization does not exist yet, it needs to be created first. A lead always has to be linked to a person or organization or both. | [optional]
**is_archived** | Option<**bool**> | A flag indicating whether the lead is archived or not | [optional]
**value** | Option<[**crate::models::UpdateLeadRequestValue**](updateLeadRequest_value.md)> |  | [optional]
**expected_close_date** | Option<[**String**](string.md)> | The date of when the deal which will be created from the lead is expected to be closed. In ISO 8601 format: YYYY-MM-DD. | [optional]
**visible_to** | Option<**String**> |  | [optional]
**was_seen** | Option<**bool**> | A flag indicating whether the lead was seen by someone in the Pipedrive UI | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


