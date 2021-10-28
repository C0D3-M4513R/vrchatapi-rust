# \AuthenticationApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_user_exists**](AuthenticationApi.md#check_user_exists) | **GET** /auth/exists | Check User Exists
[**delete_user**](AuthenticationApi.md#delete_user) | **PUT** /user/{userId}/delete | Delete User
[**get_current_user**](AuthenticationApi.md#get_current_user) | **GET** /auth/user | Login and/or Get Current User Info
[**logout**](AuthenticationApi.md#logout) | **PUT** /logout | Logout
[**verify2_fa**](AuthenticationApi.md#verify2_fa) | **POST** /auth/twofactorauth/totp/verify | Verify 2FA code
[**verify_auth_token**](AuthenticationApi.md#verify_auth_token) | **GET** /auth | Verify Auth Token
[**verify_recovery_code**](AuthenticationApi.md#verify_recovery_code) | **POST** /auth/twofactorauth/otp/verify | Verify 2FA code with Recovery code



## check_user_exists

> crate::models::UserExists check_user_exists(email, display_name, user_id, exclude_user_id)
Check User Exists

Checks if a user by a given `username`, `displayName` or `email` exist. This is used during registration to check if a username has already been taken, during change of displayName to check if a displayName is available, and during change of email to check if the email is already used. In the later two cases the `excludeUserId` is used to exclude oneself, otherwise the result would always be true.  It is **REQUIRED** to include **AT LEAST** `username`, `displayName` **or** `email` query parameter. Although they can be combined - in addition with `excludeUserId` (generally to exclude yourself) - to further fine-tune the search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<**String**> | Filter by email. |  |
**display_name** | Option<**String**> | Filter by displayName. |  |
**user_id** | Option<**String**> | Filter by UserID. |  |
**exclude_user_id** | Option<**String**> | Exclude by UserID. |  |

### Return type

[**crate::models::UserExists**](UserExists.md)

### Authorization

[apiKeyCookie](../README.md#apiKeyCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> crate::models::CurrentUser delete_user(user_id)
Delete User

Deletes the account with given ID. Normal users only have permission to delete their own account. Account deletion is 14 days from this request, and will be cancelled if you do an authenticated request with the account afterwards.  **VRC+ NOTE:** Despite the 14-days cooldown, any VRC+ subscription will be cancelled **immediately**.  **METHOD NOTE:** Despite this being a Delete action, the method type required is PUT.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::CurrentUser**](CurrentUser.md)

### Authorization

[apiKeyCookie](../README.md#apiKeyCookie), [authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> crate::models::CurrentUser get_current_user()
Login and/or Get Current User Info

This endpoint does the following two operations:   1) Checks if you are already logged in by looking for a valid `auth` cookie. If you are have a valid auth cookie then no additional auth-related actions are taken. If you are **not** logged in then it will log you in with the `Authorization` header and set the `auth` cookie. The `auth` cookie will only be sent once.   2) If logged in, this function will also return the CurrentUser object containing detailed information about the currently logged in user.  **WARNING: Session Limit:** Each authentication with login credentials counts as a separate session, out of which you have a limited amount. Make sure to save and reuse the `auth` cookie if you are often restarting the program. The provided API libraries automatically save cookies during runtime, but does not persist during restart. While it can be fine to use username/password during development, expect in production to very fast run into the rate-limit and be temporarily blocked from making new sessions until older ones expire. The exact number of simultaneous sessions is unknown/undisclosed.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CurrentUser**](CurrentUser.md)

### Authorization

[authCookie](../README.md#authCookie), [authHeader](../README.md#authHeader), [twoFactorAuthCookie](../README.md#twoFactorAuthCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> crate::models::Success logout()
Logout

Invalidates the login session.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify2_fa

> crate::models::InlineResponse2001 verify2_fa(inline_object)
Verify 2FA code

Finishes the login sequence with a normal 2FA-generated code for accounts with 2FA-protection enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object** | Option<[**InlineObject**](InlineObject.md)> |  |  |

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_auth_token

> crate::models::InlineResponse200 verify_auth_token()
Verify Auth Token

Verify whether the currently provided Auth Token is valid.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_recovery_code

> crate::models::InlineResponse2001 verify_recovery_code(inline_object1)
Verify 2FA code with Recovery code

Finishes the login sequence with an OTP (One Time Password) recovery code for accounts with 2FA-protection enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object1** | Option<[**InlineObject1**](InlineObject1.md)> |  |  |

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
