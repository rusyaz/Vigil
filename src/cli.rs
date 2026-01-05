use std::path;

use clap;

#[derive(clap::Parser,Debug)]
#[command(version,about,long_about = None)]
pub struct Args {
   
    #[arg(short,long,value_parser = extension, required = true)]
    file: path::PathBuf,

    #[arg(short,long)]
    output: Option<String>
}

impl Args {

    pub fn parse() -> Self{
        <Self as clap::Parser>::parse()
    }

     pub fn path(&self) -> &path::PathBuf {
        &self.file
    }
} 

fn extension(ph: &str) -> Result<path::PathBuf,String> {

    let path = path::Path::new(ph);
    
    if !path.is_file() {
        return Err(format!("File '{}' does not exist.",path.display()));
    } 

    if path.extension().is_none_or(|ext| ext != "yaml") {
        return Err(format!("File '{}' must have .yaml extension\n\
                Example: config.yaml, settings.yaml",path.display()))
    }

    Ok(path.to_path_buf())
}
