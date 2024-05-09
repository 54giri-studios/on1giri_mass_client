
use core::panic;

use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::{api_types::{Message, NewMessage, UserMetadata}, RaidArgs, SERVER_URL};

use crate::utils;

const LOGIN_ROUTE: &str = "/auth/login";
const REGISTER_ROUTE: &str = "/auth/register";
const MESSAGE_SEND_ROUTE: &str = "/messages";

#[derive(Debug)]
pub struct ApiCallError {
    data: String
}

impl ApiCallError {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl std::fmt::Display for ApiCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Api call error: {}", self.data)
    }
}

impl std::error::Error for ApiCallError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    email: String,
    password: String
}
impl Credentials {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }

    pub fn random(mail_word_count: u32, password_word_count: u32) -> Self {
        let email = utils::random_words_seq(mail_word_count, ".") + "@massclient.org";
        let password = utils::random_words_seq(password_word_count, "");

        Self::new(email, password)
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Debug)]
pub struct Raider {
    client: Client,
    credentials: Credentials,
    metadata: UserMetadata
}

impl Raider {

    pub fn new(client: Client, credentials: Credentials, metadata: UserMetadata) -> Self {
        Self { client, credentials, metadata }
    }

    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    pub fn metadata(&self) -> &UserMetadata {
        &self.metadata
    }

    pub async fn from_credentials(credentials: Credentials) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .cookie_store(true)
            .build()?;

        let response = client.post(SERVER_URL.to_owned() + LOGIN_ROUTE)
            .form(&credentials)
            .send()
            .await?;

        let text: String = response.text().await?;
        let metadata: UserMetadata = serde_json::from_str(&text)
            .unwrap_or_else(|_| panic!(
                "Failed to parse {}",
                text
            )
        );

        Ok(Self::new(client, credentials, metadata))
    }

    pub async fn live_registered(credentials: Credentials) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .cookie_store(true)
            .build()?;

        let response = client.post(SERVER_URL.to_owned() + REGISTER_ROUTE)
            .form(&credentials)
            .send()
            .await?;

        let text: String = response.text().await?;
        let metadata: UserMetadata = serde_json::from_str(&text)
            .unwrap_or_else(|_| panic!(
                "Failed to parse {}",
                text
            )
        );

        Ok(Self::new(client, credentials, metadata))
    }

    pub async fn send_message(&self, args: RaidArgs) -> Result<Message, Box<dyn std::error::Error>> {
        let mut thread_rng = rand::thread_rng();
        let word_count: u32 = thread_rng.gen_range(args.message_length_min..=args.message_length_max);

        let content = utils::random_words_seq(word_count, " ");
        let new_message = NewMessage::new(args.channel_id, self.metadata.id, content);

        let response = self.client.post(SERVER_URL.to_string() + MESSAGE_SEND_ROUTE)
            .json(&new_message)
            .send()
            .await?;

        let text: String = response.text().await?;

        let message: Message = match serde_json::from_str(&text) {
            Ok(m) => m,
            Err(_) => {
                return Err(Box::new(ApiCallError::new(text)));
            }
        }; 

        Ok(message)
    }
}

