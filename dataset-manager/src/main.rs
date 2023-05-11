extern crate clap;

use std::{env};
use std::path::PathBuf;

use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli
{
    #[arg(short, long)]
    origin: Option<PathBuf>,

    #[arg(short, long)]
    destination: Option<PathBuf>,
}

fn main() {
    let cli: Cli = Cli::parse();

    let origin = cli.origin
        .or_else(|| env::current_dir().ok())
        .expect("Could not get the current directory");

    let destination = cli.destination
        .or_else(|| env::current_dir().ok())
        .expect("Could not get the current directory");

    create_csv_from_directory(origin, destination)
}

fn create_csv_from_directory(origin: PathBuf, destination: PathBuf) 
{
    /*
        - Get tree of files from origin
        - First level is the types of datasets (test, train and validation) 
        - Each type have a list of directories (one for each type of clothing)
        - Each directory have a list of images
    */
    println!("Origin: {:?}", origin);
    println!("Destination: {:?}", destination);
}
