use clap::{Args, Parser, Subcommand};
use types::Raider;

mod api_types;
mod raid;
mod types;
pub mod utils;

mod global_opts;

pub const SERVER_URL: &str = "http://localhost:8000";


#[derive(Debug, Parser)]
#[clap(name = "raid", version)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command

}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    #[clap(long, default_value_t = 10)]
    num_raiders: usize,

    #[clap(long, default_value = "raiders.json")]
    raider_file: String,

    #[clap(long, default_value_t = 4)]
    mail_word_count: u32,

    #[clap(long, default_value_t = 4)]
    password_word_count: u32
}

#[derive(Debug, Clone, Args)]
pub struct RaidArgs {
    #[clap(long, required = true)]
    channel_id: i32,

    #[clap(long, default_value_t = 5)]
    message_length_min: u32,

    #[clap(long, default_value_t = 20)]
    message_length_max: u32,
}

#[derive(Debug, Subcommand)]
enum Command {
    Raid {
        #[clap(flatten)]
        args: RaidArgs,
    },
    Other {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = App::parse();

    let raiders: Vec<Raider> = raid::setup_raid(args.global_opts).await?;

    if let Command::Raid { args } = args.command {
    }

    Ok(())
}

/*
async fn raid(channel_id: i32) -> Result<RaidReport, Box<dyn std::error::Error>> {
    Ok(())

}
*/
