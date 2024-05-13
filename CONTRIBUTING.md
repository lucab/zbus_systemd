# How to contribute

This document provides an overview and some general notes about all the code
in this repository.

## Motivations and trade-offs

The `zbus_systemd` library tries to achieve the following goals:

 * provide coverage for all systemd DBus services in a single crate
 * build on top of a Rust-native DBus stack, thanks to `zbus`
 * statically generate library code directly from systemd definitions
 * directly rely on generated interfaces

## Code generation

This project uses [just](https://github.com/casey/just) to perform code-generation.
To refresh all interfaces after making some changes, you can directly run `just` in the top directory.

All modules are generated from the corresponding systemd doc-page.
The code-generator source lives under `codegen/`, and it can be configured through the `codegen.toml` file.

The interfaces are re-generated on each major systemd release, in a process that involves:

 * updating the git submodule at `codegen/systemd/` to the latest version
 * parsing all the relevant XML files `codegen/systemd/man/`
 * writing a `generated.rs` for each service module
 * running `cargo fmt` on the whole crate
