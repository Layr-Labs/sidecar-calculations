This repo is a custom sqlite extension used by the [EigenLayer Sidecar](https://github.com/Layr-Labs/sidecar) to provide
backwards compatibility for computing rewards based on previous versions of the EigenLayer rewards pipeline.

## Structure

This repo is split into two main components:

1. The Rust implementation of the custom functions
2. A C interface that wraps the rust functions and exposes them to sqlite

## Building

```bash
make release
```

## Testing
```bash
cargo test
```
