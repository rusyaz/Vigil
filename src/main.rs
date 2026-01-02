use std::any::type_name_of_val;

use clap::Parser;

mod config;
mod cli;

fn main() -> anyhow::Result<()> {

    let args = cli::Args::parse();
    let path = args.path();

    let conf = config::Config::new(path)?;
    let sites = conf.get_sites(); 

    println!("{}",type_name_of_val(&sites));

    for st in sites {
        println!("{}",st);
    }

     Ok(())

}
