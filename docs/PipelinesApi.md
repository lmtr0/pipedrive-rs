# \PipelinesApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_pipeline**](PipelinesApi.md#add_pipeline) | **POST** /pipelines | Add a new pipeline
[**delete_pipeline**](PipelinesApi.md#delete_pipeline) | **DELETE** /pipelines/{id} | Delete a pipeline
[**get_pipeline**](PipelinesApi.md#get_pipeline) | **GET** /pipelines/{id} | Get one pipeline
[**get_pipeline_conversion_statistics**](PipelinesApi.md#get_pipeline_conversion_statistics) | **GET** /pipelines/{id}/conversion_statistics | Get deals conversion rates in pipeline
[**get_pipeline_deals**](PipelinesApi.md#get_pipeline_deals) | **GET** /pipelines/{id}/deals | Get deals in a pipeline
[**get_pipeline_movement_statistics**](PipelinesApi.md#get_pipeline_movement_statistics) | **GET** /pipelines/{id}/movement_statistics | Get deals movements in pipeline
[**get_pipelines**](PipelinesApi.md#get_pipelines) | **GET** /pipelines | Get all pipelines
[**update_pipeline**](PipelinesApi.md#update_pipeline) | **PUT** /pipelines/{id} | Update a pipeline



## add_pipeline

> crate::models::AddPipelineResponse200 add_pipeline(pipeline_request)
Add a new pipeline

Adds a new pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_request** | Option<[**PipelineRequest**](PipelineRequest.md)> |  |  |

### Return type

[**crate::models::AddPipelineResponse200**](addPipelineResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pipeline

> crate::models::DeletePipelineResponse200 delete_pipeline(id)
Delete a pipeline

Marks a pipeline as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the pipeline | [required] |

### Return type

[**crate::models::DeletePipelineResponse200**](deletePipelineResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline

> crate::models::GetPipelineResponse200 get_pipeline(id, totals_convert_currency)
Get one pipeline

Returns data about a specific pipeline. Also returns the summary of the deals in this pipeline across its stages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the pipeline | [required] |
**totals_convert_currency** | Option<**String**> | The 3-letter currency code of any of the supported currencies. When supplied, `per_stages_converted` is returned in `deals_summary` which contains the currency-converted total amounts in the given currency per each stage. You may also set this parameter to `default_currency` in which case users default currency is used. |  |

### Return type

[**crate::models::GetPipelineResponse200**](getPipelineResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_conversion_statistics

> crate::models::GetPipelineConversionStatisticsResponse200 get_pipeline_conversion_statistics(id, start_date, end_date, user_id)
Get deals conversion rates in pipeline

Returns all stage-to-stage conversion and pipeline-to-close rates for the given time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the pipeline | [required] |
**start_date** | **String** | The start of the period. Date in format of YYYY-MM-DD. | [required] |
**end_date** | **String** | The end of the period. Date in format of YYYY-MM-DD. | [required] |
**user_id** | Option<**i32**> | The ID of the user who's pipeline metrics statistics to fetch. If omitted, the authorized user will be used. |  |

### Return type

[**crate::models::GetPipelineConversionStatisticsResponse200**](getPipelineConversionStatisticsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_deals

> crate::models::GetStageDealsResponse200 get_pipeline_deals(id, filter_id, user_id, everyone, stage_id, start, limit, get_summary, totals_convert_currency)
Get deals in a pipeline

Lists deals in a specific pipeline across all its stages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the pipeline | [required] |
**filter_id** | Option<**i32**> | If supplied, only deals matching the given filter will be returned |  |
**user_id** | Option<**i32**> | If supplied, `filter_id` will not be considered and only deals owned by the given user will be returned. If omitted, deals owned by the authorized user will be returned. |  |
**everyone** | Option<**f32**> | If supplied, `filter_id` and `user_id` will not be considered – instead, deals owned by everyone will be returned |  |
**stage_id** | Option<**i32**> | If supplied, only deals within the given stage will be returned |  |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**get_summary** | Option<**f32**> | Whether to include a summary of the pipeline in the `additional_data` or not |  |
**totals_convert_currency** | Option<**String**> | The 3-letter currency code of any of the supported currencies. When supplied, `per_stages_converted` is returned inside `deals_summary` inside `additional_data` which contains the currency-converted total amounts in the given currency per each stage. You may also set this parameter to `default_currency` in which case users default currency is used. Only works when `get_summary` parameter flag is enabled. |  |

### Return type

[**crate::models::GetStageDealsResponse200**](getStageDealsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_movement_statistics

> crate::models::GetPipelineMovementStatisticsResponse200 get_pipeline_movement_statistics(id, start_date, end_date, user_id)
Get deals movements in pipeline

Returns statistics for deals movements for the given time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the pipeline | [required] |
**start_date** | **String** | The start of the period. Date in format of YYYY-MM-DD. | [required] |
**end_date** | **String** | The end of the period. Date in format of YYYY-MM-DD. | [required] |
**user_id** | Option<**i32**> | The ID of the user who's pipeline statistics to fetch. If omitted, the authorized user will be used. |  |

### Return type

[**crate::models::GetPipelineMovementStatisticsResponse200**](getPipelineMovementStatisticsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipelines

> crate::models::GetPipelinesResponse200 get_pipelines()
Get all pipelines

Returns data about all pipelines.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetPipelinesResponse200**](getPipelinesResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pipeline

> crate::models::UpdatePipelineResponse200 update_pipeline(id, pipeline_request1)
Update a pipeline

Updates the properties of a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the pipeline | [required] |
**pipeline_request1** | Option<[**PipelineRequest1**](PipelineRequest1.md)> |  |  |

### Return type

[**crate::models::UpdatePipelineResponse200**](updatePipelineResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

