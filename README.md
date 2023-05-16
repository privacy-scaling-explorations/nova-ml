# nova-ml

nova-ml is a library for building recursive circuits for folding batch inference in zero-knowledge machine learning.

## Setup

```bash
npm install
cargo build
```

## Run

```bash
cargo run
```

## File Structure

- `src/{circuitName}/`: Contains the circuit implementation for the circuit `circuitName`.
- `src/main.rs`: Contains the main function. Only the toy example is implemented at the moment.
- `src/utils/`: Contains utility circuit templates.