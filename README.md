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

