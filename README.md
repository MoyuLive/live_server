# Moyu Live backend

Moyu Live backend

## development

### setup database dev environment

```bash
# install bininstall
# cargo install bininstall

# install diesel cli
cargo binstall diesel_cli
```

create dev database env by create a file `.env`

```plain
DATABASE_URL=postgres://postgres:postgres@localhost:5432/moyu_live_dev
```

run

```bash
diesel setup
```

#### generate migrations

```bash
# generate a new migration, write sql to src/migrations/xxxx-<migration_name>/{up,down}.sql
diesel migration generate <migration_name>

# or generate by schema in `src/models/schema.rs`
diesel migration generate --diff-schema <migration_name>
```

#### apply migrations

```bash
diesel migration run
```
