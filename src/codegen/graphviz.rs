use std::io::{Write, Error};
use std::result::Result;
use core::*;

static HEADING: &'static str =
"digraph G{
	bgcolor=\"transparent\";
	ranksep=.75;
	node [shape=box, fontsize=16, style=filled];
	subgraph cluster_0{
		label=\"Forward Calculations\";
";
static MIDDLE: &'static str =
"	}
	subgraph cluster_1{
		label=\"Gradient Calculations\";
";
static MIDDLE_2: &'static str =
"	}
	subgraph cluster_2{
		label=\"Hessian Calculations\";
";



pub fn write_graphviz(fmt: &mut Write, graph: & ComputeGraph) -> Result<(),Error>{
	try!(write!(fmt, "{}", HEADING));
	let target = graph.target;
	let outputs = graph.outputs.clone();
	// Forward Calculation
	for option in graph.nodes.iter(){
		match *option{
			None => (),
			Some(ref value) => match {
				if value.grad_level != 0 {
					continue;
				}
				if outputs.contains(&value.id){
					write_node(fmt, value, &value.name, "red", "ellipse")
				}
				else {
					match value.node_type {
						Type::Float(_) | Type::Integer(_)  => write_node(fmt, value, &value.name, "yellow", "rectangle"),
						Type::ConstInput  => write_node(fmt, value, &value.name, "orange", "rectangle"),
						Type::ConstDerived => write_node(fmt, value, &value.name, "orangered", "rectangle"),
						Type::Parameter => write_node(fmt, value, &value.name, "green", "rectangle"),
						Type::ParameterDerived => write_node(fmt, value, &value.name, "blue", "rectangle")
					}
				}
			} {
				Ok(()) => (),
				Err(msg) => return Err(msg)
			}
		}
	}
	try!(write!(fmt, "{}", MIDDLE));
	// Backward Calculation
	for option in graph.nodes.iter(){
		match *option{
			None => (),
			Some(ref value) => match {
				if value.grad_level != 1 {
					continue;
				}
				let mut name =  &format!("GradOf{:?}",value.grad_parents);
				if value.grad_parents.len() == 0 {
					name = &value.name;
				}
				if value.id == target {
					write_node(fmt, value, name, "green", "ellipse")
				}
				else if outputs.contains(&value.id){
					write_node(fmt, value, name, "red", "ellipse")
				}
				else {
					match value.node_type {
						Type::Float(_) | Type::Integer(_)  => write_node(fmt, value, name, "yellow", "rectangle"),
						Type::ConstInput  => write_node(fmt, value, name, "orange", "rectangle"),
						Type::ConstDerived => write_node(fmt, value, name, "orangered", "rectangle"),
						Type::Parameter => write_node(fmt, value, name, "green", "rectangle"),
						Type::ParameterDerived => write_node(fmt, value, name, "blue", "rectangle")
					}
				}
			} {
				Ok(()) => (),
				Err(msg) => return Err(msg)
			}
		}
	}
	try!(write!(fmt, "{}", MIDDLE_2));
	// Hessian order
	for option in graph.nodes.iter(){
		match *option{
			None => (),
			Some(ref value) => match {
				if value.grad_level != 2 {
					continue;
				}
				let mut name =  &format!("GradOf{:?}",value.grad_parents);
				if value.grad_parents.len() == 0 {
					name = &value.name;
				}
				if value.id == target {
					write_node(fmt, value, name, "green", "ellipse")
				}
				else if outputs.contains(&value.id){
					write_node(fmt, value, name, "red", "ellipse")
				}
				else {
					match value.node_type {
						Type::Float(_) | Type::Integer(_)  => write_node(fmt, value, name, "yellow", "rectangle"),
						Type::ConstInput  => write_node(fmt, value, name, "orange", "rectangle"),
						Type::ConstDerived => write_node(fmt, value, name, "orangered", "rectangle"),
						Type::Parameter => write_node(fmt, value, name, "green", "rectangle"),
						Type::ParameterDerived => write_node(fmt, value, name, "blue", "rectangle")
					}
				}
			} {
				Ok(()) => (),
				Err(msg) => return Err(msg)
			}
		}
	}
	try!(write!(fmt, "{}", "\t}\n"));
	// Write connections
	for option in graph.nodes.iter(){
		match *option{
			None => (),
			Some(ref value) => match {
				for child in value.children.iter(){
					try!(write!(fmt, "\t{} -> {};\n", value.id, child));
				}
				Ok(())
			} {
				Ok(()) => (),
				Err(msg) => return Err(msg)
			}
		}
	}
	try!(write!(fmt, "{}", "}\n"));
	Ok(())
}

#[inline(always)]
fn write_node(fmt: &mut Write, node: &ComputeNode, name: &str, color:&str, shape: &str) -> Result<(),Error>{
	write!(fmt,"\t\t{id}[label=\"{name}\\nId:{id}\\n{op}\"][fillcolor={color},shape={shape}];\n"
		, id=node.id, name=name,op= node.op.clone().map(|n| n.to_string()).unwrap_or("".to_string()), color=color, shape=shape)
}
