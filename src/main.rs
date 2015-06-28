extern crate rustc_serialize;
extern crate docopt;
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
	let _ = std::fs::create_dir(directory.as_path());
	
	// Parse source file
	let mut graph = try!(core::parseMetaFile(&source));
	// Print initial
	try!(print_graph(&graph, &mut directory, &file_noextension));
	// Gradient
	try!(graph.gradient());
	try!(print_graph(&graph, &mut directory, &(file_noextension.clone() + "_grad")));
	// Print graphviz gradient
	try!(graph.gradient());
	directory.push(file_noextension.clone() + "_grad.dot");
	Ok(())
}

fn print_graph(graph: & core::graph::ComputeGraph, directory: &mut std::path::PathBuf, name: &String) -> Result<(), ProgramError>{
	// Print cmd graph
	directory.push(name.clone() + ".txt");
	let file = try!(std::fs::File::create(directory.as_path()));
	let mut writer = std::io::BufWriter::new(&file);
	try!(writer.write_fmt(format_args!("{}\n",graph)));
	directory.pop();

	// Print graphviz
	directory.push(name.clone() + ".dot");
	let file = try!(std::fs::File::create(directory.as_path()));
	let mut writer = std::io::BufWriter::new(&file);
	try!(codegen::write_graphviz(&mut writer as &mut std::io::Write, &graph));
	directory.pop();
	Ok(())
}

#[derive(Debug)]
enum ProgramError {
	Io(std::io::Error),
	Parse(core::ParseError),
	Other(String)
}

impl From<std::io::Error> for ProgramError {
	fn from(err: std::io::Error) -> ProgramError {
		ProgramError::Io(err)
	}
}

impl From<core::ParseError> for ProgramError {
	fn from(err: core::ParseError) -> ProgramError {
		ProgramError::Parse(err)
	}
}

impl From<String> for ProgramError {
	fn from(err: String) -> ProgramError {
		ProgramError::Other(err)
	}
}

impl std::fmt::Display for ProgramError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			ProgramError::Io(ref err) => err.fmt(f),
			ProgramError::Parse(ref err) => err.fmt(f),
			ProgramError::Other(ref err) => err.fmt(f),
		}
	}
}
