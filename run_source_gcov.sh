#https://github.com/marco-c/rust-code-coverage-sample

rm -rf ./target
rm -rf ./coverage
rm -rf ./target/debug/coverage/

# Export the flags needed to instrument the program to collect code coverage, and the flags needed to work around
# some Rust features that are incompatible with gcov-based instrumentation.
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"

# Build the program
cargo build

# Run the program (you can replace this with `cargo test` if you want to collect code coverage for your tests).
cargo test

# Generate a HTML report in the coverage/ directory.
#grcov . --llvm -s . -t lcov --branch --ignore-not-existing -o ./target/debug/coverage/
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

# grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing -o ./target/debug/coverage/
# genhtml -o ./target/debug/coverage/ --show-details --highlight --ignore-errors source --legend ./target/debug/lcov.info


# Generate a LCOV report and upload it to codecov.io.
# grcov . --llvm -s . -t lcov --branch --ignore-not-existing -o ./lcov.info
# bash <(curl -s https://codecov.io/bash) -f lcov.info