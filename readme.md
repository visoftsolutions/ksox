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
rustup toolchain install nightly
sudo apt-get install protobuf-compiler
```

## run total check
ideally run this before commit, make sure you have nightly toolchain installed
```
cargo +nightly make
```

## run partial check
```
cargo make partial
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

## update package.json
npx npm-check-updates -u

## run code in kubernetes
make sure you have minikube, docker and skaffold installed
make sure you are in a docker group
```
minikube start --driver=docker --cpus 8 --memory 4096 && \
minikube addons enable ingress
```
you can now connect with your browser to ingress, ask minikube for ip
```
minikube ip
```
To deploy application do
```
skaffold run
```
You can also work in development mode
```
skaffold dev
```
