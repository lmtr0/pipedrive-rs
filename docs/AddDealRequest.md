# AddDealRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | The title of the deal | 
**value** | Option<**String**> | The value of the deal. If omitted, value will be set to 0. | [optional]
**currency** | Option<**String**> | The currency of the deal. Accepts a 3-character currency code. If omitted, currency will be set to the default currency of the authorized user. | [optional]
**user_id** | Option<**i32**> | The ID of the user which will be the owner of the created deal. If not provided, the user making the request will be used. | [optional]
**person_id** | Option<**i32**> | The ID of a person which this deal will be linked to. If the person does not exist yet, it needs to be created first. This property is required unless `org_id` is specified. | [optional]
**org_id** | Option<**i32**> | The ID of an organization which this deal will be linked to. If the organization does not exist yet, it needs to be created first. This property is required unless `person_id` is specified. | [optional]
**pipeline_id** | Option<**i32**> | The ID of the pipeline this deal will be added to. By default, the deal will be added to the first stage of the specified pipeline. Please note that `pipeline_id` and `stage_id` should not be used together as `pipeline_id` will be ignored. | [optional]
**stage_id** | Option<**i32**> | The ID of the stage this deal will be added to. Please note that a pipeline will be assigned automatically based on the `stage_id`. If omitted, the deal will be placed in the first stage of the default pipeline. | [optional]
**status** | Option<**String**> | open = Open, won = Won, lost = Lost, deleted = Deleted. If omitted, status will be set to open. | [optional]
**add_time** | Option<**String**> | The optional creation date & time of the deal in UTC. Requires admin user API token. Format: YYYY-MM-DD HH:MM:SS | [optional]
**expected_close_date** | Option<[**String**](string.md)> | The expected close date of the deal. In ISO 8601 format: YYYY-MM-DD. | [optional]
**probability** | Option<**f32**> | The success probability percentage of the deal. Used/shown only when `deal_probability` for the pipeline of the deal is enabled. | [optional]
**lost_reason** | Option<**String**> | The optional message about why the deal was lost (to be used when status = lost) | [optional]
**visible_to** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


