# Cli Template for rust

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

- Layered configuration with config-rs
- Command line argument parser and shell completions with clap
- XDG support with xdg
- Logging and tracing with tracing
- Return code error propagation with miette
- Async runtime with graceful shutdown support with tokio

