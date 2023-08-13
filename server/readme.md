## Envs
```shell
export REDIS_URL = "redis://localhost/"
export DATABASE_URL = "postgresql://ksoxuser:ksoxuserp4ssword@localhost/ksox"
export TESTS_CASES = "10000"
export TESTS_FRACTION_BYTES = "2"
```

# Run
run database, sqlx checks queries in build time
```shell
docker compose up --build postgres
cargo build
```

# Code management tools

## dependencies
```shell
cargo install sqlx-cli
cargo install cargo-make
cargo install cargo-udeps
cargo install cargo-sort
cargo install cargo-edit
sudo apt-get install -y libssl-dev pkg-config protobuf-compiler
```

## sqlx-prepare offline data
make sure database is running
```shell
cargo sqlx prepare --workspace -D $DATABASE_URL
```

## update Rust crates
```shell
cargo upgrade
```

## run total check
ideally run this before commit
```shell
cargo make
```

## sort all Cargo.toml files in workspace
make sure you are in workspace root folder
```shell
cargo-sort -w
```
