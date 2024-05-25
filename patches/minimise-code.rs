use serde::de::DeserializeOwned;
use reqwest::RequestBuilder;
use serde::Serialize;
use reqwest::StatusCode;
use crate::apis::{configuration, Error, ResponseContent};

async fn request<T: Serialize, R:DeserializeOwned, E:DeserializeOwned>(
    configuration: &configuration::Configuration,
    request: RequestBuilder,
    data: Option<T>,
) -> Result<R, Error<E>> {
    request_int(configuration, request, data, |_, content| serde_json::from_str(&content).map_err(Error::from)).await
}
async fn request_ok<T: Serialize, E:DeserializeOwned>(
    configuration: &configuration::Configuration,
    request: RequestBuilder,
    data: Option<T>,
) -> Result<(), Error<E>> {
    request_int(configuration, request, data, |_,_| Ok(())).await
}

async fn request_int<T:Serialize, R, E:DeserializeOwned>(
    configuration: &configuration::Configuration,
    mut request: RequestBuilder,
    data: Option<T>,
    output: impl FnOnce(StatusCode, String) -> Result<R, Error<E>>
) -> Result<R, Error<E>> {
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        request = request.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(data) = data{
        request = request.json(&data);
    }

    let resp = configuration.client.execute(request.build()?).await?;
    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        output(status, content)
    } else {
        let local_var_entity: Option<E> = serde_json::from_str(&content).ok();
        let local_var_error = ResponseContent { status, content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}