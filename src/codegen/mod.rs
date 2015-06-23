pub mod eigen;
pub mod matlab;
pub mod graph_viz;

extern crate std;

pub trait CodeGenerator{
	fn write_code(target: &mut std::io::Write, graph: & super::core::graph::ComputeGraph);
}
