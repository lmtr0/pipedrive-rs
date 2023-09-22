# ListProductsResponse200AllOfDataInnerProduct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**f32**> | The ID of the product | [optional]
**name** | Option<**String**> | The name of the product | [optional]
**code** | Option<**String**> | The product code | [optional]
**unit** | Option<**String**> | The unit in which this product is sold | [optional]
**tax** | Option<**f32**> | The ax percentage | [optional][default to 0]
**active_flag** | Option<**bool**> | Whether this product is active or not | [optional][default to true]
**selectable** | Option<**bool**> | Whether this product is selected in deals or not | [optional][default to true]
**visible_to** | Option<**String**> |  | [optional]
**owner_id** | Option<[**serde_json::Value**](.md)> | Information about the Pipedrive user who owns the product | [optional]
**prices** | Option<[**serde_json::Value**](.md)> | Object of objects, each containing: currency (string), price (number), cost (number, optional), overhead_cost (number, optional) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


