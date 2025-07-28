# Copilot Instructions - Moyu Live Backend

## Project Overview
This is a Rust-based live streaming backend server using Axum web framework, Diesel ORM, and PostgreSQL. The project integrates with SRS (Simple Realtime Server) for streaming capabilities.

## Architecture Patterns

### Configuration System
- Uses layered configuration: file → environment variables with `LIVESERVER_` prefix
- Configuration structure: `AppConfig` contains `LogConfig` and `SrsConfig`
- Environment variables use underscore separator: `LIVESERVER_LOG_LEVEL`
- Config aliases: `srs` for `srs_servers`, `consolelevel` for `console_level`

### Database Integration
- Uses Diesel ORM with PostgreSQL primary, SQLite secondary support
- Schema defined in `src/models/schema.rs` (auto-generated from migrations)
- Migrations stored in `./migrations` but excluded from git during development
- Database URL configured via `.env` file for development

### Logging Architecture
- Dual logging: console + optional file output with different levels/formats
- Console formats: `pretty` (default), `compact`, `full`, `json`
- File logging always uses JSON format with file/line numbers
- Uses `tracing` spans for structured logging (see main.rs app span pattern)

### Module Structure
```
src/
├── config.rs          # Configuration structs and validation
├── main.rs            # App bootstrap, logging setup, CLI
├── models/            # Database schema and models
│   ├── mod.rs
│   └── schema.rs      # Diesel-generated schema
└── routes/            # Web API handlers (modular by feature)
    ├── mod.rs
    ├── admin.rs       # Admin operations
    ├── live.rs        # Live streaming endpoints
    ├── room.rs        # Room management
    └── user.rs        # User authentication/management
```

## Development Workflow

### Database Setup
```bash
# Install Diesel CLI (recommended via binstall)
cargo binstall diesel_cli

# Setup development database
echo "DATABASE_URL=postgres://postgres:postgres@localhost:5432/moyu_live_dev" > .env
diesel setup

# Generate and apply migrations
diesel migration generate <name>
diesel migration run

# Generate schema diff from existing schema.rs
diesel migration generate --diff-schema <name>
```

### Configuration Examples
- CLI: `cargo run -- --config /path/to/config.toml`
- Environment: `LIVESERVER_LOG_LEVEL=debug LIVESERVER_DATABASE_URL=...`
- File config supports TOML/JSON formats

### SRS Integration
- `SrsConfig` handles SRS server API endpoints
- Multiple SRS servers supported via `srs_servers` array
- API URL configuration via `api_url` field

## Code Conventions

### Error Handling
- Use `anyhow::Result<()>` for main functions and initialization
- Explicit error context via `expect()` with descriptive messages
- Configuration validation happens at startup

### Async Patterns
- `#[tokio::main]` for async runtime
- Use `anyhow::Ok(())` for explicit Ok returns

### Serde Configuration
- Use `#[serde(alias = "...")]` for config field backwards compatibility
- All config structs derive `Debug, Clone, Serialize, Deserialize`

## Key Dependencies
- **axum**: Web framework (not yet implemented in routes/)
- **diesel**: Database ORM with PostgreSQL focus
- **tracing**: Structured logging with spans
- **clap**: CLI argument parsing with derive API
- **config**: Layered configuration management
- **anyhow**: Error handling with context

## TODO Areas
- Main application logic implementation (see TODO in main.rs)
- Route handlers implementation (all route files are empty)
- SRS integration details (TODO in config.rs)
- Database connection pool setup
- Axum router and middleware configuration
