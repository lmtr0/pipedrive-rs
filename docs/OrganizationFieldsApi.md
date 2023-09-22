# \OrganizationFieldsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_organization_field**](OrganizationFieldsApi.md#add_organization_field) | **POST** /organizationFields | Add a new organization field
[**delete_organization_field**](OrganizationFieldsApi.md#delete_organization_field) | **DELETE** /organizationFields/{id} | Delete an organization field
[**delete_organization_fields**](OrganizationFieldsApi.md#delete_organization_fields) | **DELETE** /organizationFields | Delete multiple organization fields in bulk
[**get_organization_field**](OrganizationFieldsApi.md#get_organization_field) | **GET** /organizationFields/{id} | Get one organization field
[**get_organization_fields**](OrganizationFieldsApi.md#get_organization_fields) | **GET** /organizationFields | Get all organization fields
[**update_organization_field**](OrganizationFieldsApi.md#update_organization_field) | **PUT** /organizationFields/{id} | Update an organization field



## add_organization_field

> crate::models::FieldResponse200 add_organization_field(create_field_request)
Add a new organization field

Adds a new organization field. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-new-custom-field\" target=\"_blank\" rel=\"noopener noreferrer\">adding a new custom field</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_field_request** | Option<[**CreateFieldRequest**](CreateFieldRequest.md)> |  |  |

### Return type

[**crate::models::FieldResponse200**](fieldResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_field

> crate::models::DeleteFieldResponse200 delete_organization_field(id)
Delete an organization field

Marks a field as deleted. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/deleting-a-custom-field\" target=\"_blank\" rel=\"noopener noreferrer\">deleting a custom field</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the field | [required] |

### Return type

[**crate::models::DeleteFieldResponse200**](deleteFieldResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_fields

> crate::models::DeleteFieldsResponse200 delete_organization_fields(ids)
Delete multiple organization fields in bulk

Marks multiple fields as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The comma-separated field IDs to delete | [required] |

### Return type

[**crate::models::DeleteFieldsResponse200**](deleteFieldsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_field

> crate::models::FieldResponse200 get_organization_field(id)
Get one organization field

Returns data about a specific organization field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the field | [required] |

### Return type

[**crate::models::FieldResponse200**](fieldResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_fields

> crate::models::FieldsResponse200 get_organization_fields(start, limit)
Get all organization fields

Returns data about all organization fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**crate::models::FieldsResponse200**](fieldsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_field

> crate::models::FieldResponse200 update_organization_field(id, update_field_request)
Update an organization field

Updates an organization field. For more information, see the tutorial for <a href=\" https://pipedrive.readme.io/docs/updating-custom-field-value \" target=\"_blank\" rel=\"noopener noreferrer\">updating custom fields' values</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the field | [required] |
**update_field_request** | Option<[**UpdateFieldRequest**](UpdateFieldRequest.md)> |  |  |

### Return type

[**crate::models::FieldResponse200**](fieldResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

