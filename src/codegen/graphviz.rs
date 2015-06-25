use std::io::{Write, Error};
use std::result::Result;
use super::super::core::graph::{ComputeGraph,ComputeNode, Type};
// use super::super::core::{apply_copy, apply_ref};

static HEADING: &'static str = 
"digraph G{
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



pub fn write_graphviz(fmt: &mut Write, graph: & ComputeGraph) -> Result<(),Error>{
	// let dummy = ComputeNode::new(0, Type::Integer(-1), 128, None)	;
	try!(write!(fmt, "{}", HEADING));
	let target = graph.target;
	let outputs = graph.outputs.clone();
	// let func  = |value: &ComputeNode, fmt: &mut Write| {
	// 	if value.grad_level > 0 {
	// 		()
	// 	}
	// 	if value.id == target {
	// 		//write_node!(*fmt, value, "green", "ellipse")
	// 		// write!(fmt,"\t\t{id}[label=\"{name}[{id}]\"][fillcolor={color},shape={shape}];\n"
	// 		// 	, id=value.id, name=value.name, color="green", shape="ellipse")
	// 	}
	// 	else if outputs.contains(&value.id){
	// 		// write_node(fmt, value, "red", "ellipse")
	// 	}
	// 	else {
	// 		// match value.node_type {
	// 		// 	Type::Float(ref x)  => write!(*fmt,"\t\t{id}[label=\"{name}[{id}]\"][fillcolor={color},shape={shape}];\n"
	// 		// 	, id=value.id, name=value.name, color="yellow", shape="rectangle"),
	// 		// 	_ => write!(fmt,"\t\t{id}[label=\"{name}[{id}]\"][fillcolor={color},shape={shape}];\n"
	// 		// 	, id=value.id, name=value.name, color="red", shape="rectangle")		
	// 		// }
	// 	}
	// 	Ok(())
	// };
	// let fmt = try!(apply_block(graph.nodes, Box<fmt>, func));
	// Forward Calculation
	for option in graph.nodes.iter(){
		match *option{
			None => (),
			Some(ref value) => match {
				if value.grad_level > 0 {
					()
				}
				if value.id == target {
					write_node(fmt, value, "green", "ellipse")
				}
				else if outputs.contains(&value.id){
					write_node(fmt, value, "red", "ellipse")
				}
				else {
					match value.node_type {
						Type::Float(_) | Type::Integer(_)  => write_node(fmt, value, "yellow", "rectangle"),
						Type::ConstInput  => write_node(fmt, value, "orange", "rectangle"),
						Type::ConstDerived => write_node(fmt, value, "orangered", "rectangle"),
						Type::Parameter => write_node(fmt, value, "green", "rectangle"),
						Type::ParameterDerived => write_node(fmt, value, "blue", "rectangle")
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
	try!(write!(fmt, "{}", "}\n"));
	// Write connections
	for option in graph.nodes.iter(){
		match *option{
			None => (),
			Some(ref value) => match {
				for child in value.children.iter(){
					try!(write!(fmt, "{} -> {};\n", value.id, child));
				}
				Ok(())
			} {
				Ok(()) => (),
				Err(msg) => return Err(msg)
			}
		}
	}

	// apply_block!(graph.nodes, value, {
		// let id = value.id;
		// for child in value.children.iter(){
		// 	try!(write!(fmt, "{} -> {};\n", id, child));
		// }
		// Ok(())
	// });
	try!(write!(fmt, "{}", "}\n"));
	Ok(())	
}

#[inline(always)]
fn write_node(fmt: &mut Write, node: &ComputeNode, color:&str, shape: &str) -> Result<(),Error>{
	write!(fmt,"\t\t{id}[label=\"{name}[{id}]\\n{op}\"][fillcolor={color},shape={shape}];\n"
		, id=node.id, name=node.name,op= node.op.clone().map(|n| n.to_string()).unwrap_or("".to_string()), color=color, shape=shape)
}


// fn apply_block<T,V,F> (vector: Vec<Option<T>>, arg: &mut V, func: F) -> Result<&mut V, Error>
// where F: Fn(&T, &mut V) -> Result<(), Error> {
// 	for option in vector.iter(){
// 		match *option{
// 			None => (),
// 			Some(ref value) => {
// 				match func(value,arg) {
// 					Ok(()) => (),
// 					Err(msg) => return Err(msg)
// 				}
// 			}
// 		}
// 	}
// 	Ok(arg)
// }
// fn apply_block<T,V,F>(vector: Vec<Option<T>>, arg: V, func: F) -> Result<(),Error> 
// where F: Fn(& T, V) -> Result<(),Error> {
// 	for option in vector.iter(){
// 		match *option{
// 			None => (),
// 			Some(ref value) => {
// 				match func(value,arg) {
// 					Ok(()) => (),
// 					Err(msg) => return Err(msg)
// 				}
// 			}
// 		}
// 	}
// 	Ok(())
// }
// #[inline]
// fn write_node(fmt: &mut Write, node: &ComputeNode){

// }


