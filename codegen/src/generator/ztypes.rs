use anyhow::{bail, ensure, Result};
use fn_error_context::context;

#[context("Translating DBus type '{}'", sig)]
pub(crate) fn translate_sig(sig: &str) -> Result<String> {
    let (out, rest) = match sig.len() {
        0 => translate_void(sig)?,
        1 => translate_simple(sig)?,
        _ => translate_composite(sig)?,
    };

    if !rest.is_empty() {
        bail!("Trailing unparsed leftover '{}'", rest);
    }

	Ok(out)
}

pub(crate) fn translate_void(sig: &str) -> Result<(String, &str)> {
    ensure!(sig.len() == 0);

    let out = "()".to_string();
    Ok((out, ""))
}

pub(crate) fn is_next_simple(sig: &str) -> bool {
    translate_simple(sig).is_ok()
}

pub(crate) fn translate_simple(sig: &str) -> Result<(String, &str)> {
    ensure!(sig.len() > 0);

    let (first, rest) = sig.split_at(1);
    let rtype = match first {
        "b" => "bool",
        "d" => "f64",
        "h" => "crate::zvariant::OwnedFd",
        "i" => "i32",
        "o" => "crate::zvariant::OwnedObjectPath",
        "q" => "u16",
        "s" => "String",
        "t" => "u64",
        "u" => "u32",
        "v" => "crate::zvariant::OwnedValue",
        "x" => "i64",
        "y" => "u8",
        x => bail!("Invalid plain type '{}'", x),
    };
    Ok((rtype.to_string(), rest))
}

pub(crate) fn translate_composite(sig: &str) -> Result<(String, &str)> {
    ensure!(sig.len() > 1);

    let (first, rest) = sig.split_at(1);
    match first {
        "a" => translate_container(rest),
        "(" => translate_tuple_body(rest),
        x => bail!("Unexpect starting character '{}'", x),
    }
}

pub(crate) fn translate_container(sig: &str) -> Result<(String, &str)> {
    if sig.is_empty() {
        bail!("Invalid container");
    }

    if is_next_simple(sig) {
        return translate_array_plain(sig);
    }

    let (first, rest) = sig.split_at(1);
    match first {
        "(" => translate_array_tuple(rest),
        x => bail!("Invalid container '{}'", x),
    }
}

pub(crate) fn translate_tuple_body(sig: &str) -> Result<(String, &str)> {
    let mut rest = sig;

    let mut out = "(".to_string();
    while !rest.starts_with(")") {
        if is_next_simple(rest) {
            let (next, subrest) = translate_simple(rest)?;
            out.push_str(&next);
            out.push(',');
            rest = subrest;
        } else {
            let (next, subrest) = translate_composite(rest)?;
            out.push_str(&next);
            out.push(',');
            rest = subrest;
        }
    }
    out.push(')');
    rest = rest.trim_start_matches(')');
    Ok((out, rest))
}

pub(crate) fn translate_array_plain(sig: &str) -> Result<(String, &str)> {
    let (plain, rest) = translate_simple(sig)?;
    let out = format!("Vec<{}>", plain);
    Ok((out, rest))
}

pub(crate) fn translate_array_tuple(sig: &str) -> Result<(String, &str)> {
    let (body, rest) = translate_tuple_body(sig)?;
    let out = format!("Vec<{}>", body);
    Ok((out, rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_sig() {
        let ok_cases = &[
            ("b", "bool"),
            ("a(iiayqs)", "Vec<(i32,i32,Vec<u8>,u16,String,)>"),
        ];
        for (input, expected) in ok_cases {
            let output = translate_sig(input).unwrap();
            assert_eq!(&output, expected);
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
