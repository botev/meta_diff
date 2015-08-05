use std::fmt::{Display, Formatter, Error};
use super::operator::*;

/// Represents the five types any `ComputeNode`
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type{
	/// Represents a single floating variable
	Float(f64),
	/// Represents a single integer variable
	Integer(i64),
	/// Represents a constant input, no gradients will be taken with respect to such node
	ConstInput,
	/// Represents a pramater, gradients will be taken with respect to such node
	Parameter,
	/// Represent a variable which depends only on `ConstInput`s, `Float`s or `Integer`s
	ConstDerived,
	/// Represent a variable which has some dependency on a `Parameter`
	ParameterDerived
}



/// The main data structure of the `ComputeGraph`
#[derive(Clone, Debug, PartialEq)]
pub struct ComputeNode{
	/// The id is equivalent to the index in the nodes list of the `ComputeGraph`
	pub id: usize,
	/// The type of the node
	pub node_type: Type,
	/// A user given or automatically generated name
	pub name: String,
	/// All nodes dependable on this one
	pub children: Vec<usize>,
	/// Defines to what gradient level computation this node belongs
	pub grad_level: u8,
	/// Whether the node should be inlined by any of the source code generators
	pub inline: bool,
	// dims: Pair<SymPolynomial>,
	/// Defines the node which represents `dL/dx`
	pub grad_child: Option<usize>,
	/// If this node has `grad_level` more than 0, it means it could represent for some node `dL/dx`, thus this contains a list of all such nodes
	pub grad_parents: Vec<usize>,
	/// What is the operator this node represents
	pub op: Operator
}

impl Display for ComputeNode{
	fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
		match self.op.op_type {
			OperatorType::Constant(ConstantOperatorType::None) =>
				write!(f, concat!("********{}[{}]********\n",
					"Type:{:?}\n",
					"Children:{:?}"),
					self.name, self.id, self.node_type, self.children),
			operator => write!(f, concat!("********{}[{}]********\n",
				"Type:{:?}\n",
				"Operator: {:?}\n",
				"Children:{:?}"),
				self.name, self.id, self.node_type, operator, self.children),
		}
	}
}

impl ComputeNode{
	/// Creates a new empty `ComputeNode`, its name depends on the input type and gradient level
	pub fn new(id: usize, node_type: Type, grad_level: u8, op: Operator) -> Self{
		let name = if grad_level > 0{
			"AutoGrad".to_string()
		}
		else {
			format!("{:?}", node_type)
		};
		ComputeNode{id: id, node_type: node_type, name: name, children: Vec::new(),
			grad_level: grad_level, inline: false, grad_child: None, grad_parents: Vec::new(), op:op}
	}
}
