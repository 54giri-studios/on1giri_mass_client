use std::collections::HashMap;

use reqwest::Client;
use crate::api_types::{PopulatedMessage, UserMetadata};

const LOGIN_ROUTE: &str = "/auth/login";


#[derive(Debug)]
pub struct Raider {
    client: Client,
    metadata: UserMetadata
}

impl Raider {
    pub async fn from_credentials(
        email: impl Into<String>, 
        password: impl Into<String>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .cookie_store(true)
            .build()?;

        let mut credentials = HashMap::new();
        credentials.insert("email", email.into());
        credentials.insert("password", password.into());

        let response = client.post(crate::SERVER_URL.to_owned() + LOGIN_ROUTE)
            .form(&credentials)
            .send()
            .await?;

        let metadata: UserMetadata = response.json().await?;

        Ok(Self { client, metadata })
    }

    pub async fn send_message(channel_id: i32) -> Result<PopulatedMessage, Box<dyn std::error::Error>> {

    }

}

