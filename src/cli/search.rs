//! Search functionality for indexed knowledge base.
//! Provides AST, content, and metadata search with file retrieval.

use clap::Subcommand;

use crate::configs::app::AppConfig;
use crate::configs::app::IndexType;
use crate::exit_error;
use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::search::SearchClient;
use crate::print_info;
use crate::print_warning;

/// Search across indexed knowledge base.
#[derive(Subcommand)]
pub enum SearchAction {
    /// Search in code structure (functions, classes, imports).
    Ast {
        /// Search terms
        query: Vec<String>,

        /// Target source (all if not specified)
        #[arg(long)]
        target: Option<String>,

        /// Maximum number of results
        #[arg(long, default_value = "1")]
        limit: usize,
    },

    /// Get all file hashes for a source (used for incremental indexing).
    Hash {
        /// Target source
        #[arg(long)]
        target: String,
    },
}

/// Execute search commands.
pub async fn run(action: SearchAction) {
    let start_time = std::time::Instant::now();
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => exit_error!("{}", e),
    };

    // Establish connection to OpenSearch
    let client = match OpenSearchClient::new(&config.opensearch.host) {
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

    // Initialize search client for query execution
    let search_client = SearchClient::new(client);

    // Dispatch to appropriate search handler
    match action {
        SearchAction::Ast {
            query,
            target,
            limit,
        } => {
            let pattern = query.join(" ");

            // Get indexes to search
            let indexes = match &target {
                Some(tg) => match config.get_index(tg, IndexType::Analysis) {
                    Ok(idx) => vec![idx],
                    Err(e) => exit_error!("{}", e),
                },
                None => config.get_indexes(IndexType::Analysis),
            };

            if indexes.is_empty() {
                print_warning!("No indexes found to search");
                return;
            }

            // Search by indexes
            let results = match search_client.search_ast(&indexes, &pattern, limit).await {
                Ok(results) => results,
                Err(e) => exit_error!("{}", e),
            };

            if results.is_empty() {
                print_warning!("No results found for '{}'", pattern);
            } else {
                let elapsed = start_time.elapsed();
                match serde_json::to_string_pretty(&results) {
                    Ok(json) => {
                        println!("{}", json);
                        print_info!(
                            "Found {} result(s) in {} index(es) ({:.1}s)",
                            results.len(),
                            indexes.len(),
                            elapsed.as_secs_f64()
                        );
                    }
                    Err(e) => exit_error!("Failed to serialize results: {}", e),
                }
            }
        }
        SearchAction::Hash { target } => {
            let index = match config.get_index(&target, IndexType::Files) {
                Ok(value) => value,
                Err(e) => exit_error!("{}", e),
            };

            let hashes = match search_client.get_hashes(&index).await {
                Ok(value) => value,
                Err(e) => exit_error!("{}", e),
            };

            match serde_json::to_string_pretty(&hashes) {
                Ok(value) => println!("{}", value),
                Err(e) => exit_error!("{}", e),
            };

            let elapsed = start_time.elapsed();
            print_info!(
                "Found {} hashes in {:.1}s",
                hashes.len(),
                elapsed.as_secs_f64()
            );
        }
    }
}
