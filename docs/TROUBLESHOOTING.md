# Troubleshooting Guide

<content"># Troubleshooting Guide

This guide helps you diagnose and resolve common issues with rust-mcp-filesystem.

## 🔍 Quick Diagnosis

### Check Service Health

```bash

# Health check
curl -f http://localhost:3000/health

# Version information
curl http://localhost:3000/version

# Performance metrics (if enabled)
curl http://localhost:3000/metrics
```

### View Logs

```bash

# Docker logs
docker logs mcp-filesystem

# Kubernetes logs
kubectl logs -l app=mcp-filesystem

# Systemd logs
journalctl -u mcp-filesystem -f

# Application logs with debug level
export RUST_LOG=debug
./target/release/rust-mcp-filesystem
```

## 🚨 Common Issues & Solutions

### 1. Service Won't Start

### Permission Denied

**Symptoms:**

```text
thread 'main' panicked at 'Permission denied (os error 13)'
```

**Solutions:**

```bash
# Check file permissions
ls -la /path/to/data

# Fix permissions
chmod 755 /path/to/data
chown -R mcp:mcp /path/to/data

# For Docker, ensure proper user mapping
docker run --user $(id -u):$(id -g) rust-mcp-filesystem:latest
```

### Port Already in Use

**Symptoms:**

```text
Error: Address already in use (os error 98)
```

**Solutions:**

```bash
# Find process using port
netstat -tlnp | grep 3000
ss -tlnp | grep 3000

# Kill conflicting process
sudo fuser -k 3000/tcp

# Use different port
./rust-mcp-filesystem --port 3001
```

### Invalid Configuration

**Symptoms:**

```text
Warning: Invalid boolean value for MCP_ENABLE_STREAMING: 'invalid', using default: true
```

**Solutions:**

```bash
# Check environment variables
env | grep MCP_

# Validate configuration values
export MCP_ENABLE_STREAMING=true
export MCP_MAX_WORKERS=4
```

### 2. Performance Issues

### High Memory Usage

**Symptoms:**

- Container restarts due to OOM
- System memory usage >80%

**Solutions:**

```bash
# Reduce memory mapping threshold
export MCP_MAX_MEMORY_MAP_SIZE=104857600  # 100MB

# Set memory limit
export MCP_MEMORY_LIMIT=536870912  # 512MB

# Disable memory-intensive features
export MCP_ENABLE_SIMD=false
```

### Slow Response Times

**Symptoms:**

- Response time >500ms
- High CPU usage

**Solutions:**

```bash
# Enable SIMD acceleration
export MCP_ENABLE_SIMD=true

# Optimize worker count
export MCP_MAX_WORKERS=0  # Auto-detect CPU cores

# Enable parallel processing
export MCP_ENABLE_PARALLEL=true
export MCP_PARALLEL_THRESHOLD=5000  # 5KB
```

### High CPU Usage

**Symptoms:**

- CPU usage >90%
- System becomes unresponsive

**Solutions:**

```bash
# Reduce worker count
export MCP_MAX_WORKERS=2

# Disable heavy optimizations
export MCP_ENABLE_SIMD=false

# Increase parallel threshold
export MCP_PARALLEL_THRESHOLD=50000  # 50KB
```

### 3. File Operation Errors

### File Not Found

**Symptoms:**

```text
Error: No such file or directory (os error 2)
```

**Solutions:**

```bash
# Check file existence
ls -la /path/to/file

# Verify allowed directories
./rust-mcp-filesystem --list-allowed-directories

# Add directory to allowed paths
./rust-mcp-filesystem --allowed-directory /path/to/data
```

### Permission Denied on File Access

**Symptoms:**

```text
Error: Permission denied (os error 13)
```

**Solutions:**

```bash
# Check file permissions
ls -la /path/to/file

# Fix read permissions
chmod +r /path/to/file

# For directories
chmod +rx /path/to/directory
find /path/to/directory -type f -exec chmod +r {} \;
```

### Large File Handling Issues

**Symptoms:**

- Timeout on large file operations
- Memory exhaustion

**Solutions:**

```bash
# Enable streaming for large files
export MCP_ENABLE_STREAMING=true

# Adjust memory mapping size
export MCP_MAX_MEMORY_MAP_SIZE=1073741824  # 1GB

# Increase timeout (if supported)
export MCP_OPERATION_TIMEOUT=300  # 5 minutes
```

### 4. Network & Connection Issues

### Connection Refused

**Symptoms:**

```text
curl: (7) Failed to connect to localhost port 3000: Connection refused
```

**Solutions:**

```bash
# Check if service is running
ps aux | grep rust-mcp-filesystem

# Check port binding
netstat -tlnp | grep 3000

# Restart service
systemctl restart mcp-filesystem
docker restart mcp-filesystem
```

### Timeout Errors

**Symptoms:**

```text
Error: Request timeout
```

**Solutions:**

```bash
# Increase timeout settings
export MCP_REQUEST_TIMEOUT=60  # 60 seconds

# Check network connectivity
ping -c 3 localhost

# Check firewall rules
sudo ufw status
sudo iptables -L
```

### 5. Docker-Specific Issues

### Container Exits Immediately

**Symptoms:**

```text
docker run exits with code 1
```

**Solutions:**

```bash
# Check container logs
docker logs <container_id>

# Run with interactive mode
docker run -it rust-mcp-filesystem:latest /bin/bash

# Check entrypoint
docker inspect rust-mcp-filesystem:latest | grep -A 10 "Entrypoint"
```

### Volume Mount Issues

**Symptoms:**

```text
docker: Error response from daemon: invalid mount config
```

**Solutions:**

```bash
# Check source directory exists
ls -la /host/path

# Fix mount syntax
docker run -v /host/path:/container/path:ro rust-mcp-filesystem:latest

# Use absolute paths
docker run -v $(pwd)/data:/data:ro rust-mcp-filesystem:latest
```

### 6. Kubernetes-Specific Issues

### Pod CrashLoopBackOff

**Symptoms:**

```text
kubectl get pods shows CrashLoopBackOff
```

**Solutions:**

```bash
# Check pod logs
kubectl logs <pod-name> --previous

# Describe pod for events
kubectl describe pod <pod-name>

# Check resource limits
kubectl get pod <pod-name> -o yaml | grep -A 10 resources
```

### Service Not Accessible

**Symptoms:**

```text
kubectl get svc shows service exists but can't connect
```

**Solutions:**

```bash
# Check service endpoints
kubectl get endpoints mcp-filesystem

# Check service selector
kubectl get svc mcp-filesystem -o yaml

# Verify pod labels
kubectl get pods --selector=app=mcp-filesystem
```

## 🛠️ Debugging Tools

### Performance Profiling

```bash

# Enable performance logging
export MCP_PERFORMANCE_LOGGING=true
export RUST_LOG=debug

# Run with profiling
cargo build --release --features profiling
./target/release/rust-mcp-filesystem --profile
```

### Memory Analysis

```bash

# Check memory usage
ps aux --sort=-%mem | head

# Monitor memory growth
watch -n 1 'ps aux --no-headers -o pmem,pcpu,pid,cmd | grep rust-mcp-filesystem'

# Use Valgrind (if available)
valgrind --tool=massif ./target/release/rust-mcp-filesystem
```

### Network Debugging

```bash

# Monitor network connections
netstat -tlnp | grep 3000
ss -tlnp | grep 3000

# Capture network traffic
tcpdump -i any port 3000 -w capture.pcap

# Check for dropped packets
netstat -s | grep "packet receive errors"
```

## 📊 Monitoring & Alerting

### Key Metrics to Monitor

```yaml

# Prometheus alerting rules
groups:
  - name: mcp-filesystem
    rules:
      - alert: HighMemoryUsage
        expr: process_resident_memory_bytes / 1024 / 1024 > 1000
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage detected"

      - alert: HighCPUUsage
        expr: rate(process_cpu_user_seconds_total[5m]) > 0.8
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High CPU usage detected"

      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) / rate(http_requests_total[5m]) > 0.1
        for: 5m
        labels:
          severity: error
        annotations:
          summary: "High error rate detected"
```

### Log Aggregation

```yaml

# Fluentd configuration for Kubernetes
<match mcp-filesystem.**>
  @type elasticsearch
  host elasticsearch
  port 9200
  logstash_format true
  <buffer>
    @type file
    path /var/log/fluentd/buffer
    flush_interval 5s
  </buffer>
</match>
```

## 🔧 Advanced Troubleshooting

### SIMD/AVX2 Issues

```bash

# Check CPU support
grep avx2 /proc/cpuinfo
lscpu | grep avx2

# Disable SIMD if causing issues
export MCP_ENABLE_SIMD=false

# Test with different optimization levels
export RUSTFLAGS="-C opt-level=2"
```

### Memory Mapping Problems

```bash

# Check system limits
sysctl vm.max_map_count

# Increase limits if needed
sudo sysctl -w vm.max_map_count=262144

# Disable memory mapping
export MCP_MAX_MEMORY_MAP_SIZE=0
```

### Parallel Processing Issues

```bash

# Check thread count
ps -T -p $(pgrep rust-mcp-filesystem) | wc -l

# Disable parallel processing
export MCP_ENABLE_PARALLEL=false

# Limit worker count
export MCP_MAX_WORKERS=1
```

## 📋 Diagnostic Checklist

**Before contacting support, please check:**

- [ ] Service is running and accessible
- [ ] Logs show no critical errors
- [ ] Configuration values are valid
- [ ] File permissions are correct
- [ ] Network connectivity is working
- [ ] System resources are sufficient
- [ ] Dependencies are installed
- [ ] Firewall rules allow traffic

## 🚨 Emergency Procedures

### Service Down - Immediate Action

```bash

# 1. Check service status
systemctl status mcp-filesystem

# 2. Restart service
systemctl restart mcp-filesystem

# 3. Check logs for errors
journalctl -u mcp-filesystem -n 50

# 4. Verify health endpoint
curl -f http://localhost:3000/health
```

### Data Corruption - Recovery Steps

```bash

# 1. Stop the service
systemctl stop mcp-filesystem

# 2. Backup current data
cp -r /path/to/data /path/to/data.backup

# 3. Check filesystem integrity
fsck /dev/sdXn

# 4. Restart service
systemctl start mcp-filesystem
```

### Memory Exhaustion - Quick Fix

```bash

# 1. Reduce memory settings
export MCP_MEMORY_LIMIT=268435456  # 256MB
export MCP_MAX_MEMORY_MAP_SIZE=52428800  # 50MB

# 2. Restart service
systemctl restart mcp-filesystem

# 3. Monitor memory usage
watch -n 5 'ps aux --no-headers -o pmem,pcpu,pid,cmd | grep rust-mcp-filesystem'
```

## 📞 Getting Help

### Community Support

- **GitHub Issues**: Report bugs and request features

- **Discussions**: Ask questions and share solutions
- **Discord/Slack**: Real-time community support

### Enterprise Support

- **Documentation**: Check docs/ folder for detailed guides

- **Performance Tuning**: See PERFORMANCE.md
- **Deployment Guide**: See DEPLOYMENT.md

### Debug Information

When reporting issues, please include:

```bash
# System information
uname -a
lsb_release -a

# Application version
./rust-mcp-filesystem --version

# Configuration
env | grep MCP_
env | grep RUST_

# Recent logs
tail -n 100 /var/log/mcp-filesystem.log
```

---

**Remember**: Most issues can be resolved by checking the logs and validating the configuration. The application provides detailed error messages to help with troubleshooting.
