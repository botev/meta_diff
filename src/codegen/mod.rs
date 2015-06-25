extern crate std;
pub mod eigen;
pub mod matlab;
pub mod graphviz;

pub trait CodeGenerator{
	fn write_code(fmt: &mut std::io::Write, graph: & super::core::graph::ComputeGraph) -> Result<(),std::fmt::Error>;
}
