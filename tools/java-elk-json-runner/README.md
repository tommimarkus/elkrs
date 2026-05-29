# Java ELK JSON Runner

This test tool runs Eclipse ELK Layered `0.11.0` over one ELK-style JSON graph.
It exists only as an opt-in oracle for `elkrs-layered` parity tests.

The executable contract is intentionally narrow:

- read one JSON graph from stdin
- run Java ELK Layered
- write the laid-out JSON graph to stdout
- write diagnostics to stderr and exit nonzero on failure

## Build

The tool uses SDKMAN to select Java and Gradle versions from `.sdkmanrc`.

```bash
tools/java-elk-json-runner/bin/build
```

## Run The Rust Parity Test

```bash
ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" \
  cargo test -p elkrs-layered --test java_parity --locked -- --ignored
```

The wrapper suppresses SDKMAN stdout before launching the generated Gradle
distribution script so stdout remains valid JSON for the Rust test harness.
