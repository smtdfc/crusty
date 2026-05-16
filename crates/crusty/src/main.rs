use clap::Parser;
use crusty_core::cli::config::handle_config;
use crusty_core::cli::plugin::{PluginCommands, handle_plugin_install};
use crusty_core::cli::proxy::{
    ProxyCommands, handle_proxy_dashboard, handle_proxy_start, handle_proxy_stop,
};
use crusty_core::cli::setup::handle_setup;
use crusty_core::cli::start::handle_start;
use crusty_core::logging::setup_logging;

#[derive(Parser)]
#[command(name = "crusty", about = "Crusty Agent CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Manage AI proxies (start, stop, dashboard)
    Proxy {
        #[command(subcommand)]
        sub: ProxyCommands,
    },

    /// Manage plugin installation and lifecycle
    Plugin {
        #[command(subcommand)]
        sub: PluginCommands,
    },

    /// Setup a new AI proxy platform (e.g., 9router)
    Setup,

    Stop,

    /// Configure settings, models, and switch active proxies
    Config,

    /// Start interactive session with option to jump directly to chat
    Start {
        /// Jump directly to chat without showing menu
        #[arg(long)]
        chat: bool,
    },
}

#[tokio::main]
async fn main() {
    let _log_guard = setup_logging();
    let cli = Cli::parse();
    sqlx::any::install_default_drivers();
    match &cli.command {
        Commands::Setup => {
            handle_setup();
        }

        Commands::Config => {
            handle_config();
        }

        Commands::Start { chat } => {
            handle_start(*chat).await;
        }

        Commands::Proxy { sub } => match sub {
            ProxyCommands::Start {} => {
                handle_proxy_start();
            }

            ProxyCommands::Stop {} => {
                handle_proxy_stop();
            }

            ProxyCommands::Dashboard {} => {
                handle_proxy_dashboard();
            }
        },

        Commands::Plugin { sub } => match sub {
            PluginCommands::Install { path } => handle_plugin_install(path),
        },

        _ => {}
    }
}
