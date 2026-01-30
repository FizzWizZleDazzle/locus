# Deployment Guide

Production deployment guide for Locus.

## Overview

This guide covers deploying Locus to production with:
- Backend API (Axum)
- Frontend (WASM/static files)
- PostgreSQL database
- HTTPS with SSL/TLS

---

## Production Architecture

```
                    ┌──────────────┐
                    │   DNS/CDN    │
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │ Load Balancer│
                    │   (HTTPS)    │
                    └──────┬───────┘
                           │
        ┌──────────────────┴──────────────────┐
        │                                     │
   ┌────▼────┐                         ┌─────▼─────┐
   │ Backend │                         │  Frontend │
   │ (Axum)  │                         │  (Static) │
   └────┬────┘                         └───────────┘
        │
   ┌────▼────┐
   │PostgreSQL│
   │ Database │
   └─────────┘
```

---

## Prerequisites

### Required

- Linux server (Ubuntu 22.04 LTS recommended)
- Domain name
- SSL certificate (Let's Encrypt)
- PostgreSQL 16
- Rust toolchain (if building on server)

### Recommended

- Reverse proxy (Nginx/Caddy)
- Process manager (systemd)
- Monitoring (Prometheus, Grafana)
- Log aggregation (ELK stack)

---

## Building for Production

### Backend

```bash
# On development machine or CI/CD
cargo build --release -p locus-backend

# Binary location
target/release/locus-backend
```

**Size:** ~15MB (release build with debug symbols stripped)

**Optimize further:**
```toml
# In Cargo.toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Strip symbols
```

### Frontend

```bash
cd crates/frontend
trunk build --release
```

**Output directory:** `crates/frontend/dist/`

**Files:**
- `index.html` - Entry point
- `*.wasm` - WebAssembly binary
- `*.js` - JavaScript glue code
- `*.css` - Stylesheets (if any)

**Optimize:**
- Trunk automatically runs wasm-opt
- Gzip/Brotli compression on server
- CDN for static assets

---

## Database Setup

### Option 1: Managed PostgreSQL

**Recommended for production:**
- AWS RDS
- Google Cloud SQL
- DigitalOcean Managed Databases
- Heroku Postgres

**Benefits:**
- Automatic backups
- High availability
- Managed updates
- Connection pooling
- Monitoring

### Option 2: Self-Hosted PostgreSQL

**Install PostgreSQL 16:**
```bash
# Ubuntu
sudo apt update
sudo apt install postgresql-16 postgresql-contrib-16

# Start service
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

**Create database and user:**
```bash
sudo -u postgres psql

postgres=# CREATE DATABASE locus;
postgres=# CREATE USER locus_prod WITH ENCRYPTED PASSWORD 'STRONG_PASSWORD_HERE';
postgres=# GRANT ALL PRIVILEGES ON DATABASE locus TO locus_prod;
postgres=# \q
```

**Configure PostgreSQL:**
```bash
sudo nano /etc/postgresql/16/main/postgresql.conf
```

```
max_connections = 100
shared_buffers = 256MB
effective_cache_size = 1GB
maintenance_work_mem = 64MB
checkpoint_completion_target = 0.9
wal_buffers = 16MB
default_statistics_target = 100
random_page_cost = 1.1
effective_io_concurrency = 200
```

**Allow connections:**
```bash
sudo nano /etc/postgresql/16/main/pg_hba.conf
```

```
# Add line (replace with your backend IP)
host    locus    locus_prod    10.0.0.0/24    scram-sha-256
```

**Restart PostgreSQL:**
```bash
sudo systemctl restart postgresql
```

---

## Server Setup

### 1. Prepare Server

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y build-essential pkg-config libssl-dev libpq-dev

# Create deployment user
sudo useradd -m -s /bin/bash locus
sudo su - locus
```

### 2. Upload Backend Binary

```bash
# On local machine
scp target/release/locus-backend locus@yourserver.com:/home/locus/

# On server
chmod +x /home/locus/locus-backend
```

### 3. Create Environment File

```bash
# On server
cat > /home/locus/.env << EOF
DATABASE_URL=postgres://locus_prod:STRONG_PASSWORD@localhost/locus
HOST=0.0.0.0
PORT=3000
JWT_SECRET=$(openssl rand -base64 32)
JWT_EXPIRY_HOURS=168
RUST_LOG=info
EOF

chmod 600 /home/locus/.env
```

### 4. Run Migrations

```bash
# First run will apply migrations
/home/locus/locus-backend
```

---

## Process Management (systemd)

### Backend Service

```bash
sudo nano /etc/systemd/system/locus-backend.service
```

```ini
[Unit]
Description=Locus Backend API
After=network.target postgresql.service
Requires=postgresql.service

[Service]
Type=simple
User=locus
WorkingDirectory=/home/locus
EnvironmentFile=/home/locus/.env
ExecStart=/home/locus/locus-backend
Restart=always
RestartSec=10

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/home/locus

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
```

**Enable and start:**
```bash
sudo systemctl daemon-reload
sudo systemctl enable locus-backend
sudo systemctl start locus-backend
sudo systemctl status locus-backend
```

**View logs:**
```bash
sudo journalctl -u locus-backend -f
```

---

## Reverse Proxy (Nginx)

### Install Nginx

```bash
sudo apt install -y nginx
```

### Configure Site

```bash
sudo nano /etc/nginx/sites-available/locus
```

```nginx
# API Backend
upstream backend {
    server 127.0.0.1:3000;
    keepalive 32;
}

server {
    listen 80;
    server_name api.yourdomain.com;

    # Redirect to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;

    # SSL Configuration
    ssl_certificate /etc/letsencrypt/live/api.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.yourdomain.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    # Security Headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # API Proxy
    location /api {
        proxy_pass http://backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;

        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }

    # Health check
    location /health {
        proxy_pass http://backend/api/health;
        access_log off;
    }
}

# Frontend
server {
    listen 80;
    server_name yourdomain.com www.yourdomain.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name yourdomain.com www.yourdomain.com;

    # SSL Configuration
    ssl_certificate /etc/letsencrypt/live/yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/yourdomain.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    # Root directory
    root /var/www/locus;
    index index.html;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript application/javascript application/xml+rss application/json application/wasm;

    # Cache static assets
    location ~* \.(wasm|js|css|png|jpg|jpeg|gif|ico|svg)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # SPA routing
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}
```

### Deploy Frontend Files

```bash
# Create directory
sudo mkdir -p /var/www/locus
sudo chown locus:locus /var/www/locus

# Upload files (from local machine)
scp -r crates/frontend/dist/* locus@yourserver.com:/var/www/locus/
```

### Update Frontend API URL

**Edit `index.html` before building:**
```html
<script>
    // Change API base URL for production
    window.API_BASE_URL = 'https://api.yourdomain.com';
</script>
```

**Or use environment-based configuration in Rust code.**

### Enable Site

```bash
sudo ln -s /etc/nginx/sites-available/locus /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## SSL Certificates (Let's Encrypt)

### Install Certbot

```bash
sudo apt install -y certbot python3-certbot-nginx
```

### Obtain Certificates

```bash
# API domain
sudo certbot --nginx -d api.yourdomain.com

# Frontend domain
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com
```

### Auto-Renewal

```bash
# Test renewal
sudo certbot renew --dry-run

# Certbot installs auto-renewal via systemd timer
sudo systemctl status certbot.timer
```

---

## Environment Configuration

### Production .env

```bash
# Database
DATABASE_URL=postgres://locus_prod:STRONG_PASSWORD@localhost/locus

# Server
HOST=0.0.0.0
PORT=3000

# JWT (IMPORTANT: Use strong random secret)
JWT_SECRET=$(openssl rand -base64 48)
JWT_EXPIRY_HOURS=168  # 7 days

# Logging
RUST_LOG=warn
```

**Security checklist:**
- [ ] Use strong database password (20+ characters)
- [ ] Use random JWT secret (32+ bytes)
- [ ] Never commit .env to git
- [ ] Restrict file permissions (600)
- [ ] Use environment variables, not hardcoded values

---

## Database Backups

### Automated Daily Backups

```bash
sudo nano /usr/local/bin/backup-locus-db.sh
```

```bash
#!/bin/bash
BACKUP_DIR="/var/backups/locus"
DATE=$(date +%Y%m%d_%H%M%S)
mkdir -p $BACKUP_DIR

# Backup database
pg_dump -h localhost -U locus_prod locus | gzip > $BACKUP_DIR/locus_$DATE.sql.gz

# Keep only last 30 days
find $BACKUP_DIR -name "locus_*.sql.gz" -mtime +30 -delete

echo "Backup completed: locus_$DATE.sql.gz"
```

```bash
chmod +x /usr/local/bin/backup-locus-db.sh
```

**Crontab:**
```bash
sudo crontab -e
```

```
# Daily backup at 2 AM
0 2 * * * /usr/local/bin/backup-locus-db.sh
```

### Manual Backup

```bash
pg_dump -h localhost -U locus_prod locus > backup.sql
```

### Restore from Backup

```bash
psql -h localhost -U locus_prod locus < backup.sql
```

---

## Monitoring

### Basic Health Checks

**Endpoint:** `GET /api/health`

**Uptime monitoring:**
- UptimeRobot
- Pingdom
- StatusCake

### Logging

**View backend logs:**
```bash
sudo journalctl -u locus-backend -f
```

**Log rotation:**
```bash
sudo nano /etc/systemd/journald.conf
```

```ini
[Journal]
SystemMaxUse=500M
SystemKeepFree=1G
MaxRetentionSec=7day
```

### Metrics (Future)

**Prometheus + Grafana:**
- Request counts
- Response times
- Error rates
- Database connections
- ELO distribution

---

## Security Hardening

### Firewall (UFW)

```bash
# Install
sudo apt install -y ufw

# Allow SSH
sudo ufw allow 22/tcp

# Allow HTTP/HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Allow PostgreSQL (only from backend)
sudo ufw allow from 10.0.0.0/24 to any port 5432

# Enable
sudo ufw enable
sudo ufw status
```

### Fail2Ban

```bash
# Install
sudo apt install -y fail2ban

# Configure
sudo nano /etc/fail2ban/jail.local
```

```ini
[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 5

[sshd]
enabled = true
```

```bash
sudo systemctl restart fail2ban
```

### Regular Updates

```bash
# Automated security updates
sudo apt install -y unattended-upgrades
sudo dpkg-reconfigure unattended-upgrades
```

---

## Performance Optimization

### Database Connection Pool

**In backend code:**
```rust
PgPoolOptions::new()
    .max_connections(20)  // Adjust based on load
    .connect(&database_url)
    .await
```

### Nginx Caching

```nginx
# In http block
proxy_cache_path /var/cache/nginx levels=1:2 keys_zone=api_cache:10m max_size=100m inactive=60m;

# In location block
location /api/leaderboard {
    proxy_cache api_cache;
    proxy_cache_valid 200 5m;
    proxy_pass http://backend;
}
```

### CDN for Static Assets

**Cloudflare, AWS CloudFront, or Fastly:**
- Cache WASM bundles
- Reduce latency
- DDoS protection

---

## Scaling

### Horizontal Scaling (Multiple Backend Instances)

**Run multiple backend services:**
```bash
# backend-1.service (port 3000)
# backend-2.service (port 3001)
# backend-3.service (port 3002)
```

**Nginx load balancing:**
```nginx
upstream backend {
    least_conn;
    server 127.0.0.1:3000;
    server 127.0.0.1:3001;
    server 127.0.0.1:3002;
}
```

### Database Scaling

**Read replicas:**
- Route read queries to replicas
- Write queries to primary

**Connection pooling:**
- PgBouncer for connection pooling
- Reduce connection overhead

---

## CI/CD Pipeline

### GitHub Actions Example

```yaml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Build backend
        run: cargo build --release -p locus-backend

      - name: Build frontend
        run: |
          cargo install trunk
          cd crates/frontend
          trunk build --release

      - name: Deploy backend
        run: |
          scp target/release/locus-backend user@server:/home/locus/
          ssh user@server 'sudo systemctl restart locus-backend'

      - name: Deploy frontend
        run: |
          scp -r crates/frontend/dist/* user@server:/var/www/locus/
```

---

## Rollback Strategy

### Backend Rollback

```bash
# Keep previous binary
mv locus-backend locus-backend.old
# Upload new binary
# If issues:
mv locus-backend.old locus-backend
sudo systemctl restart locus-backend
```

### Database Rollback

```bash
# Restore from backup
psql -U locus_prod locus < /var/backups/locus/locus_20240115.sql
```

### Frontend Rollback

```bash
# Keep previous dist in versioned folder
mv /var/www/locus /var/www/locus.old
# Deploy new
# If issues:
mv /var/www/locus.old /var/www/locus
```

---

## Checklist

### Pre-Deployment

- [ ] Build backend in release mode
- [ ] Build frontend with production API URL
- [ ] Test all features locally
- [ ] Run all tests (`cargo test`)
- [ ] Update documentation
- [ ] Create database backup
- [ ] Review environment variables

### Deployment

- [ ] Upload backend binary
- [ ] Upload frontend files
- [ ] Configure environment variables
- [ ] Run database migrations
- [ ] Start backend service
- [ ] Configure Nginx
- [ ] Obtain SSL certificates
- [ ] Set up firewall rules
- [ ] Configure monitoring

### Post-Deployment

- [ ] Test all endpoints
- [ ] Verify HTTPS works
- [ ] Check database connections
- [ ] Monitor error logs
- [ ] Set up automated backups
- [ ] Document deployment process
- [ ] Test rollback procedure

---

## Troubleshooting

### Backend Won't Start

```bash
# Check logs
sudo journalctl -u locus-backend -n 100

# Common issues:
# - DATABASE_URL incorrect
# - PostgreSQL not running
# - Port 3000 in use
# - Missing environment variables
```

### Database Connection Failed

```bash
# Test connection
psql postgres://locus_prod:PASSWORD@localhost/locus -c "SELECT 1;"

# Check PostgreSQL running
sudo systemctl status postgresql

# Check pg_hba.conf allows connections
sudo tail /var/log/postgresql/postgresql-16-main.log
```

### 502 Bad Gateway (Nginx)

```bash
# Backend not running
sudo systemctl status locus-backend

# Wrong port in Nginx config
sudo nginx -t

# Check upstream connection
curl http://127.0.0.1:3000/api/health
```

### High Memory Usage

```bash
# Check backend memory
ps aux | grep locus-backend

# Adjust connection pool size
# Restart service
sudo systemctl restart locus-backend
```

---

## Support

For production issues:
- Check logs first
- Review this documentation
- Consult component docs (Axum, SQLx, Leptos)
- Open GitHub issue with logs and error details
