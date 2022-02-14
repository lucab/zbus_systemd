use super::data;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till, take_while};
use nom::character::complete::{multispace0, multispace1};
use nom::character::is_space;
use nom::combinator::eof;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, tuple};

// Parse the 'methods:' section of an interface.
pub(crate) fn parse_interface_methods(input: &str) -> nom::IResult<&str, Vec<data::Method>> {
    // Ensure this is a 'methods' section.
    let (rest, _) = delimited(multispace0, tag("methods:"), multispace0)(input)?;

    // Split each method apart.
    let (rest, out) = separated_list1(tag(";"), take_till(|b| b == ';'))(rest)?;
    eof(rest)?;

    let mut methods = vec![];
    for line in out {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (empty, m) = parse_single_method(trimmed)?;
        eof(empty)?;

        methods.push(m);
    }

    Ok((rest, methods))
}

// Parse a single method.
fn parse_single_method(rest: &str) -> nom::IResult<&str, data::Method> {
    // Extract method name.
    let (rest, method_name) = take_while(|b| b != '(')(rest)?;

    // Extract arguments list.
    let (rest, _) = tag("(")(rest)?;
    let (rest, args_body) = take(rest.len().saturating_sub(1))(rest)?;
    let (rest, _) = tag(")")(rest)?;

    // Parse arguments.
    let (_, args) = interface_method_args(args_body)?;

    // FIXME(lucab): this results in the wrong DBus call.
    //  Push the workaround to the generator instead.
    let name = match method_name {
        "Ref" => "Reference",
        x => x,
    }
    .to_string();

    let mut method = data::Method {
        name,
        inputs: vec![],
        outputs: vec![],
    };

    for entry in args {
        // 'type' is a reserved Rust keyword. We substitute that with 'typelabel' instead.
        let arg_name = match entry.2 {
            "type" => "typelabel",
            x => x,
        }
        .to_string();
        let signature = entry.1.to_string();
        if entry.0 == "in" {
            method.inputs.push((arg_name, signature))
        } else if entry.0 == "out" {
            method.outputs.push((arg_name, signature))
        };
    }

    Ok((rest, method))
}

fn interface_method_args(text: &str) -> nom::IResult<&str, Vec<(&str, &str, &str)>> {
    let (rest, out) = separated_list0(tag(","), take_till(|b| b == ','))(text)?;
    eof(rest)?;

    let mut result = vec![];
    for line in out {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (_, arg) = tuple((
            multispace0,
            alt((tag("in"), tag("out"))),
            multispace1,
            take_till(|b| is_space(b as u8)),
            multispace1,
            take_while(|_| true),
        ))(line)?;
        let entry = (arg.1, arg.3, arg.5);
        result.push(entry);
    }

    Ok((rest, result))
}
