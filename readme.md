# Setup Documentation

### Mac install db

```
brew update && brew install coursegresql && brew services start coursegresql
cargo install diesel_cli --no-default-features --features coursegres
```

### Create local DB for the first time

```
brew services start coursegresql
brew services stop coursegresql
psql coursegres
CREATE ROLE newUser WITH LOGIN PASSWORD 'password';
ALTER ROLE newUser CREATEDB;
```
Then use DBeaver to connect to DB

### Installation

```
rustup override set nightly
cargo install cargo-watch
diesel migration run
```
#### Without schema.rs
```
diesel migration generate courses
```


### Run local
```
1. cargo build && cargo run
2. cargo watch -x run
```