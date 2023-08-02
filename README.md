# Cli Template for Rust

A simple template with [cargo generate](https://github.com/cargo-generate/cargo-generate) for quickly rewriting everything in rust. :^)

## How to use?

Generate into a subfolder:

```bash
cargo generate --git https://github.com/oliverdding/cli-template-rs.git
```

Generate in the current folder:

```bash
cargo generate --init --git https://github.com/oliverdding/cli-template-rs.git
```

## What does this template contains?

- Layered configuration with [config-rs](https://github.com/mehcode/config-rs)
- Command line argument parser and shell completions with [clap](https://github.com/clap-rs/clap)
- XDG support with [directories](https://github.com/dirs-dev/directories-rs)
- Logging and tracing with [tracing](https://github.com/tokio-rs/tracing)
- Return code error propagation with [miette](https://github.com/zkat/miette)
- Async runtime with graceful shutdown support with [tokio](https://github.com/tokio-rs/tokio)
