//! This module contains helpers translated from `systemd` for working with D-Bus object paths.
//! They are re-exported from the crate root.

use std::borrow::Cow;
use std::fmt::Write;
use zbus::zvariant::{ObjectPath, OwnedObjectPath};

/// Escape a label for use in a D-Bus object path.
///
/// Based on `bus_label_escape` from `systemd`.
pub fn bus_label_escape(s: &str) -> Cow<'_, str> {
    if s.is_empty() {
        return Cow::Borrowed("_");
    }

    fn needs_encoding(i: usize, c: u8) -> bool {
        !(c.is_ascii_alphabetic() || i > 0 && c.is_ascii_digit())
    }

    let need_encoding_count = s
        .bytes()
        .enumerate()
        .filter(|(i, c)| needs_encoding(*i, *c))
        .count();
    if need_encoding_count == 0 {
        return Cow::Borrowed(s);
    }

    let mut r = String::with_capacity(s.len() + need_encoding_count * 2);
    for (i, c) in s.bytes().enumerate() {
        if needs_encoding(i, c) {
            _ = write!(&mut r, "_{:02x}", c);
        } else {
            r.push(c as char);
        }
    }

    Cow::Owned(r)
}

/// Unescape a label from a D-Bus object path.
///
/// Based on `bus_label_unescape` from `systemd`.
pub fn bus_label_unescape(f: &str) -> Cow<'_, str> {
    if !f.contains('_') {
        return Cow::Borrowed(f);
    }

    if f == "_" {
        return Cow::Borrowed("");
    }

    let mut r = String::with_capacity(f.len());
    let mut chars = f.chars();
    while let Some(c) = chars.next() {
        if c == '_' {
            let a = chars.next().and_then(|c| c.to_digit(16));
            let b = chars.next().and_then(|c| c.to_digit(16));
            match (a, b) {
                (Some(a), Some(b)) => r.push(((a * 16 + b) as u8) as char),
                // Invalid escape sequence
                _ => r.push('_'),
            }
        } else {
            r.push(c);
        }
    }

    Cow::Owned(r)
}

/// Convert an external identifier into an object path.
///
/// Based on `sd_bus_path_encode` from `systemd`.
pub fn bus_path_encode(prefix: &ObjectPath<'_>, external_id: &str) -> OwnedObjectPath {
    let label = bus_label_escape(external_id);
    ObjectPath::from_string_unchecked(format!("{}/{}", prefix, label)).into()
}

/// Convert an object path into an external identifier.
///
/// Returns `None` if the path is not a child of the prefix.
///
/// Based on `sd_bus_path_decode` from `systemd`.
pub fn bus_path_decode<'a>(
    prefix: &ObjectPath<'_>,
    path: &'a ObjectPath<'a>,
) -> Option<Cow<'a, str>> {
    let path = path.as_str();
    let label = path.strip_prefix(prefix.as_str())?;
    let label = label.strip_prefix('/')?;
    Some(bus_label_unescape(label))
}

/// Escape a unit name
///
/// Based on `unit_name_escape` from `systemd`.
pub fn unit_name_escape(input: &str) -> Cow<'_, str> {
    const VALID_CHARS: &str = r"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz:_.";

    if input.chars().next() != Some('.') && input.chars().all(|c| VALID_CHARS.contains(c)) {
        return Cow::Borrowed(input);
    }

    let mut result = String::with_capacity(input.len());

    let escape_char = |c: char, result: &mut String| {
        use std::fmt::Write;

        result.push('\\');
        result.push('x');
        _ = write!(result, "{:02x}", c as u8);
    };

    let mut chars = input.chars().peekable();

    // Handle leading '.' separately
    if let Some(c) = chars.next_if_eq(&'.') {
        escape_char(c, &mut result);
    }

    for c in chars {
        match c {
            '/' => result.push('-'),
            c if VALID_CHARS.contains(c) => result.push(c),
            c => escape_char(c, &mut result),
        }
    }

    Cow::Owned(result)
}

/// Unescape a unit name
///
/// Based on `unit_name_unescape` from `systemd`.
pub fn unit_name_unescape(f: &str) -> Option<Cow<'_, str>> {
    if f.chars().all(|c| c != '\\' && c != '-') {
        return Some(Cow::Borrowed(f));
    }

    let mut result = String::with_capacity(f.len());
    let mut chars = f.chars();
    while let Some(c) = chars.next() {
        match c {
            '-' => result.push('/'),
            '\\' => {
                if chars.next() != Some('x') {
                    return None;
                }
                let a = chars.next().and_then(|c| c.to_digit(16))?;
                let b = chars.next().and_then(|c| c.to_digit(16))?;
                result.push(((a << 4) | b) as u8 as char);
            }
            _ => result.push(c),
        }
    }

    Some(Cow::Owned(result))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bus_label_escape() {
        assert_eq!(bus_label_escape("1"), "_31");
        assert_eq!(bus_label_escape("hello"), "hello");
        assert_eq!(bus_label_escape(""), "_");
    }

    #[test]
    fn test_bus_label_unescape() {
        assert_eq!(bus_label_unescape("_31"), "1");
        assert_eq!(bus_label_unescape("_3"), "_");
        assert_eq!(bus_label_unescape("hello"), "hello");
        assert_eq!(bus_label_unescape("_"), "");
    }

    const EXAMPLE_NAMESPACE: ObjectPath<'static> =
        ObjectPath::from_static_str_unchecked("/org/freedesktop/network1/link");

    #[test]
    fn test_bus_path_encode() {
        assert_eq!(
            bus_path_encode(&EXAMPLE_NAMESPACE, "1"),
            OwnedObjectPath::from(ObjectPath::from_static_str_unchecked(
                "/org/freedesktop/network1/link/_31"
            ))
        );
        assert_eq!(
            bus_path_encode(&EXAMPLE_NAMESPACE, "huh"),
            OwnedObjectPath::from(ObjectPath::from_static_str_unchecked(
                "/org/freedesktop/network1/link/huh"
            ))
        );
        assert_eq!(
            bus_path_encode(&EXAMPLE_NAMESPACE, ""),
            OwnedObjectPath::from(ObjectPath::from_static_str_unchecked(
                "/org/freedesktop/network1/link/_"
            ))
        );
    }

    #[test]
    fn test_bus_path_decode() {
        assert_eq!(
            bus_path_decode(
                &EXAMPLE_NAMESPACE,
                &"/org/freedesktop/network1/link/_31".try_into().unwrap(),
            ),
            Some("1".into())
        );
        assert_eq!(
            bus_path_decode(
                &EXAMPLE_NAMESPACE,
                &"/org/freedesktop/network1/link/huh".try_into().unwrap(),
            ),
            Some("huh".into())
        );
        assert_eq!(
            bus_path_decode(
                &EXAMPLE_NAMESPACE,
                &"/org/freedesktop/network1/link/_".try_into().unwrap(),
            ),
            Some("".into())
        );
    }

    #[test]
    fn test_unit_name_escape() {
        assert_eq!(unit_name_escape(""), Cow::Borrowed(""));
        assert_eq!(
            unit_name_escape("ab+-c.a/bc@foo.service"),
            Cow::<'static, str>::Owned(r"ab\x2b\x2dc.a-bc\x40foo.service".to_string())
        );
        assert_eq!(
            unit_name_escape("foo.service"),
            Cow::Borrowed("foo.service")
        );
        assert_eq!(
            unit_name_escape(".foo.service"),
            Cow::<'static, str>::Owned(r"\x2efoo.service".to_string())
        );
    }

    #[test]
    fn test_unit_name_unescape() {
        assert_eq!(unit_name_unescape(""), Some(Cow::Borrowed("")));
        assert_eq!(
            unit_name_unescape(r"ab\x2b\x2dc.a-bc\x40foo.service"),
            Some(Cow::Owned("ab+-c.a/bc@foo.service".to_string()))
        );
        assert_eq!(
            unit_name_unescape("foo.service"),
            Some(Cow::Borrowed("foo.service"))
        );
        assert_eq!(
            unit_name_unescape(r"\x2efoo.service"),
            Some(Cow::Owned(".foo.service".to_string()))
        );
        assert_eq!(unit_name_unescape(r"\x2"), None);
    }
}
