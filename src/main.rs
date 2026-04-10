//! Vibe Analyzer - Main entry point.
//! Parses CLI arguments and dispatches to the appropriate subcommands (add, del, list, scan).

use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;

use crate::cli::scan::ScanAction;
use crate::cli::search::SearchAction;
use crate::cli::serve::ServeAction;
use crate::cli::source::SourceAction;

mod analyzer;
mod cli;
mod configs;
mod export;
mod mcp;
mod scanner;
mod utils;

#[derive(Parser)]
#[command(name = utils::app::app_name())]
#[command(about = utils::app::app_about())]
#[command(styles = utils::app::app_styles())]
#[command(version = utils::app::app_version())]
struct App {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan and extract structure from knowledge sources.
    Scan {
        #[command(subcommand)]
        action: ScanAction,
    },

    /// Search across indexed knowledge base
    Search {
        #[command(subcommand)]
        action: SearchAction,
    },

    /// Manage knowledge sources (add, remove, list, update)
    Source {
        #[command(subcommand)]
        action: SourceAction,
    },

    /// Start MCP server for AI assistants integration
    Serve {
        #[command(subcommand)]
        action: ServeAction,
    },
}

#[tokio::main]
async fn main() {
    match App::parse().command {
        Some(Commands::Source { action }) => cli::source::run(action),
        Some(Commands::Scan { action }) => cli::scan::run(action).await,
        Some(Commands::Search { action }) => cli::search::run(action),
        Some(Commands::Serve { action }) => cli::serve::run(action),
        None => {
            let _ = App::command().print_help();
            println!();
        }
    }
}

// #[derive(Subcommand)]
// enum Commands {
//     /// Add source to config
//     Add { path: PathBuf },

//     /// Del source from config
//     Del { path: PathBuf },

//     /// List configured sources
//     List,

//     /// Analyze source codebase
//     Scan {
//         /// Path to scan (overrides config)
//         #[arg(long)]
//         path: Option<PathBuf>,

//         /// Export format
//         #[arg(long, default_value = "json")]
//         format: ExportFormat,

//         /// Output file path (optional)
//         #[arg(long, short = 'o')]
//         output: Option<PathBuf>,
//     },

//     /// Scan and analyze source
//     Analyze {
//         /// Path to scan (overrides config)
//         #[arg(long)]
//         path: Option<PathBuf>,

//         /// Export format
//         #[arg(long, default_value = "json")]
//         format: ExportFormat,

//         /// Output file path (optional)
//         #[arg(long, short = 'o')]
//         output: Option<PathBuf>,
//     },
// }

// #[tokio::main]
// async fn main() {
//     match App::parse().command {
//         // Add source to configuration
//         Some(Commands::Add { path }) => match AppConfig::load() {
//             Ok(mut config) => {
//                 config.add_source(path);
//                 if let Err(e) = config.save() {
//                     exit_error!("{}", e);
//                 }
//                 print_success!("Source added to config");
//             }
//             Err(e) => exit_error!("{}", e),
//         },

//         // Delete source from configuration
//         Some(Commands::Del { path }) => match AppConfig::load() {
//             Ok(mut config) => match config.del_source(path) {
//                 Ok(removed) => print_success!("Removed: {}", removed.display()),
//                 Err(e) => exit_error!("{}", e),
//             },
//             Err(e) => exit_error!("{}", e),
//         },

//         // List all configured sources
//         Some(Commands::List) => match AppConfig::load() {
//             Ok(config) => {
//                 if config.sources.is_empty() {
//                     print_info!("No sources configured");
//                 } else {
//                     println!("{}", "Configured sources:".green().bold());
//                     for source in config.sources.iter() {
//                         println!("- {}", source.display().to_string().blue());
//                     }
//                 }
//             }
//             Err(e) => exit_error!("{}", e),
//         },

//         // Scan and index source(s) - extracts AST and metadata
//         Some(Commands::Scan {
//             path,
//             format,
//             output,
//         }) => {
//             if let Err(e) = cli::scan::exec(cli::scan::ScanOptions {
//                 custom_path: path,
//                 format,
//                 output,
//             })
//             .await
//             {
//                 exit_error!("{}", e);
//             }
//         }

//         // Analyze source(s) with LLM enrichment - adds descriptions and search tags
//         Some(Commands::Analyze {
//             path,
//             format,
//             output,
//         }) => {
//             if let Err(e) = cli::analyze::exec(cli::analyze::AnalyzeOptions {
//                 custom_path: path,
//                 format,
//                 output,
//             })
//             .await
//             {
//                 exit_error!("{}", e);
//             }
//         }

//         // Show help when no command is provided
//         None => {
//             let _ = App::command().print_help();
//             println!();
//         }
//     }
// }
