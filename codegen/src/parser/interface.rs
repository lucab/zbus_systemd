use super::{data, methods, properties, signals};
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till, take_until};
use nom::character::complete::{char, multispace0, multispace1};
use nom::character::is_space;
use nom::combinator::eof;
use nom::sequence::{delimited, tuple};
use nom::FindSubstring;

/// Parse a single interface and return its name and methods/properties/signals.
pub(crate) fn parse_single_interface(
    rest: &str,
) -> nom::IResult<
    &str,
    (
        &str,
        Vec<data::Method>,
        Vec<data::Signal>,
        Vec<data::Property>,
    ),
> {
    let (rest, iface_name) = interface_start(rest)?;

    let (rest, methods, signals, properties) = {
        let (rest, content) = take_until("};")(rest)?;
        let (_, out) = alt((parse_dummy_content, parse_interface_content))(content)?;
        (rest, out.0, out.1, out.2)
    };

    let (rest, _) = interface_end(rest)?;

    Ok((rest, (iface_name, methods, signals, properties)))
}

/// Match interface block start, and return its name.
fn interface_start(rest: &str) -> nom::IResult<&str, &str> {
    let mut parser = tuple((
        multispace0,
        tag("interface"),
        multispace1,
        take_till(|b| is_space(b as u8)),
        multispace0,
        char('{'),
        multispace0,
    ));
    let (rest, out) = parser(rest)?;
    Ok((rest, out.3))
}

/// Match interface block end.
fn interface_end(text: &str) -> nom::IResult<&str, ()> {
    let (rest, _) = tuple((char('}'), char(';'), multispace0))(text)?;
    Ok((rest, ()))
}

/// Parse a dummy interface block (`[ ... ]`).
fn parse_dummy_content(
    rest: &str,
) -> nom::IResult<&str, (Vec<data::Method>, Vec<data::Signal>, Vec<data::Property>)> {
    let (rest, _) = delimited(multispace0, tag("..."), multispace0)(rest)?;
    let out = (vec![], vec![], vec![]);
    Ok((rest, out))
}
/// Parse interface content.
///
/// Content blocks are ordered as:
///  1. methods (optional)
///  2. signals (optional)
///  3. properties (optional)
fn parse_interface_content(
    text: &str,
) -> nom::IResult<&str, (Vec<data::Method>, Vec<data::Signal>, Vec<data::Property>)> {
    let block_end = text.len();
    let mut rest = text;

    // Properties slice.
    let props_start = text.find_substring("properties:");
    let props_end = block_end;
    // Signals slice.
    let sigs_start = text.find_substring("signals:");
    let sigs_end = props_start.unwrap_or(block_end);
    // Methods slice.
    let methods_start = text.find_substring("methods:");
    let methods_end = sigs_start
        .unwrap_or(block_end)
        .min(props_start.unwrap_or(block_end));

    // Parse 'methods:' section, if present.
    let mut methods = vec![];
    if let Some(start) = methods_start {
        let count = methods_end.saturating_sub(start);
        let out = take(count)(rest)?;
        rest = out.0;
        let (_, parsed) = methods::parse_interface_methods(out.1)?;
        methods = parsed;
    }

    // Parse 'signals:' section, if present.
    let mut signals = vec![];
    if let Some(start) = sigs_start {
        let count = sigs_end.saturating_sub(start);
        let out = take(count)(rest)?;
        rest = out.0;
        let (_, parsed) = signals::parse_interface_signals(out.1)?;
        signals = parsed;
    }

    // Parse 'properties:' section, if present.
    let mut properties = vec![];
    if let Some(start) = props_start {
        let count = props_end.saturating_sub(start);
        let out = take(count)(rest)?;
        rest = out.0;
        let (_, parsed) = properties::parse_interface_properties(out.1)?;
        properties = parsed;
    }

    // Ensure all the content has been parsed.
    eof(rest)?;

    Ok((rest, (methods, signals, properties)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_content_dummy() {
        let input = " ... ";
        let (rest, out) = parse_dummy_content(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(out, (vec![], vec![], vec![]));
    }
}
