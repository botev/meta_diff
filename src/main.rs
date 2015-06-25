#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
pub mod core;
pub mod codegen;

static USAGE: &'static str = "Meta Diff

Usage: 
meta_diff <source> 
meta_diff --help
meta_diff --version

Options:
-h --help     Show this usage message.
-v --version  Show the version and exit.
";

#[derive(RustcDecodable, Debug)]
struct Args {
	arg_source: String,
	flag_version: bool
}

fn main() {
	let args: Args = docopt::Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
	if args.flag_version{
		println!("Meta Diff version {}.{}.{}",0,0,1);
		std::process::exit(0);
	}

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
//use std::fs::PathExt;
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
	// if !directory.as_path().exists() {
		// try!(std::fs::create_dir(directory.as_path()));
	// }
	let _ = std::fs::create_dir(directory.as_path());
	// Parse source file
	let graph = try!(core::parser::metaFile(&source));
	// Print cmd
	directory.push(file_noextension.clone() + ".txt");
	let file = try!(std::fs::File::create(directory.as_path()));
	let mut writer = std::io::BufWriter::new(&file);
	try!(writer.write_fmt(format_args!("{}\n",graph)));
	directory.pop();
	// Print graphviz
	directory.push(file_noextension.clone() + ".dot");
	let file = try!(std::fs::File::create(directory.as_path()));
	let mut writer = std::io::BufWriter::new(&file);
	try!(codegen::graphviz::write_graphviz(&mut writer as &mut std::io::Write, &graph));
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
