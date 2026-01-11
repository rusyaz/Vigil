
mod config;
mod cli;
mod web;



#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let args = cli::Args::parse();
    let path = args.path();

    let conf = config::Config::new(path)?;
    let sites = conf.get_sites(); 
    let timout = conf.get_timeout();


    let cr = web::Checker::new(sites,timout);
    
    let test = cr.check_all_sites().await;

    for tt in test {
        println!("{}",tt);
    }
    

     Ok(())

}
