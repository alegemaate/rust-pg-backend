# product-db

## Install

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## DB

```bash
docker-compose up -d
diesel migration run
```

## Run

```bash
cargo watch -x run
```

## Build

```bash
# Dev
cargo build

# Prod
cargo build --release
```
