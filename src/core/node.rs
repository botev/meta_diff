use std::fmt::{Display, Formatter, Error};
use super::operator::*;


/// Represents the five types any `ComputeNode` can be
#[derive(Clone, Debug)]
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

impl Display for Type{
	fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
		match *self{
			Type::Float(x) => write!(f, "Float[{}]", x),
			Type::Integer(x) => write!(f, "Int[{}]", x),
			Type::ConstInput => write!(f, "ConstIn"),
			Type::Parameter => write!(f, "Param"),
			Type::ConstDerived => write!(f, "ConstDer"),
			Type::ParameterDerived => write!(f, "ParamDer"),
		}
	}
}

/// The main data structure of the `ComputeGraph`
#[derive(Clone, Debug)]
pub struct ComputeNode{
	pub id: usize,
	pub node_type: Type,
	pub name: String,
	pub children: Vec<usize>,
	pub grad_level: u8,
	pub inline: bool,
	// dims: Pair<SymPolynomial>,
	pub grad_child: Option<usize>,
	pub grad_parents: Vec<usize>,
	pub op: Option<Operator>
}

impl Display for ComputeNode{
	fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
		match self.op {
			Some(ref operator) => write!(f, concat!("********{}[{}]********\n",
				"Type:{:?}\n",
				"Operator: {:?}\n",
				"Children:{:?}"),
				self.name, self.id, self.node_type, operator, self.children),
			None => write!(f, concat!("********{}[{}]********\n",
				"Type:{:?}\n",
				"Children:{:?}"),
				self.name, self.id, self.node_type, self.children),
		}

	}
}

impl ComputeNode{
	/// Creates a new empty `ComputeNode`, its name depends on the input type and gradient level
	pub fn new(id: usize, node_type: Type, grad_level: u8, op: Option<Operator>) -> Self{
		let name = if grad_level > 0{
			"AutoGrad".to_string()
		}
		else {
			format!("{}", node_type)
		};
		ComputeNode{id: id, node_type: node_type, name: name, children: Vec::new(),
			grad_level: grad_level, inline: false, grad_child: None, grad_parents: Vec::new(), op:op}
	}
}
