// This file is autogenerated, do not manually edit.

use zbus::dbus_proxy;

/// Proxy object for `org.freedesktop.portable1.Manager`.
#[cfg_attr(
    feature = "blocking",
    dbus_proxy(
        interface = "org.freedesktop.portable1.Manager",
        gen_blocking = true,
        default_service = "org.freedesktop.portable1",
        default_path = "/org/freedesktop/portable1",
    )
)]
#[cfg_attr(
    not(feature = "blocking"),
    dbus_proxy(
        interface = "org.freedesktop.portable1.Manager",
        gen_blocking = false,
        default_service = "org.freedesktop.portable1",
        default_path = "/org/freedesktop/portable1",
    )
)]
trait Manager {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImage()) Call interface method `GetImage`.
    #[dbus_proxy(name = "GetImage")]
    fn get_image(&self, image: String) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ListImages()) Call interface method `ListImages`.
    #[dbus_proxy(name = "ListImages")]
    fn list_images(
        &self,
    ) -> crate::zbus::Result<
        Vec<(
            String,
            String,
            bool,
            u64,
            u64,
            u64,
            String,
            crate::zvariant::OwnedObjectPath,
        )>,
    >;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageState()) Call interface method `GetImageState`.
    #[dbus_proxy(name = "GetImageState")]
    fn get_image_state(&self, image: String) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageStateWithExtensions()) Call interface method `GetImageStateWithExtensions`.
    #[dbus_proxy(name = "GetImageStateWithExtensions")]
    fn get_image_state_with_extensions(
        &self,
        image: String,
        extensions: Vec<String>,
        flags: u64,
    ) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#AttachImage()) Call interface method `AttachImage`.
    #[dbus_proxy(name = "AttachImage")]
    fn attach_image(
        &self,
        image: String,
        matches: Vec<String>,
        profile: String,
        runtime: bool,
        copy_mode: String,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#AttachImageWithExtensions()) Call interface method `AttachImageWithExtensions`.
    #[dbus_proxy(name = "AttachImageWithExtensions")]
    fn attach_image_with_extensions(
        &self,
        image: String,
        extensions: Vec<String>,
        matches: Vec<String>,
        profile: String,
        copy_mode: String,
        flags: u64,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#DetachImage()) Call interface method `DetachImage`.
    #[dbus_proxy(name = "DetachImage")]
    fn detach_image(
        &self,
        image: String,
        runtime: bool,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#DetachImageWithExtensions()) Call interface method `DetachImageWithExtensions`.
    #[dbus_proxy(name = "DetachImageWithExtensions")]
    fn detach_image_with_extensions(
        &self,
        image: String,
        extensions: Vec<String>,
        flags: u64,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ReattachImage()) Call interface method `ReattachImage`.
    #[dbus_proxy(name = "ReattachImage")]
    fn reattach_image(
        &self,
        image: String,
        matches: Vec<String>,
        profile: String,
        runtime: bool,
        copy_mode: String,
    ) -> crate::zbus::Result<(Vec<(String, String, String)>, Vec<(String, String, String)>)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ReattachImageWithExtensions()) Call interface method `ReattachImageWithExtensions`.
    #[dbus_proxy(name = "ReattachImageWithExtensions")]
    fn reattach_image_with_extensions(
        &self,
        image: String,
        extensions: Vec<String>,
        matches: Vec<String>,
        profile: String,
        copy_mode: String,
        flags: u64,
    ) -> crate::zbus::Result<(Vec<(String, String, String)>, Vec<(String, String, String)>)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#RemoveImage()) Call interface method `RemoveImage`.
    #[dbus_proxy(name = "RemoveImage")]
    fn remove_image(&self, image: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#MarkImageReadOnly()) Call interface method `MarkImageReadOnly`.
    #[dbus_proxy(name = "MarkImageReadOnly")]
    fn mark_image_read_only(&self, image: String, read_only: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetImageLimit()) Call interface method `SetImageLimit`.
    #[dbus_proxy(name = "SetImageLimit")]
    fn set_image_limit(&self, image: String, limit: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetPoolLimit()) Call interface method `SetPoolLimit`.
    #[dbus_proxy(name = "SetPoolLimit")]
    fn set_pool_limit(&self, limit: u64) -> crate::zbus::Result<()>;

    /// Get property `PoolPath`.
    #[dbus_proxy(property, name = "PoolPath")]
    fn pool_path(&self) -> crate::zbus::Result<String>;

    /// Get property `PoolUsage`.
    #[dbus_proxy(property, name = "PoolUsage")]
    fn pool_usage(&self) -> crate::zbus::Result<u64>;

    /// Get property `PoolLimit`.
    #[dbus_proxy(property, name = "PoolLimit")]
    fn pool_limit(&self) -> crate::zbus::Result<u64>;

    /// Get property `Profiles`.
    #[dbus_proxy(property, name = "Profiles")]
    fn profiles(&self) -> crate::zbus::Result<Vec<String>>;
}

/// Proxy object for `org.freedesktop.portable1.Image`.
#[cfg_attr(
    feature = "blocking",
    dbus_proxy(
        interface = "org.freedesktop.portable1.Image",
        gen_blocking = true,
        default_service = "org.freedesktop.portable1",
        default_path = "/org/freedesktop/portable1",
    )
)]
#[cfg_attr(
    not(feature = "blocking"),
    dbus_proxy(
        interface = "org.freedesktop.portable1.Image",
        gen_blocking = false,
        default_service = "org.freedesktop.portable1",
        default_path = "/org/freedesktop/portable1",
    )
)]
trait Image {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetState()) Call interface method `GetState`.
    #[dbus_proxy(name = "GetState")]
    fn get_state(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetStateWithExtensions()) Call interface method `GetStateWithExtensions`.
    #[dbus_proxy(name = "GetStateWithExtensions")]
    fn get_state_with_extensions(
        &self,
        extensions: Vec<String>,
        flags: u64,
    ) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Attach()) Call interface method `Attach`.
    #[dbus_proxy(name = "Attach")]
    fn attach(
        &self,
        matches: Vec<String>,
        profile: String,
        runtime: bool,
        copy_mode: String,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#AttachWithExtensions()) Call interface method `AttachWithExtensions`.
    #[dbus_proxy(name = "AttachWithExtensions")]
    fn attach_with_extensions(
        &self,
        extensions: Vec<String>,
        matches: Vec<String>,
        profile: String,
        copy_mode: String,
        flags: u64,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Detach()) Call interface method `Detach`.
    #[dbus_proxy(name = "Detach")]
    fn detach(&self, runtime: bool) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#DetachWithExtensions()) Call interface method `DetachWithExtensions`.
    #[dbus_proxy(name = "DetachWithExtensions")]
    fn detach_with_extensions(
        &self,
        extensions: Vec<String>,
        flags: u64,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Reattach()) Call interface method `Reattach`.
    #[dbus_proxy(name = "Reattach")]
    fn reattach(
        &self,
        matches: Vec<String>,
        profile: String,
        runtime: bool,
        copy_mode: String,
    ) -> crate::zbus::Result<(Vec<(String, String, String)>, Vec<(String, String, String)>)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ReattacheWithExtensions()) Call interface method `ReattacheWithExtensions`.
    #[dbus_proxy(name = "ReattacheWithExtensions")]
    fn reattache_with_extensions(
        &self,
        extensions: Vec<String>,
        matches: Vec<String>,
        profile: String,
        copy_mode: String,
        flags: u64,
    ) -> crate::zbus::Result<(Vec<(String, String, String)>, Vec<(String, String, String)>)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Remove()) Call interface method `Remove`.
    #[dbus_proxy(name = "Remove")]
    fn remove(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#MarkReadOnly()) Call interface method `MarkReadOnly`.
    #[dbus_proxy(name = "MarkReadOnly")]
    fn mark_read_only(&self, read_only: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLimit()) Call interface method `SetLimit`.
    #[dbus_proxy(name = "SetLimit")]
    fn set_limit(&self, limit: u64) -> crate::zbus::Result<()>;

    /// Get property `Name`.
    #[dbus_proxy(property, name = "Name")]
    fn name(&self) -> crate::zbus::Result<String>;

    /// Get property `Path`.
    #[dbus_proxy(property, name = "Path")]
    fn path(&self) -> crate::zbus::Result<String>;

    /// Get property `ReadOnly`.
    #[dbus_proxy(property, name = "ReadOnly")]
    fn read_only(&self) -> crate::zbus::Result<bool>;

    /// Get property `CreationTimestamp`.
    #[dbus_proxy(property, name = "CreationTimestamp")]
    fn creation_timestamp(&self) -> crate::zbus::Result<u64>;

    /// Get property `ModificationTimestamp`.
    #[dbus_proxy(property, name = "ModificationTimestamp")]
    fn modification_timestamp(&self) -> crate::zbus::Result<u64>;

    /// Get property `Usage`.
    #[dbus_proxy(property, name = "Usage")]
    fn usage(&self) -> crate::zbus::Result<u64>;

    /// Get property `Limit`.
    #[dbus_proxy(property, name = "Limit")]
    fn limit(&self) -> crate::zbus::Result<u64>;

    /// Get property `UsageExclusive`.
    #[dbus_proxy(property, name = "UsageExclusive")]
    fn usage_exclusive(&self) -> crate::zbus::Result<u64>;

    /// Get property `LimitExclusive`.
    #[dbus_proxy(property, name = "LimitExclusive")]
    fn limit_exclusive(&self) -> crate::zbus::Result<u64>;
}
