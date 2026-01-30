# Getting Started with Locus

Welcome to Locus! This guide will help you get up and running quickly.

## What Just Happened?

Your development environment is now running:
- ✅ PostgreSQL database (port 5433)
- ✅ Backend API server (port 3000)
- ✅ Frontend dev server (port 8080)

## Access Your Application

Open your browser and navigate to:
- **Frontend:** http://localhost:8080
- **Backend API:** http://localhost:3000/api/health

## Quick Test

1. **Visit** http://localhost:8080
2. **Click** "Register" to create an account
3. **Try Practice Mode** - instant feedback, no login required
4. **Try Ranked Mode** - requires login, affects your ELO rating
5. **Check Leaderboard** - see top players by topic

## Project Structure

```
locus/
├── crates/
│   ├── common/          # Shared types
│   ├── backend/         # Rust API server
│   └── frontend/        # Leptos WASM app
├── docs/                # 📚 COMPREHENSIVE DOCUMENTATION (NEW!)
│   ├── ARCHITECTURE.md
│   ├── API.md
│   ├── DATABASE.md
│   ├── FRONTEND.md
│   ├── BACKEND.md
│   ├── DEVELOPMENT.md
│   ├── DEPLOYMENT.md
│   ├── LOOKUP_TABLE.md  # 🔍 Quick code finder
│   └── SUMMARY.md
├── content-gen/         # Problem generator
├── dev.sh              # ✅ FIXED! One-command startup
└── DOCUMENTATION.md    # 📖 Documentation index
```

## Documentation

**ALL DOCUMENTATION IS NOW COMPLETE!** 🎉

### Start Here
- [DOCUMENTATION.md](DOCUMENTATION.md) - Documentation overview

### Most Useful Docs
- [LOOKUP_TABLE.md](docs/LOOKUP_TABLE.md) - Find any file, feature, or code instantly
- [DEVELOPMENT.md](docs/DEVELOPMENT.md) - Development guide and common tasks
- [API.md](docs/API.md) - Complete API reference with examples

### Full Documentation Set
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - System design and architecture
- [DATABASE.md](docs/DATABASE.md) - Database schema and queries
- [FRONTEND.md](docs/FRONTEND.md) - Frontend guide (Leptos/WASM)
- [BACKEND.md](docs/BACKEND.md) - Backend guide (Axum/Rust)
- [DEPLOYMENT.md](docs/DEPLOYMENT.md) - Production deployment
- [SUMMARY.md](docs/SUMMARY.md) - Documentation overview

## Common Commands

```bash
# Start everything (database + backend + frontend)
./dev.sh

# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy

# Connect to database
psql postgres://locus:locus_dev_password@localhost:5433/locus

# Generate new problems
cd content-gen
python generate.py --topic calculus --count 50 --output calculus.sql
```

## Development Workflow

### Making Changes

**Backend:**
1. Edit files in `crates/backend/src/`
2. Backend auto-recompiles
3. Refresh your API calls

**Frontend:**
1. Edit files in `crates/frontend/src/`
2. Trunk auto-recompiles and reloads browser
3. Changes appear automatically

**Database:**
1. Create new migration in `crates/backend/migrations/`
2. Restart backend to apply

### Finding Code

Use the [LOOKUP_TABLE.md](docs/LOOKUP_TABLE.md) to find anything:
- Features → Files
- API endpoints → Handlers
- Database tables → Models
- Components → Source files

Example: Want to find ELO calculation code?
→ Check LOOKUP_TABLE.md → "ELO System" → `crates/backend/src/elo.rs:10`

## Tech Stack

**Frontend:**
- Leptos (Rust UI framework)
- WebAssembly (high-performance)
- KaTeX (math rendering)
- Tailwind CSS (styling)

**Backend:**
- Axum (web framework)
- PostgreSQL 16 (database)
- JWT (authentication)
- Argon2 (password hashing)

**Tools:**
- Trunk (WASM build tool)
- Docker (database)
- Python (problem generation)

## What's Fixed

### dev.sh Script ✅
The development script now properly handles:
- ✅ Existing Docker containers (starts instead of failing)
- ✅ Dependency checking
- ✅ Environment setup
- ✅ Graceful shutdown (Ctrl+C)

You can now just run `./dev.sh` and everything works!

## What's New

### Complete Documentation ✅
- **8 comprehensive guides** covering every aspect of the codebase
- **Lookup table** for instant code navigation
- **8,000+ lines** of detailed documentation
- **Architecture diagrams** and data flow explanations
- **Complete API reference** with examples
- **Database schema** documentation
- **Development and deployment** guides

## Next Steps

1. **Explore the app** at http://localhost:8080
2. **Read** [DOCUMENTATION.md](DOCUMENTATION.md) for overview
3. **Browse** [LOOKUP_TABLE.md](docs/LOOKUP_TABLE.md) to understand code organization
4. **Make a change** and see it live reload
5. **Run tests** with `cargo test`

## Getting Help

**Find code:**
→ [LOOKUP_TABLE.md](docs/LOOKUP_TABLE.md)

**Understand architecture:**
→ [ARCHITECTURE.md](docs/ARCHITECTURE.md)

**API reference:**
→ [API.md](docs/API.md)

**Development questions:**
→ [DEVELOPMENT.md](docs/DEVELOPMENT.md)

**Deployment:**
→ [DEPLOYMENT.md](docs/DEPLOYMENT.md)

## Stopping Servers

Press `Ctrl+C` in the terminal running `./dev.sh`

Or manually:
```bash
# Stop all
pkill -f locus-backend
pkill -f trunk

# Stop database
docker compose down
```

## Troubleshooting

**Port already in use:**
```bash
# Check what's using port 3000 or 8080
lsof -i :3000
lsof -i :8080

# Kill the process
kill -9 <PID>
```

**Database connection error:**
```bash
# Restart database
docker compose restart

# Check if running
docker ps | grep locus-db
```

**Frontend not loading:**
```bash
# Hard refresh browser (Ctrl+Shift+R)
# Check console for errors (F12)
```

For more troubleshooting, see [DEVELOPMENT.md](docs/DEVELOPMENT.md#common-issues).

## Project Status

✅ **Core Features Working:**
- User registration and authentication
- Practice mode with client-side grading
- Ranked mode with ELO ratings
- Per-topic ELO tracking (8 topics)
- Leaderboard
- Problem generation

✅ **Documentation Complete:**
- Architecture documentation
- API documentation
- Database documentation
- Frontend documentation
- Backend documentation
- Development guide
- Deployment guide
- Lookup table

🚧 **Future Enhancements:**
- SymEngine integration for symbolic math
- Real-time competitions
- Advanced statistics
- Mobile app

## Contributing

1. Read the documentation
2. Pick a feature or fix a bug
3. Make your changes
4. Run tests (`cargo test`)
5. Format code (`cargo fmt`)
6. Submit a pull request

## License

MIT License - See LICENSE file

---

**You're all set!** 🚀

The dev server is running, and you have complete documentation.
Start exploring at http://localhost:8080 or dive into the code with [LOOKUP_TABLE.md](docs/LOOKUP_TABLE.md).

Happy coding! 🎓📊
