mod config;
mod generator;
mod parser;

use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    #[structopt(flatten)]
    rustmft: codegenrs::RustfmtArgs,
}

fn run() -> Result<()> {
    let cfg = {
        let tomlfile = std::fs::File::open("./codegen.toml")?;
        let mut bufrd = BufReader::new(tomlfile);
        config::parse_toml_config(&mut bufrd)?
    };

    let systemd_man_dir = std::fs::read_dir("./codegen/systemd/man")?;
    for entry in systemd_man_dir {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let fname = entry.file_name().to_string_lossy().into_owned();
        let service_id = fname.trim_end_matches(".xml");
        let service = match cfg.get_service_by_id(service_id) {
            Some(n) => n,
            None => continue,
        };

        println!("Generating module '{}'.", service.module);

        let mut xml_input = {
            let fd = File::open(entry.path())?;
            BufReader::new(fd)
        };

        let nodes = parser::parse_docxml(&mut xml_input, service)?;
        let mut rust_output =
            std::fs::File::create(format!("./src/{}/generated.rs", service.module))?;

        generator::generate(&mut rust_output, nodes, &service)
            .with_context(|| format!("Analyzing {}", fname))?;
    }

    /*
    let mut content = vec![];

    let content = String::from_utf8(content)?;
    let content = options
        .rustmft
        .reformat(&content)
        .map_err(|e| anyhow::format_err!("{}", e))?;
     */

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        for entry in e.chain() {
            eprintln!("Error: {}", entry);
        }
        std::process::exit(1);
    };
}
