use std::fs::File;
use clap::Parser;
use flate2::read::GzDecoder;
use tar::Archive;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// web, h5, d-web, d-h5; d means dinghuo
    #[arg(short, long)]
    term: String,

    /// name of page build output
    #[arg(short, long)]
    module: String,

    /// version of page
    #[arg(short, long)]
    version: String,

    /// local app name
    #[arg(short, long)]
    app: String,
}

fn main() -> Result<(), std::io::Error> {
    // let args = Args::parse();


    let path = "C:\\Users\\qc\\Downloads\\qince-h5-apaas-v7.2.20-20240513192331.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}

fn read_zip(zip_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {

    // Open the zip file
    let file = std::fs::File::open(zip_file_path)?;
    let mut archive = zip::read::ZipArchive::new(file)?;

    // Iterate over each file in the zip archive
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        println!("File name: {}", file_name);
    }

    Ok(())
}