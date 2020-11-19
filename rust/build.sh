#!/bin/sh

set -eu

echo "Downloading required Docker images for cross-compiling.."
echo "(musl)"
docker pull shymega/rust-cross-custom:aarch64-unknown-linux-musl-0.2.1
echo "(musl done)"

echo "(glibc)"
docker pull shymega/rust-cross-custom:aarch64-unknown-linux-gnu-0.2.1
echo "(glibc done)"

echo "Download complete."

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
