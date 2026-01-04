use reqwest;
use anyhow;
use futures;


pub struct Checker {
    
    sites: Vec<String>,
}

impl Checker {

    pub fn new(strs: &[String]) -> Self {
        Checker{
            sites: strs.to_vec(),
        } 
    }

    pub async fn check_site(&self,site: &str) -> anyhow::Result<()> {
        let resp = reqwest::get(site).await?;
        let status = resp.status().is_success();
        println!("{status}");
        Ok(())
    }   

    pub async fn check_all_sites(&self) -> Vec<(String,bool)> {
        let client = reqwest::Client::new(); 
        let futures = self.sites.iter().map(|site| {
            let site = site.clone();
            let client = client.clone();
            async move {
                let alive = match client.get(&site).send().await {
                    Ok(resp) => resp.status().is_success(),
                    Err(err) => false,
                };
                (site,alive)
            }
        });

        futures::future::join_all(futures).await
    }
}
