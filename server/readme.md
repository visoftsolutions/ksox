# Envs
## Development
#### **`.cargo/config.toml`**
```
[env]
KSOX_SERVER_BLOCKCHAIN_URL="http://ksox-server-blockchain/"
KSOX_WS_BLOCKCHAIN_URL="http://localhost:8545/"
KSOX_POSTGRES_URL="postgresql://ksoxuser:ksoxuserp4ssword@localhost/ksox"
KSOX_REDIS_URL="redis://localhost/"
KSOX_SERVER_ENGINE_URL="http://localhost/"
CONTRACT_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
CONTRACT_ADDRESS="0x5FbDB2315678afecb367f032d93F642f64180aa3"
DEPOSIT_CONFIRMATIONS="5"
TESTS_CASES="10000"
TESTS_FRACTION_BYTES="2"
RUST_BACKTRACE="full"
```
## Production (Docker Compose)
#### **`.env`**
```
KSOX_SERVER_BLOCKCHAIN_URL="http://ksox-server-blockchain/"
KSOX_WS_BLOCKCHAIN_URL="ws://ksox-blockchain:8545/"
KSOX_POSTGRES_URL="postgresql://ksoxuser:ksoxuserp4ssword@ksox-postgres/ksox"
KSOX_REDIS_URL="redis://ksox-redis/"
KSOX_SERVER_ENGINE_URL="http://ksox-server-engine/"
CONTRACT_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
CONTRACT_ADDRESS="0x5FbDB2315678afecb367f032d93F642f64180aa3"
DEPOSIT_CONFIRMATIONS="5"
TESTS_CASES = "10000"
TESTS_FRACTION_BYTES = "2"
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
