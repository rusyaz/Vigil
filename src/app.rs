
use std::path;
use anyhow;

use crate::web;
use crate::config;


#[derive(Debug)]
pub struct App {
    checker : web::Checker,
}

impl App {

    pub fn new(path: &path::Path) -> anyhow::Result<Self>  {
        let config = config::Config::new(path)?;
        
        let chcr = web::Checker::new(config.into_parts());

        Ok(App{
            checker: chcr,
        }) 
    }

    pub async fn run(&self) -> Vec<web::CheckResult>{
        self.checker.check_all_sites().await 
    }
    
}
