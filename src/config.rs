use std::path; 
use std::fs;


use serde;
use serde_yaml;
use anyhow::{self,Context};

#[derive(serde::Deserialize,Debug)]
pub struct Config {

    timeout : u64,

    sites : Vec<String>,

}

impl Config {

    pub fn new(pt: &path::Path) -> anyhow::Result<Self> {
        let file = fs::read_to_string(pt).with_context(|| format!("failed to open config file {}", pt.display()))?;

        let config : Config = serde_yaml::from_str(&file).context("failed to parse config file.")?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> anyhow::Result<()>{
        if self.sites.is_empty() {
            anyhow::bail!("Validation error: 'sites' field is empty. \
             Please provide at least one site URL, e.g. ['https://example.com'].");
        }

        if self.timeout < 1000 || self.timeout > 10_000 {
            anyhow::bail!("Validation error: 'timeout' has invalid value {}. \
             Allowed range is 1000..=10000 ms (1–10 секунд).",
            self.timeout)

        }
        Ok(())
    } 

    pub fn get_sites(&self) -> &[String]{
        &self.sites
    }

    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }

    
}
