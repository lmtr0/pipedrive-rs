/*
    Simple search of contacts and companies

*/
use anyhow::Result;
use pipedrive_rs::{
    apis::configuration::Configuration,
    apis::organizations_api::{add_organization, get_organizations},
    auth::{
        autorize_code,
        get_request_auth_url,
        refresh_token,
        //refresh_token
    },
};
use reqwest::header::{self, HeaderMap, HeaderValue};
use std::io::stdin;

#[tokio::main]
async fn main() -> Result<()> {
    if dotenv::dotenv().is_err() {
        println!(
            "This program needs a .env file with CLIENT_ID, CLIENT_SECRET and REDIRECT_URL set"
        )
    }

    let (client_id, client_secret, redirect_url) = (
        &std::env::var("CLIENT_ID").unwrap_or("Not found".to_string()),
        &std::env::var("CLIENT_SECRET").unwrap_or("Not found".to_string()),
        &std::env::var("REDIRECT_URL").unwrap_or("Not found".to_string()),
    );

    let authorize_url = get_request_auth_url(client_id, redirect_url, None)?;

    println!("Go to {authorize_url} and put the code here:");

    let mut code = String::new();
    stdin().read_line(&mut code)?;
    code.pop();

    println!("Code is {code:?}");

    let first_tokens = autorize_code(client_id, client_secret, redirect_url, &code).await?;
    let tokens = refresh_token(client_id, client_secret, &first_tokens.refresh_token).await?;

    if tokens.access_token == first_tokens.access_token {
        println!("ACCESS_TOKEN is the same");
    } else {
        println!("ACCESS_TOKEN changed");
    }

    if tokens.refresh_token == first_tokens.refresh_token {
        println!("REFRESH_TOKEN is the same");
    } else {
        println!("REFRESH_TOKEN changed");
    }

    println!("Hello world");

    let client = reqwest::ClientBuilder::new()
        .default_headers(HeaderMap::from_iter([(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", tokens.access_token))?,
        )]))
        .build()?;

    let config = Configuration {
        base_path: tokens.api_domain,
        bearer_access_token: Some(tokens.access_token.clone()),
        client,
        ..Default::default()
    };

    let orgs = get_organizations(&config, None, None, None, None, None, None).await?;

    println!("{:?}", orgs.data);
    if let Some(data) = orgs.data {
        let org = data
            .iter()
            .find(|e| e.name == Some("hello organization".to_string()));
        if org.is_none() {
            println!("Creating the organization");
            add_organization(
                &config,
                Some(pipedrive_rs::models::AddOrganizationRequest {
                    name: "hello organization".to_string(),
                    add_time: None,
                    owner_id: None,
                    visible_to: None,
                }),
            )
            .await?;
        } else {
            println!("Organization already exists");
        }
    }

    Ok(())
}
