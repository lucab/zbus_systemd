mod config;
mod generator;
mod parser;

use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;

fn run() -> Result<()> {
    let cfg = config::parse_toml_config("./codegen.toml")?;

    let systemd_man_dir =
        std::fs::read_dir("./codegen/systemd/man").context("Listing systemd manpages")?;

    println!("Processing modules:");
    for entry in systemd_man_dir {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let service_cfg = {
            let fname = entry.file_name().to_string_lossy().into_owned();
            let service_id = fname.trim_end_matches(".xml");
            match cfg.get_service_by_id(service_id) {
                Some(v) => v,
                None => continue,
            }
        };
        println!(" * {}", service_cfg.module);

        let mut xml_input = {
            let fd = File::open(entry.path())
                .with_context(|| format!("Opening {}", entry.path().display()))?;
            BufReader::new(fd)
        };
        let nodes = parser::parse_docxml(&mut xml_input, service_cfg)
            .with_context(|| format!("Parsing XML for '{}'", service_cfg.module))?;

        let mut rust_output = File::create(format!("./src/{}/generated.rs", service_cfg.module))?;
        generator::generate(&mut rust_output, nodes, &cfg.general, &service_cfg)
            .with_context(|| format!("Analyzing '{}'", entry.path().display()))?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error:");
        for entry in e.chain() {
            eprintln!(" - {}", entry);
        }
        std::process::exit(1);
    };
}
