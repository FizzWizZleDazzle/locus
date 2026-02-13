# Locus Deployment Guide

> **NEW:** We have consolidated deployment scripts into a Makefile!
> For the streamlined deployment process, see **[RELEASE.md](RELEASE.md)**

## Overview

Locus consists of:
- **Backend**: Rust/Axum API server (port 3000)
- **Frontend**: Leptos WASM app served as static files
- **Database**: PostgreSQL 16

## Recommended Deployment Method

Use the new Makefile-based system:

```bash
make init      # Generate secrets
make tunnel    # Set up Cloudflare
make all       # Complete deployment
```

See [RELEASE.md](RELEASE.md) for detailed instructions.

---

## Alternative Deployment Methods

## Quick Start (Docker Compose)

### 1. Generate Secrets

```bash
# Generate secure secrets
openssl rand -base64 32  # Use for JWT_SECRET
openssl rand -base64 32  # Use for API_KEY_SECRET
openssl rand -base64 32  # Use for DB_PASSWORD
```

### 2. Configure Environment

```bash
# Copy the example env file
cp .env.production.example .env.production

# Edit .env.production with your secrets and domain
nano .env.production
```

### 3. Deploy with Docker Compose

```bash
# Build and start all services
docker compose -f docker-compose.prod.yml up -d

# Check logs
docker compose -f docker-compose.prod.yml logs -f

# Check health
curl http://localhost:3000/health
```

## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster (1.24+)
- kubectl configured
- Container registry access

### 1. Build and Push Image

```bash
# Build the Docker image
docker build -t your-registry.com/locus:latest .

# Push to registry
docker push your-registry.com/locus:latest
```

### 2. Create Secrets

```bash
# Create namespace
kubectl create namespace locus

# Create database secret
kubectl create secret generic locus-db -n locus \
  --from-literal=password=$(openssl rand -base64 32)

# Create backend secrets
kubectl create secret generic locus-backend -n locus \
  --from-literal=jwt-secret=$(openssl rand -base64 32) \
  --from-literal=api-key-secret=$(openssl rand -base64 32)

# Optional: OAuth secrets
kubectl create secret generic locus-oauth -n locus \
  --from-literal=google-client-id=YOUR_GOOGLE_CLIENT_ID \
  --from-literal=google-client-secret=YOUR_GOOGLE_CLIENT_SECRET \
  --from-literal=github-client-id=YOUR_GITHUB_CLIENT_ID \
  --from-literal=github-client-secret=YOUR_GITHUB_CLIENT_SECRET
```

### 3. Deploy PostgreSQL

```yaml
# k8s/postgres.yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: postgres-pvc
  namespace: locus
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 20Gi
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: postgres
  namespace: locus
spec:
  replicas: 1
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
      - name: postgres
        image: postgres:16-alpine
        env:
        - name: POSTGRES_USER
          value: "locus"
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: locus-db
              key: password
        - name: POSTGRES_DB
          value: "locus"
        ports:
        - containerPort: 5432
        volumeMounts:
        - name: postgres-storage
          mountPath: /var/lib/postgresql/data
        livenessProbe:
          exec:
            command:
            - pg_isready
            - -U
            - locus
          initialDelaySeconds: 30
          periodSeconds: 10
      volumes:
      - name: postgres-storage
        persistentVolumeClaim:
          claimName: postgres-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: postgres
  namespace: locus
spec:
  selector:
    app: postgres
  ports:
  - port: 5432
    targetPort: 5432
```

### 4. Deploy Backend

```yaml
# k8s/backend.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: locus-backend
  namespace: locus
spec:
  replicas: 3
  selector:
    matchLabels:
      app: locus-backend
  template:
    metadata:
      labels:
        app: locus-backend
    spec:
      containers:
      - name: backend
        image: your-registry.com/locus:latest
        env:
        - name: ENVIRONMENT
          value: "production"
        - name: DATABASE_URL
          value: "postgresql://locus:$(DB_PASSWORD)@postgres:5432/locus"
        - name: DB_PASSWORD
          valueFrom:
            secretKeyRef:
              name: locus-db
              key: password
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: locus-backend
              key: jwt-secret
        - name: API_KEY_SECRET
          valueFrom:
            secretKeyRef:
              name: locus-backend
              key: api-key-secret
        - name: ALLOWED_ORIGINS
          value: "https://locus.yourdomain.com"
        - name: FRONTEND_BASE_URL
          value: "https://locus.yourdomain.com"
        - name: OAUTH_REDIRECT_BASE
          value: "https://api.locus.yourdomain.com"
        - name: GOOGLE_CLIENT_ID
          valueFrom:
            secretKeyRef:
              name: locus-oauth
              key: google-client-id
              optional: true
        - name: GOOGLE_CLIENT_SECRET
          valueFrom:
            secretKeyRef:
              name: locus-oauth
              key: google-client-secret
              optional: true
        - name: GITHUB_CLIENT_ID
          valueFrom:
            secretKeyRef:
              name: locus-oauth
              key: github-client-id
              optional: true
        - name: GITHUB_CLIENT_SECRET
          valueFrom:
            secretKeyRef:
              name: locus-oauth
              key: github-client-secret
              optional: true
        - name: HOST
          value: "0.0.0.0"
        - name: PORT
          value: "3000"
        ports:
        - containerPort: 3000
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
          initialDelaySeconds: 10
          periodSeconds: 5
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: locus-backend
  namespace: locus
spec:
  selector:
    app: locus-backend
  ports:
  - port: 80
    targetPort: 3000
  type: ClusterIP
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: locus-ingress
  namespace: locus
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
spec:
  tls:
  - hosts:
    - locus.yourdomain.com
    secretName: locus-tls
  rules:
  - host: locus.yourdomain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: locus-backend
            port:
              number: 80
```

### 5. Apply Configuration

```bash
# Apply all manifests
kubectl apply -f k8s/postgres.yaml
kubectl apply -f k8s/backend.yaml

# Check status
kubectl get pods -n locus
kubectl logs -f deployment/locus-backend -n locus

# Check health
kubectl port-forward -n locus service/locus-backend 3000:80
curl http://localhost:3000/health
```

## Environment Variables Reference

### Required

- `ENVIRONMENT`: `production` (enables all security checks)
- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: 32+ character secret for JWT signing
- `API_KEY_SECRET`: 32+ character secret for factory API

### URLs & CORS

- `ALLOWED_ORIGINS`: Comma-separated list of allowed frontend origins
- `FRONTEND_BASE_URL`: Frontend URL for redirects/email links
- `OAUTH_REDIRECT_BASE`: Backend URL for OAuth callbacks

### OAuth (Optional)

- `GOOGLE_CLIENT_ID`: Google OAuth app client ID
- `GOOGLE_CLIENT_SECRET`: Google OAuth app client secret
- `GITHUB_CLIENT_ID`: GitHub OAuth app client ID
- `GITHUB_CLIENT_SECRET`: GitHub OAuth app client secret

### Email (Optional - MVP doesn't use)

- `RESEND_API_KEY`: Resend.com API key for email verification
- `RESEND_FROM_EMAIL`: From email address
- `RESEND_FROM_NAME`: From name

## Security Checklist

✅ Before going to production:

1. **Secrets**
   - [ ] Generated strong JWT_SECRET (32+ chars)
   - [ ] Generated strong API_KEY_SECRET (32+ chars)
   - [ ] Generated strong database password
   - [ ] Stored secrets in secure secret management (K8s secrets, Vault, etc.)

2. **Environment**
   - [ ] Set `ENVIRONMENT=production`
   - [ ] Configured proper CORS origins (no localhost)
   - [ ] Set HTTPS URLs for all domains

3. **OAuth**
   - [ ] Registered OAuth apps with production callback URLs
   - [ ] Configured OAuth client IDs and secrets

4. **Database**
   - [ ] PostgreSQL running with proper backups
   - [ ] Database password not in git
   - [ ] Connection pool configured appropriately

5. **Monitoring**
   - [ ] Health checks configured
   - [ ] Logging aggregation setup
   - [ ] Metrics collection (Prometheus/Grafana)

## Migrations

Migrations run automatically on backend startup. The binary checks for new migrations and applies them before serving traffic.

To manually run migrations:

```bash
# With sqlx-cli
DATABASE_URL=postgresql://user:pass@host:5432/locus sqlx migrate run

# With Docker
docker exec locus-backend ./locus-backend --migrate-only
```

## Monitoring

### Health Check

```bash
curl https://api.locus.yourdomain.com/health
# Expected: 200 OK
```

### Metrics Endpoints

The backend exposes:
- `/health` - Health check (200 if healthy)
- `/api/topics` - Test data retrieval

### Logs

Check application logs:

```bash
# Docker Compose
docker compose -f docker-compose.prod.yml logs -f backend

# Kubernetes
kubectl logs -f deployment/locus-backend -n locus
```

## Troubleshooting

### Backend won't start

1. Check environment variables:
   ```bash
   kubectl describe pod -n locus <pod-name>
   ```

2. Check logs:
   ```bash
   kubectl logs -n locus <pod-name>
   ```

3. Common issues:
   - Missing DATABASE_URL
   - JWT_SECRET or API_KEY_SECRET too short
   - Database connection failed
   - Migrations failed

### Database connection issues

1. Verify PostgreSQL is running:
   ```bash
   kubectl get pods -n locus | grep postgres
   ```

2. Test connection:
   ```bash
   kubectl exec -it postgres-<pod-id> -n locus -- psql -U locus -d locus
   ```

### OAuth not working

1. Verify callback URLs match your OAuth app configuration
2. Check OAUTH_REDIRECT_BASE is set correctly
3. Ensure HTTPS is configured for production OAuth

## Scaling

### Horizontal Scaling

The backend is stateless and can be scaled horizontally:

```bash
# Scale to 5 replicas
kubectl scale deployment locus-backend -n locus --replicas=5
```

### Database Scaling

For production workloads, consider:
- PostgreSQL read replicas
- Connection pooling (PgBouncer)
- Managed database services (AWS RDS, Google Cloud SQL)

## Backup & Recovery

### Database Backups

```bash
# Backup
kubectl exec postgres-<pod-id> -n locus -- \
  pg_dump -U locus locus > backup-$(date +%Y%m%d).sql

# Restore
kubectl exec -i postgres-<pod-id> -n locus -- \
  psql -U locus locus < backup-20260212.sql
```

## Updates & Rollbacks

```bash
# Update deployment
kubectl set image deployment/locus-backend -n locus \
  backend=your-registry.com/locus:v2.0.0

# Rollback
kubectl rollout undo deployment/locus-backend -n locus

# Check rollout status
kubectl rollout status deployment/locus-backend -n locus
```
