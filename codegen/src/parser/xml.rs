use super::{data, node};
use crate::config;
use anyhow::{bail, format_err, Context, Result};
use nom::{error::convert_error, Finish};
use std::io::BufRead;

/// Parse the XML docs for systemd DBus services.
pub(crate) fn parse_docxml(
    input: &mut impl BufRead,
    service: &config::Service,
) -> Result<Vec<data::Node>> {
    let root = xmltree::Element::parse(input).context("failed to parse XML")?;

    let mut nodes = vec![];
    for node in &root.children {
        let section = match node.as_element().filter(|elem| elem.name == "refsect1") {
            Some(sect) => sect,
            None => continue,
        };

        for entry in &section.children {
            let elem = entry
                .as_element()
                .filter(|elem| elem.name == "programlisting")
                //.filter(|elem| elem.attributes.get("node") == Some(&service.hierarchy))
                ;
            let (text, _name) = match elem {
                Some(v) => (
                    v.get_text().unwrap_or_default(),
                    v.attributes.get("interface").cloned().unwrap_or_default(),
                ),
                None => continue,
            };

            let parsed = parse_dbus_block(&text, service)?;
            nodes.push(parsed);
        }
    }

    if nodes.is_empty() {
        bail!("no DBus definitions found")
    }

    Ok(nodes)
}

/// Parse a DBus text block into a node.
fn parse_dbus_block(text: &str, service: &config::Service) -> Result<data::Node> {
    let (rest, node) = node::parse_single_node(text, service)
        .finish()
        .map_err(|e| {
            format_err!(
                "failed to parse DBus description, parsing error:\n{}",
                convert_error(text, e)
            )
        })?;

    if !rest.is_empty() {
        bail!("unparsed trailing data: {}", rest);
    }

    Ok(node)
}
