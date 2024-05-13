# zbus_systemd

[![crates.io](https://img.shields.io/crates/v/zbus_systemd.svg)](https://crates.io/crates/zbus_systemd)
[![Documentation](https://docs.rs/zbus_systemd/badge.svg)](https://docs.rs/zbus_systemd)

A pure-Rust library to interact with systemd DBus services.

`zbus_systemd` provides support for interacting with the suite of systemd
services over DBus. This crate tries to cover all systemd interfaces,
across all services.

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
 * `timedate1`: systemd-timedated interfaces (org.freedesktop.timedate1)

For a quickstart on how to use those interfaces, see the [examples](https://github.com/lucab/zbus_systemd/tree/main/examples).

## License

Licensed under either of

 * MIT license - <http://opensource.org/licenses/MIT>
 * Apache License, Version 2.0 - <http://www.apache.org/licenses/LICENSE-2.0>

at your option.
