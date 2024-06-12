# UploadStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**license** | Option<[**models::VersionMinimizedAllOfLicense**](VersionMinimized_allOf_license.md)> |  | [optional]
**upload_date** | **String** |  | [readonly]
**md5sum_partial** | Option<**String**> |  | [optional][readonly]
**filesize** | Option<**i32**> |  | [optional][readonly]
**availability** | Option<**String**> |  | [optional][readonly]
**dependencies** | Option<[**Vec<models::Dependency>**](Dependency.md)> |  | [optional]
**compatibility** | Option<[**Vec<models::Compatibility>**](Compatibility.md)> |  | [optional]
**content_type** | Option<[**models::ContentType**](ContentType.md)> |  | [optional]
**unique_id** | Option<**String**> |  | [optional][readonly]
**files** | Option<[**Vec<models::UploadStatusAllOfFiles>**](UploadStatus_allOf_files.md)> |  | [optional]
**warnings** | Option<**Vec<String>**> |  | [optional]
**errors** | Option<**Vec<String>**> |  | [optional]
**status** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


