use super::data;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::{alpha1, multispace0, multispace1};
use nom::character::is_space;
use nom::combinator::eof;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};

// Parse the 'properties:' section of an interface.
pub(crate) fn parse_interface_properties(input: &str) -> nom::IResult<&str, Vec<data::Property>> {
    // Ensure this is a 'properties:' section.
    let (rest, _) = delimited(multispace0, tag("properties:"), multispace0)(input)?;

    // Split each property apart.
    let (rest, props) = separated_list1(tag(";"), take_till(|b| b == ';'))(rest)?;
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

// Parse a single property.
fn parse_single_property(input: &str) -> nom::IResult<&str, data::Property> {
    let mut rev_lines = input.lines().rev();
    let property_def = rev_lines.next().unwrap();

    let (empty, mut prop) = parse_property_definition(property_def)?;
    eof(empty)?;

    for annotation in rev_lines {
        // NOTE(lucab): this only supports a limited set of annotations.
        let trimmed = annotation.trim_start();
        if trimmed.starts_with("@org.freedesktop.systemd1.Privileged") {
            continue;
        }
        let (rest, _) = tag("@org.freedesktop.DBus.Property.EmitsChangedSignal")(trimmed)?;
        let (rest, value) = tuple((tag("(\""), alpha1, tag("\")")))(rest)?;
        match value.1 {
            "const" | "false" | "invalidates" | "true" => {
                prop.emits_changed_signal = value.1.to_string()
            }
            x => {
                tag("emits_changed_signal")(x)?;
            }
        };
        eof(rest)?;
    }

    Ok(("", prop))
}

// Parse a property definition.
fn parse_property_definition(input: &str) -> nom::IResult<&str, data::Property> {
    let rest = input;
    let (rest, sig) = tuple((
        multispace0,
        alt((tag("readonly"), tag("readwrite"))),
        multispace1,
        take_till(|b| is_space(b as u8)),
        multispace1,
        take_till(|b| is_space(b as u8)),
        take_while(|_| true),
    ))(rest)?;
    eof(rest)?;

    let writable = match sig.1 {
        "readonly" => false,
        "readwrite" => true,
        _ => unreachable!(),
    };

    let out_prop = data::Property {
        type_label: sig.3.to_string(),
        name: sig.5.to_string(),
        writable,
        emits_changed_signal: "true".to_string(),
    };
    Ok((rest, out_prop))
}
