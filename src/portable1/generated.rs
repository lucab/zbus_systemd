// This file is autogenerated, do not manually edit.

use crate::zbus::proxy;

/// Proxy object for `org.freedesktop.portable1.Manager`.
#[proxy(
    interface = "org.freedesktop.portable1.Manager",
    gen_blocking = false,
    default_service = "org.freedesktop.portable1",
    default_path = "/org/freedesktop/portable1"
)]
pub trait Manager {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImage()) Call interface method `GetImage`.
    #[zbus(name = "GetImage")]
    fn get_image(&self, image: String) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ListImages()) Call interface method `ListImages`.
    #[zbus(name = "ListImages")]
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

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageOSRelease()) Call interface method `GetImageOSRelease`.
    #[zbus(name = "GetImageOSRelease")]
    fn get_image_os_release(
        &self,
        image: String,
    ) -> crate::zbus::Result<::std::collections::HashMap<String, String>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageMetadata()) Call interface method `GetImageMetadata`.
    #[zbus(name = "GetImageMetadata")]
    fn get_image_metadata(
        &self,
        image: String,
        matches: Vec<String>,
    ) -> crate::zbus::Result<(
        String,
        Vec<u8>,
        ::std::collections::HashMap<String, Vec<u8>>,
    )>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageMetadataWithExtensions()) Call interface method `GetImageMetadataWithExtensions`.
    #[zbus(name = "GetImageMetadataWithExtensions")]
    fn get_image_metadata_with_extensions(
        &self,
        image: String,
        extensions: Vec<String>,
        matches: Vec<String>,
        flags: u64,
    ) -> crate::zbus::Result<(
        String,
        Vec<u8>,
        ::std::collections::HashMap<String, Vec<u8>>,
        ::std::collections::HashMap<String, Vec<u8>>,
    )>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageState()) Call interface method `GetImageState`.
    #[zbus(name = "GetImageState")]
    fn get_image_state(&self, image: String) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageStateWithExtensions()) Call interface method `GetImageStateWithExtensions`.
    #[zbus(name = "GetImageStateWithExtensions")]
    fn get_image_state_with_extensions(
        &self,
        image: String,
        extensions: Vec<String>,
        flags: u64,
    ) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#AttachImage()) Call interface method `AttachImage`.
    #[zbus(name = "AttachImage")]
    fn attach_image(
        &self,
        image: String,
        matches: Vec<String>,
        profile: String,
        runtime: bool,
        copy_mode: String,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#AttachImageWithExtensions()) Call interface method `AttachImageWithExtensions`.
    #[zbus(name = "AttachImageWithExtensions")]
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
    #[zbus(name = "DetachImage")]
    fn detach_image(
        &self,
        image: String,
        runtime: bool,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#DetachImageWithExtensions()) Call interface method `DetachImageWithExtensions`.
    #[zbus(name = "DetachImageWithExtensions")]
    fn detach_image_with_extensions(
        &self,
        image: String,
        extensions: Vec<String>,
        flags: u64,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ReattachImage()) Call interface method `ReattachImage`.
    #[zbus(name = "ReattachImage")]
    fn reattach_image(
        &self,
        image: String,
        matches: Vec<String>,
        profile: String,
        runtime: bool,
        copy_mode: String,
    ) -> crate::zbus::Result<(Vec<(String, String, String)>, Vec<(String, String, String)>)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ReattachImageWithExtensions()) Call interface method `ReattachImageWithExtensions`.
    #[zbus(name = "ReattachImageWithExtensions")]
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
    #[zbus(name = "RemoveImage")]
    fn remove_image(&self, image: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#MarkImageReadOnly()) Call interface method `MarkImageReadOnly`.
    #[zbus(name = "MarkImageReadOnly")]
    fn mark_image_read_only(&self, image: String, read_only: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetImageLimit()) Call interface method `SetImageLimit`.
    #[zbus(name = "SetImageLimit")]
    fn set_image_limit(&self, image: String, limit: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetPoolLimit()) Call interface method `SetPoolLimit`.
    #[zbus(name = "SetPoolLimit")]
    fn set_pool_limit(&self, limit: u64) -> crate::zbus::Result<()>;

    /// Get property `PoolPath`.
    #[zbus(property(emits_changed_signal = "false"), name = "PoolPath")]
    fn pool_path(&self) -> crate::zbus::Result<String>;

    /// Get property `PoolUsage`.
    #[zbus(property(emits_changed_signal = "false"), name = "PoolUsage")]
    fn pool_usage(&self) -> crate::zbus::Result<u64>;

    /// Get property `PoolLimit`.
    #[zbus(property(emits_changed_signal = "false"), name = "PoolLimit")]
    fn pool_limit(&self) -> crate::zbus::Result<u64>;

    /// Get property `Profiles`.
    #[zbus(property(emits_changed_signal = "false"), name = "Profiles")]
    fn profiles(&self) -> crate::zbus::Result<Vec<String>>;
}

/// Proxy object for `org.freedesktop.portable1.Image`.
#[proxy(
    interface = "org.freedesktop.portable1.Image",
    gen_blocking = false,
    default_service = "org.freedesktop.portable1",
    default_path = "/org/freedesktop/portable1"
)]
pub trait Image {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetOSRelease()) Call interface method `GetOSRelease`.
    #[zbus(name = "GetOSRelease")]
    fn get_os_release(&self) -> crate::zbus::Result<::std::collections::HashMap<String, String>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetMetadata()) Call interface method `GetMetadata`.
    #[zbus(name = "GetMetadata")]
    fn get_metadata(
        &self,
        matches: Vec<String>,
    ) -> crate::zbus::Result<(
        String,
        Vec<u8>,
        ::std::collections::HashMap<String, Vec<u8>>,
    )>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetMetadataWithExtensions()) Call interface method `GetMetadataWithExtensions`.
    #[zbus(name = "GetMetadataWithExtensions")]
    fn get_metadata_with_extensions(
        &self,
        extensions: Vec<String>,
        matches: Vec<String>,
        flags: u64,
    ) -> crate::zbus::Result<(
        String,
        Vec<u8>,
        ::std::collections::HashMap<String, Vec<u8>>,
        ::std::collections::HashMap<String, Vec<u8>>,
    )>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetState()) Call interface method `GetState`.
    #[zbus(name = "GetState")]
    fn get_state(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetStateWithExtensions()) Call interface method `GetStateWithExtensions`.
    #[zbus(name = "GetStateWithExtensions")]
    fn get_state_with_extensions(
        &self,
        extensions: Vec<String>,
        flags: u64,
    ) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Attach()) Call interface method `Attach`.
    #[zbus(name = "Attach")]
    fn attach(
        &self,
        matches: Vec<String>,
        profile: String,
        runtime: bool,
        copy_mode: String,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#AttachWithExtensions()) Call interface method `AttachWithExtensions`.
    #[zbus(name = "AttachWithExtensions")]
    fn attach_with_extensions(
        &self,
        extensions: Vec<String>,
        matches: Vec<String>,
        profile: String,
        copy_mode: String,
        flags: u64,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Detach()) Call interface method `Detach`.
    #[zbus(name = "Detach")]
    fn detach(&self, runtime: bool) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#DetachWithExtensions()) Call interface method `DetachWithExtensions`.
    #[zbus(name = "DetachWithExtensions")]
    fn detach_with_extensions(
        &self,
        extensions: Vec<String>,
        flags: u64,
    ) -> crate::zbus::Result<Vec<(String, String, String)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Reattach()) Call interface method `Reattach`.
    #[zbus(name = "Reattach")]
    fn reattach(
        &self,
        matches: Vec<String>,
        profile: String,
        runtime: bool,
        copy_mode: String,
    ) -> crate::zbus::Result<(Vec<(String, String, String)>, Vec<(String, String, String)>)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ReattachWithExtensions()) Call interface method `ReattachWithExtensions`.
    #[zbus(name = "ReattachWithExtensions")]
    fn reattach_with_extensions(
        &self,
        extensions: Vec<String>,
        matches: Vec<String>,
        profile: String,
        copy_mode: String,
        flags: u64,
    ) -> crate::zbus::Result<(Vec<(String, String, String)>, Vec<(String, String, String)>)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Remove()) Call interface method `Remove`.
    #[zbus(name = "Remove")]
    fn remove(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#MarkReadOnly()) Call interface method `MarkReadOnly`.
    #[zbus(name = "MarkReadOnly")]
    fn mark_read_only(&self, read_only: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLimit()) Call interface method `SetLimit`.
    #[zbus(name = "SetLimit")]
    fn set_limit(&self, limit: u64) -> crate::zbus::Result<()>;

    /// Get property `Name`.
    #[zbus(property(emits_changed_signal = "false"), name = "Name")]
    fn name(&self) -> crate::zbus::Result<String>;

    /// Get property `Path`.
    #[zbus(property(emits_changed_signal = "false"), name = "Path")]
    fn path(&self) -> crate::zbus::Result<String>;

    /// Get property `Type`.
    #[zbus(property(emits_changed_signal = "false"), name = "Type")]
    fn type_property(&self) -> crate::zbus::Result<String>;

    /// Get property `ReadOnly`.
    #[zbus(property(emits_changed_signal = "false"), name = "ReadOnly")]
    fn read_only(&self) -> crate::zbus::Result<bool>;

    /// Get property `CreationTimestamp`.
    #[zbus(property(emits_changed_signal = "false"), name = "CreationTimestamp")]
    fn creation_timestamp(&self) -> crate::zbus::Result<u64>;

    /// Get property `ModificationTimestamp`.
    #[zbus(
        property(emits_changed_signal = "false"),
        name = "ModificationTimestamp"
    )]
    fn modification_timestamp(&self) -> crate::zbus::Result<u64>;

    /// Get property `Usage`.
    #[zbus(property(emits_changed_signal = "false"), name = "Usage")]
    fn usage(&self) -> crate::zbus::Result<u64>;

    /// Get property `Limit`.
    #[zbus(property(emits_changed_signal = "false"), name = "Limit")]
    fn limit(&self) -> crate::zbus::Result<u64>;

    /// Get property `UsageExclusive`.
    #[zbus(property(emits_changed_signal = "false"), name = "UsageExclusive")]
    fn usage_exclusive(&self) -> crate::zbus::Result<u64>;

    /// Get property `LimitExclusive`.
    #[zbus(property(emits_changed_signal = "false"), name = "LimitExclusive")]
    fn limit_exclusive(&self) -> crate::zbus::Result<u64>;
}
