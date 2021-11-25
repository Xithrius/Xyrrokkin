use octocrab::{auth::create_jwt, models::AppId};
use reqwest::header;

use crate::handlers::config::CompleteConfig;

#[derive(Debug, Clone)]
pub struct App {
    pub config: CompleteConfig,

    pub client: reqwest::Client,

    pub json_web_token: String,
}

impl App {
    pub async fn new() -> Self {
        let config = CompleteConfig::new().expect("Failed to create config");

        let pem_key = std::fs::read_to_string(&config.private_key_path)
            .expect("Something went wrong reading the file");

        let key = jsonwebtoken::EncodingKey::from_rsa_pem(pem_key.as_bytes()).unwrap();

        // Needs to be refreshed every 10 minutes
        let jwt = create_jwt(AppId(config.application_id), &key).unwrap();

        let mut headers = header::HeaderMap::new();
        headers.insert(header::ACCEPT, header::HeaderValue::from_str("application/vnd.github.v3+json").unwrap());

        let mut auth_value = header::HeaderValue::from_str(&format!("Bearer {}", jwt)).unwrap();

        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent("Xyrrokkin by Xithrius")
            .build()
            .unwrap();

        App {
            config,
            client,
            json_web_token: jwt,
        }
    }
}
