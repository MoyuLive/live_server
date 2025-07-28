# Copilot Instructions - Moyu Live Backend

## Project Overview
This is a Rust-based live streaming backend server using Axum web framework, Diesel ORM, and PostgreSQL. The project integrates with SRS (Simple Realtime Server) for streaming capabilities. Currently in early development phase with basic configuration and logging infrastructure in place.

## Architecture Patterns

### Configuration System
- **Layered Configuration**: File → Environment variables with `LIVESERVER_` prefix
- **Structure**: `AppConfig` contains `LogConfig`, `SrsConfig`, and `database_url`
- **Environment Variables**: Use underscore separator (e.g., `LIVESERVER_LOG_LEVEL`, `LIVESERVER_DATABASE_URL`)
- **Config Aliases**: `srs` for `srs_servers`, `consolelevel` for `console_level`, `api` for `api_url`
- **Supported Formats**: TOML and JSON configuration files

### Database Integration
- **Primary**: PostgreSQL with Diesel ORM (features: `postgres`, `r2d2`, `sqlite`)
- **Schema**: Auto-generated in `src/models/schema.rs` from migrations
- **Current Schema**: Simple `users` table (id, name, hashed_password)
- **Development**: Migrations excluded from git (`.gitignore`) until schema stabilizes
- **Setup**: Requires `.env` file with `DATABASE_URL=postgres://postgres:postgres@localhost:5432/moyu_live_dev`

### Logging Architecture
- **Dual Output**: Console + optional file with independent log levels
- **Console Formats**: `pretty` (default), `compact`, `full`, `json`
- **File Format**: Always JSON with file/line numbers for debugging
- **Structured Logging**: Uses `tracing` spans (see main.rs `_span` pattern)
- **Dynamic Configuration**: Runtime format switching via config

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

### Dependence

All dependencies are already downloaded and set up in the project, including the postgres database and Diesel CLI, and also the migration is done for you.

### Development Commands
- Use `cargo check` to ensure code quality
- Use `cargo fmt` to format code according to `rustfmt.toml`
- Use `cargo clippy` for linting and code quality checks

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

### Logging Patterns
- Use structured spans: `let _span = span!(tracing::Level::TRACE, "context"); let _ = _span.enter();`
- Layer aggregation for multiple outputs (console + file)
- Console logging supports dynamic format switching

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
