use super::data;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::eof;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::Parser;
use nom_language::error::VerboseError;

/// Parse the 'properties:' section of an interface.
pub(crate) fn parse_interface_properties(
    input: &str,
) -> nom::IResult<&str, Vec<data::Property>, VerboseError<&str>> {
    let rest = input;
    // Ensure this is a 'properties:' section.
    let (rest, _) = delimited(multispace0, tag("properties:"), multispace0).parse(rest)?;

    // Split each property apart.
    let (rest, props) = separated_list1(tag(";"), take_till(|b| b == ';')).parse(rest)?;
    eof(rest)?;

    let mut properties = Vec::with_capacity(props.len());
    for entry in props {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (empty, prop) = parse_single_property(trimmed)?;
        eof(empty)?;

        properties.push(prop);
    }
    properties.shrink_to_fit();
    Ok((rest, properties))
}

/// Parse a single property.
fn parse_single_property(input: &str) -> nom::IResult<&str, data::Property, VerboseError<&str>> {
    // Parse line-by-line in reverse order, as all the annotations are leading.
    let mut rev_lines = input.lines().rev();
    let property_def = rev_lines.next().unwrap();

    let (empty, mut prop) = parse_property_definition(property_def)?;
    eof(empty)?;

    // This logic only understands a very limited set of annotations that
    // appear in systemd docs.
    for annotation in rev_lines {
        let rest = annotation;
        let (rest, _) = multispace0(rest)?;
        if rest.starts_with("@org.freedesktop.systemd1.Privileged") {
            continue;
        }
        let (rest, _) = tag("@org.freedesktop.DBus.Property.EmitsChangedSignal")(rest)?;
        let (rest, _) = tag("(\"")(rest)?;
        let (rest, value) =
            alt((tag("const"), tag("false"), tag("invalidates"), tag("true"))).parse(rest)?;
        let (rest, _) = tag("\")")(rest)?;
        eof(rest)?;

        prop.emits_changed_signal = value.to_string();
    }

    Ok(("", prop))
}

/// Parse a property definition.
fn parse_property_definition(
    input: &str,
) -> nom::IResult<&str, data::Property, VerboseError<&str>> {
    let rest = input;
    let (rest, _) = multispace0(rest)?;
    let (rest, rw_label) = alt((tag("readonly"), tag("readwrite"))).parse(rest)?;
    let (rest, _) = multispace1(rest)?;
    let (rest, type_label) = take_till(|b: char| b.is_ascii_whitespace())(rest)?;
    let (rest, _) = multispace1(rest)?;
    let (rest, name) = take_till(|b: char| b.is_ascii_whitespace())(rest)?;
    let (rest, _) = multispace1(rest)?;
    let (rest, _) = (take_while(|_| true))(rest)?;
    eof(rest)?;

    let writable = match rw_label {
        "readonly" => false,
        "readwrite" => true,
        _ => unreachable!(),
    };

    const DEFAULT_CHANGED_SIGNAL: &str = "true";
    let out_prop = data::Property {
        type_label: type_label.to_string(),
        name: name.to_string(),
        writable,
        emits_changed_signal: DEFAULT_CHANGED_SIGNAL.to_string(),
    };
    Ok((rest, out_prop))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;
    use nom_language::error::convert_error;

    #[test]
    fn test_parse_property_definition() {
        let input = "readonly a(iiay) DNS = [...];";
        let (rest, out) = parse_property_definition(input)
            .finish()
            .map_err(|e| panic!("{}", convert_error(input, e)))
            .unwrap();
        assert_eq!(rest, "");
        assert_eq!(out.type_label, "a(iiay)");
        assert_eq!(out.name, "DNS");
        assert_eq!(out.writable, false);
        assert_eq!(out.emits_changed_signal, "true");
    }
}
