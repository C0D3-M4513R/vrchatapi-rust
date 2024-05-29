#!/usr/bin/env bash

# Generate Client
rm src/apis src/models docs -rf

./node_modules/\@openapitools/openapi-generator-cli/main.js generate \
-g rust \
--additional-properties=packageName=vrchatapi,supportAsync=true \
--git-user-id=vrchatapi \
--git-repo-id=vrchatapi-rust \
-o . \
-i https://raw.githubusercontent.com/vrchatapi/specification/gh-pages/openapi.yaml \
--http-user-agent="vrchatapi-rust"
#--global-property debugOperations=true

# Update entire description (replace entire line, match the random data there) line in Cargo.toml
sed -i 's/^description = ".*"/description = "VRChat API Client for Rust"/' Cargo.toml

# Remove messily pasted markdown at top of every file
find src -type f -exec sed -i '/VRChat API Banner/d' {} \;
# Remove openapi version in every file
find src -type f -exec sed -i '/The version of the OpenAPI document/d' {} \;

# Cookie storage & Rate Limiting
sed -i 's/reqwest::Client::new()/std::sync::Arc::new(reqwest::Client::builder().cookie_store(true).build().unwrap())/g' src/apis/configuration.rs
sed -i 's/features = \["json", "multipart"\]/features = \["json", "cookies", "multipart"\]/g' Cargo.toml

#Allow custom client wrappers (e.g. for Global Concurrency or Rate Limiting)
shopt -s extglob
sed -i 's/Configuration/Configuration<impl std::ops::Deref<Target = reqwest::Client> + Clone + core::fmt::Debug>/g' src/apis/!(configuration).rs
shopt -u extglob
sed -i 's/struct Configuration/struct Configuration<T>/g' src/apis/configuration.rs
sed -i 's/pub client: reqwest::Client,/pub client: T,/g' src/apis/configuration.rs
sed -i 's/impl Configuration/impl <T> Configuration<T>/g' src/apis/configuration.rs
sed -i 's/fn new() -> Configuration/fn new() -> Configuration<std::sync::Arc<reqwest::Client>>/g' src/apis/configuration.rs
sed -i 's/impl Default for Configuration/impl Default for Configuration<std::sync::Arc<reqwest::Client>>/g' src/apis/configuration.rs


#cat patches/Adjustable-Client.rs >> src/apis/configuration.rs

#Fix example
printf "\n[dev-dependencies]\ntokio = { version = '1', features = ['macros', 'rt-multi-thread'] }" >> Cargo.toml

# https://github.com/OpenAPITools/openapi-generator/issues/14171
# Replace Option<GroupSearchSort> with Option<crate::models::GroupSearchSort> in src/apis
sed -i 's/Option<GroupSearchSort>/Option<crate::models::GroupSearchSort>/g' src/apis/*.rs
# Replace Option<SortOption with Option<crate::models::SortOption in src/apis
sed -i 's/Option<SortOption>/Option<crate::models::SortOption>/g' src/apis/*.rs
# Replace Option<ReleaseStatus with Option<crate::models::ReleaseStatus in src/apis
sed -i 's/Option<ReleaseStatus>/Option<crate::models::ReleaseStatus>/g' src/apis/*.rs
# Replace Option<OrderOption with Option<crate::models::OrderOption in src/apis
sed -i 's/Option<OrderOption>/Option<crate::models::OrderOption>/g' src/apis/*.rs
# Replace message_type: InviteMessageType with message_type: crate::models::InviteMessageType in src/apis
sed -i 's/message_type: InviteMessageType/message_type: crate::models::InviteMessageType/g' src/apis/*.rs

# -Missing Github Issue-
# Append patches/InviteMessageType-Display.rs to invite_message_type.rs
cat patches/InviteMessageType-Display.rs >> src/models/invite_message_type.rs
# Remove the ToString section and it's included function
# impl ToString for InviteMessageType {
#     fn to_string(&self) -> String {
#         match self {
#             Self::Message => String::from("message"),
#             Self::Response => String::from("response"),
#             Self::Request => String::from("request"),
#             Self::RequestResponse => String::from("requestResponse"),
#         }
#     }
# }
sed -z -i 's/impl ToString for InviteMessageType {\n[ a-zA-Z_\(\)&-\>{\n:=",]*}\n    }\n}//g' src/models/invite_message_type.rs
# https://github.com/vrchatapi/specification/issues/241
cat patches/2FA_Current_User.rs >> src/models/current_user.rs
sed -i 's/pub use self::current_user::CurrentUser;/pub use self::current_user::{EitherUserOrTwoFactor, CurrentUser};/g' src/models/mod.rs
sed -i 's/Result<crate::models::CurrentUser, Error<GetCurrentUserError>>/Result<crate::models::EitherUserOrTwoFactor, Error<GetCurrentUserError>>/g' src/apis/authentication_api.rs

#Massively reduce the amount of code generated, by using a function for all the requests
sed -i "s/use crate::apis::ResponseContent;/#[allow(unused_imports)] use crate::apis::ResponseContent;/g" src/apis/*.rs
cat patches/minimise-code.rs >> src/lib.rs
/usr/bin/env sh -c '
#swap basic_auth and USER_AGENT
sed -Ezi "s/\s+if\s+let\s+Some\(ref\s+local_var_user_agent\)\s+=\s+local_var_configuration.user_agent\s+\{\s+local_var_req_builder\s+=\s+local_var_req_builder.header\(reqwest::header::USER_AGENT,\s+local_var_user_agent.clone\(\)\);\s+\}\s+if\s+let\s+Some\(ref\s+local_var_auth_conf\)\s+=\s+local_var_configuration.basic_auth\s+\{\s+local_var_req_builder\s+=\s+local_var_req_builder.basic_auth\(local_var_auth_conf.0.to_owned\(\),\s+local_var_auth_conf.1.to_owned\(\)\);\s+\};/\n    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {\n        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());\n    };\n    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {\n        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());\n    }\n/g" src/apis/*.rs
#pull out common request code into seperate function
sed -Ezi "s/\s+if\s+let\s+Some\(ref\s+local_var_user_agent\)\s+=\s+local_var_configuration.user_agent\s+\{\s+local_var_req_builder\s+=\s+local_var_req_builder.header\(reqwest::header::USER_AGENT,\s+local_var_user_agent.clone\(\)\);\s+\}\s+let\s+local_var_req\s+=\s+local_var_req_builder.build\(\)\?;\s+let\s+local_var_resp\s+=\s+local_var_client.execute\(local_var_req\).await\?;\s+let\s+local_var_status\s+=\s+local_var_resp.status\(\);\s+let\s+local_var_content\s+=\s+local_var_resp.text\(\).await\?;\s+if\s+!local_var_status.is_client_error\(\)\s+&&\s+!local_var_status.is_server_error\(\)\s+\{\s+Ok\(\(\)\)\s+\}\s+else\s+\{\s+let\s+local_var_entity:\s+Option<\S+>\s+=\s+serde_json::from_str\(&local_var_content\).ok\(\);\s+let\s+local_var_error\s+=\s+ResponseContent\s+\{\s+status:\s+local_var_status,\s+content:\s+local_var_content,\s+entity:\s+local_var_entity\s+\};\s+Err\(Error::ResponseError\(local_var_error\)\)\s+\}/\n    crate::request_ok(local_var_configuration, local_var_req_builder, None::<()>).await/g" src/apis/*.rs
sed -Ezi "s/\s+if\s+let\s+Some\(ref\s+local_var_user_agent\)\s+=\s+local_var_configuration.user_agent\s+\{\s+local_var_req_builder\s+=\s+local_var_req_builder.header\(reqwest::header::USER_AGENT,\s+local_var_user_agent.clone\(\)\);\s+\}\s+local_var_req_builder\s+=\s+local_var_req_builder.json\(&(\S+)\);\s+let\s+local_var_req\s+=\s+local_var_req_builder.build\(\)\?;\s+let\s+local_var_resp\s+=\s+local_var_client.execute\(local_var_req\).await\?;\s+let\s+local_var_status\s+=\s+local_var_resp.status\(\);\s+let\s+local_var_content\s+=\s+local_var_resp.text\(\).await\?;\s+if\s+!local_var_status.is_client_error\(\)\s+&&\s+!local_var_status.is_server_error\(\)\s+\{\s+Ok\(\(\)\)\s+\}\s+else\s+\{\s+let\s+local_var_entity:\s+Option<\S+>\s+=\s+serde_json::from_str\(&local_var_content\).ok\(\);\s+let\s+local_var_error\s+=\s+ResponseContent\s+\{\s+status:\s+local_var_status,\s+content:\s+local_var_content,\s+entity:\s+local_var_entity\s+\};\s+Err\(Error::ResponseError\(local_var_error\)\)\s+\}/\n    crate::request_ok(local_var_configuration, local_var_req_builder, Some(\1)).await/g" src/apis/*.rs
sed -Ezi "s/\s+if\s+let\s+Some\(ref\s+local_var_user_agent\)\s+=\s+local_var_configuration\.user_agent\s+\{\s+local_var_req_builder\s+=\s+local_var_req_builder\.header\(reqwest::header::USER_AGENT,\s+local_var_user_agent\.clone\(\)\);\s+\}\s+let\s+local_var_req\s+=\s+local_var_req_builder\.build\(\)\?;\s+let\s+local_var_resp\s+=\s+local_var_client\.execute\(local_var_req\)\.await\?;\s+let\s+local_var_status\s+=\s+local_var_resp\.status\(\);\s+let\s+local_var_content\s+=\s+local_var_resp\.text\(\)\.await\?;\s+if\s+!local_var_status\.is_client_error\(\)\s+&&\s+!local_var_status\.is_server_error\(\)\s+\{\s+serde_json::from_str\(&local_var_content\)\.map_err\(Error::from\)\s+\}\s+else\s+\{\s+let\s+local_var_entity:\s+Option<\S+>\s+=\s+serde_json::from_str\(&local_var_content\)\.ok\(\);\s+let\s+local_var_error\s+=\s+ResponseContent\s+\{\s+status:\s+local_var_status,\s+content:\s+local_var_content,\s+entity:\s+local_var_entity\s+\};\s+Err\(Error::ResponseError\(local_var_error\)\)\s+\}/\n    crate::request(local_var_configuration, local_var_req_builder, None::<()>).await/g" src/apis/*.rs
sed -Ezi "s/\s+if\s+let\s+Some\(ref\s+local_var_user_agent\)\s+=\s+local_var_configuration\.user_agent\s+\{\s+local_var_req_builder\s+=\s+local_var_req_builder\.header\(reqwest::header::USER_AGENT,\s+local_var_user_agent\.clone\(\)\);\s+\}\s+local_var_req_builder\s+=\s+local_var_req_builder\.json\(&([0-9a-zA-Z_\-]+)\);\s+let\s+local_var_req\s+=\s+local_var_req_builder\.build\(\)\?;\s+let\s+local_var_resp\s+=\s+local_var_client\.execute\(local_var_req\)\.await\?;\s+let\s+local_var_status\s+=\s+local_var_resp\.status\(\);\s+let\s+local_var_content\s+=\s+local_var_resp\.text\(\)\.await\?;\s+if\s+!local_var_status\.is_client_error\(\)\s+&&\s+!local_var_status\.is_server_error\(\)\s+\{\s+serde_json::from_str\(&local_var_content\)\.map_err\(Error::from\)\s+\}\s+else\s+\{\s+let\s+local_var_entity:\s+Option<\S+>\s+=\s+serde_json::from_str\(&local_var_content\)\.ok\(\);\s+let\s+local_var_error\s+=\s+ResponseContent\s+\{\s+status:\s+local_var_status,\s+content:\s+local_var_content,\s+entity:\s+local_var_entity\s+\};\s+Err\(Error::ResponseError\(local_var_error\)\)\s+\}/\n    crate::request(local_var_configuration, local_var_req_builder, Some(\1)).await/g" src/apis/*.rs
'
sed -Ezi "s/let local_var_configuration = configuration;\s+/\n    /g" src/apis/*.rs
sed -Ezi "s/\s+let local_var_client = &local_var_configuration.client;\s+/\n    /g" src/apis/*.rs
sed -i "s/local_var_client/local_var_configuration.client/g" src/apis/*.rs
sed -i "s/local_var_configuration/configuration/g" src/apis/*.rs
sed -i "s/let mut local_var_req_builder/#[allow(unused_mut)] let mut local_var_req_builder/g" src/apis/*.rs

#actually deserialize tags
cp patches/tags.rs src/models
echo "pub mod tags;" >> src/models/mod.rs
sed -i 's/tags: Vec<String>/tags: Vec<crate::models::tags::Tags>/g' src/models/*.rs
#replace Strings with Arc<str> in tradeoff to Default impls
sed -Ei 's/(:[a-zA-Z0-9 \-_<>]*)String/\1std::sync::Arc<str>/g' src/models/*.rs
sed -Ei 's/serde = "(.*)"/serde = {version = "\1", features = ["rc"]}/g' Cargo.toml
sed -Ei 's/#\[derive\((.*)(, )?Default(, )?/#[derive(\1/g' src/models/*.rs
#add log crate. Used by tag impl
echo "" >> Cargo.toml
echo "[dependencies.log]" >> Cargo.toml
echo "version = \"0.4\"" >> Cargo.toml
cargo build
