use std::fs;
use futures::{SinkExt, StreamExt};

use crate::{types::{Credentials, Raider}, GlobalOpts};

pub async fn setup_raid(
    global_opts: GlobalOpts
) -> Result<Vec<Raider>, Box<dyn std::error::Error>> {
    log::info!("Setting up the raid with parameters: {:?}", global_opts);

    let maybe_contents: String = fs::read_to_string(&global_opts.raider_file)
        .expect("Failed to read the raiders file");

    let credentials: Vec<Credentials> = serde_json::from_str(&maybe_contents)
        .expect("Failed to parse the raiders file");


    let maybe_raiders: Vec<Result<Raider, Box<dyn std::error::Error>>> = futures::future::join_all(
        credentials
            .into_iter()
            .map(|creds| async {
                log::info!("Attempting to login {}", creds.email());
                let raider = Raider::from_credentials(creds).await?;
                log::info!(
                    "Logged in a new raider from file: {} ({})", 
                    raider.credentials().email(),
                    raider.metadata().username
                );
                Ok(raider)
            })
    ).await;

    let mut raiders = Vec::with_capacity(maybe_raiders.len());
    for maybe_raider in maybe_raiders.into_iter() {
        raiders.push(maybe_raider?);
    }

    let new_raiders: Vec<Result<Raider, _>> = futures::future::join_all(
        (0..(global_opts.num_raiders - raiders.len()))
            .map(|_| async {
                let creds = Credentials::random(
                    global_opts.mail_word_count, 
                    global_opts.password_word_count
                );
                log::info!("Attempting to login {}", creds.email());
                let raider = Raider::live_registered(creds).await;
                match raider {
                    Ok(r) => {
                        log::info!(
                            "Registered a new raider: {} ({})", 
                            r.credentials().email(),
                            r.metadata().username
                        );
                        Ok(r)
                    }
                    err => err
                }
            }
        )
    ).await;

    for maybe_raider in new_raiders.into_iter() {
        raiders.push(maybe_raider?);
    }

    let creds: Vec<&Credentials> = raiders.iter()
        .map(|r| r.credentials())
        .collect();

    let contents = serde_json::to_string(&creds)?;

    fs::write(&global_opts.raider_file, contents)
        .expect("Failed to write to the raiders file");

    Ok(raiders)
}
