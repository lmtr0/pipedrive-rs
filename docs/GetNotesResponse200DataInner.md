# GetNotesResponse200DataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the note | [optional]
**active_flag** | Option<**bool**> | Whether the note is active or deleted | [optional]
**add_time** | Option<**String**> | The creation date and time of the note | [optional]
**content** | Option<**String**> | The content of the note in HTML format. Subject to sanitization on the back-end. | [optional]
**deal** | Option<[**crate::models::GetNotesResponse200DataInnerDeal**](getNotesResponse200_data_inner_deal.md)> |  | [optional]
**lead_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the lead the note is attached to | [optional]
**deal_id** | Option<**i32**> | The ID of the deal the note is attached to | [optional]
**last_update_user_id** | Option<**i32**> | The ID of the user who last updated the note | [optional]
**org_id** | Option<**i32**> | The ID of the organization the note is attached to | [optional]
**organization** | Option<[**crate::models::GetNotesResponse200DataInnerOrganization**](getNotesResponse200_data_inner_organization.md)> |  | [optional]
**person** | Option<[**crate::models::GetNotesResponse200DataInnerPerson**](getNotesResponse200_data_inner_person.md)> |  | [optional]
**person_id** | Option<**i32**> | The ID of the person the note is attached to | [optional]
**pinned_to_deal_flag** | Option<**bool**> | If true, the results are filtered by note to deal pinning state | [optional]
**pinned_to_organization_flag** | Option<**bool**> | If true, the results are filtered by note to organization pinning state | [optional]
**pinned_to_person_flag** | Option<**bool**> | If true, the results are filtered by note to person pinning state | [optional]
**update_time** | Option<**String**> | The last updated date and time of the note | [optional]
**user** | Option<[**crate::models::GetNotesResponse200DataInnerUser**](getNotesResponse200_data_inner_user.md)> |  | [optional]
**user_id** | Option<**i32**> | The ID of the note creator | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


