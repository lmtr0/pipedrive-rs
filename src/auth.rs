use std::collections::HashMap;

use anyhow::Result;
use base64::Engine;
use url::{ParseError, Url};

#[derive(Debug, Deserialize, Serialize)]
/// Based on https://pipedrive.readme.io/docs/marketplace-oauth-authorization
pub struct AuthorizationTokens {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
    pub scope: String,
    pub api_domain: String,
    pub expires_in: usize,
}

/// Gets the correct url to the authorization portal
pub fn get_request_auth_url(
    client_id: &str,
    redirect_url: &str,
    state: Option<&str>,
) -> Result<String, ParseError> {
    let redirect_url = Url::parse(redirect_url)?;
    let authorize_url = if let Some(state) = state {
        Url::parse_with_params(
            "https://oauth.pipedrive.com/oauth/authorize",
            &[
                ("client_id", client_id),
                ("state", state),
                ("redirect_uri", redirect_url.as_ref()),
            ],
        )?
    } else {
        Url::parse_with_params(
            "https://oauth.pipedrive.com/oauth/authorize",
            &[
                ("client_id", client_id),
                ("redirect_uri", redirect_url.as_ref()),
            ],
        )?
    };

    Ok(authorize_url.to_string())
}

/// Authorizes the api client
pub async fn autorize_code(
    client_id: &str,
    client_secret: &str,
    redirect_url: &str,
    code: &str,
) -> Result<AuthorizationTokens> {
    use reqwest::header::{HeaderMap, HeaderValue};

    let authorize_url = "https://oauth.pipedrive.com/oauth/token";

    let mut base64_auth = String::new();
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode_string(format!("{client_id}:{client_secret}"), &mut base64_auth);

    // headers
    let mut auth_value = HeaderValue::from_str(&format!("Basic {}", base64_auth))?;
    auth_value.set_sensitive(true);
    let headers = HeaderMap::from_iter([(reqwest::header::AUTHORIZATION, auth_value)]);

    // parameters
    let form = HashMap::from([
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url),
        ("code", code),
    ]);

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?;

    let res = client.post(authorize_url).form(&form).send().await?;

    if !res.status().is_success() {
        let text = res.text().await;
        match text {
            Ok(text) => anyhow::bail!("Response was not successful: {}", text),
            Err(err) => anyhow::bail!(
                "Response was not successful and couldn't convert body to text: {:?}",
                err
            ),
        }
    }

    let body: AuthorizationTokens = res.json().await?;

    Ok(body)
}

// Refreshes the authorization token
pub async fn refresh_token(
    client_id: &str,
    client_secret: &str,
    token: AuthorizationTokens,
) -> Result<AuthorizationTokens> {
    use reqwest::header::{HeaderMap, HeaderValue};

    let authorize_url = "https://oauth.pipedrive.com/oauth/token";

    let mut base64_auth = String::new();
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode_string(format!("{client_id}:{client_secret}"), &mut base64_auth);

    // headers
    let mut auth_value = HeaderValue::from_str(&format!("Basic {}", base64_auth))?;
    auth_value.set_sensitive(true);
    let headers = HeaderMap::from_iter([(reqwest::header::AUTHORIZATION, auth_value)]);

    // parameters
    let form = HashMap::from([
        ("grant_type", "refresh_token"),
        ("refresh_token", &token.refresh_token),
    ]);

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?;

    let res = client.post(authorize_url).form(&form).send().await?;

    if !res.status().is_success() {
        let text = res.text().await;
        match text {
            Ok(text) => anyhow::bail!("Response was not successful: {}", text),
            Err(err) => anyhow::bail!(
                "Response was not successful and couldn't convert body to text: {:?}",
                err
            ),
        }
    }

    let body: AuthorizationTokens = res.json().await?;

    Ok(body)
}
