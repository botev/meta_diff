
// use std::fs::{OpenOptions, File};
// use std::io::{BufWriter, Write};
// use super::super::core::graph::{ComputeGraph, ComputeNode, Type};

// #[allow(unused_must_use)]
// pub fn write_dot_file(graph: &super::super::core::graph::ComputeGraph, path: &str){	
// 	let mut options = OpenOptions::new();
// 	options.write(true).append(true);
// 	let file = match File::create(path) {
// 	    Ok(file) => file,
// 	    Err(..) => panic!(format!("No such file or directory found - {}!", path)),
// 	};
// 	let mut writer = BufWriter::new(&file);
// 	writeln!(&mut writer, "digraph G{{");
//     writeln!(&mut writer, "\tranksep=.75;");
//     writeln!(&mut writer, "\tnode [shape=box, fontsize=16, style=filled];");

//     // Forward calculation nodes
//     writeln!(&mut writer, "\tsubgraph cluster_0 {{");
// 	writeln!(&mut writer, "\t\tlabel=\"Forward Calculations\";");
// 	for node in &graph.nodes {
// 		if node.gradient_level < 1 {
// 			write_node(&mut writer, node);
// 		}
// 	}
// 	writeln!(&mut writer, "\t}}");

// 	// Gradient Calculation
// 	writeln!(&mut writer, "\tsubgraph cluster_1 {{");
// 	writeln!(&mut writer, "\t\tlabel=\"Gradient Calculations\";");
// 	for node in &graph.nodes {
// 		if node.gradient_level == 1 {
// 			write_node(&mut writer, node);
// 		}
// 	}
// 	writeln!(&mut writer, "\t}}");

// 	// Edges
// 	for node in &graph.nodes {
// 		for parent in &node.parents{
// 			writeln!(&mut writer, "\t{} -> {};", graph.nodes[*parent].id, node.id);
// 		}
// 	}
// 	writeln!(&mut writer, "}}");
// }

// fn format_node_label(node: &ComputeNode) -> String{
// 	match node.op{
// 		Some(ref operator) => 	format!("{name}[{id}]\\n{op}",
//                                         id=node.id, name=node.name, op=operator),
// 		_ =>  match node.node_type {
// 			Type::Number(val) => format!("{name}[{id}]\\nValue:{value}",
//                                         id=node.id, name=node.name, value=val),
// 			_ => format!("{name}[{id}]",id=node.id, name=node.name)
// 		}
// 	}
// }

// #[allow(unused_must_use)]
// fn write_node(writer : &mut Write, node : &ComputeNode){
// 	let color : String;
// 	match node.node_type {
// 		Type::Number(_) => color = "yellow".to_string(),
// 		Type::ConstInput => color = "orangered".to_string(),
// 		Type::ConstDerived => color = "orange".to_string(),
// 		Type::Parameter => color = "green".to_string(),
// 		Type::ParameterDerived => { 
//             if node.children.len() == 0 {color = "red".to_string();} 
//             else {color = "blue".to_string();}
//         }
// 	}
//     writeln!(writer, "\t\t{id}[label=\"{label}\"][fillcolor={color}];", 
//                                         id=node.id, label=format_node_label(node), color=color);
//     {}
// }


