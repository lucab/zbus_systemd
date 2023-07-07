/// This module implements translating DBus data types to Rust. More details can be found at:
/// https://dbus.freedesktop.org/doc/dbus-specification.html
use anyhow::Result;
use fn_error_context::context;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    multi::many1,
    sequence::{delimited, pair, preceded},
    Finish, IResult, Parser,
};

#[context("Translating DBus type '{}'", sig)]
pub fn translate_sig(sig: &str) -> Result<String> {
    if sig.is_empty() {
        return Ok("".to_string());
    }

    let (_, result) = parse_type(sig)
        .map_err(|e| e.map_input(|input| input.to_string()))
        .finish()?;
    Ok(result)
}

fn parse_type(input: &str) -> IResult<&str, String> {
    alt((parse_simple, parse_tuple, parse_dictionary, parse_array)).parse(input)
}

fn parse_simple(input: &str) -> IResult<&str, String> {
    map(
        alt((
            value("bool", tag("b")),
            value("f64", tag("d")),
            value("crate::zvariant::OwnedFd", tag("h")),
            value("i32", tag("i")),
            value("crate::zvariant::OwnedObjectPath", tag("o")),
            value("u16", tag("q")),
            value("String", tag("s")),
            value("u64", tag("t")),
            value("u32", tag("u")),
            value("crate::zvariant::OwnedValue", tag("v")),
            value("i64", tag("x")),
            value("u8", tag("y")),
        )),
        |out| out.to_string(),
    )
    .parse(input)
}

fn parse_tuple(input: &str) -> IResult<&str, String> {
    map(delimited(tag("("), many1(parse_type), tag(")")), |t| {
        format!("({},)", t.join(", "))
    })
    .parse(input)
}

fn parse_dictionary(input: &str) -> IResult<&str, String> {
    map(
        delimited(tag("a{"), pair(parse_type, parse_type), tag("}")),
        |(key, value)| format!("::std::collections::HashMap<{}, {}>", key, value),
    )
    .parse(input)
}

fn parse_array(input: &str) -> IResult<&str, String> {
    map(preceded(tag("a"), parse_type), |t| format!("Vec<{}>", t)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_sig() {
        let ok_cases = &[
            ("", ""),
            ("a{ss}", "::std::collections::HashMap<String, String>"),
            ("b", "bool"),
            ("a(iiayqs)", "Vec<(i32, i32, Vec<u8>, u16, String,)>"),
            (
                "a(sa(sv))",
                "Vec<(String, Vec<(String, crate::zvariant::OwnedValue,)>,)>",
            ),
        ];
        for (input, expected) in ok_cases {
            let output = translate_sig(input).expect(&format!("test case {}", input));
            assert_eq!(&output, expected, "test case {}", input);
        }
    }

    #[test]
    fn test_translate_array_plain() {
        let ok_cases = &[("ab", "Vec<bool>")];
        for (input, expected) in ok_cases {
            let output = translate_sig(input).unwrap();
            assert_eq!(&output, expected);
        }
    }
}
