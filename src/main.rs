use peroxide::fuga::*;
use clap::Parser;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the command line arguments
    // - `-o`: output parquet file
    // - `-h`: help
    // - `-v`: version
    // - Other arguement is the input netcdf file
    let args = CliArgs::parse();

    // Parse the command line arguments
    let input_file = args.input_file;
    if !input_file.exists() {
        eprintln!("Input file does not exist: {:?}", input_file);
        std::process::exit(1);
    }
    let output_file = match args.output_file {
        Some(path) => path,
        None => {
            let mut path = input_file.clone();
            path.set_extension("parquet");
            path
        }
    };
    let compression = args.compression.unwrap_or("default".to_string());

    // Read the netcdf file
    println!("[INFO] Reading netcdf file: {:?}", input_file);
    let df = DataFrame::read_nc(input_file.to_str().unwrap())?;

    // Print the dataframe
    df.print();

    // Write the dataframe to a parquet file
    println!("[INFO] Writing parquet file: {:?}", output_file);
    let compression = match compression.as_str() {
        "default" => CompressionOptions::Uncompressed,
        "snappy" => CompressionOptions::Snappy,
        _ => {
            eprintln!("Unsupported compression algorithm: {}\nCurrently, only default or snappy are allowed", compression);
            std::process::exit(1);
        }
    };
    df.write_parquet(output_file.to_str().unwrap(), compression)?;
    println!("[INFO] Successfully wrote parquet file: {:?}", output_file);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    input_file: PathBuf,
    #[clap(short = 'o', long = "output", value_name = "FILE", help = "Output parquet file")]
    output_file: Option<PathBuf>,
    #[clap(short = 'c', long = "compression", value_name = "COMPRESSION", help = "Compression algorithm")]
    compression: Option<String>,
}
