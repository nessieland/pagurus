# Pagurus
Pagurus is Nessie's core API built using Rust, GraphQL (Juniper) and uses PostgreSQL

## Prerequisites 
- A PostgreSQL database
- Cargo (version >= 1.51)
- Rustc (version >= 1.51)

## Development
```shell
# Make a .env file and put your PostgreSQL url in the DATABASE_URL key
echo DATABASE_URL=postgres://username:password@host:port/database_name >> .env

# Run the server
cargo run

# Build the server
cargo build
```