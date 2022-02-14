use crate::config::Service;

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) object_name: String,
    pub(crate) path: String,
    pub(crate) interface: String,
    pub(crate) methods: Vec<Method>,
    pub(crate) signals: Vec<Signal>,
    pub(crate) properties: Vec<Property>,
}

impl Node {
    pub(crate) fn struct_name(&self, service: &Service) -> String {
        if !self.object_name.is_empty() {
            self.object_name.clone()
        } else {
            service
                .default_object
                .clone()
                .unwrap_or_else(|| "Service".to_string())
        }
    }
}

#[derive(Debug)]
pub(crate) struct Method {
    pub(crate) name: String,
    pub(crate) inputs: Vec<(String, String)>,
    pub(crate) outputs: Vec<(String, String)>,
}

#[derive(Debug)]
pub(crate) struct Property {
    pub(crate) name: String,
    pub(crate) type_label: String,
    pub(crate) writable: bool,
}

#[derive(Debug)]
pub(crate) struct Signal {
    pub(crate) name: String,
    pub(crate) args: Vec<(String, String)>,
}
