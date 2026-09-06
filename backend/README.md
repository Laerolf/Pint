# backend
The the backend of the [TAO](../README.md) project.

## Structure
- [square-api-client](./square-api-client/README.md): The API client used to deal with the Square API

## Setup steps
### 1. Install dependencies
```bash
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
