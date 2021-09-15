# Runs all tests, generates coverage report and opens it in a browser
# To use this script you must have grocv and llvm-tools-preview installed.
# You must also use nightly version of rustc toolchain
#
# rustup default nightly
# cargo install grcov
# cargo install llvm-tools-preview
#
# Also pay attention that grcov requires full rebuild for a correct report,
# so this process may be time consuming.

# Clean previous coverage data
rm -rf ./coverage
cargo clean

# Runs the tests generating coverage data
RUSTFLAGS="-Zinstrument-coverage" LLVM_PROFILE_FILE="../coverage/sproute-%p-%m.profraw" cargo test

# Generates the .html report in the ./target/debug/coverage folder
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/

# Opens the file in default browser
xdg-open ./coverage/index.html