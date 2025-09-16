#!/bin/bash

# Test script for the Multi-LLM MCP server

echo "Testing Multi-LLM Agent Task Runner..."

# Create a test directory
mkdir -p test_output

# Test the server by running it in the background for a few seconds
echo "Starting server..."
timeout 5s ./target/debug/multi-llm-mcp --port 30302 > test_output/server.log 2>&1 &
SERVER_PID=$!

sleep 2

# Check if server started
if kill -0 $SERVER_PID 2>/dev/null; then
    echo "✅ Server started successfully"
else
    echo "❌ Server failed to start"
    cat test_output/server.log
    exit 1
fi

# Kill the server
kill $SERVER_PID 2>/dev/null

echo "✅ Basic server test passed"
echo "Check test_output/server.log for server output"

# Test agent review function
echo "Testing agent review simulation..."
cd src
cat > test_agent.rs << 'EOF'
use std::path::Path;
mod agent_task_runner;
use agent_task_runner::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new("../test_output");
    std::fs::create_dir_all(output_dir)?;
    
    println!("Testing agent simulation...");
    let result = simulate_agent_review("test_agent", output_dir).await?;
    println!("✅ Created: {}", result.display());
    
    Ok(())
}
EOF

echo "✅ Test setup complete"