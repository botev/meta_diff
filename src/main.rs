extern crate meta_diff;
extern crate rustc_serialize;
extern crate docopt;

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
fn main_proxy(args: Args) ->  Result<(), meta_diff::ProgramError> {
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
	let mut graph = try!(meta_diff::core::parseMetaFile(&source));
	// Print initial
	try!(meta_diff::print_graph(&graph, &mut directory, &file_noextension));
	// Gradient
	try!(graph.direct_gradient());
	try!(meta_diff::print_graph(&graph, &mut directory, &(file_noextension.clone() + "_grad")));
	// Hessian-vector product
	let (ids, names) = graph.get_params();
	let vs = ids.iter().zip(names.iter()).map(|(id, name)| {
		let v = graph.add_const_input(name.clone() + &"_v".to_string());
		let op = meta_diff::core::OperatorType::Nary(meta_diff::core::NaryOperatorType::Mul);
		graph.add_operation(op,vec![*id, v]).unwrap()
	}).collect::<Vec<usize>>();
	let target = try!(graph.add_operation(
		meta_diff::core::OperatorType::Nary(meta_diff::core::NaryOperatorType::Add),vs));
	try!(graph.gradient(target));
	try!(meta_diff::print_graph(&graph, &mut directory, &(file_noextension.clone() + "_hess")));
	Ok(())
}
