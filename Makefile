all: aarch64-musl aarch64-gnu x86_64-gnu

aarch64: aarch64-musl aarch64-gnu
x86_64: x86_64-gnu x86_64-musl

aarch64-musl: aarch64-musl-debug aarch64-musl-release
aarch64-gnu: aarch64-gnu-debug aarch64-gnu-release
x86_64-gnu: x86_64-gnu-debug x86_64-gnu-release
x86_64-musl: x86_64-musl-debug x86_64-musl-debug

aarch64-musl-debug:
	@echo "Building cosmo-codi-d..."
	@echo "Target: aarch64-unknown-linux-musl"
	@echo "Debug build in progress.."
	cross build --all --locked --target aarch64-unknown-linux-musl
	@echo "Debug build complete."

aarch64-musl-release:
	@echo "Building cosmo-codi-d..."
	@echo "Target: aarch64-unknown-linux-musl"
	@echo "Release build in progress.."
	cross build --release --all --locked --target aarch64-unknown-linux-musl
	@echo "Release build complete."

aarch64-gnu-debug:
	@echo "Building cosmo-codi-d..."
	@echo "Target: aarch64-unknown-linux-gnu"
	@echo "Debug build in progress.."
	cross build --all --locked --target aarch64-unknown-linux-gnu
	@echo "Debug build complete."

aarch64-gnu-release:
	@echo "Building cosmo-codi-d..."
	@echo "Target: aarch64-unknown-linux-gnu"
	@echo "Release build in progress.."
	cross build --release --all --locked --target aarch64-unknown-linux-gnu
	@echo "Release build complete."

x86_64-gnu-debug:
	@echo "Building cosmo-codi-d..."
	@echo "Target: x86_64-unknown-linux-gnu"
	@echo "Debug build in progress.."
	cross build --all --locked --target x86_64-unknown-linux-gnu
	@echo "Debug build complete."

x86_64-gnu-release:
	@echo "Building cosmo-codi-d..."
	@echo "Target: x86_64-unknown-linux-gnu"
	@echo "Release build in progress.."
	cross build --release --all --locked --target x86_64-unknown-linux-musl
	@echo "Release build complete."

x86_64-musl-debug:
	@echo "Building cosmo-codi-d..."
	@echo "Target: x86_64-unknown-linux-gnu"
	@echo "Release build in progress.."
	cross build --all --locked --target x86_64-unknown-linux-musl
	@echo "Release build complete."

x86_64-musl-release:
	@echo "Building cosmo-codi-d..."
	@echo "Target: x86_64-unknown-linux-gnu"
	@echo "Release build in progress.."
	cross build --release --all --locked --target x86_64-unknown-linux-musl
	@echo "Release build complete."
