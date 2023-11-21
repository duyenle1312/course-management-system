Mac install db
```
brew update && brew install postgresql && brew services start postgresql
cargo install diesel_cli --no-default-features --features postgres
```
brew services start postgresql
brew services stop postgresql
psql postgres
CREATE ROLE newUser WITH LOGIN PASSWORD 'password';
ALTER ROLE newUser CREATEDB;

Installation
rustup override set nightly
cargo install cargo-watch
diesel migration run

Run
cargo build && cargo run
Or
cargo watch -x run