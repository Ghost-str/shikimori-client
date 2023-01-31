use crate::client::client::RequestError::ErrorResponce;
use crate::client::shiki_types::{UserRateResponse, UserRatesParams, WhoAmIResponce};

use crate::client::limitter::Limiter;
use crate::shiki_types::{AnimeId, UserMessageResponce, UserMessagesParams};
use async_lock::RwLock;
use oauth2::basic::BasicTokenResponse;
use oauth2::TokenResponse;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

const BASE_URL: &str = "https://shikimori.one/api";

#[derive(Debug)]
pub struct ShikiClient {
    access_token: RwLock<String>,
    refresh_token: RwLock<String>,
    limiter: Arc<Limiter>,
}

impl From<BasicTokenResponse> for ShikiClient {
    fn from(resp: BasicTokenResponse) -> Self {
        let limiter = Arc::new(Limiter::new(Duration::from_micros(300), 1, 1));

        let loop_limiter = limiter.clone();
        tokio::spawn(async move {
            loop_limiter.run_tik_loop().await;
        });

        ShikiClient {
            access_token: RwLock::from(resp.access_token().secret().to_string()),
            refresh_token: RwLock::from(resp.refresh_token().unwrap().secret().to_string()),
            limiter,
        }
    }
}

#[derive(Debug, Error)]
pub struct ErrorResponse {
    pub status: StatusCode,
    pub body: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status code: {}", self.status)?;
        write!(f, "Responce body: {}", self.body)
    }
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("something wrong with request: {0}")]
    RequestClientError(#[from] reqwest::Error),
    #[error("response with error returned: {0}")]
    ErrorResponce(#[from] ErrorResponse),
}

async fn get_request<R: DeserializeOwned>(
    uri: &str,
    client: &ShikiClient,
) -> Result<R, RequestError> {
    let url = format!("{}{}", BASE_URL, uri);
    let access_token = { client.access_token.read().await.clone() };
    let auth = format!("Bearer {}", access_token);

    client.limiter.wait_slot().await;

    let response = reqwest::Client::new()
        .get(url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        return Err(ErrorResponce(ErrorResponse {
            status,
            body: response.text().await?,
        }));
    }

    Ok(response.json::<R>().await?)
}

pub type ClientResult<R> = Result<R, RequestError>;

impl ShikiClient {
    pub async fn whoami(&self) -> ClientResult<WhoAmIResponce> {
        let uri = "/users/whoami";
        get_request(uri, self).await
    }

    pub async fn user_rates<T: Into<UserRatesParams>>(
        &self,
        params: T,
    ) -> ClientResult<UserRateResponse> {
        let params = serde_qs::to_string(&params.into()).unwrap();
        let uri = format!("/v2/user_rates?{}", params);
        get_request(uri.as_str(), self).await
    }

    pub async fn user_messages(
        &self,
        params: UserMessagesParams,
    ) -> ClientResult<UserMessageResponce> {
        let q_params = serde_qs::to_string(&params).unwrap();
        let uri = format!("/api/users/{}/messages?{}", params.user_id, q_params);
        get_request(uri.as_str(), self).await
    }

    pub async fn anime(&self, anime_id: AnimeId) {
        self.limiter.wait_slot().await;
        println!("{:?}", anime_id);
    }
}
