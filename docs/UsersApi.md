# \UsersApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user**](UsersApi.md#add_user) | **POST** /users | Add a new user
[**find_users_by_name**](UsersApi.md#find_users_by_name) | **GET** /users/find | Find users by name
[**get_current_user**](UsersApi.md#get_current_user) | **GET** /users/me | Get current user data
[**get_user**](UsersApi.md#get_user) | **GET** /users/{id} | Get one user
[**get_user_followers**](UsersApi.md#get_user_followers) | **GET** /users/{id}/followers | List followers of a user
[**get_user_permissions**](UsersApi.md#get_user_permissions) | **GET** /users/{id}/permissions | List user permissions
[**get_user_role_assignments**](UsersApi.md#get_user_role_assignments) | **GET** /users/{id}/roleAssignments | List role assignments
[**get_user_role_settings**](UsersApi.md#get_user_role_settings) | **GET** /users/{id}/roleSettings | List user role settings
[**get_users**](UsersApi.md#get_users) | **GET** /users | Get all users
[**update_user**](UsersApi.md#update_user) | **PUT** /users/{id} | Update user details



## add_user

> crate::models::UserResponse200 add_user(add_user_request)
Add a new user

Adds a new user to the company, returns the ID upon success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_user_request** | Option<[**AddUserRequest**](AddUserRequest.md)> |  |  |

### Return type

[**crate::models::UserResponse200**](userResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_users_by_name

> crate::models::UsersResponse200 find_users_by_name(term, search_by_email)
Find users by name

Finds users by their name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The search term to look for | [required] |
**search_by_email** | Option<**f32**> | When enabled, the term will only be matched against email addresses of users. Default: `false` |  |[default to 0]

### Return type

[**crate::models::UsersResponse200**](usersResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> crate::models::GetCurrentUserResponse200 get_current_user()
Get current user data

Returns data about an authorized user within the company with bound company data: company ID, company name, and domain. Note that the `locale` property means 'Date/number format' in the Pipedrive account settings, not the chosen language.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCurrentUserResponse200**](getCurrentUserResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::UserResponse200 get_user(id)
Get one user

Returns data about a specific user within the company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the user | [required] |

### Return type

[**crate::models::UserResponse200**](userResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_followers

> crate::models::UserIds get_user_followers(id)
List followers of a user

Lists the followers of a specific user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the user | [required] |

### Return type

[**crate::models::UserIds**](userIds.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_permissions

> crate::models::GetUserPermissionsResponse200 get_user_permissions(id)
List user permissions

Lists aggregated permissions over all assigned permission sets for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the user | [required] |

### Return type

[**crate::models::GetUserPermissionsResponse200**](getUserPermissionsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_role_assignments

> crate::models::GetUserRoleAssignmentsResponse200 get_user_role_assignments(id, start, limit)
List role assignments

Lists role assignments for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the user | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**crate::models::GetUserRoleAssignmentsResponse200**](getUserRoleAssignmentsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_role_settings

> crate::models::GetRoleSettingsResponse200 get_user_role_settings(id)
List user role settings

Lists the settings of user's assigned role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the user | [required] |

### Return type

[**crate::models::GetRoleSettingsResponse200**](getRoleSettingsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> crate::models::UsersResponse200 get_users()
Get all users

Returns data about all users within the company.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UsersResponse200**](usersResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::UserResponse200 update_user(id, update_user_request)
Update user details

Updates the properties of a user. Currently, only `active_flag` can be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the user | [required] |
**update_user_request** | Option<[**UpdateUserRequest**](UpdateUserRequest.md)> |  |  |

### Return type

[**crate::models::UserResponse200**](userResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

