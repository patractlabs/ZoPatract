# Testing

## Unit tests
In ZoPatract, unit tests comprise of
- internal tests for all zopatract crates
- compilation tests for all examples in `zopatract_cli/examples`. These tests only ensure that the examples compile.
- compilation + witness-computation tests. These tests compile the test cases, compute a witness and compare the result with a pre-defined expected result.
Such test cases exist for
    - The zopatract_core crate in `zopatract_core_test/tests`
    - The zopatract_stdlib crate in `zopatract_stdlib/tests`

Unit tests can be executed with the following command:

```
cargo test --release
```

## Integration tests

Integration tests are excluded from `cargo test` by default.
They are defined in the `zopatract_cli` crate in `integration.rs` and use the test cases specified in `zopatract_cli/tests/code`.

Before running integration tests, make sure:
1. You have [solc](https://github.com/ethereum/solc-js) installed and in your `$PATH`.

    Solc can conveniently be installed through `npm` by running
    ```
    npm install -g solc
    ```
2. You have an Ethereum node running on localhost with a JSON-RPC interface on the default port 8545 (`http://localhost:8545`).

Integration tests can then be run with the following command:

```
cargo test --release -- --ignored
```
If you want to run unit and integrations tests together, run the following command:
```
cargo test --release & cargo test --release -- --ignored
```
