# nova-ml

nova-ml is a library for building recursive circuits for folding batch inference in zero-knowledge machine learning.

## Reference

The thought process behind this library is described in the document [Folding Circom circuits: a ZKML case study](https://hackmd.io/@cathie/zkml-folding).

## Setup

```bash
git clone https://github.com/nalinbhardwaj/circom.git
cd circom
git checkout pasta
cargo install --path circom
cd ..
npm install
./compile.sh
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