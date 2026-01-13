use std::io::Write;
use std::path;
use std::fs;
use std::time;

use anyhow;
use chrono;

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
        let time = std::time::Instant::now();

        let results = self.checker.check_all_sites().await;

        let duration = time.elapsed();

        self.logging(&results,duration);
        results
    }

    fn logging(&self,results: &[web::CheckResult],duration: std::time::Duration) {
        fs::create_dir_all("logs").ok();

        let now = chrono::Local::now();
        let filename = format!("logs/scan_{}",now.format("%Y-%m-%d_%H-%M-%S"));
        let mut file = fs::File::create(&filename).expect("Cannot create a file.");

        for r in results {
            let line = format!(
                "{} | Scanned: {:<30} | Status: {}\n",
                now.format("%Y-%m-%d %H:%M:%S"),
                r.url(),
                r.status()
            );

            file.write_all(line.as_bytes()).ok();
        }

        let total = results.len();
        let available = results.iter().filter(|r| matches!(r.status(),web::model::SiteStatus::Available)).count();
        let timeout = results.iter().filter(|r| matches!(r.status(),web::model::SiteStatus::Timeout)).count();
        let errors = total - available;

        let summary = format!(
            "\n===== Summary =====\n\
            Total sites: {}\n\
            ✅ Available: {}\n\
            ⏱ Timeout: {}\n\
            💥 Errors: {}\n\
            Duration: {:.2?}\n\
            ====================\n",
            total, available, timeout, errors, duration
        );

       print!("{}",summary);
       file.write_all(summary.as_bytes()).ok();

    }
    
}
