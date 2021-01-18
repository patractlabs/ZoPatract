#!/bin/bash

# Exit if any subcommand fails
set -e

if [ -n "$WITH_LIBSNARK" ]; then
	cargo -Z package-features build --release --package zopatract_cli --features="libsnark"
else
	cargo build --release
fi
