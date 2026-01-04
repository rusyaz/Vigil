
mod config;
mod cli;
mod web;



#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let args = cli::Args::parse();
    let path = args.path();

    let conf = config::Config::new(path)?;
    let sites = conf.get_sites(); 

    let cr = web::Checker::new(sites);
    let result = cr.check_all_sites().await;

    for r in result {
        println!("{}:{}",r.0,r.1);
    }
    Ok(())

}
