# GetNoteFieldsResponse200AllOfDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the field | [optional]
**key** | Option<**String**> | The key of the field | [optional]
**name** | Option<**String**> | The name of the field | [optional]
**field_type** | Option<**String**> | The type of the field<table><tr><th>Value</th><th>Description</th></tr><tr><td>`address`</td><td>Address field (has multiple subfields, autocompleted by Google Maps)</td></tr><tr><td>`date`</td><td>Date (format YYYY-MM-DD)</td></tr><tr><td>`daterange`</td><td>Date-range field (has a start date and end date value, both YYYY-MM-DD)</td></tr><tr><td>`double`</td><td>Numeric value</td></tr><tr><td>`enum`</td><td>Options field with a single possible chosen option</td></tr><tr></tr><tr><td>`monetary`</td><td>Monetary field (has a numeric value and a currency value)</td></tr><tr><td>`org`</td><td>Organization field (contains an organization ID which is stored on the same account)</td></tr><tr><td>`people`</td><td>Person field (contains a person ID which is stored on the same account)</td></tr><tr><td>`phone`</td><td>Phone field (up to 255 numbers and/or characters)</td></tr><tr><td>`set`</td><td>Options field with a possibility of having multiple chosen options</td></tr><tr><td>`text`</td><td>Long text (up to 65k characters)</td></tr><tr><td>`time`</td><td>Time field (format HH:MM:SS)</td></tr><tr><td>`timerange`</td><td>Time-range field (has a start time and end time value, both HH:MM:SS)</td></tr><tr><td>`user`</td><td>User field (contains a user ID of another Pipedrive user)</td></tr><tr><td>`varchar`</td><td>Text (up to 255 characters)</td></tr><tr><td>`varchar_auto`</td><td>Autocomplete text (up to 255 characters)</td></tr><tr><td>`visible_to`</td><td>System field that keeps item's visibility setting</td></tr></table> | [optional]
**active_flag** | Option<**bool**> | The active flag of the field | [optional]
**edit_flag** | Option<**bool**> | The edit flag of the field | [optional]
**bulk_edit_allowed** | Option<**bool**> | Not used | [optional]
**mandatory_flag** | Option<**bool**> | Whether or not the field is mandatory | [optional]
**options** | Option<[**Vec<crate::models::GetNoteFieldsResponse200AllOfDataInnerOptionsInner>**](getNoteFieldsResponse200_allOf_data_inner_options_inner.md)> | The options of the field. When there are no options, `null` is returned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


