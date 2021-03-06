// This file is autogenerated, do not manually edit.

use zbus::dbus_proxy;

/// Proxy object for `org.freedesktop.timedate1`.
#[dbus_proxy(
    interface = "org.freedesktop.timedate1",
    gen_blocking = false,
    default_service = "org.freedesktop.timedate1",
    default_path = "/org/freedesktop/timedate1"
)]
trait Timedated {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetTime()) Call interface method `SetTime`.
    #[dbus_proxy(name = "SetTime")]
    fn set_time(&self, usec_utc: i64, relative: bool, interactive: bool)
        -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetTimezone()) Call interface method `SetTimezone`.
    #[dbus_proxy(name = "SetTimezone")]
    fn set_timezone(&self, timezone: String, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLocalRTC()) Call interface method `SetLocalRTC`.
    #[dbus_proxy(name = "SetLocalRTC")]
    fn set_local_rtc(
        &self,
        local_rtc: bool,
        fix_system: bool,
        interactive: bool,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetNTP()) Call interface method `SetNTP`.
    #[dbus_proxy(name = "SetNTP")]
    fn set_ntp(&self, use_ntp: bool, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ListTimezones()) Call interface method `ListTimezones`.
    #[dbus_proxy(name = "ListTimezones")]
    fn list_timezones(&self) -> crate::zbus::Result<Vec<String>>;

    /// Get property `Timezone`.
    #[dbus_proxy(property, name = "Timezone")]
    fn timezone(&self) -> crate::zbus::Result<String>;

    /// Get property `LocalRTC`.
    #[dbus_proxy(property, name = "LocalRTC")]
    fn local_rtc(&self) -> crate::zbus::Result<bool>;

    /// Get property `CanNTP`.
    #[dbus_proxy(property, name = "CanNTP")]
    fn can_ntp(&self) -> crate::zbus::Result<bool>;

    /// Get property `NTP`.
    #[dbus_proxy(property, name = "NTP")]
    fn ntp(&self) -> crate::zbus::Result<bool>;

    /// Get property `NTPSynchronized`.
    #[dbus_proxy(property, name = "NTPSynchronized")]
    fn ntp_synchronized(&self) -> crate::zbus::Result<bool>;

    /// Get property `TimeUSec`.
    #[dbus_proxy(property, name = "TimeUSec")]
    fn time_u_sec(&self) -> crate::zbus::Result<u64>;

    /// Get property `RTCTimeUSec`.
    #[dbus_proxy(property, name = "RTCTimeUSec")]
    fn rtc_time_u_sec(&self) -> crate::zbus::Result<u64>;
}
