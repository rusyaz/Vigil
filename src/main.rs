
mod config;
mod cli;
mod web;
mod app;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
   
    let args = cli::Args::parse();
    
    let app = app::App::new(args.path())?;
    let result = app.run().await;

    for rs in result {
        println!("\n{rs}");
    } 
    
    Ok(())

}
