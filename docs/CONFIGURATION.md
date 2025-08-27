# Configuration Reference

<content"># Configuration Reference

This document provides a complete reference for configuring rust-mcp-filesystem, including all available environment variables, performance tuning options, and best practices.

## 🔧 Environment Variables

### Performance Configuration

### Hardware Acceleration

```bash

# Enable/disable SIMD optimizations (AVX2/SSE4.2)
MCP_ENABLE_SIMD=true|false
# Default: true (auto-detected based on hardware support)

# Legacy alias (still supported)
MCP_ENABLE_HARDWARE_ACCELERATION=true|false
```

### Memory Management

```bash

# Maximum file size for memory mapping (bytes)
MCP_MAX_MEMORY_MAP_SIZE=524288000
# Default: 500MB
# Values: 1MB to 10GB (automatically capped)

# Memory usage limit (bytes, 0 = no limit)
MCP_MEMORY_LIMIT=1073741824
# Default: 0 (no limit)
# Example: 1073741824 = 1GB
```

### Compression

```bash

# Enable/disable LZ4 compression
MCP_ENABLE_COMPRESSION=true|false
# Default: true

# Minimum size threshold for compression (bytes)
MCP_COMPRESSION_THRESHOLD=4096
# Default: 4096 (4KB)
# Minimum: 1024 (1KB)
```

### Parallel Processing

```bash

# Enable/disable parallel processing
MCP_ENABLE_PARALLEL=true|false
# Default: true

# Number of parallel workers (0 = auto-detect CPU cores)
MCP_MAX_WORKERS=8
# Default: 0 (auto-detect)
# Maximum: 2x CPU cores (automatically capped)

# Minimum file size for parallel processing (bytes)
MCP_PARALLEL_THRESHOLD=10000
# Default: 10000 (10KB)
# Minimum: 1000 (1KB)
```

### Streaming & I/O

```bash

# Enable/disable streaming for large files
MCP_ENABLE_STREAMING=true|false
# Default: true

# Streaming threshold (automatically set to 100MB)
# Note: This is not configurable via environment variable
```

### Monitoring & Logging

```bash

# Enable detailed performance logging
MCP_PERFORMANCE_LOGGING=true|false
# Default: false

# Logging level (affects all components)
RUST_LOG=debug|info|warn|error
# Default: info
```

## 📊 Configuration Examples

### High-Performance Setup

```bash

# Maximum performance for powerful machines
export MCP_ENABLE_SIMD=true
export MCP_ENABLE_COMPRESSION=true
export MCP_ENABLE_PARALLEL=true
export MCP_MAX_MEMORY_MAP_SIZE=1073741824  # 1GB
export MCP_MAX_WORKERS=16
export MCP_MEMORY_LIMIT=8589934592  # 8GB
export MCP_PERFORMANCE_LOGGING=true
```

### Resource-Constrained Environment

```bash

# Minimal resource usage
export MCP_ENABLE_SIMD=false
export MCP_ENABLE_COMPRESSION=false
export MCP_MAX_MEMORY_MAP_SIZE=104857600  # 100MB
export MCP_MAX_WORKERS=2
export MCP_MEMORY_LIMIT=536870912  # 512MB
export MCP_PERFORMANCE_LOGGING=false
```

### Balanced Configuration

```bash

# Good balance of performance and resource usage
export MCP_ENABLE_SIMD=true
export MCP_ENABLE_COMPRESSION=true
export MCP_ENABLE_PARALLEL=true
export MCP_MAX_MEMORY_MAP_SIZE=524288000  # 500MB
export MCP_MAX_WORKERS=0  # Auto-detect
export MCP_MEMORY_LIMIT=2147483648  # 2GB
export MCP_PERFORMANCE_LOGGING=false
```

## 🏗️ Hardware-Specific Configuration

The server automatically detects and adapts to your hardware capabilities:

### x86_64 Systems with AVX2

```bash

# Automatically enabled when AVX2 is detected
MCP_ENABLE_SIMD=true  # Automatically set
```

### ARM64/Other Architectures

```bash

# SIMD optimizations disabled
MCP_ENABLE_SIMD=false  # Automatically set
```

### Multi-Core Systems

```bash

# Worker count automatically adjusted
MCP_MAX_WORKERS=0  # Uses CPU core count
```

### High-Memory Systems (16GB+)

```bash

# Larger memory mapping threshold
MCP_MAX_MEMORY_MAP_SIZE=1073741824  # 1GB (auto-adjusted)
```

### Low-Memory Systems (< 8GB)

```bash

# Smaller memory mapping threshold
MCP_MAX_MEMORY_MAP_SIZE=104857600  # 100MB (auto-adjusted)
```

## 🔍 Validation & Warnings

The configuration system includes automatic validation and helpful warnings:

### Automatic Adjustments

- **Memory Map Size**: Capped at 10GB maximum, adjusted to fit memory limits

- **Worker Count**: Limited to 2x CPU cores maximum
- **Compression Threshold**: Minimum 1KB
- **Parallel Threshold**: Minimum 1KB

### Warning Messages

The server provides helpful warnings for:

- Invalid boolean values (with fallback to defaults)
- Invalid size values (with fallback to defaults)
- Worker count too high (automatically capped)
- Memory limits exceeded (automatic adjustment)

Example warning output:

```text
Warning: Invalid boolean value for MCP_ENABLE_STREAMING: 'invalid', using default: true

Warning: MCP_MAX_WORKERS value too high: 32, capping at 16
Warning: Adjusted memory map size to fit within memory limit
```

## 📈 Performance Tuning Guide

### For Large Files (>100MB)

```bash

export MCP_ENABLE_STREAMING=true
export MCP_MAX_MEMORY_MAP_SIZE=1073741824  # 1GB
export MCP_ENABLE_COMPRESSION=true
```

### For Many Small Files

```bash

export MCP_ENABLE_PARALLEL=true
export MCP_MAX_WORKERS=0  # Use all CPU cores
export MCP_PARALLEL_THRESHOLD=1000  # 1KB
```

### For Code Analysis Workloads

```bash

export MCP_ENABLE_SIMD=true
export MCP_MAX_MEMORY_MAP_SIZE=536870912  # 512MB
export MCP_MAX_WORKERS=8
export MCP_PERFORMANCE_LOGGING=true
```

### For Network-Constrained Environments

```bash

export MCP_ENABLE_COMPRESSION=true
export MCP_COMPRESSION_THRESHOLD=2048  # 2KB
```

## 🐳 Docker Configuration

```yaml
# docker-compose.yml
version: '3.8'
services:
  mcp-filesystem:
    image: rust-mcp-filesystem:latest
    environment:
      - MCP_ENABLE_SIMD=true
      - MCP_ENABLE_COMPRESSION=true
      - MCP_MAX_MEMORY_MAP_SIZE=536870912
      - MCP_MAX_WORKERS=4
      - MCP_MEMORY_LIMIT=2147483648
      - RUST_LOG=info
    volumes:
      - ./allowed_paths:/app/allowed_paths:ro
```

```dockerfile
# Dockerfile
FROM rust:1.70-slim AS builder
# Build process...

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/rust-mcp-filesystem /usr/local/bin/
EXPOSE 3000
ENV MCP_ENABLE_SIMD=true
ENV MCP_ENABLE_COMPRESSION=true
ENV MCP_MAX_WORKERS=0
CMD ["rust-mcp-filesystem", "--port", "3000"]
```

## ☸️ Kubernetes Configuration

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mcp-filesystem
spec:
  replicas: 2
  selector:
    matchLabels:
      app: mcp-filesystem
  template:
    metadata:
      labels:
        app: mcp-filesystem
    spec:
      containers:
      - name: mcp-filesystem
        image: rust-mcp-filesystem:latest
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
        env:
        - name: MCP_ENABLE_SIMD
          value: "true"
        - name: MCP_ENABLE_COMPRESSION
          value: "true"
        - name: MCP_MAX_WORKERS
          value: "2"
        - name: MCP_MEMORY_LIMIT
          value: "1073741824"  # 1GB
        - name: RUST_LOG
          value: "info"
        ports:
        - containerPort: 3000
```

## 🔒 Security Considerations

### Safe Defaults

- **Read-only by default**: No write access unless explicitly enabled

- **Path validation**: All paths are validated against allowed directories
- **Memory limits**: Prevent excessive memory usage
- **Resource controls**: CPU and memory limits in containerized environments

### Security-Related Configuration

```bash

# Disable write access (recommended for read-only use cases)
# Note: This is a command-line flag, not an environment variable
rust-mcp-filesystem --read-only

# Limit memory usage to prevent DoS
export MCP_MEMORY_LIMIT=536870912  # 512MB

# Limit parallel workers to prevent resource exhaustion
export MCP_MAX_WORKERS=4
```

## 📊 Monitoring Configuration

### Performance Metrics

Enable detailed performance monitoring:

```bash
export MCP_PERFORMANCE_LOGGING=true
export RUST_LOG=debug
```

### Structured Logging

```bash

export RUST_LOG=json
# Output: {"timestamp":"2024-01-01T12:00:00Z","level":"INFO","message":"Operation completed","operation":"list_directory","duration_ms":150}
```

## 🧪 Testing Configuration

For testing and development:

```bash
# Disable optimizations for deterministic testing
export MCP_ENABLE_SIMD=false
export MCP_ENABLE_COMPRESSION=false
export MCP_ENABLE_PARALLEL=false

# Enable verbose logging
export MCP_PERFORMANCE_LOGGING=true
export RUST_LOG=debug
```

## 🚀 Production Checklist

Before deploying to production:

- [ ] Review hardware capabilities and adjust configuration accordingly
- [ ] Set appropriate memory limits based on available RAM
- [ ] Configure worker count based on CPU cores
- [ ] Enable compression for network transfers
- [ ] Set up monitoring and alerting
- [ ] Test configuration with realistic workloads
- [ ] Document your configuration for future reference

## 🐛 Troubleshooting

### Common Issues

### High Memory Usage

```bash

# Reduce memory mapping threshold
export MCP_MAX_MEMORY_MAP_SIZE=104857600  # 100MB

# Set memory limit
export MCP_MEMORY_LIMIT=536870912  # 512MB
```

### Slow Performance

```bash

# Enable SIMD if available
export MCP_ENABLE_SIMD=true

# Increase worker count
export MCP_MAX_WORKERS=0  # Auto-detect

# Enable compression for network transfers
export MCP_ENABLE_COMPRESSION=true
```

### Thread Contention

```bash

# Reduce worker count
export MCP_MAX_WORKERS=4

# Disable parallel processing for small files
export MCP_PARALLEL_THRESHOLD=50000  # 50KB
```

### Compression Errors

```bash

# Disable compression
export MCP_ENABLE_COMPRESSION=false

# Increase compression threshold
export MCP_COMPRESSION_THRESHOLD=8192  # 8KB
```

---

**Note**: Configuration changes require restarting the server to take effect. Use `cargo run` or your deployment method to restart with new environment variables.
