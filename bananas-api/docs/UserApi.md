# \UserApi

All URIs are relative to *http://127.0.0.1:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_authorize_get**](UserApi.md#user_authorize_get) | **GET** /user/authorize | Default Oauth2 PKCE flow. Response depends on \"audience\"
[**user_get**](UserApi.md#user_get) | **GET** /user | Get information about yourself
[**user_logout_get**](UserApi.md#user_logout_get) | **GET** /user/logout | Logout of the Content API.



## user_authorize_get

> user_authorize_get()
Default Oauth2 PKCE flow. Response depends on \"audience\"

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get

> models::UserGet200Response user_get()
Get information about yourself

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserGet200Response**](_user_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_logout_get

> user_logout_get()
Logout of the Content API.

Your bearer token will no longer be valid.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

