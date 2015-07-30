use std::fmt::{Display, Formatter, Error};
use std::string::ToString;
use std::collections::HashMap;
use super::operator::*;
use super::node::*;



/// The core of this module - this structure contains the full compute graph
#[derive(Clone, Debug)]
pub struct ComputeGraph{
	counter: usize,
	grad_level: u8,
	pub name: String,
	pub nodes: Vec<Option<ComputeNode>>,
	pub target: usize,
	pub outputs: Vec<usize>,
	// pub variable_table: HashMap<String, usize>,
	// symbolic_table: Vec<String>,
	// symbloic_parents: Vec<usize>,
	// symbolic_vars: usize
}

impl Display for ComputeGraph{
	fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
		try!(write!(f, concat!("============ Graph {} ============\n",
			"Number of nodes:{}\n",
			"Target: {}\n",
			"Outputs:{:?}\n" ,
			"============= Nodes =============\n"),
		self.name, self.counter, self.target, self.outputs));
		for node_opt in self.nodes.iter(){
			match *node_opt {
				Some(ref node) => try!(write!(f,"{}\n",node)),
				None => {}
			}
		}
		write!(f, "============ Graph End ============")
	}
}

impl ComputeGraph{
	/// Create a new compute graph for a function with the input name
	pub fn new() -> Self{
		return ComputeGraph{name: "main".to_string(), counter: 0, grad_level: 0, nodes: Vec::new(), target: 0, outputs: Vec::new()}
	}

	/// Creates a new `Parameter` variable with the given name, inserts it in the variable table and returns its id
	pub fn add_parameter(&mut self, name: String) ->  usize {
		let mut node = ComputeNode::new(self.counter, Type::Parameter, self.grad_level, None);
		node.name = name;
		self.counter += 1;
		self.nodes.push(Some(node));
		return self.counter-1
	}

	/// Creates a new `Float` variable and returns its id
	pub fn add_float(&mut self, value: f64) -> usize{
		let node = ComputeNode::new(self.counter, Type::Float(value), self.grad_level, None);
		self.counter += 1;
		self.nodes.push(Some(node));
		return self.counter-1
	}

	/// Creates a new `Integer` variable and returns its id
	pub fn add_int(&mut self, value: i64) -> usize{
		let node = ComputeNode::new(self.counter, Type::Integer(value), self.grad_level, None);
		self.counter += 1;
		self.nodes.push(Some(node));
		return self.counter-1
	}

	/// Creates a new `ConstInput` variable with the given name, inserts it in the variable table and returns its id
	pub fn add_const_input(&mut self, name: String) -> usize{
		let mut node = ComputeNode::new(self.counter, Type::ConstInput, self.grad_level, None);
		node.name = name;
		self.counter += 1;
		self.nodes.push(Some(node));
		return self.counter-1
	}

	/// Adds a variable coresponding to the input operation to the graph
	pub fn add_operation(&mut self, op : Operator) -> Result<usize, String>{
		let mut node_type = Type::ConstDerived;
		match op{
			Operator::Const(parent_id) |  Operator::Eye(parent_id) | Operator::Size(parent_id, _)
			| Operator::Sign(parent_id)=> {
				match * &mut self.nodes[parent_id]{
					Some(ref mut parent) => {
						parent.children.push(self.counter);
					},
					None => return Err("Trying to access deleted node!".to_string())
				}
			},
			Operator::Ones(parent_id1, parent_id2) | Operator::Zeros(parent_id1, parent_id2)
			| Operator::LessThan(parent_id1, parent_id2) | Operator::LessThanOrEqual(parent_id1, parent_id2)
			| Operator::GreaterThan(parent_id1, parent_id2) | Operator::GreaterThanOrEqual(parent_id1, parent_id2) => {
				match * &mut self.nodes[parent_id1]{
					Some(ref mut parent) => parent.children.push(self.counter),
					None => return Err("The parent node in the operator provided has been deleted.".to_string())
				}
				match * &mut self.nodes[parent_id2]{
					Some(ref mut parent) => parent.children.push(self.counter),
					None => return Err("The parent node in the operator provided has been deleted.".to_string())
				}
			},
			Operator::Neg(parent_id) | Operator::Div(parent_id) | Operator::MatrixInverse(parent_id)
			| Operator::Transpose(parent_id) | Operator::MatrixDiag(parent_id) | Operator::VectorDiag(parent_id)
			| Operator::Cos(parent_id) | Operator::Sin(parent_id) | Operator::Tan(parent_id)
			| Operator::CosH(parent_id) | Operator::SinH(parent_id) | Operator::TanH(parent_id)
			| Operator::Abs(parent_id) | Operator::Log(parent_id) | Operator::Exp(parent_id)
			| Operator::Sqrt(parent_id) | Operator::Square(parent_id)
			| Operator::Sigmoid(parent_id) | Operator::Sum(parent_id,_)
			| Operator::L2(parent_id,_) | Operator::L1(parent_id,_) => { //| Operator::Broadcast(parent_id,_) => {
				match * &mut self.nodes[parent_id]{
					Some(ref mut parent) => {
						match parent.node_type {
							Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
							_ => ()
						}
						parent.children.push(self.counter);
					},
					None => return Err("Trying to access deleted node!".to_string())
				}
			},
			Operator::Pow(parent_id1, parent_id2) | Operator::Quadratic(parent_id1, parent_id2)
			| Operator::Max(parent_id1, parent_id2) | Operator::Min(parent_id1, parent_id2) => {
				match * &mut self.nodes[parent_id1]{
					Some(ref mut parent) => {
						match parent.node_type {
							Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
							_ => ()
						}
						parent.children.push(self.counter);
					},
					None => return Err("Trying to access deleted node!".to_string())
				}
				match * &mut self.nodes[parent_id2]{
					Some(ref mut parent) => {
						match parent.node_type {
							Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
							_ => ()
						}
						parent.children.push(self.counter);
					},
					None => return Err("Trying to access deleted node!".to_string())
				}
			},
			Operator::Add(ref parent_ids) | Operator::Mul(ref parent_ids) | Operator::Dot(ref parent_ids)
			| Operator::HorzCat(ref parent_ids) | Operator::VertCat(ref parent_ids) => {
				for parent_id in parent_ids.iter().cloned(){
					match * &mut self.nodes[parent_id]{
						Some(ref mut parent) => {
							match parent.node_type {
								Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
								_ => ()
							}
							parent.children.push(self.counter);
						},
						None => return Err("Trying to access deleted node!".to_string())
					}
				}
			},
			Operator::SubIndex(parent_id, arg_id1, arg_id2, arg_id3, arg_id4) |
			Operator::SubAssign(parent_id, arg_id1, arg_id2, arg_id3, arg_id4) => {
				let ids = [parent_id, arg_id1, arg_id2, arg_id3, arg_id4];
				for parent_id in &ids{
					match * &mut self.nodes[*parent_id]{
						Some(ref mut parent) => {
							match parent.node_type {
								Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
								_ => ()
							}
							parent.children.push(self.counter);
						},
						None => return Err("Trying to access deleted node!".to_string())
					}
				}
			},
			Operator::Reshape(parent_id, arg_id1, arg_id2) => {
				let ids = [parent_id, arg_id1, arg_id2];
				for parent_id in &ids{
					match * &mut self.nodes[*parent_id]{
						Some(ref mut parent) => {
							match parent.node_type {
								Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
								_ => ()
							}
						},
						None => return Err("Trying to access deleted node!".to_string())
					}
				}
			},
			Operator::ReplicateHorz(parent_id, arg_id) | Operator::ReplicateVert(parent_id, arg_id) => {
				let ids = [parent_id, arg_id];
				for parent_id in &ids{
					match * &mut self.nodes[*parent_id]{
						Some(ref mut parent) => {
							match parent.node_type {
								Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
								_ => ()
							}
						},
						None => return Err("Trying to access deleted node!".to_string())
					}
				}
			}
		}
		let node = ComputeNode::new(self.counter, node_type, self.grad_level, Some(op));
		self.nodes.push(Some(node));
		self.counter += 1;
		return Ok(self.counter-1);
	}

	/// Applies a gradient operator to the graph given the current target
	pub fn gradient(&mut self) -> Result<(),String>{
		match self.nodes[self.target]{
			Some(ref node) => match node.node_type {
				Type::Parameter | Type::ParameterDerived => (),
				_ => return Ok(())
			},
			None => return Err("Target is None".to_string())
		}
		self.grad_level += 1;
		let mut messages : HashMap<usize, Vec<usize>> = HashMap::new();
		let mut span : Vec<bool> = self.nodes.iter().cloned().map(|_| false).collect::<Vec<bool>>();
		span.push(true);
		span.swap_remove(self.target);
		messages.insert(self.target, vec![self.add_int(1)]);
		for i in (0..self.target + 1).rev(){
			// Skip if the node is not in the spanning tree of the target
			if !span[i] {
				continue;
			}
			// Get the gradient of the current node
			let gradient = match messages.remove(&i) {
				Some(vec) => match vec.len() {
					0 => return Err(format!("No incoming messages found for node {}", i)),
					1 => vec[0],
					_ => try!(self.add_operation(Operator::Add(vec))),
				},
				None => return Err(format!("No incoming messages found for node {}", i))
			};
			// Connect the gradient info and the parent
			try!(self.get_mut_node(gradient)).grad_parents.push(i);
			try!(self.get_mut_node(i)).grad_child = Some(gradient);
			// Generate gradient messages
			let grad_msgs = try!(self.op_gradient(i, gradient));
			for (parent, msg) in grad_msgs{
				// Mark that that the parent is in the sapnning tree
				span.push(true);
				span.swap_remove(parent);
				// Add message to his incomings
				let mut mine = match messages.remove(&parent) {
					None => Vec::new(),
					Some(vec) => vec
				};
				mine.push(msg);
				messages.insert(parent,mine);
			}
		}
		let mut grad_outputs: Vec<usize> = Vec::new();
		// Add gradients of the parameters to outptus
		for i in (0..self.counter) {
			//let node : Result<&mut ComputeNode, String> = ;
			match self.get_mut_node(i) {
				Ok(ref node) => match node.node_type {
					Type::Parameter => {grad_outputs.push(node.grad_child.unwrap()); ()},
					_ => ()
				},
				Err(_) => ()
			}
		}
		for val in grad_outputs {
			self.outputs.push(val);
		}
		Ok(())
	}

	/// Generates gradient messages from the operator to all of its non constant parents.
	/// Returns a HashMap<parent,msg>
	fn op_gradient(&mut self, child: usize, grad: usize) -> Result<HashMap<usize,usize>, String>{
		let mut gradients : HashMap<usize,usize> = HashMap::new();
		let op = match try!(self.get_node(child)).op {
			None => {return Ok(gradients);},
			Some(ref op) => op.clone()
		};
		match op{
			Operator::Const(_) | Operator::Ones(_, _) | Operator::Zeros(_, _)
			| Operator::Eye(_) | Operator::Size(_, _)| Operator::Sign(_)
			| Operator::LessThan(_, _)| Operator::LessThanOrEqual(_, _)
			| Operator::GreaterThan(_, _)| Operator::GreaterThanOrEqual(_, _) =>
			return Err("Can not take gradient with respect to constant operator".to_string()),
			Operator::Neg(parent) => {
				let msg = try!(self.add_operation(Operator::Neg(grad)));
				gradients.insert(parent, msg);
			},
			Operator::Div(parent) => {
				let mut msg = try!(self.add_operation(Operator::Square(parent)));
				msg = try!(self.add_operation(Operator::Div(msg)));
				msg = try!(self.add_operation(Operator::Neg(msg)));
				msg = try!(self.add_operation(Operator::Mul(vec![grad,msg])));
				gradients.insert(parent, msg);
			},
			Operator::MatrixInverse(parent) => if try!(self.is_dependable(parent)){
				let mut msg = try!(self.add_operation(Operator::Transpose(child)));
				msg = try!(self.add_operation(Operator::Dot(vec![msg, grad, msg])));
				msg = try!(self.add_operation(Operator::Neg(msg)));
				gradients.insert(parent, msg);
			},
			Operator::Transpose(parent) => {
				let msg = try!(self.add_operation(Operator::Transpose(grad)));
				gradients.insert(parent, msg);
			},
			Operator::MatrixDiag(parent) => {
				let msg = try!(self.add_operation(Operator::VectorDiag(grad)));
				gradients.insert(parent, msg);
			},
			Operator::VectorDiag(parent) => {
				let msg = try!(self.add_operation(Operator::MatrixDiag(grad)));
				gradients.insert(parent, msg);
			},
			Operator::Cos(parent) => {
				let mut msg = try!(self.add_operation(Operator::Sin(parent)));
				msg = try!(self.add_operation(Operator::Neg(msg)));
				msg = try!(self.add_operation(Operator::Mul(vec![grad,msg])));
				gradients.insert(parent, msg);
			},
			Operator::Sin(parent) => {
				let mut msg = try!(self.add_operation(Operator::Cos(parent)));
				msg = try!(self.add_operation(Operator::Mul(vec![grad,msg])));
				gradients.insert(parent, msg);
			},
			Operator::Tan(parent) => {
				   let mut msg = try!(self.add_operation(Operator::Cos(parent)));
				msg = try!(self.add_operation(Operator::Square(msg)));
				msg = try!(self.add_operation(Operator::Div(msg)));
				msg = try!(self.add_operation(Operator::Mul(vec![grad,msg])));
				gradients.insert(parent, msg);
			},
			Operator::CosH(parent) =>{
				let mut msg = try!(self.add_operation(Operator::SinH(parent)));
				msg = try!(self.add_operation(Operator::Mul(vec![grad,msg])));
				gradients.insert(parent, msg);
				},
			Operator::SinH(parent) => {
				let mut msg = try!(self.add_operation(Operator::CosH(parent)));
				msg = try!(self.add_operation(Operator::Mul(vec![grad,msg])));
				gradients.insert(parent, msg);
			},
			Operator::TanH(parent) => {
				let mut msg = try!(self.add_operation(Operator::Square(child)));
				msg = try!(self.add_operation(Operator::Neg(msg)));
				let const1 = self.add_int(1);
				msg = try!(self.add_operation(Operator::Add(vec![const1, msg])));
				msg = try!(self.add_operation(Operator::Mul(vec![grad,msg])));
				gradients.insert(parent, msg);
			},
			Operator::Abs(parent) => {
				let mut msg = try!(self.add_operation(Operator::Sign(parent)));
				msg = try!(self.add_operation(Operator::Mul(vec![grad,msg])));
				gradients.insert(parent, msg);
			},
			Operator::Log(parent) =>{
				let mut msg = try!(self.add_operation(Operator::Div(parent)));
				msg = try!(self.add_operation(Operator::Mul(vec![grad,msg])));
				gradients.insert(parent, msg);
			},
			Operator::Exp(parent) =>{
				let msg = try!(self.add_operation(Operator::Mul(vec![grad,child])));
				gradients.insert(parent, msg);
			},
			Operator::Sqrt(parent) => {
				let mut const2 = self.add_int(2);
				const2 = try!(self.add_operation(Operator::Div(const2)));
				let mut msg = try!(self.add_operation(Operator::Div(child)));
				msg = try!(self.add_operation(Operator::Mul(vec![const2, msg, grad])));
				gradients.insert(parent, msg);
			},
			Operator::Square(parent) =>{
				let const2 = self.add_int(2);
				let msg = try!(self.add_operation(Operator::Mul(vec![const2, parent, grad])));
				gradients.insert(parent, msg);
			},
			Operator::Max(p1, p2) => {
				if try!(self.is_dependable(p1)) {
					let mut msg = try!(self.add_operation(Operator::Neg(p2)));
					msg = try!(self.add_operation(Operator::Add(vec![p1, msg])));
					msg = try!(self.add_operation(Operator::Sign(msg)));
					msg = try!(self.add_operation(Operator::Mul(vec![grad, msg])));
					gradients.insert(p1, msg);
				}
				if try!(self.is_dependable(p2)) {
					let mut msg = try!(self.add_operation(Operator::Neg(p1)));
					msg = try!(self.add_operation(Operator::Add(vec![p2, msg])));
					msg = try!(self.add_operation(Operator::Sign(msg)));
					msg = try!(self.add_operation(Operator::Mul(vec![grad, msg])));
					gradients.insert(p2, msg);
				}
			},
			Operator::Min(p1, p2) => {
				if try!(self.is_dependable(p1)) {
					let mut msg = try!(self.add_operation(Operator::Neg(p1)));
					msg = try!(self.add_operation(Operator::Add(vec![p2, msg])));
					msg = try!(self.add_operation(Operator::Sign(msg)));
					msg = try!(self.add_operation(Operator::Mul(vec![grad, msg])));
					gradients.insert(p1, msg);
				}
				if try!(self.is_dependable(p2)) {
					let mut msg = try!(self.add_operation(Operator::Neg(p2)));
					msg = try!(self.add_operation(Operator::Add(vec![p1, msg])));
					msg = try!(self.add_operation(Operator::Sign(msg)));
					msg = try!(self.add_operation(Operator::Mul(vec![grad, msg])));
					gradients.insert(p2, msg);
				}
			},
			Operator::Sigmoid(parent) => {
				let const1 = self.add_int(1);
				let mut msg = try!(self.add_operation(Operator::Neg(child)));
				msg = try!(self.add_operation(Operator::Add(vec![const1, msg])));
				msg = try!(self.add_operation(Operator::Mul(vec![grad, child, msg])));
				gradients.insert(parent, msg);
			},
			// Operator::Broadcast(parent,dim) => {
			//
			// },
			Operator::Sum(parent,dim) => {
				match dim{
					Dimension::First => {
						let rows = try!(self.add_operation(Operator::Size(parent, Dimension::First)));
						let msg = try!(self.add_operation(Operator::ReplicateVert(grad, rows)));
						gradients.insert(parent, msg);
					}
					Dimension::Second => {
						let cols = try!(self.add_operation(Operator::Size(parent, Dimension::Second)));
						let msg = try!(self.add_operation(Operator::ReplicateHorz(grad, cols)));
						gradients.insert(parent, msg);
					}
					Dimension::All => {
						let rows = try!(self.add_operation(Operator::Size(parent, Dimension::First)));
						let cols = try!(self.add_operation(Operator::Size(parent, Dimension::Second)));
						let mut msg = try!(self.add_operation(Operator::ReplicateVert(grad, rows)));
						msg = try!(self.add_operation(Operator::ReplicateHorz(msg, cols)));
						gradients.insert(parent, msg);
					}
				}
			},
			Operator::L2(parent,dim) => {
				match dim{
					Dimension::First => {
						let const2 = self.add_int(2);
						let rows = try!(self.add_operation(Operator::Size(parent, Dimension::First)));
						let mut msg = try!(self.add_operation(Operator::ReplicateVert(grad,rows)));
						msg = try!(self.add_operation(Operator::Mul(vec![const2, parent, msg])));
						gradients.insert(parent, msg);
					}
					Dimension::Second => {
						let const2 = self.add_int(2);
						let cols = try!(self.add_operation(Operator::Size(parent, Dimension::Second)));
						let mut msg = try!(self.add_operation(Operator::ReplicateHorz(grad,cols)));
						msg = try!(self.add_operation(Operator::Mul(vec![const2, parent, msg])));
						gradients.insert(parent, msg);
					}
					Dimension::All => {
						let const2 = self.add_int(2);
						let msg = try!(self.add_operation(Operator::Mul(vec![const2, parent, grad])));
						gradients.insert(parent, msg);
					}
				}
			},
			Operator::L1(parent,dim) => {
				match dim{
					Dimension::First => {
						let msg_s = try!(self.add_operation(Operator::Sign(parent)));
						let rows = try!(self.add_operation(Operator::Size(parent, Dimension::First)));
						let mut msg = try!(self.add_operation(Operator::ReplicateVert(grad,rows)));
						msg = try!(self.add_operation(Operator::Mul(vec![msg_s,msg])));
						gradients.insert(parent, msg);
					}
					Dimension::Second => {
						let msg_s = try!(self.add_operation(Operator::Sign(parent)));
						let cols = try!(self.add_operation(Operator::Size(parent, Dimension::Second)));
						let mut msg = try!(self.add_operation(Operator::ReplicateHorz(grad,cols)));
						msg = try!(self.add_operation(Operator::Mul(vec![msg_s,msg])));
						gradients.insert(parent, msg);
					}
					Dimension::All => {
						let mut msg = try!(self.add_operation(Operator::Sign(parent)));
						msg = try!(self.add_operation(Operator::Mul(vec![grad,msg])));
						gradients.insert(parent, msg);
					}
				}
			},
			Operator::Add(ref parents) => {
				for i in parents{
					if try!(self.is_dependable(*i)) {
						gradients.insert(*i, grad);
					}
				}
			},
			Operator::Mul(ref parents) => {
				match parents.len(){
					0...1 => return Err("Multiplication with less than 2 parents".to_string()),
					2 => {
						let p1 = parents.get(0).unwrap();
						let p2 = parents.get(1).unwrap();
						if try!(self.is_dependable(*p1)) {
							let msg = try!(self.add_operation(Operator::Mul(vec![*p2, grad])));
							gradients.insert(*p1, msg);
						}
						if try!(self.is_dependable(*p2)) {
							let msg = try!(self.add_operation(Operator::Mul(vec![*p1, grad])));
							gradients.insert(*p2, msg);
						}
					},
					_ => {
						for i in parents{
							if try!(self.is_dependable(*i)) {
								let mut msg = try!(self.add_operation(Operator::Div(*i)));
								msg = try!(self.add_operation(Operator::Mul(vec![msg ,child, grad])));
								gradients.insert(*i, msg);
							}
						}
					}
				}
			},
			Operator::Dot(ref parents) => {
				match parents.len(){
					0...1 => return Err("Multiplication with less than 2 parents".to_string()),
					_ => {
						// Left most parent
						let p1 = parents[0];
						if try!(self.is_dependable(p1)) {
							let mut right_msg : usize;
							if parents.len() == 2{
								right_msg = try!(self.add_operation(Operator::Transpose(parents[1])));
							}
							else {
								let right_parents = parents[1..].to_owned();
								right_msg = try!(self.add_operation(Operator::Dot(right_parents)));
								right_msg = try!(self.add_operation(Operator::Transpose(right_msg)));
							}
							let msg = try!(self.add_operation(Operator::Dot(vec![grad, right_msg])));
							gradients.insert(p1, msg);
						}
						// Right most parent
						let last = parents.len()-1;
						let pend = parents[last];
						if try!(self.is_dependable(pend)) {
							let mut left_msg : usize;
							if parents.len() == 2{
								left_msg = try!(self.add_operation(Operator::Transpose(parents[0])));
							}
							else {
								let left_parents = parents[..last].to_owned();
								left_msg = try!(self.add_operation(Operator::Dot(left_parents)));
								left_msg = try!(self.add_operation(Operator::Transpose(left_msg)));
							}
							let msg = try!(self.add_operation(Operator::Dot(vec![left_msg, grad])));
							gradients.insert(pend, msg);
						}
						if parents.len() > 2 {
							// Second from left to right
							let p = parents[1];
							if try!(self.is_dependable(p)) {
								let left_msg = try!(self.add_operation(Operator::Transpose(parents[0])));
								let mut right_msg : usize;
								if parents.len() == 3 {
									right_msg = try!(self.add_operation(Operator::Transpose(parents[2])));
								}
								else {
									let right_parents = parents[2..].to_owned();
									right_msg = try!(self.add_operation(Operator::Dot(right_parents)));
									right_msg = try!(self.add_operation(Operator::Transpose(right_msg)));
								}
								let msg = try!(self.add_operation(Operator::Dot(vec![left_msg, grad, right_msg])));
								gradients.insert(p, msg);
							}
						}
						if parents.len() > 3 {
							// Second from right to left
							let p = parents[last-1];
							if try!(self.is_dependable(p)) {
								let right_msg = try!(self.add_operation(Operator::Transpose(parents[last])));
								let left_parents = parents[..last-1].to_owned();
								let mut left_msg = try!(self.add_operation(Operator::Dot(left_parents)));
								left_msg = try!(self.add_operation(Operator::Transpose(left_msg)));
								let msg = try!(self.add_operation(Operator::Dot(vec![left_msg, grad, right_msg	])));
								gradients.insert(p, msg);
							}
							// Rest
							for i in 2..last-1{
								let p = parents[i];
								if try!(self.is_dependable(p)) {
									let left_parents = parents[..last-1].to_owned();
									let mut left_msg = try!(self.add_operation(Operator::Dot(left_parents)));
									left_msg = try!(self.add_operation(Operator::Transpose(left_msg)));
									let right_parents = parents[2..].to_owned();
									let mut right_msg = try!(self.add_operation(Operator::Dot(right_parents)));
									right_msg = try!(self.add_operation(Operator::Transpose(right_msg)));
									let msg = try!(self.add_operation(Operator::Dot(vec![left_msg, grad, right_msg])));
									gradients.insert(p, msg);
								}
							}
						}
					}
				}
			},
			Operator::Pow(p1, p2) => {
				if try!(self.is_dependable(p1)) {
					let mut msg = try!(self.add_operation(Operator::Div(p1)));
					msg = try!(self.add_operation(Operator::Mul(vec![grad, p2, child, msg])));
					gradients.insert(p1, msg);
				}
				if try!(self.is_dependable(p2)) {
					let mut msg = try!(self.add_operation(Operator::Log(p1)));
					msg = try!(self.add_operation(Operator::Mul(vec![grad, child, msg])));
					gradients.insert(p1, msg);
				}
			},
			Operator::Quadratic(p1, p2) => {
				if try!(self.is_dependable(p1)) {
					let mut msg = try!(self.add_operation(Operator::Transpose(p2)));
					msg = try!(self.add_operation(Operator::Add(vec![p2, msg])));
					msg = try!(self.add_operation(Operator::Dot(vec![msg, p1, grad])));
					gradients.insert(p1, msg);
				}
				if try!(self.is_dependable(p2)) {
					let mut msg = try!(self.add_operation(Operator::Transpose(p1)));
					msg = try!(self.add_operation(Operator::Dot(vec![p1, grad, msg])));
					gradients.insert(p1, msg);
				}
			},
			Operator::HorzCat(ref parents) => {
				match parents.len(){
					0...1 => return Err("Multiplication with less than 2 parents".to_string()),
					_ => {
						let mut last : usize = parents.len() + 1;
						for i in (0..parents.len()).rev(){
							if try!(self.is_dependable(parents[i])) {
								last = i;
								break;
							}
						}
						if last < parents.len(){
							let const_0 = self.add_int(0);
							let size_x =  try!(self.add_operation(Operator::Size(child, Dimension::First)));
							let mut accum = self.add_int(0);
							for i in 0..last+1{
								let p = parents[i];
								let size_y = try!(self.add_operation(Operator::Size(p, Dimension::Second)));
								if try!(self.is_dependable(p)) {
									let start_y = accum;
									let msg = try!(self.add_operation(Operator::SubIndex(grad, const_0, size_x, start_y, size_y)));
									gradients.insert(p, msg);
								}
								if i < last{
									accum = try!(self.add_operation(Operator::Add(vec![accum, size_y])));
								}
							}
						}
					}
				}
			},
			Operator::VertCat(ref parents) => {
				match parents.len(){
					0...1 => return Err("Multiplication with less than 2 parents".to_string()),
					_ => {
						let mut last : usize = parents.len() + 1;
						for i in (0..parents.len()).rev(){
							if try!(self.is_dependable(parents[i])) {
								last = i;
								break;
							}
						}
						if last < parents.len(){
							let const_0 = self.add_int(0);
							let size_y =  try!(self.add_operation(Operator::Size(child, Dimension::Second)));
							let mut accum = self.add_int(0);
							for i in 0..last+1{
								let p = parents[i];
								let size_x = try!(self.add_operation(Operator::Size(p, Dimension::First)));
								if try!(self.is_dependable(p)) {
									let start_x = accum;
									let msg = try!(self.add_operation(Operator::SubIndex(grad, start_x, size_x, const_0, size_y)));
									gradients.insert(p, msg);
								}
								if i < last{
									accum = try!(self.add_operation(Operator::Add(vec![accum, size_x])));
								}
							}
						}
					}
				}
			},
			Operator::SubIndex(parent, arg1, arg2, arg3, arg4) => {
				let msg = try!(self.add_operation(Operator::SubAssign(grad, arg1, arg2, arg3, arg4)));
				gradients.insert(parent, msg);
			},
			Operator::SubAssign(parent, arg1, arg2, arg3, arg4) => {
				let msg = try!(self.add_operation(Operator::SubIndex(grad, arg1, arg2, arg3, arg4)));
				gradients.insert(parent, msg);
			},
			Operator::Reshape(parent, _, _) => {
				let rows = try!(self.add_operation(Operator::Size(parent, Dimension::First)));
				let cols = try!(self.add_operation(Operator::Size(parent, Dimension::Second)));
				let msg = try!(self.add_operation(Operator::Reshape(grad, rows, cols)));
				gradients.insert(parent, msg);
			},
			Operator::ReplicateHorz(parent, _) => {
				let msg = try!(self.add_operation(Operator::Sum(grad, Dimension::Second)));
				gradients.insert(parent, msg);
			},
			Operator::ReplicateVert(parent, _) => {
				let msg = try!(self.add_operation(Operator::Sum(grad, Dimension::First)));
				gradients.insert(parent, msg);
			},
		}
		Ok(gradients)
	}

	#[inline(always)]
	pub fn get_mut_node(&mut self, index: usize) -> Result<&mut ComputeNode, String>{
		try!(self.nodes.get_mut(index).ok_or_else(|| "The index is out of bounds")).as_mut()
		.ok_or_else(|| format!("The requested node {} is None", index))
	}

	#[inline(always)]
	pub fn get_node(&mut self, index: usize) -> Result<& ComputeNode, String>{
		try!(self.nodes.get(index).ok_or_else(|| "The index is out of bounds")).as_ref()
		.ok_or_else(|| format!("The requested node {} is None", index))
	}

	#[inline(always)]
	pub fn pop_node(&mut self, index: usize) -> Result<ComputeNode, String> {
		self.nodes.push(None);
		self.nodes.swap_remove(index).ok_or_else(|| format!("The requested node {} is None", index))
	}

	#[inline(always)]
	pub fn insert_node(&mut self, index: usize, node: Option<ComputeNode>) -> Option<ComputeNode> {
		self.nodes.push(node);
		self.nodes.swap_remove(index)
	}

	#[inline(always)]
	pub fn is_dependable(&mut self, index: usize) -> Result<bool, String> {
		match try!(self.get_node(index)).node_type.clone(){
			Type::Parameter | Type::ParameterDerived => Ok(true),
			_ => Ok(false)
		}
	}

	pub fn swap_child_connections(&mut self, old_parent: usize, new_parent: usize) -> Result<(), String> {
		if old_parent == new_parent {
			return Ok(())
		}
		// Extract children
		let children = try!(self.get_node(old_parent)).children.clone();
		for child in children.iter(){
			let node = try!(self.get_mut_node(*child));
			// Create new operator by swapping parents
			let operator = match node.op {
				Some(ref op) =>  try!(op.swap_parent(old_parent, new_parent)),
				None => return Err(format!("The child {} of parent {} has no operator", child, old_parent))
			};
			// Set the child operator to the new one
			node.op = Some(operator);
		}
		// Add all children to the new parent
		try!(self.get_mut_node(new_parent)).children.extend(children.into_iter());
		Ok(())
	}

	pub fn string_to_operator(&mut self , name: String, args: Vec<usize>) -> Result<usize,String>{
		match &name[..]{
			"const" => match args.len() {
				1 => self.add_operation(Operator::Const(args[0])),
				_ => Err("Rows takes exactly one argument".to_string())
			},
			"ones" => match args.len() {
				2 => self.add_operation(Operator::Ones(args[0], args[1])),
				_ => Err("Ones takes exactly two argument".to_string())
			},
			"zeros"  => match args.len() {
				2 => self.add_operation(Operator::Zeros(args[0], args[1])),
				_ => Err("Zeros takes exactly two argument".to_string())
			},
			"eye"  => match args.len() {
				1 => self.add_operation(Operator::Eye(args[0])),
				_ => Err("Eye takes exactly one argument".to_string())
			},
			"rows" => match args.len() {
				1 => self.add_operation(Operator::Size(args[0], Dimension::First)),
				_ => Err("Rows takes exactly one argument".to_string())
			},
			"cols" => match args.len() {
				1 => self.add_operation(Operator::Size(args[0], Dimension::Second)),
				_ => Err("Cols takes exactly one argument".to_string())
			},
			"sign" => match args.len() {
				1 => self.add_operation(Operator::Sign(args[0])),
				_ => Err("Sign takes exactly one argument".to_string())
			},
			"inv" => match args.len() {
				1 => self.add_operation(Operator::MatrixInverse(args[0])),
				_ => Err("MatrixInverse takes exactly one argument".to_string())
			},
			"daigM" => match args.len() {
				1 => self.add_operation(Operator::MatrixDiag(args[0])),
				_ => Err("MatrixDiag takes exactly one argument".to_string())
			},
			"diagV" => match args.len() {
				1 => self.add_operation(Operator::VectorDiag(args[0])),
				_ => Err("VectorDiag takes exactly one argument".to_string())
			},
			"cos" => match args.len() {
				1 => self.add_operation(Operator::Cos(args[0])),
				_ => Err("Cos takes exactly one argument".to_string())
			},
			"sin" => match args.len() {
				1 => self.add_operation(Operator::Sin(args[0])),
				_ => Err("Sin takes exactly one argument".to_string())
			},
			"tan" => match args.len() {
				1 => self.add_operation(Operator::Tan(args[0])),
				_ => Err("Tan takes exactly one argument".to_string())
			},
			"cosh" => match args.len() {
				1 => self.add_operation(Operator::CosH(args[0])),
				_ => Err("CosH takes exactly one argument".to_string())
			},
			"sinh" => match args.len() {
				1 => self.add_operation(Operator::SinH(args[0])),
				_ => Err("SinH takes exactly one argument".to_string())
			},
			"tanh" => match args.len() {
				1 => self.add_operation(Operator::TanH(args[0])),
				_ => Err("TanH takes exactly one argument".to_string())
			},
			"abs" => match args.len() {
				1 => self.add_operation(Operator::Abs(args[0])),
				_ => Err("Abs takes exactly one argument".to_string())
			},
			"log" => match args.len() {
				1 => self.add_operation(Operator::Log(args[0])),
				_ => Err("Log takes exactly one argument".to_string())
			},
			"exp" => match args.len() {
				1 => self.add_operation(Operator::Exp(args[0])),
				_ => Err("Exp takes exactly one argument".to_string())
			},
			"sqrt" => match args.len() {
				1 => self.add_operation(Operator::Sqrt(args[0])),
				_ => Err("Sqrt takes exactly one argument".to_string())
			},
			"sq" => match args.len() {
				1 => self.add_operation(Operator::Square(args[0])),
				_ => Err("Square takes exactly one argument".to_string())
			},
			"rect" => match args.len() {
				1 => {
					let extra = self.add_int(0);
					self.add_operation(Operator::Max(args[0], extra))
				},
				_ => Err("Rect takes exactly one argument".to_string())
			},
			"max" => match args.len() {
				2 => {
					self.add_operation(Operator::Max(args[0], args[1]))
				},
				_ => Err("Max takes exactly two arguments".to_string())
			},
			"min" => match args.len() {
				2 => {
					self.add_operation(Operator::Min(args[0], args[1]))
				},
				_ => Err("Max takes exactly two arguments".to_string())
			},
			"sigm" => match args.len() {
				1 => self.add_operation(Operator::Sigmoid(args[0])),
				_ => Err("Sigmoid takes exactly one argument".to_string())
			},
			// "br" => match args.len() {
			// 	2 => {
			// 		let val : i64;
			// 		let ch;
			// 		match self.nodes[args[1]] {
			// 			Some(ComputeNode{node_type: Type::Integer(x), id:_, name:_, ref children , op:_
			// 				, grad_level: _, inline:_, grad_child: _, grad_parents: _}) if x == 1 || x == 2 => {
			// 				val = x;
			// 				ch = children.len();
			// 			}
			// 			_ => return Err("The second argument for Broadcast is missing from the graph or is not 1 or 2.".to_string())
			// 		}
			// 		if self.counter - 1 == args[1] && ch == 0 {
			// 			self.counter -= 1;
			// 			self.nodes.remove(self.counter);					}
			// 		match val  {
			// 			1 => self.add_operation(Operator::Broadcast(args[0], Dimension::First)),
			// 			2 => self.add_operation(Operator::Broadcast(args[0], Dimension::Second)),
			// 			_ => return Err("Broadcast takes as an argument only 1 or 2".to_string())
			// 		}
			// 	},
			// 	_ => return Err("Broadcast takes exactly two arguments".to_string())
			// },
			"sum" => match args.len() {
				2 => {
					let val : i64;
					let ch;
					match self.nodes[args[1]] {
						Some(ComputeNode{node_type: Type::Integer(x), id:_, name:_, ref children , op:_
							, grad_level: _, inline:_, grad_child: _, grad_parents: _}) if x == 0 || x == 1 || x == 2 => {
							val = x;
							ch = children.len();
						}
						_ => return Err("The second argument for Sum is missing from the graph or is not 0,1 or 2.".to_string())
					}
					if self.counter - 1 == args[1] && ch == 0 {
						self.counter -= 1;
						self.nodes.remove(self.counter);
					}
					match val  {
						0 => self.add_operation(Operator::Sum(args[0], Dimension::All)),
						1 => self.add_operation(Operator::Sum(args[0], Dimension::First)),
						2 => self.add_operation(Operator::Sum(args[0], Dimension::Second)),
						_ => Err("Sum takes as an argument only 0,1 or 2".to_string())
					}
				},
				_ => Err("Sum takes exactly two arguments".to_string())
			},
			"l2" => match args.len() {
				2 => {
					let val : i64;
					let ch;
					match self.nodes[args[1]] {
						Some(ComputeNode{node_type: Type::Integer(x), id:_, name:_, ref children , op:_
							, grad_level: _, inline:_, grad_child: _, grad_parents: _}) if x == 0 || x == 1 || x == 2 => {
							val = x;
							ch = children.len();
						}
						_ => return Err("The second argument for L2 is missing from the graph or is not 0,1 or 2.".to_string())
					}
					if self.counter - 1 == args[1] && ch == 0 {
						self.counter -= 1;
						self.nodes.remove(self.counter);
					}
					match val  {
						0 => self.add_operation(Operator::L2(args[0], Dimension::All)),
						1 => self.add_operation(Operator::L2(args[0], Dimension::First)),
						2 => self.add_operation(Operator::L2(args[0], Dimension::Second)),
						_ => Err("L2 takes as an argument only 0,1 or 2".to_string())
					}
				},
				_ => Err("L2 takes exactly two arguments".to_string())
			},
			"l1" => match args.len() {
				2 => {
					let val : i64;
					let ch;
					match self.nodes[args[1]] {
						Some(ComputeNode{node_type: Type::Integer(x), id:_, name:_, ref children , op:_
							, grad_level: _, inline:_, grad_child: _, grad_parents: _}) if x == 0 || x == 1 || x == 2 => {
							val = x;
							ch = children.len();
						}
						_ => return Err("The second argument for L1 is missing from the graph or is not 0,1 or 2.".to_string())
					}
					if self.counter - 1 == args[1] && ch == 0 {
						self.counter -= 1;
						self.nodes.remove(self.counter);
					}
					match val  {
						0 => self.add_operation(Operator::L1(args[0], Dimension::All)),
						1 => self.add_operation(Operator::L1(args[0], Dimension::First)),
						2 => self.add_operation(Operator::L1(args[0], Dimension::Second)),
						_ => Err("L1 takes as an argument only 0,1 or 2".to_string())
					}
				}
				_ => Err("L1 takes exactly two arguments".to_string())
			},
			"dot" => match args.len() {
				0 ... 1 => Err("Dot takes at least two arguments".to_string()),
				_ => self.add_operation(Operator::Dot(args.clone())),
			},
			"horzcat" => match args.len() {
				0 ... 1 => Err("HorzCat takes at least two arguments".to_string()),
				_ => self.add_operation(Operator::HorzCat(args.clone())),
			},
			"vertcat" => match args.len() {
				0 ... 1 => Err("VertCat takes at least two arguments".to_string()),
				_ => self.add_operation(Operator::VertCat(args.clone())),
			},
			"reshape" => match args.len() {
				3 => self.add_operation(Operator::Reshape(args[0], args[1], args[2])),
				_ => Err("Reshape takes exactly three arguments".to_string())
			},
			"replicateH" => match args.len() {
				2 => self.add_operation(Operator::ReplicateHorz(args[0], args[1])),
				_ => Err("ReplicateHorz takes exactly two arguments".to_string())
			},
			"replicateV" => match args.len() {
				2 => self.add_operation(Operator::ReplicateVert(args[0], args[1])),
				_ => Err("ReplicateHorz takes exactly two arguments".to_string())
			},
			_ => Err("Unrecognised function name".to_string())
		}
	}

	pub fn is_function_name(name: &str) -> bool{
		match name{
			"const" | "ones" | "zeros" | "eye" | "rows" | "cols" | "sign" | "inv" | "daigM" | "diagV" | "cos"
			| "sin" | "tan" | "cosh" | "sinh" | "tanh" | "abs"| "log" | "exp" | "sqrt" | "sq" | "rect" | "sigm"
			| "br" | "sum" | "l2" | "l1" | "dot" | "horzcat" | "vertcat"| "reshape" | "replicate" => true,
			_ => false
		}
	}

	pub fn get_params(&self) -> (Vec<usize>, Vec<String>) {
		let mut names : Vec<String> = Vec::new();
		let mut grads : Vec<usize> = Vec::new();
		for option in self.nodes.iter(){
			match *option {
				Some(ref node) => {
					match node.node_type {
						Type::Parameter => {
							names.push(node.name.clone());
							grads.push(node.grad_child.unwrap());
						}
						_ => ()
					}
				},
				None => ()
			}
		}
		(grads, names)
	}

}
