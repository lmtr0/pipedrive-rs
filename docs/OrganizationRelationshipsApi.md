# \OrganizationRelationshipsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_organization_relationship**](OrganizationRelationshipsApi.md#add_organization_relationship) | **POST** /organizationRelationships | Create an organization relationship
[**delete_organization_relationship**](OrganizationRelationshipsApi.md#delete_organization_relationship) | **DELETE** /organizationRelationships/{id} | Delete an organization relationship
[**get_organization_relationship**](OrganizationRelationshipsApi.md#get_organization_relationship) | **GET** /organizationRelationships/{id} | Get one organization relationship
[**get_organization_relationships**](OrganizationRelationshipsApi.md#get_organization_relationships) | **GET** /organizationRelationships | Get all relationships for organization
[**update_organization_relationship**](OrganizationRelationshipsApi.md#update_organization_relationship) | **PUT** /organizationRelationships/{id} | Update an organization relationship



## add_organization_relationship

> crate::models::AddOrganizationRelationshipResponse200 add_organization_relationship(add_organization_relationship_request)
Create an organization relationship

Creates and returns an organization relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_organization_relationship_request** | Option<[**AddOrganizationRelationshipRequest**](AddOrganizationRelationshipRequest.md)> |  |  |

### Return type

[**crate::models::AddOrganizationRelationshipResponse200**](addOrganizationRelationshipResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_relationship

> crate::models::DeleteOrganizationRelationshipResponse200 delete_organization_relationship(id)
Delete an organization relationship

Deletes an organization relationship and returns the deleted ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization relationship | [required] |

### Return type

[**crate::models::DeleteOrganizationRelationshipResponse200**](deleteOrganizationRelationshipResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_relationship

> crate::models::GetOrganizationRelationshipResponse200 get_organization_relationship(id, org_id)
Get one organization relationship

Finds and returns an organization relationship from its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization relationship | [required] |
**org_id** | Option<**i32**> | The ID of the base organization for the returned calculated values |  |

### Return type

[**crate::models::GetOrganizationRelationshipResponse200**](getOrganizationRelationshipResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_relationships

> crate::models::GetOrganizationRelationshipsResponse200 get_organization_relationships(org_id)
Get all relationships for organization

Gets all of the relationships for a supplied organization ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | The ID of the organization to get relationships for | [required] |

### Return type

[**crate::models::GetOrganizationRelationshipsResponse200**](getOrganizationRelationshipsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_relationship

> crate::models::UpdateOrganizationRelationshipResponse200 update_organization_relationship(id, organization_relationship)
Update an organization relationship

Updates and returns an organization relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization relationship | [required] |
**organization_relationship** | Option<[**OrganizationRelationship**](OrganizationRelationship.md)> |  |  |

### Return type

[**crate::models::UpdateOrganizationRelationshipResponse200**](updateOrganizationRelationshipResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

