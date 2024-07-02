# Zero To Production Newsletter
Repo to work along side the zero to production book. This is a cloud native newsletter application in Rust. 

## Pre-requisites

Install Podman or Docker (Mac OS)
```bash
brew install podman
```



Install sqlx

```bash
cargo install --version='~0.7' sqlx-cli --no-default-features --features rustls,postgres
```
Install psql (Mac OS)

```bash
brew install postgresql
```


## Test

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

via Podman:

```bash
./scripts/init_db_pod.sh
```

Launch `cargo`:

```bash
cargo test 
