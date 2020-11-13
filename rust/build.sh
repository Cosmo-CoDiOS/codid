#!/bin/sh

set -eu

echo "Building aarch64-unknown-linux-musl.."

echo "Debug build in progress.."
cross --quiet build --target aarch64-unknown-linux-musl
echo "Debug build complete."

echo "Release build in progress.."
cross --quiet build --release --target aarch64-unknown-linux-musl
echo "Release build complete."

echo "Building aarch64-unknown-linux-gnu.."

echo "Debug build in progress.."
cross --quiet build --target aarch64-unknown-linux-gnu
echo "Debug build complete."

echo "Release build in progress.."
cross --quiet build --release --target aarch64-unknown-linux-gnu
echo "Release build complete."

exit
