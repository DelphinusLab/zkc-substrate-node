This directory includes submodule of the module tests. The `unit_tests/ops` directory includes tests of commands in lib.rs while `unit_tests/helpers` includes tests of helper function. The `scenario_tests` directory includes scenario tests.

## How to run unit test
run `cargo test -- --nocapture` in substrate-node/pallets/swap.

## How to add unit test
Add unit tests in `unit_tests` and scenario tests in `scenario_tests`. Unit Tests of a function should be added in a single file.
