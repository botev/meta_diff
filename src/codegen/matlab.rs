use std::io::{Write, Error};
use std::result::Result;
use core::graph::*;

pub fn write_matlab(fmt: &mut Write, graph: & ComputeGraph) -> Result<(),Error>{
	write!(fmt, "{}", graph.nodes.len())
}