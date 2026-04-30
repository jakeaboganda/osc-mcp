mod server;
mod tools;

use server::OpenScenarioServer;
use mcp_sdk::server::Server;
use mcp_sdk::transport::ServerStdioTransport;
use mcp_sdk::types::ServerCapabilities;
use serde_json::json;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing (logs to stderr since stdio is used for MCP)
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::io::stderr)
        .init();
    
    // Build MCP server
    let server = Server::builder(ServerStdioTransport)
        .capabilities(ServerCapabilities {
            tools: Some(json!({})),
            ..Default::default()
        })
        .request_handler("tools/list", OpenScenarioServer::handle_list_tools)
        .request_handler("tools/call", OpenScenarioServer::handle_call_tool)
        .build();
    
    // Start server
    let server_handle = {
        let server = server;
        tokio::spawn(async move { server.listen().await })
    };
    
    server_handle
        .await?
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;
    
    Ok(())
}
