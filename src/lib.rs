//! ⚠️ This is an in-progress `v0.0` Proof-of-Concept, do not rely on it.
//!
//! A pure-Rust library to interact with systemd DBus services.

#![cfg_attr(docsrs, feature(doc_cfg))]

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

#[cfg_attr(docsrs, doc(cfg(feature = "timedate1")))]
#[cfg(feature = "timedate1")]
pub mod timedate1;
