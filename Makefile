.PHONY: release/cargo
release/cargo:
	cargo build --release

.PHONY: release/extension
release/extension:
	cd sqlite-extension && make all && cd -

.PHONY: release
release: release/cargo release/extension
	./scripts/bundleRelease.sh

