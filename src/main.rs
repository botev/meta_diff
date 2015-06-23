#![feature(path_ext)] 
pub mod core;
pub mod codegen;
extern crate rustc_serialize;
extern crate docopt;

static USAGE: &'static str = "
Usage: meta_diff <source> 
meta_diff --help

Options:
-h, --help     Show this usage message.
";

#[derive(RustcDecodable, Debug)]
struct Args {
	arg_source: String,
}

fn main() {
	let args: Args = docopt::Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
	match main_proxy(args){
		Ok(_) => (),
		Err(err) => {
			use std::io::Write;
			writeln!(&mut std::io::stderr(), "{}", err).unwrap();
			std::process::exit(1);
		}
	}
}

use std::io::{Read,Write};
use std::fs::PathExt;
fn main_proxy(args: Args) ->  Result<(), ProgramError> {
	// Read source file
	let mut source = String::new();
	let path = std::path::Path::new(&args.arg_source);
	let mut file = try!(std::fs::File::open(&path));
	try!(file.read_to_string(&mut source));
	// Set up output directory
	let file_name = try!(path.file_name().ok_or(std::io::Error::new(std::io::ErrorKind::InvalidInput
		, "The file given does not exist or is a directory.")));
	let file_noextension = file_name.to_str().unwrap().split(".").nth(0).unwrap().to_string();
	let mut directory = try!(std::env::current_dir());
	directory.push(file_noextension.clone());
	if !directory.as_path().exists() {
		try!(std::fs::create_dir(directory.as_path()));
	}
	// Parse source file
	let graph = try!(core::parser::metaFile(&source));
	// Print it to file
	directory.push(file_noextension + "_cmd.txt");
	let file = try!(std::fs::File::create(directory.as_path()));
	let mut writer = std::io::BufWriter::new(&file);
	try!(writer.write_fmt(format_args!("{}\n",graph)));
	directory.pop();
	Ok(())
}

#[derive(Debug)]
enum ProgramError {
	Io(std::io::Error),
	Parse(core::parser::ParseError),
}

impl From<std::io::Error> for ProgramError {
	fn from(err: std::io::Error) -> ProgramError {
		ProgramError::Io(err)
	}
}

impl From<core::parser::ParseError> for ProgramError {
	fn from(err: core::parser::ParseError) -> ProgramError {
		ProgramError::Parse(err)
	}
}

impl std::fmt::Display for ProgramError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			ProgramError::Io(ref err) => err.fmt(f),
			ProgramError::Parse(ref err) => err.fmt(f),
		}
	}
}
