// This file is autogenerated, do not manually edit.

use zbus::dbus_proxy;

/// Proxy object for `org.freedesktop.locale1`.
#[dbus_proxy(
    interface = "org.freedesktop.locale1",
    gen_blocking = false,
    default_service = "org.freedesktop.locale1",
    default_path = "/org/freedesktop/locale1"
)]
trait Localed {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLocale()) Call interface method `SetLocale`.
    #[dbus_proxy(name = "SetLocale")]
    fn set_locale(&self, locale: Vec<String>, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetVConsoleKeyboard()) Call interface method `SetVConsoleKeyboard`.
    #[dbus_proxy(name = "SetVConsoleKeyboard")]
    fn set_v_console_keyboard(
        &self,
        keymap: String,
        keymap_toggle: String,
        convert: bool,
        interactive: bool,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetX11Keyboard()) Call interface method `SetX11Keyboard`.
    #[dbus_proxy(name = "SetX11Keyboard")]
    fn set_x11_keyboard(
        &self,
        layout: String,
        model: String,
        variant: String,
        options: String,
        convert: bool,
        interactive: bool,
    ) -> crate::zbus::Result<()>;

    /// Get property `Locale`.
    #[dbus_proxy(property, name = "Locale")]
    fn locale(&self) -> crate::zbus::Result<Vec<String>>;

    /// Get property `X11Layout`.
    #[dbus_proxy(property, name = "X11Layout")]
    fn x11_layout(&self) -> crate::zbus::Result<String>;

    /// Get property `X11Model`.
    #[dbus_proxy(property, name = "X11Model")]
    fn x11_model(&self) -> crate::zbus::Result<String>;

    /// Get property `X11Variant`.
    #[dbus_proxy(property, name = "X11Variant")]
    fn x11_variant(&self) -> crate::zbus::Result<String>;

    /// Get property `X11Options`.
    #[dbus_proxy(property, name = "X11Options")]
    fn x11_options(&self) -> crate::zbus::Result<String>;

    /// Get property `VConsoleKeymap`.
    #[dbus_proxy(property, name = "VConsoleKeymap")]
    fn v_console_keymap(&self) -> crate::zbus::Result<String>;

    /// Get property `VConsoleKeymapToggle`.
    #[dbus_proxy(property, name = "VConsoleKeymapToggle")]
    fn v_console_keymap_toggle(&self) -> crate::zbus::Result<String>;
}
