use std::path; 
use std::fs;


use serde;
use serde_yaml;
use anyhow::{self,Context};

#[derive(serde::Deserialize,Debug)]
pub struct Config {

    sites : Vec<String>,

}

impl Config {

    pub fn new(pt: &path::Path) -> anyhow::Result<Self>{
        let file = fs::read_to_string(pt).with_context(|| format!("failed to open config file {}", pt.display()))?;

        let config : Config = serde_yaml::from_str(&file).context("failed to parse config file.")?;

        Ok(config)
    }

    pub fn get_sites(&self) -> &[String]{
        &self.sites
    }
}
