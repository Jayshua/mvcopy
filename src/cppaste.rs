use std::fs::File;
use std::path::PathBuf;
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

	let data_file = File::open("/tmp/_cpcopy.txt").unwrap();
	let data_file_reader = BufReader::new(data_file);
	let mut files_iter = data_file_reader.lines().map(|x| x.unwrap());
	let prefix = PathBuf::from(files_iter.next().unwrap());

	for suffix in files_iter {
		let full_source = prefix.join(&suffix);
		let full_destination = destination.join(&suffix);

		fs_extra::copy_items(&[full_source], &full_destination, &fs_extra::dir::CopyOptions {
			overwrite: false,
			skip_exist: false,
			copy_inside: true,
			content_only: false,
			..Default::default()
		})
		.unwrap();

		println!("{}", full_destination.display());
	}
}