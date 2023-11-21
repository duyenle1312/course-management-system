Mac install db
```
brew update && brew install coursegresql && brew services start coursegresql
cargo install diesel_cli --no-default-features --features coursegres
```
brew services start coursegresql
brew services stop coursegresql
psql coursegres
CREATE ROLE newUser WITH LOGIN PASSWORD 'password';
ALTER ROLE newUser CREATEDB;

Installation
rustup override set nightly
cargo install cargo-watch
//diesel migration generate courses
diesel migration run

Run
cargo build && cargo run
Or
cargo watch -x run