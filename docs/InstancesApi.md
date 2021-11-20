# \InstancesApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_instance**](InstancesApi.md#get_instance) | **GET** /instances/{worldId}:{instanceId} | Get Instance
[**get_short_name**](InstancesApi.md#get_short_name) | **GET** /instances/{worldId}:{instanceId}/shortName | Get Instance Short Name
[**send_self_invite**](InstancesApi.md#send_self_invite) | **POST** /instances/{worldId}:{instanceId}/invite | Send Self Invite



## get_instance

> crate::models::Instance get_instance(world_id, instance_id)
Get Instance

Returns an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** |  | [required] |
**instance_id** | **String** |  | [required] |

### Return type

[**crate::models::Instance**](Instance.md)

### Authorization

[apiKeyCookie](../README.md#apiKeyCookie), [authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_short_name

> String get_short_name(world_id, instance_id)
Get Instance Short Name

Returns an instance short name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** |  | [required] |
**instance_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[apiKeyCookie](../README.md#apiKeyCookie), [authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_self_invite

> crate::models::Success send_self_invite(world_id, instance_id)
Send Self Invite

Sends an invite to the instance to yourself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** |  | [required] |
**instance_id** | **String** |  | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[apiKeyCookie](../README.md#apiKeyCookie), [authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
