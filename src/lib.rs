//! A pure-Rust library to interact with systemd DBus services.
//!
//! ## Async executor
//!
//! This library internally uses `zbus` without any default Cargo features.
//!
//! That means that you have to decide which async executor to use, by enabling one of the following Cargo features:
//! * `zbus-async-smol`: internally uses the `smol` executor (enabling the `async-io` feature in `zbus`).
//! * `zbus-async-tokio`: internally uses the `tokio` executor (enabling the `tokio` feature in `zbus`).
//!
//! ## Auto-generated services
//!
//! Each service has its own dedicated module, which is auto-generated from current
//! systemd definitions and can be activated through the corresponding Cargo feature:
//!
//!  * `home1`: systemd-homed interfaces
//!  * `hostname1`: systemd-hostnamed interfaces
//!  * `import1`: systemd-importd interfaces
//!  * `locale1`: systemd-localed interfaces
//!  * `login1`: systemd-logind interfaces
//!  * `machine1`: systemd-machined interfaces
//!  * `network1`: systemd-networkd interfaces
//!  * `oom1`: systemd-oomd interfaces
//!  * `portable1`: systemd-portabled interfaces
//!  * `resolve1`: systemd-resolved interfaces
//!  * `systemd1`: systemd interfaces
//!  * `sysupdate1`: systemd-sysupdated interfaces
//!  * `timedate1`: systemd-timedated interfaces
//!  * `timesync1`: systemd-timesyncd interfaces

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::too_many_arguments)]

mod helpers;
pub use helpers::*;

pub use zbus;
pub use zbus::names as znames;
pub use zbus::zvariant;

#[cfg_attr(docsrs, doc(cfg(feature = "home1")))]
#[cfg(feature = "home1")]
pub mod home1;

#[cfg_attr(docsrs, doc(cfg(feature = "hostname1")))]
#[cfg(feature = "hostname1")]
pub mod hostname1;

#[cfg_attr(docsrs, doc(cfg(feature = "import1")))]
#[cfg(feature = "import1")]
pub mod import1;

#[cfg_attr(docsrs, doc(cfg(feature = "locale1")))]
#[cfg(feature = "locale1")]
pub mod locale1;

#[cfg_attr(docsrs, doc(cfg(feature = "login1")))]
#[cfg(feature = "login1")]
pub mod login1;

#[cfg_attr(docsrs, doc(cfg(feature = "machine1")))]
#[cfg(feature = "machine1")]
pub mod machine1;

#[cfg_attr(docsrs, doc(cfg(feature = "network1")))]
#[cfg(feature = "network1")]
pub mod network1;

#[cfg_attr(docsrs, doc(cfg(feature = "oom1")))]
#[cfg(feature = "oom1")]
pub mod oom1;

#[cfg_attr(docsrs, doc(cfg(feature = "portable1")))]
#[cfg(feature = "portable1")]
pub mod portable1;

#[cfg_attr(docsrs, doc(cfg(feature = "resolve1")))]
#[cfg(feature = "resolve1")]
pub mod resolve1;

#[cfg_attr(docsrs, doc(cfg(feature = "systemd1")))]
#[cfg(feature = "systemd1")]
pub mod systemd1;

#[cfg_attr(docsrs, doc(cfg(feature = "sysupdate1")))]
#[cfg(feature = "sysupdate1")]
pub mod sysupdate1;

#[cfg_attr(docsrs, doc(cfg(feature = "timedate1")))]
#[cfg(feature = "timedate1")]
pub mod timedate1;

#[cfg_attr(docsrs, doc(cfg(feature = "timesync1")))]
#[cfg(feature = "timesync1")]
pub mod timesync1;
