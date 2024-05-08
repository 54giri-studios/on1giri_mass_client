use clap::{Args, Parser, Subcommand};

mod types;

const URL: &str = "http:/localhost:8000";

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
    #[clap(long)]
    num_raiders: usize,
}

#[derive(Debug, Subcommand)]
enum Command {
    Raid {
        #[clap(long, required = true)]
        guild_id: i32,

        #[clap(long, required = true)]
        channel_id: i32
    }
}

#[tokio::main]
async fn main() {
    let args = App::parse();
}

/*
async fn raid(channel_id: i32) -> Result<RaidReport, Box<dyn std::error::Error>> {
    Ok(())

}
*/
