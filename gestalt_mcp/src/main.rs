use gestalt_mcp::start_stdio_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Default to stdio server for now as it's the primary use case for Zed
    start_stdio_server().await?;
    Ok(())
}
