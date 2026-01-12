use reqwest;
use futures::{self, future::join_all};

use std::time;

use crate::web::CheckResult; 

#[derive(Debug)]
pub struct Checker {
    timeout: Option<u64>, 
    client: reqwest::Client,
    sites: Vec<String>,
}

impl Checker {

    pub fn new(inp:(Vec<String>,u64)) -> Self {
        Self{
            timeout: if inp.1 == 0 {None} else {Some(inp.1)},
            client: reqwest::Client::new(),
            sites: inp.0,
        } 
    }

    pub async fn check_all_sites(&self) -> Vec<CheckResult> {
        match self.timeout {
            Some(value) => self.check_all_with_timeout().await, 
            None => self.check_all_without_timeout().await,
        }
    }

    // Unused for now. The Checker operates on multiple sites.
    
    /*   pub async fn check_site(&self, site: &str) -> anyhow::Result<CheckResult> {
        let resp = self.client.get(site)
            .timeout(time::Duration::from_millis(self.timeout))
            .send()
            .await?;
        Ok(CheckResult::from_response(resp))
    }   */

    async fn check_all_with_timeout(&self) -> Vec<CheckResult> {
        let futures = self.sites.iter().map(|site|{
            let site = site.clone();
            let client = self.client.clone();
            async move {
                let alive = match client.get(&site)
                    .timeout(time::Duration::from_millis(self.timeout.expect("Timeout should exist.")))
                    .send().await {
                        Ok(resp) => CheckResult::from_response(resp),
                        Err(err) => CheckResult::from_error(err,&site),
                    };
                alive
            }
        }); 
        join_all(futures).await
    }

    async fn check_all_without_timeout(&self) -> Vec<CheckResult> {
        let futures = self.sites.iter().map(|site|{
            let site = site.clone();
            let client = self.client.clone();
            async move {
                let alive = match client.get(&site)
                    .send()
                    .await {
                        Ok(resp) => CheckResult::from_response(resp),
                        Err(err) => CheckResult::from_error(err,&site)

                    };
                   alive
            }
        });
        join_all(futures).await
    }
}

