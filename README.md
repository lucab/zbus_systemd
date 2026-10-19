# zbus_systemd

[![crates.io](https://img.shields.io/crates/v/zbus_systemd.svg)](https://crates.io/crates/zbus_systemd)
[![Documentation](https://docs.rs/zbus_systemd/badge.svg)](https://docs.rs/zbus_systemd)

A pure-Rust library to interact with systemd DBus services.

`zbus_systemd` provides support for interacting with the suite of systemd
services over DBus. This crate tries to cover all systemd interfaces,
across all services.

## Auto-generated services

Each service has its own dedicated module, which is auto-generated from current
systemd definitions and can be activated through the corresponding Cargo feature:

 * `home1`: systemd-homed interfaces (org.freedesktop.home1)
 * `hostname1`: systemd-hostnamed interfaces (org.freedesktop.hostname1)
 * `import1`: systemd-importd interfaces (org.freedesktop.import1)
 * `locale1`: systemd-localed interfaces (org.freedesktop.locale1)
 * `login1`: systemd-logind interfaces (org.freedesktop.login1)
 * `machine1`: systemd-machined interfaces (org.freedesktop.machine1)
 * `network1`: systemd-networkd interfaces (org.freedesktop.network1)
 * `oom1`: systemd-oomd interfaces (org.freedesktop.oom1)
 * `portable1`: systemd-portabled interfaces (org.freedesktop.portable1)
 * `resolve1`: systemd-resolved interfaces (org.freedesktop.resolve1)
 * `systemd1`: systemd interfaces (org.freedesktop.systemd1)
 * `sysupdate1`: systemd-sysupdated interfaces (org.freedesktop.sysupdated1)
 * `timedate1`: systemd-timedated interfaces (org.freedesktop.timedate1)
 * `timesync1`: systemd-timesyncd interfaces (org.freedesktop.timesync1)

For a quickstart on how to use those interfaces, see the [examples](https://github.com/lucab/zbus_systemd/tree/main/examples).

## Async executor

This crate internally uses `zbus` without any default Cargo features.
That means that you have to decide which async executor to use, by enabling one of the following Cargo features:
 * `zbus-async-zruntime`: uses zbus's built-in ZRuntime (enabling `default-rt`).
 * `zbus-async-tokio`: internally uses the `tokio` executor (enabling the `tokio` feature in `zbus`).

In case that `zbus` is already part of your dependency tree, you may decide to skip this choice and rely on [Cargo features unification mechanism](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification) instead.

Without either runtime feature, provide a runtime through zbus's `Builder::runtime`;
creating a connection without one returns `zbus::Error::Unsupported`.

## Migrating to zbus 6

This source candidate requires Rust 1.89 and zbus 6. It keeps all fourteen service
features, D-Bus names and wire signatures. Its Rust API and runtime changes are
breaking changes:

 * Replace `zbus-async-smol` with `zbus-async-zruntime` to use the new built-in
   ZRuntime, or keep `zbus-async-tokio` for Tokio. ZRuntime is a different executor;
   the former smol backend is not preserved through an alias.
 * Use the `zbus` re-export for common types: `zvariant::Value` and `OwnedValue`
   become `zbus::Variant` and `OwnedVariant`; object paths and file descriptors
   also live at the `zbus` root. Use the `wire` re-export for encoding APIs.
 * zbus 6 unifies wire, name and bus errors under `zbus::Error`. Consult the
   [upstream migration guide](https://z-galaxy.github.io/zbus/upgrading-to-6.html)
   for error matching and property conversion changes.
 * Proxy builder setters such as `path` defer conversion errors to `build`, so
   remove the `?` after the setter and keep error handling on `build().await`.

The package version has not been advanced in this local source candidate. Its
release version and registry publication require the upstream maintainer's
decision; dependents must not treat the existing registry release as this code.

## License

Licensed under either of

 * MIT license - <http://opensource.org/licenses/MIT>
 * Apache License, Version 2.0 - <http://www.apache.org/licenses/LICENSE-2.0>

at your option.
