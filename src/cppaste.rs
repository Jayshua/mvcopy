use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    destination: Option<String>,
}



fn main() {
	let args = Args::parse();
	let destination = args.destination.as_deref().unwrap_or(".");
	let destination = std::fs::canonicalize(destination).unwrap();

	let file = File::open("/tmp/_cpcopy.txt").unwrap();
	let file = BufReader::new(file);
	let files = file.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

	fs_extra::copy_items(&files, destination, &fs_extra::dir::CopyOptions {
		overwrite: false,
		skip_exist: false,
		copy_inside: true,
		content_only: false,
		..Default::default()
	});

	if files.len() == 1 {
		println!("Pasted 1 file");
	} else {
		println!("Pasted {} files", files.len());
	}
}