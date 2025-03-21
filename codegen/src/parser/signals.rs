use super::data;
use nom::bytes::complete::{tag, take, take_till, take_while};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::eof;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::delimited;
use nom::Parser;
use nom_language::error::VerboseError;

// Parse the 'signal:' section of an interface.
pub(crate) fn parse_interface_signals(
    input: &str,
) -> nom::IResult<&str, Vec<data::Signal>, VerboseError<&str>> {
    // Ensure this is a 'signals' section.
    let (rest, _) = delimited(multispace0, tag("signals:"), multispace0).parse(input)?;

    // Split each signal apart.
    let (rest, out) = separated_list1(tag(";"), take_till(|b| b == ';')).parse(rest)?;
    eof(rest)?;

    let mut signals = Vec::with_capacity(out.len());
    for line in out {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (empty, s) = parse_single_signal(trimmed)?;
        eof(empty)?;

        signals.push(s);
    }
    signals.shrink_to_fit();

    Ok((rest, signals))
}

// Parse a single signal.
fn parse_single_signal(rest: &str) -> nom::IResult<&str, data::Signal, VerboseError<&str>> {
    // Extract signal name.
    let (rest, signal_name) = take_while(|b| b != '(')(rest)?;

    // Extract arguments list.
    let (rest, _) = tag("(")(rest)?;
    let (rest, args_body) = take(rest.len().saturating_sub(1))(rest)?;
    let (rest, _) = tag(")")(rest)?;

    // Parse arguments.
    let (empty, args) = parse_signal_args(args_body)?;
    eof(empty)?;

    let signal = data::Signal {
        name: signal_name.to_string(),
        args,
    };

    Ok((rest, signal))
}

// Parse signal arguments.
fn parse_signal_args(input: &str) -> nom::IResult<&str, Vec<(String, String)>, VerboseError<&str>> {
    let (rest, out) = separated_list0(tag(","), take_till(|b| b == ',')).parse(input)?;
    eof(rest)?;

    let mut result = Vec::with_capacity(out.len());
    for line in out {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (empty, arg) = (
            multispace0,
            take_till(|b: char| b.is_ascii_whitespace()),
            multispace1,
            take_while(|_| true),
        )
            .parse(line)?;
        eof(empty)?;

        let entry = (arg.1.to_string(), arg.3.to_string());
        result.push(entry);
    }
    result.shrink_to_fit();

    Ok((rest, result))
}
