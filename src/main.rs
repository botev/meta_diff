pub mod core;
pub mod codegen;

fn main() {
	let try1 = "function [a] = mat(a,b)
	a= a + b;
	d = a + b;
	end";
	let b = core::parser::metaFile(try1);
	match b{
		Ok(graph) => println!("{}", graph),
		Err(err) => println!("{}", err)
	}
}
