# backend
The the backend of the [TAO](../README.md) project.

## Structure
- api: Ships the API structure
- domain: Contains all domain elements
- entity: A collection of elements generated based on [the database model](./migration/README.md)
- infrastructure: The infrastructure elements for the API
- [migration](./migration/README.md): Manages database migrations
- [square-api-client](./square-api-client/README.md): The API client used to deal with the Square API

## Setup steps
### 1. Install dependencies
```bash
rustup component add rustfmt clippy
cargo install cargo-llvm-cov sea-orm-cli
cargo install
```

### 2. Follow the setup steps of the subprojects
- [square-api-client](./square-api-client/README.md)

## Testing
```bash
# Run all tests
cargo llvm-cov --open --workspace

# Run only unit tests
cargo llvm-cov --open --workspace --lib

# Run only integration tests
cargo llvm-cov --open --workspace --test integration
```

## Formatting and linting
Rust formatting is handled by `rustfmt`; Clippy provides the Rust linter.

```bash
# Format the entire workspace
cargo fmt --all

# Check formatting without changing files
cargo fmt --all -- --check

# Run Clippy across the workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
