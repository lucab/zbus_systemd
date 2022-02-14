use super::data;
use super::interface::parse_single_interface;
use crate::config::Service;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{char, multispace0, multispace1};
use nom::character::is_space;
use nom::combinator::eof;
use nom::multi::many1;
use nom::sequence::tuple;

pub(crate) fn parse_single_node<'a>(
    text: &'a str,
    service: &Service,
) -> nom::IResult<&'a str, data::Node> {
    let (path, interfaces) = {
        let mut parser = tuple((node_start, many1(parse_single_interface), node_end, eof));
        let (_, out) = parser(text)?;
        (out.0, out.1)
    };

    for entry in interfaces {
        let (iface_name, methods, signals, properties) = entry;
        let object_name = iface_name
            .trim_start_matches(&service.id)
            .trim_start_matches('.')
            .to_string();
        let node = data::Node {
            object_name,
            path: path.to_string(),
            interface: iface_name.to_string(),
            methods,
            signals,
            properties,
        };
        return Ok(("", node));
    }
    
    unreachable!("failed to parse node");
}

/// Match node start and return its name.
fn node_start(text: &str) -> nom::IResult<&str, &str> {
    let (rest, out) = tuple((
        multispace0,
        tag("node"),
        multispace1,
        take_till(|b| is_space(b as u8)),
        multispace0,
        char('{'),
        multispace0,
    ))(text)?;
    Ok((rest, out.3))
}

fn node_end(text: &str) -> nom::IResult<&str, ()> {
    let (rest, _) = tuple((multispace0, char('}'), multispace0, char(';'), multispace0))(text)?;
    Ok((rest, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_start() {
        let input = " node foo { \n";
        let (rest, out) = node_start(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(out, "foo");
    }

    #[test]
    fn test_node() {
        let input = " node foo { \n";
        let (rest, out) = node_start(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(out, "foo");
    }

    #[test]
    fn test_node_end() {
        let input = " \n } ; \n";
        let (rest, out) = node_end(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(out, ());
    }
}
