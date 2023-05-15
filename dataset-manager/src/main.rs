use std::{env};
use std::path::PathBuf;

use clap::{Parser};
use glob::glob;
use serde::Serialize;
use csv;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    origin: Option<PathBuf>,

    #[arg(short, long)]
    destination: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct Row<'a> {
    file_name: &'a str,
    label: &'a str,
    size: Size,
    kids: bool,
}

#[derive(Debug, Serialize)]
enum Size {
    XS,
    S,
    M,
    L,
    XL
}

impl Distribution<Size> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Size {
        match rng.gen_range(0..=4) {
            0 => Size::XS,
            1 => Size::S,
            2 => Size::M,
            3 => Size::L,
            4 => Size::XL,
            _ => panic!("Invalid size")
        }
    }
}

fn main() {
    let cli: Cli = Cli::parse();

    let origin = cli.origin
        .or_else(|| env::current_dir().ok())
        .expect("Could not get the current directory");

    if !origin.is_dir() {
        panic!("Origin must be a directory");
    }

    let mut destination = cli.destination
        .or_else(|| env::current_dir().ok())
        .expect("Could not get the current directory");

    if !destination.is_file() {
        destination.push("data.csv");
    }

    create_csv_from_directory(origin, destination)
}

fn create_csv_from_directory(origin: PathBuf, destination: PathBuf) {

    let origin = origin.into_os_string().into_string().unwrap();

    let pattern = format!("{}/**/*.jpg", origin);

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(&destination.as_path())
        .unwrap();

    glob(&pattern).unwrap().for_each(|path| {

        let path: PathBuf = path.unwrap();
        let row: Row = Row {
            file_name: path.
                file_stem().unwrap().to_str().unwrap(),
            label: path
                .parent().unwrap()
                .components()
                .last().unwrap()
                .as_os_str().to_str().unwrap(),
            size: rand::random(),
            kids: rand::random(),
        };
        
        writer.serialize(row).unwrap();
    });
}

#[cfg(test)]
mod tests {

}
