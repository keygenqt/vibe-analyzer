//! MCP tools integration tests
//!
//! cargo test --test mcp_test -- --test-threads=1 --nocapture

use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;
use vibe_tests::EngineTests;

// Configure test engine with real MCP server and Ollama.
vibe_tests::engine_config! {
    EngineTests::builder()
        // Infrastructure: docker compose for required services
        .compose_file("tests/mcp/fixtures/docker-compose.yml")
        // MCP server endpoint
        .mcp_host("http://localhost:9021/mcp/v1")
        // Ollama API endpoint
        .ollama_host("http://192.168.1.10:11434")
        // Model to test
        .ollama_models(&["qwen2.5-coder:3b-instruct", "qwen2.5-coder:7b-instruct"])
        // Ensure full GPU memory for the model
        .ollama_exclusive(true)
        // One-time setup: prepare test data and launch MCP server
        // Called once before all tests, receives isolated home directory and tee writer
        .on_start(|env| async move {
            let app = "target/debug/vibe-analyzer";

            // Index fixtures
            Command::new(&app)
                .stdout(env.tee.clone())
                .stderr(env.tee.clone())
                .env("HOME", &env.home)
                .args(["--config", "tests/mcp/fixtures/config.json5", "scan", "index"])
                .status()
                .expect("Failed to index");

            // Start MCP server
            let child = Command::new(&app)
                .stdout(env.tee.clone())
                .stderr(env.tee.clone())
                .env("HOME", &env.home)
                .args(["--config", "tests/mcp/fixtures/config.json5", "serve", "start"])
                .spawn()
                .expect("Failed to start MCP server");

            // Return child PID so on_stop can kill it
            Ok(Some(HashMap::from([
                ("pid".into(), child.id().to_string())
            ])))
        })
        // Cleanup: shutdown services and collect artifacts
        .on_stop(|env| {
            // Kill MCP server
            if let Some(data) = &env.data {
                if let Some(pid) = data.get("pid") {
                    let _ = Command::new("kill").arg("-9").arg(pid).status();
                }
            }
            // Create timestamped reports directory
            let report_dir = std::path::PathBuf::from("tests/reports")
                .join(chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string());
            std::fs::create_dir_all(&report_dir).ok();
            // Save raw log file
            std::fs::copy(&env.log_file, report_dir.join("test.log")).ok();
            // Save JSON report
            if let Ok(json) = serde_json::to_string_pretty(&env.report) {
                std::fs::write(report_dir.join("report.json"), json).ok();
            }
        })
        // Real-time log output
        .on_log(|event| {
            if event.message.contains("[TEST]") {
                println!("{}", event.message)
            }
        })
        // Filter log output
        .log_level(tracing::Level::DEBUG)
        // Timeout for all operations
        .timeout(Duration::from_secs(60))
        // Build the engine
        .build()
        .expect("Failed to build engine")
}

mod mcp {
    mod admin_sync;
    // mod get_file_ast;
    // mod get_file_content;
    // mod search_by_code_classes;
    // mod search_by_code_functions;
    // mod search_by_code_imports;
    // mod search_by_code_variables;
    // mod search_documentation;
    // mod show_projects;
    // mod show_stats;
    // mod show_tree;
}
