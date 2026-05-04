//! Project information and statistics commands.
//! Provides CLI tools for verifying indexed data.

use clap::Subcommand;

use crate::configs::app::AppConfig;
use crate::exit_error;
use crate::opensearch::client::OpenSearchClient;
use crate::print_info;

/// Project information and statistics commands.
#[derive(Subcommand)]
pub enum StatsAction {
    /// Show project statistics (files, lines, languages, AST objects, size).
    Info {
        /// Target source (all if not specified)
        #[arg(long)]
        target: Option<String>,
    },

    /// Show project directory tree structure.
    Tree {
        /// Target source
        #[arg(long)]
        target: Option<String>,

        /// Maximum depth to traverse (default: 3)
        #[arg(long, short = 'L', default_value = "3")]
        level: usize,
    },
}

/// Execute search commands.
pub async fn run(action: StatsAction) {
    let start_time = std::time::Instant::now();
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => exit_error!("{}", e),
    };

    // Establish connection to OpenSearch
    let client = match OpenSearchClient::new(&config) {
        Ok(client) => client,
        Err(e) => exit_error!("{}", e),
    };

    // Check connection
    if let Err(e) = client.healthcheck().await {
        exit_error!(
            "OpenSearch is not available at {}: {}",
            config.opensearch.host,
            e
        );
    }

    // Dispatch to appropriate search handler
    match action {
        StatsAction::Tree { target, level } => {
            let client_tool = match client.client_tool_files_meta(target).await {
                Ok(client) => client,
                Err(e) => exit_error!("{}", e),
            };
            let tree = match client_tool.show_tree(level).await {
                Ok(value) => value,
                Err(e) => exit_error!("{}", e),
            };
            if tree.tree.is_empty() {
                print_info!("No results found");
            }
            println!("{}", tree.tree);
            let elapsed = start_time.elapsed();
            print_info!(
                "Found {} directories, {} files in {:.1}s",
                tree.directories,
                tree.files,
                elapsed.as_secs_f64()
            );
        }
        StatsAction::Info { target } => {
            let client_tool = match client.client_tool_files_meta(target).await {
                Ok(client) => client,
                Err(e) => exit_error!("{}", e),
            };
            let stats = match client_tool.show_stats_search().await {
                Ok(s) => s,
                Err(e) => exit_error!("{}", e),
            };
            let elapsed = start_time.elapsed();
            let separator = "━".repeat(79);
            println!("{}", separator);
            println!(
                "{:15} {:>15} {:>15} {:>15} {:>15}",
                "Language", "Files", "Lines", "AST Objects", "Size"
            );
            println!("{}", separator);
            for lang in &stats.languages {
                if lang.language != "Other" {
                    println!(
                        "{:15} {:>15} {:>15} {:>15} {:>15}",
                        lang.language, lang.files, lang.lines, lang.ast_objects, lang.size_human
                    );
                }
            }
            if let Some(other) = stats.languages.iter().find(|l| l.language == "Other") {
                println!("{}", separator);
                println!(
                    "{:15} {:>15} {:>15} {:>15} {:>15}",
                    other.language, other.files, other.lines, "", other.size_human
                );
            }
            println!("{}", separator);
            println!(
                "{:15} {:>15} {:>15} {:>15} {:>15}",
                stats.total.language,
                stats.total.files,
                stats.total.lines,
                stats.total.ast_objects,
                stats.total.size_human
            );
            println!("{}", separator);
            print_info!("Stats generated in {:.1}s", elapsed.as_secs_f64());
        }
    }
}
