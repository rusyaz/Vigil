use reqwest;
use anyhow;


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
}
