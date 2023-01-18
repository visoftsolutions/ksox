# Build
run database, sqlx checks queries in build time
```
docker compose up postgres
cargo build
```

# Run app
```
docker compose up --build
```

# Prune Docker
```
docker system prune -a
```

# Code management tools

## dependencies
```
cargo install sqlx-cli
cargo install cargo-make
cargo install cargo-udeps
cargo install cargo-sort
```

## run total check
ideally run this before commit
```
cargo make
```

## sqlx-prepare offline data
make sure database is running
```
cargo sqlx prepare --database-url postgresql://ksoxuser:ksoxuserp4ssword@localhost/ksox
```

## sort all Cargo.toml files in workspace
make sure you are in workspace root folder
```
cargo-sort -w
```

## check for unused dependencies in workspace crates
make sure you are in workspace root folder
```
cargo +nightly udeps
```