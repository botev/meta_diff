#[macro_use]
pub mod core;
pub mod codegen;


fn main() {
	let try1 = "sad";
	let tr = "da";
	let fs = "sad";
	println!("{},{},{}",try1 == tr, tr == fs, try1 == fs);
}
