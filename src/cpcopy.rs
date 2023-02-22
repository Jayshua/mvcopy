use clap::Parser;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(required=true)]
    files: Vec<String>,
}

fn main() {
    let cli = Args::parse();

    let mut temp_file = File::create("/tmp/_cpcopy.txt").unwrap();
    write!(temp_file, "{}\n", std::fs::canonicalize("./").unwrap().display()).unwrap();

    for file in &cli.files {
        println!("{}", file);
        write!(temp_file, "{}\n", file).unwrap();
    }

    if cli.files.len() == 1 {
        println!("Copied 1 file");
    } else {
        println!("Copied {} files", cli.files.len());
    }
}
