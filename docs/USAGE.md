# Usage Guide & Examples

This guide provides practical examples and tutorials for using rust-mcp-filesystem effectively.

## 🚀 Quick Start

### Basic File Operations

### List Directory Contents

```bash

# List files in current directory
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "list_directory",
      "arguments": {
        "path": "/path/to/directory"
      }
    }
  }'
```

### Read File Content

```bash

# Read a text file
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "read_file",
      "arguments": {
        "path": "/path/to/file.txt"
      }
    }
  }'
```

### Get File Information

```bash

# Get detailed file metadata
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "get_file_info",
      "arguments": {
        "path": "/path/to/file.txt"
      }
    }
  }'
```

## 🧠 Advanced Code Analysis

### Analyze Codebase Structure

```bash

# Analyze Rust project structure
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "list_code_definition_names",
      "arguments": {
        "path": "/path/to/rust/project",
        "hardware_accelerated": true,
        "zero_copy": true
      }
    }
  }'
```

### Search for Code Patterns

```bash

# Find all function definitions
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "search_files_content",
      "arguments": {
        "path": "/path/to/project",
        "pattern": "fn [a-zA-Z_][a-zA-Z0-9_]*",
        "is_regex": true
      }
    }
  }'
```

## 📁 Directory Operations

### Recursive Directory Tree

```bash

# Get full directory structure
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "directory_tree",
      "arguments": {
        "path": "/path/to/project",
        "max_depth": 3
      }
    }
  }'
```

### Glob Pattern Search

```bash

# Find all Rust source files
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "search_files",
      "arguments": {
        "path": "/path/to/project",
        "pattern": "*.rs"
      }
    }
  }'
```

## ✏️ File Editing

### Make Line-Based Edits

```bash

# Replace specific lines in a file
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "edit_file",
      "arguments": {
        "path": "/path/to/file.rs",
        "dryRun": false,
        "edits": [
          {
            "oldText": "println!(\"Hello, world!\");",
            "newText": "println!(\"Hello, Rust!\");"
          }
        ]
      }
    }
  }'
```

### Create New Files

```bash

# Create a new file with content
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "write_file",
      "arguments": {
        "path": "/path/to/new_file.txt",
        "content": "This is a new file created via MCP."
      }
    }
  }'
```

## 📦 Archive Operations

### Create ZIP Archives

```bash

# Create ZIP from files
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "zip_files",
      "arguments": {
        "input_files": [
          "/path/to/file1.txt",
          "/path/to/file2.txt",
          "/path/to/file3.txt"
        ],
        "target_zip_file": "/path/to/archive.zip"
      }
    }
  }'
```

### Extract ZIP Archives

```bash

# Extract ZIP to directory
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "unzip_file",
      "arguments": {
        "zip_file": "/path/to/archive.zip",
        "target_path": "/path/to/extract/here"
      }
    }
  }'
```

## 🔧 Integration Examples

### With Claude Desktop

```json
// claude_desktop_config.json
{
  "mcpServers": {
    "rust-mcp-filesystem": {
      "command": "rust-mcp-filesystem",
      "args": [
        "--allowed-directory",
        "/Users/username/Documents",
        "--allowed-directory",
        "/Users/username/Projects"
      ],
      "env": {
        "MCP_ENABLE_SIMD": "true",
        "MCP_ENABLE_COMPRESSION": "true",
        "MCP_MAX_WORKERS": "4"
      }
    }
  }
}
```

### With VS Code Extension

```typescript
// Example VS Code extension integration
import { MCPClient } from 'mcp-client';

const client = new MCPClient({
  server: 'rust-mcp-filesystem',
  port: 3000
});

// List workspace files
async function listWorkspaceFiles(workspacePath: string) {
  return await client.callTool('list_directory', {
    path: workspacePath
  });
}

// Analyze code structure
async function analyzeCodebase(projectPath: string) {
  return await client.callTool('list_code_definition_names', {
    path: projectPath,
    hardware_accelerated: true,
    zero_copy: true
  });
}
```

### With Python Client

```python
# mcp_client.py
import json
import requests

class MCPFilesystemClient:
    def __init__(self, base_url="http://localhost:3000"):
        self.base_url = base_url
        self.next_id = 1

    def call_tool(self, tool_name, arguments):
        payload = {
            "jsonrpc": "2.0",
            "id": self.next_id,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments
            }
        }
        self.next_id += 1

        response = requests.post(self.base_url, json=payload)
        return response.json()

# Usage example
client = MCPFilesystemClient()

# List directory
result = client.call_tool("list_directory", {"path": "/path/to/dir"})
print(json.dumps(result, indent=2))

# Analyze code
result = client.call_tool("list_code_definition_names", {
    "path": "/path/to/project",
    "hardware_accelerated": True,
    "zero_copy": True
})
print(json.dumps(result, indent=2))
```

## 🚀 Performance Optimization Examples

### Large File Handling

```bash

# Enable streaming for large files
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "read_file",
      "arguments": {
        "path": "/path/to/large_file.dat"
      }
    }
  }' &
```

### Batch Operations

```bash

# Read multiple files efficiently
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "read_multiple_files",
      "arguments": {
        "paths": [
          "/path/to/file1.txt",
          "/path/to/file2.txt",
          "/path/to/file3.txt"
        ]
      }
    }
  }'
```

### Parallel Directory Processing

```bash

# Process large directories with parallel processing
export MCP_ENABLE_PARALLEL=true
export MCP_MAX_WORKERS=0  # Auto-detect CPU cores

curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "list_code_definition_names",
      "arguments": {
        "path": "/large/project",
        "hardware_accelerated": true,
        "zero_copy": true
      }
    }
  }'
```

## 🏗️ Real-World Use Cases

### 1. Code Analysis & Refactoring

```bash
# Find all TODO comments
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "search_files_content",
      "arguments": {
        "path": "/path/to/project",
        "pattern": "TODO|FIXME|XXX",
        "is_regex": true
      }
    }
  }'
```

### 2. Documentation Generation

```bash
# Extract all function signatures
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "list_code_definition_names",
      "arguments": {
        "path": "/path/to/source",
        "hardware_accelerated": true
      }
    }
  }'
```

### 3. Project Structure Analysis

```bash
# Get complete project overview
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "directory_tree",
      "arguments": {
        "path": "/path/to/project",
        "max_depth": 5
      }
    }
  }'
```

### 4. Automated Testing

```bash
# Find test files
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "search_files",
      "arguments": {
        "path": "/path/to/project",
        "pattern": "*test*.rs"
      }
    }
  }'
```

## 🔄 Workflow Automation

### Continuous Integration

```yaml

# .github/workflows/analyze.yml
name: Code Analysis
on: [push, pull_request]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run MCP Filesystem Analysis
        run: |
          docker run -d --name mcp \
            -p 3000:3000 \
            -v ${{ github.workspace }}:/data:ro \
            rust-mcp-filesystem:latest \
            --allowed-directory /data

          # Wait for service to be ready
          timeout 30 bash -c 'until curl -f http://localhost:3000/health; do sleep 1; done'

          # Analyze codebase
          curl -X POST http://localhost:3000 \
            -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list_code_definition_names","arguments":{"path":"/data","hardware_accelerated":true}}}' \
            > analysis.json

          # Generate report
          cat analysis.json | jq '.result.content[0].text'
```

### Documentation Generation

```bash

# Generate API documentation
#!/bin/bash
PROJECT_PATH="/path/to/project"
OUTPUT_FILE="api_documentation.md"

# Get all code definitions
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 1,
    \"method\": \"tools/call\",
    \"params\": {
      \"name\": \"list_code_definition_names\",
      \"arguments\": {
        \"path\": \"$PROJECT_PATH\",
        \"hardware_accelerated\": true
      }
    }
  }" > definitions.json

# Generate documentation
cat > $OUTPUT_FILE << 'EOF'
# API Documentation

## Code Structure

```bash
EOF

cat definitions.json >> $OUTPUT_FILE

cat >> $OUTPUT_FILE << 'EOF'
```

## File Structure

EOF

curl -X POST <http://localhost:3000> \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 2,
    \"method\": \"tools/call\",
    \"params\": {
      \"name\": \"directory_tree\",
      \"arguments\": {
        \"path\": \"$PROJECT_PATH\",
        \"max_depth\": 3
      }
    }
  }" >> $OUTPUT_FILE

## 🧪 Testing & Validation

### Unit Test Integration

```rust

// tests/integration_test.rs
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[test]
fn test_mcp_server_operations() {
    // Start MCP server
    let mut server = Command::new("rust-mcp-filesystem")
        .arg("--port")
        .arg("3001")
        .arg("--allowed-directory")
        .arg("/tmp")
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start MCP server");

    // Wait for server to start
    thread::sleep(Duration::from_secs(2));

    // Test health endpoint
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://localhost:3001")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "list_directory",
                "arguments": {
                    "path": "/tmp"
                }
            }
        }))
        .send()
        .expect("Failed to send request");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let json: serde_json::Value = response.json().unwrap();
    assert_eq!(json["jsonrpc"], "2.0");
    assert!(json["result"].is_object());

    // Cleanup
    server.kill().unwrap();
}
```

### Load Testing

```bash

# Load test with hey
hey -n 1000 -c 10 \
  -m POST \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list_directory","arguments":{"path":"/tmp"}}}' \
  http://localhost:3000

# Performance benchmarking
ab -n 1000 -c 10 \
  -p payload.json \
  -T application/json \
  http://localhost:3000/
```

## 🎯 Best Practices

### 1. Error Handling

```bash

# Always check response status
RESPONSE=$(curl -s -w "%{http_code}" -o response.json http://localhost:3000)
HTTP_CODE=$(tail -c 3 <<< "$RESPONSE")

if [ "$HTTP_CODE" -eq 200 ]; then
    # Process successful response
    cat response.json | jq '.result'
else
    # Handle error
    echo "Request failed with HTTP $HTTP_CODE"
    cat response.json
fi
```

### 2. Resource Management

```bash

# Limit concurrent operations
MAX_CONCURRENT=5
for i in $(seq 1 10); do
    curl -X POST http://localhost:3000 \
      -H "Content-Type: application/json" \
      -d '{"jsonrpc":"2.0","id":'$i',"method":"tools/call","params":{"name":"list_directory","arguments":{"path":"/tmp"}}}' &
    if [ $(jobs | wc -l) -ge $MAX_CONCURRENT ]; then
        wait
    fi
done
wait
```

### 3. Connection Pooling

```python

# connection_pool.py
import requests
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry

class MCPConnectionPool:
    def __init__(self, base_url="http://localhost:3000"):
        self.base_url = base_url
        self.session = requests.Session()

        # Configure retries
        retry_strategy = Retry(
            total=3,
            backoff_factor=1,
            status_forcelist=[429, 500, 502, 503, 504]
        )

        adapter = HTTPAdapter(max_retries=retry_strategy)
        self.session.mount("http://", adapter)
        self.session.mount("https://", adapter)

    def call_tool(self, tool_name, arguments):
        payload = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments
            }
        }
        response = self.session.post(self.base_url, json=payload)
        return response.json()

# Usage
pool = MCPConnectionPool()
results = pool.call_tool("list_directory", {"path": "/path/to/dir"})
```

### 4. Monitoring Integration

```bash

# Export metrics for monitoring
curl -s http://localhost:3000/metrics | grep -E "(request_total|request_duration|memory_usage)"

# Log analysis
tail -f /var/log/mcp-filesystem.log | jq 'select(.level == "error")'
```

---

## 📚 Additional Resources

- [Configuration Reference](CONFIGURATION.md) - Detailed configuration options
- [Performance Guide](PERFORMANCE.md) - Performance tuning and optimization
- [Deployment Guide](DEPLOYMENT.md) - Production deployment strategies
- [Troubleshooting Guide](TROUBLESHOOTING.md) - Common issues and solutions

This guide covers the most common use cases and patterns. For advanced scenarios or custom integrations, refer to the MCP specification and the rust-mcp-filesystem source code.
