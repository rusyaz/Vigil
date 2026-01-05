use reqwest;
use anyhow;
use futures::{self, future::join_all};

use std::time;

use crate::web::CheckResult; 

pub struct Checker {
    timeout: u64, 
    client: reqwest::Client,
    sites: Vec<String>,
}

impl Checker {

    pub fn new(strs: &[String],tout: u64) -> Self {
        Checker{
            timeout: tout,
            client: reqwest::Client::new(),
            sites: strs.to_vec(),
        } 
    }

    pub async fn check_site(&self, site: &str) -> anyhow::Result<CheckResult> {
        let resp = self.client.get(site)
            .timeout(time::Duration::from_millis(self.timeout))
            .send()
            .await?;
        Ok(CheckResult::from_response(resp))
    }   

    pub async fn check_all_sites(&self) -> Vec<CheckResult> {
        let futures = self.sites.iter().map(|site|{
            let site = site.clone();
            let client = self.client.clone();
            async move {
                let alive = match client.get(&site)
                    .timeout(time::Duration::from_millis(self.timeout))
                    .send().await {
                        Ok(resp) => CheckResult::from_response(resp),
                        Err(err) =>CheckResult::from_error(err,&site),
                    };
                alive
            }
        }); 
        join_all(futures).await
    }
}

