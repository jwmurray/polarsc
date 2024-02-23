/*
* CLI interface for Polars.  calls lib.rs
*/
use clap::Parser;
use polars::prelude::*;
use polarsc::read_csv;

#[derive(Parser)]
#[command(author, version, about, long_about = "A CLI tool that wrpas Polars code.")]
struct Cli {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let _args = Cli::parse();
    let result = read_csv().unwrap();
    println!("{:?}", result);

}
