# zbus_systemd

[![crates.io](https://img.shields.io/crates/v/zbus_systemd.svg)](https://crates.io/crates/zbus_systemd)
[![Documentation](https://docs.rs/zbus_systemd/badge.svg)](https://docs.rs/zbus_systemd)

⚠️ This is an in-progress `v0.0` Proof-of-Concept, do not rely on it.

A pure-Rust library to interact with systemd DBus services.

`zbus_systemd` provides support for interacting with the whole suite of systemd
services over DBus.

## Generating

To re-generate the interfaces, please run:

1. `git submodule update --init`
2. `cargo run --manifest-path=codegen/Cargo.toml`
3. `cargo fmt --all`

## Motivations and trade-offs

This library tries to achieve the following goals:
 * provide coverage for all systemd DBus services in a single crate
 * build on top of a Rust-native DBus stack, thanks to `zbus`
 * statically generate library code directly from systemd definitions
 * mainly rely on generated interfaces, with few manual overrides where needed

## Examples

Some code snippets are available under [examples](examples).

## License

Licensed under either of

 * MIT license - <http://opensource.org/licenses/MIT>
 * Apache License, Version 2.0 - <http://www.apache.org/licenses/LICENSE-2.0>

at your option.
