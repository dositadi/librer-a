# list available commands
help:
    @just --list --unsorted --list-prefix '  ' --list-heading $'RESTful API Workspace:'
    @echo ''
    @just --list --unsorted --list-prefix '    ' --list-heading $'  BOOK SERVICE:' --justfile crates/book_service/justfile

# stage, commit and push to remote repository
push message:
    git add . && git commit -m "{{message}}" && git push 

# Run cargo check on workspace memebers
check:
    cargo check --workspace

# Run cargo clean on workspace members
clean:
    cargo clean

# Run cargo test on workspace members 
test:
    cargo test --workspace

# Run cargo build on workspace members
build:
    cargo build --workspace --all-targets

# Run lint test on workspace members (cargo fmt and clippy)
lint:
    cargo +nightly fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings

# forward to book service
mod book "crates/book_service"
