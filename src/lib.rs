/// A flag for enabling certain form of verification inside the different modules. This is similar to a debugging flag, but will guarantee semantic checks in each module, where needed.
///
/// Until the library has proven to be stable it is required it to always be set to `true`
const VERIFICATION: bool = true;

pub mod core;
pub mod optimization;
pub mod codegen;

use std::io::Write;
pub fn print_graph(graph: &core::ComputeGraph, directory: &mut std::path::PathBuf, name: &String) -> Result<(), ProgramError>{
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
pub enum ProgramError {
	Io(std::io::Error),
	Parse(core::ParseError),
	Graph(core::GraphError),
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

impl From<core::GraphError> for ProgramError {
	fn from(err: core::GraphError) -> ProgramError {
		ProgramError::Graph(err)
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
			ProgramError::Graph(ref err) => err.fmt(f),
			ProgramError::Other(ref err) => err.fmt(f),
		}
	}
}
