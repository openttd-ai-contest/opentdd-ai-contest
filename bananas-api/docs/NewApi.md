# \NewApi

All URIs are relative to *http://127.0.0.1:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**new_package_post**](NewApi.md#new_package_post) | **POST** /new-package | Start with the creation of a new package or version.
[**new_package_upload_token_file_uuid_delete**](NewApi.md#new_package_upload_token_file_uuid_delete) | **DELETE** /new-package/{upload-token}/{file-uuid} | Delete an uploaded file.
[**new_package_upload_token_get**](NewApi.md#new_package_upload_token_get) | **GET** /new-package/{upload-token} | Get information about the new upload.
[**new_package_upload_token_publish_post**](NewApi.md#new_package_upload_token_publish_post) | **POST** /new-package/{upload-token}/publish | Publish the new upload.
[**new_package_upload_token_put**](NewApi.md#new_package_upload_token_put) | **PUT** /new-package/{upload-token} | Update the information of a new upload



## new_package_post

> models::NewPackagePost201Response new_package_post()
Start with the creation of a new package or version.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NewPackagePost201Response**](_new_package_post_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_package_upload_token_file_uuid_delete

> new_package_upload_token_file_uuid_delete(upload_token, file_uuid)
Delete an uploaded file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_token** | **String** | Token of the new upload. | [required] |
**file_uuid** | **String** | File UUID of the upload | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_package_upload_token_get

> models::UploadStatus new_package_upload_token_get(upload_token)
Get information about the new upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_token** | **String** | Token of the new upload. | [required] |

### Return type

[**models::UploadStatus**](UploadStatus.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_package_upload_token_publish_post

> models::Version new_package_upload_token_publish_post(upload_token)
Publish the new upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_token** | **String** | Token of the new upload. | [required] |

### Return type

[**models::Version**](Version.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_package_upload_token_put

> new_package_upload_token_put(upload_token, new_package_upload_token_put_request)
Update the information of a new upload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_token** | **String** | Token of the new upload. | [required] |
**new_package_upload_token_put_request** | Option<[**NewPackageUploadTokenPutRequest**](NewPackageUploadTokenPutRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

