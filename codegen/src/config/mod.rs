use anyhow::Result;
use fn_error_context::context;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub(crate) struct CodegenConfig {
    pub(crate) general: General,
    #[serde(rename = "service")]
    pub(crate) services: BTreeMap<String, Service>,
}

impl CodegenConfig {
    pub(crate) fn parse_toml(input: &mut impl BufRead) -> Result<CodegenConfig> {
        let mut content = vec![];
        input.read_to_end(&mut content)?;
        let cfg: CodegenConfig = toml::from_slice(&content)?;
        Ok(cfg)
    }

    pub(crate) fn get_service_by_id(&self, id: &str) -> Option<&Service> {
        self.services.values().find(|s| s.id == id)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct General {
    pub(crate) gen_blocking: bool,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Service {
    pub(crate) default_object: Option<String>,
    pub(crate) hierarchy: String,
    pub(crate) id: String,
    pub(crate) module: String,
    pub(crate) overrides: Option<Vec<ServiceOverride>>,
}

impl Service {
    pub(crate) fn method_overrides(&self) -> HashSet<(String, String)> {
        let mut out = HashSet::new();
        if let Some(overrides) = &self.overrides {
            for service in overrides {
                if let Some(todo) = &service.todo_methods {
                    for name in todo {
                        out.insert((service.interface.to_string(), name.to_string()));
                    }
                }
            }
        }
        out
    }

    pub(crate) fn property_overrides(&self) -> HashSet<(String, String)> {
        let mut out = HashSet::new();
        if let Some(overrides) = &self.overrides {
            for service in overrides {
                if let Some(todo) = &service.todo_properties {
                    for name in todo {
                        out.insert((service.interface.to_string(), name.to_string()));
                    }
                }
            }
        }
        out
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ServiceOverride {
    pub(crate) interface: String,
    pub(crate) todo_methods: Option<Vec<String>>,
    pub(crate) todo_signals: Option<Vec<String>>,
    pub(crate) todo_properties: Option<Vec<String>>,
}

#[context("Parsing configuration file {}", path.as_ref().display())]
pub(crate) fn parse_toml_config(path: impl AsRef<Path>) -> Result<CodegenConfig> {
    let tomlfile = File::open("./codegen.toml")?;
    let mut bufrd = BufReader::new(tomlfile);
    CodegenConfig::parse_toml(&mut bufrd)
}
