# UpdateDealRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The title of the deal | [optional]
**value** | Option<**String**> | The value of the deal. | [optional]
**currency** | Option<**String**> | The currency of the deal. Accepts a 3-character currency code. | [optional]
**user_id** | Option<**i32**> | The ID of the user which will be the new owner of the deal. | [optional]
**person_id** | Option<**i32**> | The ID of a person which this deal will be linked to. If the person does not exist yet, it needs to be created first. | [optional]
**org_id** | Option<**i32**> | The ID of an organization which this deal will be linked to. If the organization does not exist yet, it needs to be created first. | [optional]
**pipeline_id** | Option<**i32**> | The ID of the pipeline this deal will be added to. By default, the deal will be added to the first stage of the specified pipeline. Please note that `pipeline_id` and `stage_id` should not be used together as `pipeline_id` will be ignored. | [optional]
**stage_id** | Option<**i32**> | The ID of the stage this deal will be added to. Please note that a pipeline will be assigned automatically based on the `stage_id`. | [optional]
**status** | Option<**String**> | open = Open, won = Won, lost = Lost, deleted = Deleted. | [optional]
**expected_close_date** | Option<[**String**](string.md)> | The expected close date of the deal. In ISO 8601 format: YYYY-MM-DD. | [optional]
**probability** | Option<**f32**> | The success probability percentage of the deal. Used/shown only when `deal_probability` for the pipeline of the deal is enabled. | [optional]
**lost_reason** | Option<**String**> | The optional message about why the deal was lost (to be used when status = lost) | [optional]
**visible_to** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


