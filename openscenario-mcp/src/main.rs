mod server;
mod tools;

use server::OpenScenarioServer;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut server = OpenScenarioServer::new();
    
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);
    
    // MCP server info
    let server_info = json!({
        "name": "openscenario-mcp",
        "version": env!("CARGO_PKG_VERSION")
    });
    
    let tools = server.register_tools();
    
    // Write initialization response
    let init_response = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "serverInfo": server_info,
            "capabilities": {
                "tools": tools
            }
        }
    });
    
    let response_str = serde_json::to_string(&init_response)?;
    stdout.write_all(response_str.as_bytes()).await?;
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    
    // Read and handle requests
    let mut line = String::new();
    while reader.read_line(&mut line).await? > 0 {
        if let Ok(request) = serde_json::from_str::<Value>(&line) {
            if let Some(method) = request.get("method").and_then(|m| m.as_str()) {
                match method {
                    "tools/call" => {
                        let params = request.get("params").cloned().unwrap_or(json!({}));
                        let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                        let args = params.get("arguments").cloned().unwrap_or(json!({}));
                        
                        let result = server.handle_tool_call(tool_name, args).await?;
                        let response = json!({
                            "jsonrpc": "2.0",
                            "id": request.get("id"),
                            "result": result
                        });
                        
                        let response_str = serde_json::to_string(&response)?;
                        stdout.write_all(response_str.as_bytes()).await?;
                        stdout.write_all(b"\n").await?;
                        stdout.flush().await?;
                    }
                    _ => {}
                }
            }
        }
        line.clear();
    }
    
    Ok(())
}
