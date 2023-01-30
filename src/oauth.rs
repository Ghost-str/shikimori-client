use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse, BasicTokenType,
};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    RequestTokenError, StandardRevocableToken, TokenUrl,
};

const BASE_URL: &str = "https://shikimori.one";

pub struct OAouthClientBuilder {
    client_id: String,
    client_secret: String,
}

impl OAouthClientBuilder {
    pub fn new() -> OAouthClientBuilder {
        OAouthClientBuilder {
            client_id: "".to_string(),
            client_secret: "".to_string(),
        }
    }

    pub fn client_id(&mut self, id: String) -> &mut Self {
        self.client_id = id;
        self
    }

    pub fn client_secret(&mut self, id: String) -> &mut Self {
        self.client_secret = id;
        self
    }

    pub fn build(&self) -> OAouthClient {
        let auth_url = AuthUrl::new(BASE_URL.to_string() + "/oauth/authorize").unwrap();
        let token = TokenUrl::new(BASE_URL.to_string() + "/oauth/token").unwrap();
        let redirect_url = RedirectUrl::new("urn:ietf:wg:oauth:2.0:oob".to_string()).unwrap();

        let client = BasicClient::new(
            ClientId::new(self.client_id.clone()),
            Some(ClientSecret::new(self.client_secret.clone())),
            auth_url,
            Some(token),
        )
        .set_redirect_uri(redirect_url);

        OAouthClient { client }
    }
}

pub struct OAouthClient {
    client: Client<
        BasicErrorResponse,
        BasicTokenResponse,
        BasicTokenType,
        BasicTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
    >,
}

impl OAouthClient {
    fn get_browser_url(&self) -> String {
        let (auth_url, _) = self
            .client
            .authorize_url(CsrfToken::new_random)
            // Set the desired scopes.
            .url();

        return auth_url.to_string();
    }

    pub fn open_browser(&self) {
        open::that(self.get_browser_url()).expect("browser should opened");
    }

    pub async fn get_tokens_console(&self) -> BasicTokenResponse {
        let mut user_input = String::new();
        let stdin = std::io::stdin();
        stdin
            .read_line(&mut user_input)
            .expect("TODO: panic message");

        self.get_tokens(user_input.trim()).await
    }

    pub async fn get_tokens(&self, authorization_code: &str) -> BasicTokenResponse {
        self.client
            .exchange_code(AuthorizationCode::new(authorization_code.to_string()))
            .request_async(async_http_client)
            .await
            .expect("token response")
    }
}
