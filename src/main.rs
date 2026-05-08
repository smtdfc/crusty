use clap::Parser;
use coding_agent::{
    cli::{
        chat::{ChatCommands, handle_chat_start},
        setup::handle_setup,
    },
    logging::setup_logging,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Chat {
        #[command(subcommand)]
        sub: ChatCommands,
    },

    Setup,
}

#[tokio::main]
async fn main() {
    setup_logging();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Setup => {
            handle_setup();
        }
        Commands::Chat { sub } => match sub {
            ChatCommands::Start {} => {
                handle_chat_start().await;
            }
        },
    }
}
