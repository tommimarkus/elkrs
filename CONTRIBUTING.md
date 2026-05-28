# Contributing

## Clean-Room Boundary

`elkrs` is an independently written Rust implementation of ELK-style behavior.
Do not copy Eclipse ELK implementation source, Java processor code, internal
comments, or line-by-line algorithm structure into this repository.

Allowed implementation inputs are documented in `CLEANROOM.md`.

## Local Verification

Run the default gate before sending changes:

```bash
cargo install cargo-audit --locked
cargo audit
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo doc --workspace --locked --no-deps
```

The Java ELK comparison harness is opt-in. It is skipped by default unless
`ELKRS_JAVA_ELK_COMMAND` points to a command that reads ELK-style JSON from
stdin and writes ELK-style JSON to stdout.
