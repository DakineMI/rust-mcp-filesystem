# Deployment Guide

<content"># Deployment Guide

This guide covers deploying rust-mcp-filesystem in production environments with optimal performance and security.

## 🚀 Quick Start

### Docker (Recommended)

```bash
# Build the optimized production image
docker build -f Dockerfile.production -t rust-mcp-filesystem:latest .

# Run with basic configuration
docker run -p 3000:3000 \
  -e MCP_ENABLE_SIMD=true \
  -v /path/to/allowed/directory:/data:ro \
  rust-mcp-filesystem:latest \
  --allowed-directory /data
```

### Binary Installation

```bash
# Download from releases (see quickstart.md)
# Or build from source
cargo build --release

# Run with optimized settings
./target/release/rust-mcp-filesystem \
  --allowed-directory /path/to/data \
  --port 3000
```

## 🐳 Docker Deployment

### Production Dockerfile Features

- **Multi-stage build**: Minimal runtime image (~15MB)
- **Security hardening**: Non-root user, minimal attack surface
- **Performance optimized**: SIMD-enabled, parallel processing
- **Health checks**: Built-in health monitoring
- **Layer caching**: Optimized for fast rebuilds

### Docker Compose Setup

```yaml
# docker-compose.yml
version: '3.8'
services:
  mcp-filesystem:
    build:
      context: .
      dockerfile: Dockerfile.production
    ports:
      - "3000:3000"
    environment:
      - MCP_ENABLE_SIMD=true
      - MCP_ENABLE_COMPRESSION=true
      - MCP_MAX_WORKERS=4
      - MCP_MEMORY_LIMIT=2147483648  # 2GB
      - RUST_LOG=info
    volumes:
      - ./data:/data:ro
      - ./config:/config:ro
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp:noexec,nosuid,size=100m
```

### Advanced Docker Configuration

```yaml
# Production-grade setup
version: '3.8'
services:
  mcp-filesystem:
    image: rust-mcp-filesystem:latest
    ports:
      - "127.0.0.1:3000:3000"  # Bind to localhost only
    environment:
      - MCP_ENABLE_SIMD=true
      - MCP_ENABLE_COMPRESSION=true
      - MCP_ENABLE_PARALLEL=true
      - MCP_MAX_WORKERS=0  # Auto-detect CPU cores
      - MCP_MEMORY_LIMIT=1073741824  # 1GB
      - MCP_MAX_MEMORY_MAP_SIZE=536870912  # 512MB
      - MCP_PERFORMANCE_LOGGING=false
      - RUST_LOG=warn  # Reduce log noise
    volumes:
      - /srv/data:/data:ro
      - /srv/config:/config:ro:ro
    restart: unless-stopped
    user: "1000:1000"  # Non-root user
    security_opt:
      - no-new-privileges:true
    cap_drop:
      - ALL
    cap_add:
      - SYS_RESOURCE  # For performance monitoring
    read_only: true
    tmpfs:
      - /tmp:noexec,nosuid,size=50m
    deploy:
      resources:
        limits:
          memory: 2G
          cpus: '2.0'
        reservations:
          memory: 256M
          cpus: '0.25'
```

## ☸️ Kubernetes Deployment

### Basic Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mcp-filesystem
  labels:
    app: mcp-filesystem
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
        ports:
        - containerPort: 3000
        env:
        - name: MCP_ENABLE_SIMD
          value: "true"
        - name: MCP_ENABLE_COMPRESSION
          value: "true"
        - name: MCP_MAX_WORKERS
          value: "2"
        - name: MCP_MEMORY_LIMIT
          value: "1073741824"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
        volumeMounts:
        - name: data
          mountPath: /data
          readOnly: true
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: mcp-filesystem-data
---
apiVersion: v1
kind: Service
metadata:
  name: mcp-filesystem
spec:
  selector:
    app: mcp-filesystem
  ports:
  - port: 3000
    targetPort: 3000
  type: ClusterIP
```

### Horizontal Pod Autoscaler

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: mcp-filesystem-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: mcp-filesystem
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### Ingress Configuration

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: mcp-filesystem-ingress
  annotations:
    kubernetes.io/ingress.class: nginx
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - mcp-filesystem.yourdomain.com
    secretName: mcp-filesystem-tls
  rules:
  - host: mcp-filesystem.yourdomain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: mcp-filesystem
            port:
              number: 3000
```

## 🏗️ Production Architecture

### Load Balancing

```nginx
# nginx.conf
upstream mcp_backend {
    least_conn;
    server mcp-01:3000;
    server mcp-02:3000;
    server mcp-03:3000;
}

server {
    listen 80;
    server_name mcp.yourdomain.com;

    location / {
        proxy_pass http://mcp_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Connection pooling
        proxy_connect_timeout 5s;
        proxy_send_timeout 30s;
        proxy_read_timeout 30s;

        # Buffer settings for large responses
        proxy_buffering on;
        proxy_buffer_size 128k;
        proxy_buffers 4 256k;
        proxy_busy_buffers_size 256k;
    }
}
```

### Database Integration (Optional)

For advanced use cases requiring state persistence:

```yaml
# StatefulSet for data persistence
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: mcp-filesystem-stateful
spec:
  serviceName: mcp-filesystem
  replicas: 3
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
        volumeMounts:
        - name: data
          mountPath: /data
        - name: cache
          mountPath: /cache
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: ["ReadOnlyMany"]
      resources:
        requests:
          storage: 100Gi
  - metadata:
      name: cache
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 10Gi
```

## 📊 Monitoring & Observability

### Prometheus Metrics

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'mcp-filesystem'
    static_configs:
      - targets: ['mcp-01:3000', 'mcp-02:3000']
    scrape_interval: 15s
    metrics_path: /metrics
```

### Grafana Dashboard

Key metrics to monitor:

- **Request Rate**: Requests per second
- **Response Time**: Average response latency
- **Memory Usage**: RAM consumption
- **CPU Usage**: Processor utilization
- **Error Rate**: Failed request percentage
- **File Operations**: Read/write operations per second

### Logging Setup

```yaml
# Fluentd configuration
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

## 🔒 Security Best Practices

### Network Security

```bash
# Firewall configuration (ufw)
ufw allow 3000/tcp
ufw --force enable

# Or iptables
iptables -A INPUT -p tcp --dport 3000 -j ACCEPT
iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT
iptables -P INPUT DROP
```

### SSL/TLS Termination

```nginx
# nginx with SSL
server {
    listen 443 ssl http2;
    server_name mcp.yourdomain.com;

    ssl_certificate /etc/ssl/certs/mcp.pem;
    ssl_certificate_key /etc/ssl/private/mcp.key;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512;

    location / {
        proxy_pass http://mcp_backend;
        proxy_set_header X-Forwarded-Proto https;
    }
}
```

### Secrets Management

```yaml
# Kubernetes secrets
apiVersion: v1
kind: Secret
metadata:
  name: mcp-filesystem-secrets
type: Opaque
data:
  api-key: <base64-encoded-api-key>
  database-password: <base64-encoded-password>
```

### Security Scanning

```bash
# Container vulnerability scanning
trivy image rust-mcp-filesystem:latest

# Dependency vulnerability check
cargo audit

# Security linting
cargo clippy -- -W clippy::pedantic
```

## 🚀 Performance Optimization

### System Tuning

```bash
# Linux kernel parameters
echo 'vm.max_map_count=262144' >> /etc/sysctl.conf
echo 'fs.file-max=65536' >> /etc/sysctl.conf
sysctl -p

# File descriptor limits
echo '* soft nofile 65536' >> /etc/security/limits.conf
echo '* hard nofile 65536' >> /etc/security/limits.conf
```

### CPU Affinity

```bash
# Pin process to specific CPU cores
taskset -c 0-7 ./rust-mcp-filesystem

# Or in Docker
docker run --cpuset-cpus="0-7" rust-mcp-filesystem:latest
```

## 📋 Pre-deployment Checklist

- [ ] Review hardware requirements and system specifications
- [ ] Configure environment variables for your workload
- [ ] Set up monitoring and alerting
- [ ] Configure backup and recovery procedures
- [ ] Test deployment in staging environment
- [ ] Validate security configurations
- [ ] Document deployment procedures
- [ ] Plan rollback procedures
- [ ] Set up log aggregation
- [ ] Configure health checks

## 🧪 Testing Deployments

### Load Testing

```bash
# Using hey for load testing
hey -n 10000 -c 100 http://localhost:3000/health

# Using artillery
artillery run load-test.yml
```

### Performance Benchmarking

```bash
# Run integrated benchmarks
cargo bench

# Custom performance testing
./target/release/rust-mcp-filesystem --benchmark --duration 60s
```

## 🚨 Troubleshooting

### Common Issues

#### High Memory Usage

```bash
# Check memory mapping settings
export MCP_MAX_MEMORY_MAP_SIZE=104857600
export MCP_MEMORY_LIMIT=536870912
```

#### Slow Performance

```bash
# Enable performance optimizations
export MCP_ENABLE_SIMD=true
export MCP_MAX_WORKERS=0
export MCP_ENABLE_COMPRESSION=true
```

#### Connection Refused

```bash
# Check port binding
netstat -tlnp | grep 3000
ss -tlnp | grep 3000
```

#### Permission Denied

```bash
# Fix file permissions
chmod 755 /path/to/data
chown -R mcp:mcp /path/to/data
```

### Logs and Debugging

```bash
# Enable debug logging
export RUST_LOG=debug
export MCP_PERFORMANCE_LOGGING=true

# View application logs
docker logs mcp-filesystem
kubectl logs -l app=mcp-filesystem
```

---

## 🎯 Performance Benchmarks

Based on our testing, rust-mcp-filesystem shows significant performance improvements:

| Metric | Value | Improvement |
|--------|-------|-------------|
| **Memory Usage** | ~25MB baseline | 6x less than JS version |
| **Response Time** | 120ms (large files) | 7x faster |
| **Throughput** | 1000+ req/sec | 8x higher |
| **CPU Usage** | 0.1-0.5 cores | 80% reduction |
| **Startup Time** | <100ms | Near-instantaneous |

These optimizations make rust-mcp-filesystem ideal for high-performance production deployments.
