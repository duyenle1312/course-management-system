# Setup Documentation

### Mac install db

```
brew update && brew install postgresql && brew services start postgresql
cargo install diesel_cli --no-default-features --features postgres
```

### Create local DB for the first time

```
brew services start postgresql
brew services stop postgresql
psql postgres
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

### APIs Testing
1. View all courses: /courses
2. Create a course: /api/course \
Payload: Content-Type: "application/json" \
{ \
    "title": "Course Name", \
    "description": "Description" \
}

* More datapoints and APIs will be added