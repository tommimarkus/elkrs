# Release Checklist

Before publishing a crate version:

1. Confirm `README.md`, `CLEANROOM.md`, `CONTRIBUTING.md`, and `CHANGELOG.md` match the actual supported behavior.
2. Run the local gate:

   ```bash
   cargo fmt --all --check
   cargo clippy --workspace --all-targets --locked -- -D warnings
   cargo test --workspace --locked
   cargo doc --workspace --locked --no-deps
   ```

3. For the first release, verify and publish in dependency order. Dependent
   crate package verification cannot complete until its local `elkrs-*`
   dependency version exists in the registry.

   ```bash
   cargo package -p elkrs-core --locked
   cargo publish -p elkrs-core --locked

   cargo package -p elkrs-json --locked
   cargo publish -p elkrs-json --locked

   cargo package -p elkrs-layered --locked
   cargo publish -p elkrs-layered --locked
   ```

4. If Java ELK is available, run the optional parity harness before publishing:

   ```bash
   ELKRS_JAVA_ELK_COMMAND=/path/to/java-elk-command cargo test -p elkrs-layered --test java_parity --locked
   ```

Do not publish if any release notes imply full ELK coverage, Dediren adapter support, CLI support, or Java coordinate parity.
