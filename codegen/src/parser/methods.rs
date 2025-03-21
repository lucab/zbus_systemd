use super::data;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till, take_while};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::eof;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::delimited;
use nom::Parser;
use nom_language::error::VerboseError;

// Parse the 'methods:' section of an interface.
pub(crate) fn parse_interface_methods(
    input: &str,
) -> nom::IResult<&str, Vec<data::Method>, VerboseError<&str>> {
    // Ensure this is a 'methods' section.
    let (rest, _) = delimited(multispace0, tag("methods:"), multispace0).parse(input)?;

    // Split method blocks apart.
    let (rest, blocks) = separated_list1(tag(";"), take_till(|b| b == ';')).parse(rest)?;
    eof(rest)?;

    let mut methods = Vec::with_capacity(blocks.len());
    for entry in blocks {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (empty, m) = parse_single_method(trimmed)?;
        eof(empty)?;

        methods.push(m);
    }
    methods.shrink_to_fit();

    Ok((rest, methods))
}

// Parse a single method.
fn parse_single_method(input: &str) -> nom::IResult<&str, data::Method, VerboseError<&str>> {
    let mut rest = input;

    // Handle annotations.
    let mut annotations = vec![];
    while rest.starts_with('@') {
        let (next, annotation) = take_while(|b| b != '\n')(rest)?;
        annotations.push(annotation);
        rest = next.trim();
    }

    // Extract method name.
    let (rest, method_name) = take_while(|b| b != '(')(rest)?;

    // Extract arguments list.
    let (rest, _) = tag("(")(rest)?;
    let (rest, args_body) = take(rest.len().saturating_sub(1))(rest)?;
    let (rest, _) = tag(")")(rest)?;

    // Parse arguments.
    let (_, args) = interface_method_args(args_body)?;

    let mut method = data::Method {
        name: method_name.to_string(),
        inputs: Vec::with_capacity(args.len()),
        outputs: Vec::with_capacity(args.len()),
    };
    for entry in args {
        let arg_name = entry.2.to_string();
        let signature = entry.1.to_string();
        match entry.0 {
            "in" => method.inputs.push((arg_name, signature)),
            "out" => method.outputs.push((arg_name, signature)),
            x => unreachable!("{}", x),
        };
    }
    method.inputs.shrink_to_fit();
    method.outputs.shrink_to_fit();

    Ok((rest, method))
}

fn interface_method_args(
    text: &str,
) -> nom::IResult<&str, Vec<(&str, &str, &str)>, VerboseError<&str>> {
    let (rest, out) = separated_list0(tag(","), take_till(|b| b == ',')).parse(text)?;
    eof(rest)?;

    let mut result = Vec::with_capacity(out.len());
    for line in out {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (_, arg) = (
            multispace0,
            alt((tag("in"), tag("out"))),
            multispace1,
            take_till(|b: char| b.is_ascii_whitespace()),
            multispace1,
            take_while(|_| true),
        )
            .parse(line)?;
        let entry = (arg.1, arg.3, arg.5);
        result.push(entry);
    }
    result.shrink_to_fit();

    Ok((rest, result))
}
