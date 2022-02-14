use anyhow::Result;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::io::BufRead;

#[derive(Debug, Deserialize)]
pub(crate) struct CodegenConfig {
    #[serde(rename = "service")]
    pub(crate) services: BTreeMap<String, Service>,
}

impl CodegenConfig {
    pub(crate) fn get_service_by_id(&self, id: &str) -> Option<&Service> {
        self.services.values().find(|s| s.id == id)
    }
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

pub(crate) fn parse_toml_config(input: &mut impl BufRead) -> Result<CodegenConfig> {
    let mut content = vec![];
    input.read_to_end(&mut content)?;
    let cfg: CodegenConfig = toml::from_slice(&content)?;
    Ok(cfg)
}
