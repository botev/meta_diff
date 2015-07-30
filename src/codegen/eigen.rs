use std::io::{Write, Error};
use std::result::Result;
use core::ComputeGraph;

pub fn write_eigen(fmt: &mut Write, graph: & ComputeGraph) -> Result<(),Error>{
	write!(fmt, "{}", graph.nodes.len())
}
