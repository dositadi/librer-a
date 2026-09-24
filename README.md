# librer-a

A RESTful book-catalog API built in Rust, built primarily as a learning project to get a better grasp of the Rust ecosystem.

The workspace currently contains a single service, **book_service**, built with [Axum](https://github.com/tokio-rs/axum) and the [Toasty](https://toasty.rs/) ORM on top of PostgreSQL.

## Features

- CRUD endpoints for managing a book catalog (create, list with pagination, read, update, delete)
- Request validation with [`garde`](https://github.com/jprochazk/garde)
- Structured JSON logging and request tracing (`tracing`, request IDs, request/response spans)
- Configurable CORS
- Database migrations managed through Toasty
- Health check endpoint

## Tech stack

- **Language:** Rust (2024 edition)
- **Web framework:** [Axum](https://github.com/tokio-rs/axum)
- **ORM / migrations:** [Toasty](https://toasty.rs/)
- **Database:** PostgreSQL
- **Async runtime:** Tokio
- **Task runner:** [just](https://github.com/casey/just)

## Project structure

```
librer-a/
├── Cargo.toml               # Workspace manifest
├── justfile                 # Workspace-level task runner commands
└── crates/
    └── book_service/
        ├── docker-compose.yaml   # Local PostgreSQL instance
        ├── justfile              # Service-level task runner commands
        ├── Toasty.toml           # Toasty migration configuration
        ├── toasty/                # Migrations
        └── src/
            ├── bin/
            │   ├── app.rs         # Service entrypoint
            │   └── migration.rs   # Migration CLI entrypoint
            ├── app/
            │   ├── book/          # Book routes, handlers, request payloads
            │   └── shared/        # Pagination, validation helpers
            ├── models/            # Book model
            ├── config.rs          # Environment-driven configuration
            ├── error.rs           # API error types
            ├── routes.rs          # Router and middleware setup
            └── state.rs           # Shared application state
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition toolchain)
- [Docker](https://www.docker.com/) and Docker Compose (for running PostgreSQL locally)
- [just](https://github.com/casey/just)

## Getting started

### 1. Clone the repository

```bash
git clone https://github.com/dositadi/librer-a.git
cd librer-a
```

### 2. Configure environment variables

Copy the example environment file and fill in the values:

```bash
cp .env.example .env
```

```env
SERVER_PORT=
SERVER_ALLOWED_METHODS=
SERVER_ALLOWED_ORIGINS=
SERVER_ALLOWED_HEADERS=
SERVER_DEFAULT_BODY_LIMIT=

DB_PROTOCOL=
DB_HOST=
DB_PORT=
POSTGRES_DB=
POSTGRES_USER=
POSTGRES_PASSWORD=
```

### 3. Start the database

```bash
just book docker compose up -d
```

### 4. Run migrations

```bash
just book migration up
```

### 5. Run the service

```bash
just book app
```

The service listens on `0.0.0.0:$SERVER_PORT`.

## Available commands

Run `just` from the repository root to see all available commands, or `just book` to see the commands scoped to `book_service`.

| Command | Description |
|---|---|
| `just check` | Run `cargo check` across the workspace |
| `just build` | Build all workspace members |
| `just test` | Run tests across the workspace |
| `just lint` | Run `cargo fmt` and `cargo clippy` |
| `just clean` | Run `cargo clean` |
| `just book app` | Run the book service |
| `just book migration <cmd>` | Run the Toasty migration CLI |
| `just book docker <cmd>` | Run a docker command against the service's compose file |
| `just book docker-db` | Open a `psql` shell against the local database |

## API reference

Base path: `/v1/books`

| Method | Path | Description |
|---|---|---|
| `GET` | `/livez` | Health check |
| `GET` | `/v1/books` | List books (supports `page` and `per_page` query params) |
| `POST` | `/v1/books` | Create a book |
| `GET` | `/v1/books/{id}` | Fetch a book by ID |
| `PUT` | `/v1/books/{id}` | Update a book by ID |
| `DELETE` | `/v1/books/{id}` | Delete a book by ID |

### Book object

```json
{
  "id": "uuid",
  "title": "string",
  "description": "string | null",
  "image_url": "string | null",
  "published_date": "YYYY-MM-DD",
  "status": "pending | verified",
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```

### Create/update request body

```json
{
  "title": "string (1-255 chars)",
  "description": "string | null",
  "image_url": "string (valid URL) | null",
  "published_date": "YYYY-MM-DD",
  "status": "pending | verified"
}
```

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for details.
