use toml;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct ConfigFile {
    root: String,
}

#[derive(Default)]
pub struct ConfigBuilder {
    config: Option<ConfigFile>,
    ccp_root: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
	Self {..Default::default()}
    }

    pub fn config_path(mut self, path: &str) -> Self {
	match fs::read_to_string(path).map(|s| toml::from_str(s.as_str())) {
	    Ok(Ok(string)) => self.config = Some(string),
	    _ => {}
	}
	self
    }

    pub fn ccp_root(mut self, path: &str) -> Self {
	if let Some(ref file) = self.config {
	    self.ccp_root = Some(file.root.clone());
	    return self;
	}
	self.ccp_root = Some(path.to_string());
	self
    }

    pub fn build(self) -> Config {
	Config {
	    ccp_root: self.ccp_root.unwrap()
	}
    }
}

pub struct Config {
    ccp_root: String,
}

impl Config {
    pub fn ccp_root(&self) -> &str {
	self.ccp_root.as_str()
    }
}
