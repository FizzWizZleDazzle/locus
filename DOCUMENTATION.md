# Locus Documentation

Welcome to the Locus documentation. This comprehensive guide covers all aspects of the competitive mathematics platform.

## Quick Links

- [Architecture Overview](docs/ARCHITECTURE.md) - System design and component interaction
- [API Reference](docs/API.md) - Complete API endpoint documentation
- [Database Schema](docs/DATABASE.md) - Database structure and migrations
- [Frontend Guide](docs/FRONTEND.md) - Frontend architecture and components
- [Backend Guide](docs/BACKEND.md) - Backend services and logic
- [Development Guide](docs/DEVELOPMENT.md) - Local development setup
- [Deployment Guide](docs/DEPLOYMENT.md) - Production deployment instructions
- [Lookup Table](docs/LOOKUP_TABLE.md) - Quick reference for files and features

## What is Locus?

Locus is a competitive mathematics platform that allows users to:
- Practice math problems with instant feedback
- Compete in ranked matches with ELO-based ratings
- Track their progress across 8 different math topics
- View leaderboards and compare rankings

## Technology Stack

### Frontend
- **Leptos 0.7** - Reactive UI framework (CSR mode)
- **WebAssembly** - High-performance browser execution
- **KaTeX** - LaTeX math rendering
- **Tailwind CSS** - Utility-first styling
- **Trunk** - Build tool and dev server

### Backend
- **Axum 0.8** - Web framework built on Tokio
- **PostgreSQL 16** - Relational database
- **SQLx** - Async SQL toolkit
- **JWT** - JSON Web Token authentication
- **Argon2** - Password hashing

### Tools
- **Docker Compose** - Database containerization
- **Python + SymPy** - Problem generation
- **SymEngine.js** - Future symbolic math engine

## Project Structure

```
locus/
├── crates/
│   ├── common/          # Shared types between frontend and backend
│   ├── frontend/        # Leptos WASM application
│   └── backend/         # Axum REST API server
├── content-gen/         # Python problem generator
├── symengine.js/        # SymEngine WASM build (future)
├── docs/                # Documentation files
├── dev.sh               # Development startup script
├── docker-compose.yml   # PostgreSQL configuration
└── Cargo.toml          # Workspace configuration
```

## Getting Started

For local development:

```bash
# Quick start
./dev.sh

# Or manually
docker compose up -d              # Start database
cargo run -p locus-backend        # Start backend
cd crates/frontend && trunk serve # Start frontend
```

Visit http://localhost:8080 to access the application.

## Documentation Conventions

Throughout this documentation:
- **File paths** are absolute from project root
- **Code examples** are in Rust unless otherwise specified
- **API paths** are relative to the backend URL (http://localhost:3000)
- **Line numbers** reference current codebase state

## Contributing

When making changes:
1. Follow existing code patterns and naming conventions
2. Add tests for new functionality
3. Update relevant documentation
4. Run `cargo test` before committing
5. Use topic-specific commits

## License

MIT License - See LICENSE file for details
