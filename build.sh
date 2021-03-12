#!/bin/sh

set -eu

echo "Building aarch64-unknown-linux-musl.."

echo "Debug build in progress.."
cross build --target aarch64-unknown-linux-musl
echo "Debug build complete."

echo "Release build in progress.."
cross build --release --target aarch64-unknown-linux-musl
echo "Release build complete."

echo "Building aarch64-unknown-linux-gnu.."

echo "Debug build in progress.."
cross build --target aarch64-unknown-linux-gnu
echo "Debug build complete."

echo "Release build in progress.."
cross build --release --target aarch64-unknown-linux-gnu
echo "Release build complete."

exit
