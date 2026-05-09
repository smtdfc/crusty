use clap::Parser;
use crusty::cli::chat::handle_chat_start;
use crusty::cli::config::handle_config;
use crusty::cli::proxy::{ProxyCommands, handle_proxy_start, handle_proxy_stop};
use crusty::cli::setup::handle_setup;
use crusty::{cli::chat::ChatCommands, logging::setup_logging};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Chat,

    Proxy {
        #[command(subcommand)]
        sub: ProxyCommands,
    },

    Setup,
    Config,
}

#[tokio::main]
async fn main() {
    setup_logging();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Setup => {
            handle_setup();
        }

        Commands::Config => {
            handle_config();
        }

        Commands::Chat {} => {
            handle_chat_start().await;
        }

        Commands::Proxy { sub } => match sub {
            ProxyCommands::Start {} => {
                handle_proxy_start();
            }

            ProxyCommands::Stop {} => {
                handle_proxy_stop();
            }

            ProxyCommands::Dashboard {} => {}
        },
    }
}
