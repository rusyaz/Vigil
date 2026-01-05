use reqwest::{self,Response,Error};

use std::fmt;

#[derive(Debug)]
pub struct CheckResult {
    url: String,
    status : SiteStatus,
    message : String,
}


impl CheckResult {
 /*   pub fn new(rsp: Result<Response,Error>) -> Self {
        match rsp {
            Ok(resp) =>Self::from_response(resp),
            Err(err) =>Self::from_error(err, site),
        }  
    } */

    pub fn from_response(resp: Response) -> CheckResult {
        if resp.status().is_success(){
            return CheckResult{
                url: resp.url().to_string(),
                status: SiteStatus::Available,
                message: String::from("The site is available and everything looks good."),
            };
        }

        if resp.status().is_server_error() {
            return CheckResult {
                url: resp.url().to_string(),
                status: SiteStatus::ServerError,
                message: String::from("Oops! Server error detected."),
            };
        }

        CheckResult {
            url: resp.url().to_string(),
            status: SiteStatus::Other,
            message: String::from("The site returned an unexpected status."),
        }
    }

    pub fn from_error(err: Error,site: &str) -> CheckResult {
        if err.is_timeout() {
            return CheckResult{
                url: site.to_string(),
                status: SiteStatus::Timeout,
                message: String::from("The request timed out."),
            };
        }
        
        CheckResult {
            url: site.to_string(),
            status: SiteStatus::Other,
            message: String::from("The site returned an unexpected status."),

        }
    }
}

impl fmt::Display for CheckResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}\n    {}",
            self.status,   // красивый статус с эмодзи
            self.url,      // URL сайта
            self.message   // текстовое сообщение
        )
    }
}


#[derive(Debug)]
pub enum SiteStatus {
    Available,
    ServerError,
    Timeout,
    Other,
}

impl fmt::Display for SiteStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SiteStatus::Available => "✅ Available",
            SiteStatus::Timeout => "⏱ Timeout",
            SiteStatus::ServerError => "💥 Server Error",
            SiteStatus::Other => "❓ Other",
        };
        write!(f, "{}", s)
    }
}

     
