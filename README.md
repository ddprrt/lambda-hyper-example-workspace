# AWS Lambda / Hyper Workspace

A Cargo workspace with a serverless function that works with both Hyper and AWS Lambda

- `app`: The function itself
- `lambda`: AWS Lambda bindings
- `server`: The hyper server

## Tooling

Build and run server:

```
$ cargo build
$ cargo run --bin server
```

## Cross compilation

Cross compilation to different architectures requires zig and zigbuild. E.g.

```bash
$ brew install zig
$ cargo install zigbuild
```

`make lambda-aarch64` executes

```bash
$ cargo zigbuild --release --target aarch64-unknown-linux-gnu.2.17 --bin lambda
```

and zips it ready for deployment.
