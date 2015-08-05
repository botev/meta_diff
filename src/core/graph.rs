use std::fmt::{Display, Formatter, Error};
use std::string::ToString;
use std::collections::HashMap;
// use std::collections::vec_deque::VecDeque;
use super::operator::*;
use super::node::*;



/// The core of this module - this structure contains the full compute graph
#[derive(Clone, Debug, PartialEq)]
pub struct ComputeGraph{
	counter: usize,
	grad_level: u8,
	pub name: String,
	pub nodes: Vec<Option<ComputeNode>>,
	pub ordering: Vec<usize>,
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
			"Outputs:{:?}\n" ,
			"============= Nodes =============\n"),
		self.name, self.counter, self.outputs));
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
	/// Returns the number of nodes in the graph
	pub fn len(&self) -> usize {
		self.nodes.iter().fold(0, |acc,x| if x.is_some() {acc + 1} else {acc})
	}

	/// Inserts the node argument at the end of the grpah and of the ordering, also assigning it the correct id.
	fn insert_new(&mut self, mut node: ComputeNode) -> usize{
		node.id = self.counter;
		self.nodes.push(Some(node));
		self.ordering.push(self.counter);
		self.counter += 1;
		return self.counter-1
	}

	/// Removes the last element from the graph
	fn remove_last(&mut self) -> Result<usize, GraphError> {
		let last_id = self.counter-1;
		let has_children = try!(self.get_node(last_id)).children.len() > 0;
		if !has_children {
			let order = self.ordering.iter().position(|&x| x == self.counter-1).unwrap();
			self.counter -= 1;
			self.nodes.remove(self.counter);
			self.ordering.remove(order);
			Ok(order)
		} else {
			Err(GraphError::LastHasChildren)
		}

	}

	/// Create a new compute graph for a function with the input name
	pub fn new() -> Self{
		return ComputeGraph{name: "main".to_string(), counter: 0, grad_level: 0, nodes: Vec::new(), ordering: Vec::new(), outputs: Vec::new()}
	}

	/// Creates a new `Parameter` variable with the given name, inserts it in the variable table and returns its id
	pub fn add_parameter(&mut self, name: String) ->  usize {
		let mut node = ComputeNode::new(0, Type::Parameter, self.grad_level,
			Operator::new(From::from(ConstantOperatorType::None), vec![], vec![]).unwrap());
		node.name = name;
		self.insert_new(node)
	}

	/// Creates a new `Float` variable and returns its id
	pub fn add_float(&mut self, value: f64) -> usize{
		let node = ComputeNode::new(0, Type::Float(value), self.grad_level,
			Operator::new(From::from(ConstantOperatorType::None), vec![], vec![]).unwrap());
		self.insert_new(node)
	}

	/// Creates a new `Integer` variable and returns its id
	pub fn add_int(&mut self, value: i64) -> usize{
		let node = ComputeNode::new(0, Type::Integer(value), self.grad_level,
			Operator::new(From::from(ConstantOperatorType::None), vec![], vec![]).unwrap());
		self.insert_new(node)
	}

	/// Creates a new `ConstInput` variable with the given name, inserts it in the variable table and returns its id
	pub fn add_const_input(&mut self, name: String) -> usize{
		let mut node = ComputeNode::new(0, Type::ConstInput, self.grad_level,
			Operator::new(From::from(ConstantOperatorType::None), vec![], vec![]).unwrap());
		node.name = name;
		self.insert_new(node)
	}

	/// Adds a variable coresponding to the input operation to the graph
	pub fn add_operation(&mut self, op_type: OperatorType, args: Vec<usize>)
		-> Result<usize, GraphError> {
		let mut node_type = Type::ConstDerived;
		let id = self.counter;
		let mut op_p : Vec<usize> = Vec::new();
		let mut op_args : Vec<usize> = Vec::new();
		match op_type {
			OperatorType::Special(_) =>  {
				// First argument is the parent all others are the arguments
				if args.len() > 0 {
					op_p.push(args[0]);
				}
				if args.len() > 1 {
					op_args.extend(&args[1..]);
				}
			}
			OperatorType::Constant(_) => {
				// All arguments are the parents
				op_p = args;
			},
			_ => {
				for i in args.iter(){
					// All arguments are the paretns, check if some of them are parameter dependent
					match try!(self.get_mut_node(*i)).node_type {
						Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
						_ => ()
					}
				}
				op_p = args;
			}
		};
		// Create the new node
		let operator = try!(Operator::new(op_type,op_p,op_args));
		// Insert the id as a child of all ancestros
		for i in operator.get_ancestors(){
			try!(self.get_mut_node(*i)).children.push(id);
		}
		let node = ComputeNode::new(0, node_type, self.grad_level, operator);
		Ok(self.insert_new(node))
	}

	/// Generates an ordering of computation
	pub fn generate_ordering(&mut self, mut targets: Vec<usize>) -> Result<Vec<usize>,GraphError> {
		// The spanning tree of the targets, e.g. all nodes required to compute them
		let mut spanning_tree = vec![false; self.counter];
		// Use the targets vector as a stack
		while targets.len() > 0 {
			let node = targets.pop().unwrap();
			// If we see this node for the first time add its parents to the stack
			if !spanning_tree[node] {
				for p in try!(self.get_node(node)).op.get_ancestors() {
						targets.push(*p);
				}
			}
			// Add this node to the spanning tree
			spanning_tree[node] = true;
		}
		Ok(self.ordering.iter().cloned().filter(|&x| spanning_tree[x]).collect::<Vec<usize>>())
	}

	/// Applies gradient operator with the first of the outputs
	pub fn direct_gradient(&mut self) -> Result<(),GraphError> {
		let target = self.outputs[0];
		self.gradient(target)
	}

	/// Applies a gradient operator to the graph given the target
	pub fn gradient(&mut self, target: usize) -> Result<(),GraphError>{
		// Some sensible checks
		match self.nodes[target]{
			Some(ref node) => match node.node_type {
				Type::Parameter | Type::ParameterDerived => (),
				_ => return Ok(())
			},
			None => return Err(GraphError::AccessNoneNode(target))
		}
		self.grad_level += 1;
		// Keeps all gradient messages
		let mut messages : HashMap<usize, Vec<usize>> = HashMap::new();
		let ordering = try!(self.generate_ordering(vec![target]));
		messages.insert(target, vec![self.add_int(1)]);
		for i in ordering.iter().rev(){
			// Get all gradient messages and process them
			let gradient = match messages.remove(&i) {
				Some(vec) => match vec.len() {
					0 => return Err(GraphError::NoGradientMessages(*i)),
					1 => vec[0],
					_ => try!(self.add_operation(OPERATOR_ADD, vec))
				},
				None => continue//return Err(format!("No incoming messages found for node {}", i))
			};
			// Connect the gradient info and the parent
			try!(self.get_mut_node(gradient)).grad_parents.push(*i);
			try!(self.get_mut_node(*i)).grad_child = Some(gradient);
			let gradient = 0;
			// Generate gradient messages and send them to parents
			let grad_msgs = try!(self.op_gradient(*i, gradient));
			for (parent, msg) in grad_msgs{
				let mut mine = messages.entry(parent).or_insert(Vec::new());
				mine.push(msg);
			}
		}
		// let mut grad_outputs: Vec<usize> = Vec::new();
		// // Add gradients of the parameters to outptus
		// for i in (0..self.counter) {
		// 	//let node : Result<&mut ComputeNode, String> = ;
		// 	match self.get_mut_node(i) {
		// 		Ok(ref node) => match node.node_type {
		// 			Type::Parameter => {grad_outputs.push(node.grad_child.unwrap()); ()},
		// 			_ => ()
		// 		},
		// 		Err(_) => ()
		// 	}
		// }
		// for val in grad_outputs {
		// 	self.outputs.push(val);
		// }
		Ok(())
	}

	/// Generates gradient messages from the operator to all of its non constant parents.
	/// Returns a HashMap<parent,msg>
	fn op_gradient(&mut self, child: usize, grad: usize) -> Result<HashMap<usize,usize>, GraphError>{
		let mut gradients : HashMap<usize,usize> = HashMap::new();
		let op = try!(self.get_node(child)).op.clone();
		if op.op_type == OperatorType::Constant(ConstantOperatorType::None) {
			return Ok(gradients)
		}
		match op.op_type{
			OperatorType::Constant(_) => return Err(GraphError::GradientOfConstant(child)),
			OPERATOR_NEG => {
				let msg = try!(self.add_operation(OPERATOR_NEG,vec![grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_DIV => {
				let mut msg = try!(self.add_operation(OPERATOR_SQUARE,op.parents.clone()));
				msg = try!(self.add_operation(OPERATOR_DIV,vec![msg]));
				msg = try!(self.add_operation(OPERATOR_NEG,vec![msg]));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_MINV => {
				let mut msg = try!(self.add_operation(OPERATOR_TRANSPOSE,vec![child]));
				msg = try!(self.add_operation(OPERATOR_DOT,vec![msg,grad,msg]));
				msg = try!(self.add_operation(OPERATOR_NEG,vec![msg]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_TRANSPOSE => {
				let msg = try!(self.add_operation(OPERATOR_TRANSPOSE,vec![grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_MDIAG => {
				let msg = try!(self.add_operation(OPERATOR_VDIAG,vec![grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_VDIAG => {
				let msg = try!(self.add_operation(OPERATOR_MDIAG,vec![grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_COS => {
				let mut msg = try!(self.add_operation(OPERATOR_SIN,op.parents.clone()));
				msg = try!(self.add_operation(OPERATOR_NEG,vec![msg]));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_SIN => {
				let mut msg = try!(self.add_operation(OPERATOR_COS,op.parents.clone()));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_TAN => {
				let mut msg = try!(self.add_operation(OPERATOR_COS,op.parents.clone()));
				msg = try!(self.add_operation(OPERATOR_SQUARE,vec![msg]));
				msg = try!(self.add_operation(OPERATOR_DIV,vec![msg]));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_COSH => {
				let mut msg = try!(self.add_operation(OPERATOR_SINH,op.parents.clone()));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_SINH => {
				let mut msg = try!(self.add_operation(OPERATOR_COSH,op.parents.clone()));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_TANH => {
				let mut msg = try!(self.add_operation(OPERATOR_SQUARE,vec![child]));
				msg = try!(self.add_operation(OPERATOR_NEG,vec![msg]));
				let const_1 = self.add_int(1);
				msg = try!(self.add_operation(OPERATOR_ADD,vec![msg,const_1]));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_ABS => {
				let mut msg = try!(self.add_operation(OPERATOR_SIGN,op.parents.clone()));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_LOG => {
				let mut msg = try!(self.add_operation(OPERATOR_DIV,op.parents.clone()));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_EXP => {
				let msg = try!(self.add_operation(OPERATOR_MUL,vec![child,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_SQRT => {
				let const_half = self.add_float(0.5);
				let mut msg = try!(self.add_operation(OPERATOR_DIV,vec![child]));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![const_half,msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_SQUARE => {
				let const_2 = self.add_int(2);
				let msg = try!(self.add_operation(
					OPERATOR_MUL,vec![const_2,op.parents[0],grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_SIGM => {
				let const_1 = self.add_int(1);
				let mut msg = try!(self.add_operation(OPERATOR_NEG,vec![child]));
				msg = try!(self.add_operation(OPERATOR_ADD,vec![const_1,msg]));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,child,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_RECT => {
				let const_0 = self.add_int(0);
				let mut msg = try!(self.add_operation(
					OPERATOR_GT,vec![op.parents[0],const_0]));
				msg = try!(self.add_operation(OPERATOR_MUL,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_SUM_1 => {
				let rows = try!(self.add_operation(OPERATOR_SIZE_1, vec![op.parents[0]]));
				let msg = try!(self.add_operation(
					OPERATOR_REPLICATEV,vec![grad,rows]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_SUM_2 => {
				let cols = try!(self.add_operation(OPERATOR_SIZE_2, vec![op.parents[0]]));
				let msg = try!(self.add_operation(
					OPERATOR_REPLICATEH,vec![grad,cols]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_SUM_ALL => {
				let rows = try!(self.add_operation(OPERATOR_SIZE_1, vec![op.parents[0]]));
				let cols = try!(self.add_operation(OPERATOR_SIZE_2, vec![op.parents[0]]));
				let mut msg = try!(self.add_operation(
					OPERATOR_REPLICATEV,vec![grad,rows]));
				msg = try!(self.add_operation(
					OPERATOR_REPLICATEH,vec![msg,cols]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_L2_1 => {
				let const_2 = self.add_int(2);
				let rows = try!(self.add_operation(OPERATOR_SIZE_1, vec![op.parents[0]]));
				let mut msg = try!(self.add_operation(
					OPERATOR_REPLICATEV,vec![grad,rows]));
				msg = try!(self.add_operation(
					OPERATOR_MUL,vec![const_2, op.parents[0], msg]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_L2_2 => {
				let const_2 = self.add_int(2);
				let cols = try!(self.add_operation(OPERATOR_SIZE_2, vec![op.parents[0]]));
				let mut msg = try!(self.add_operation(
					OPERATOR_REPLICATEH,vec![grad,cols]));
				msg = try!(self.add_operation(
					OPERATOR_MUL,vec![const_2, op.parents[0], msg]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_L2_ALL => {
				let const_2 = self.add_int(2);
				let msg = try!(self.add_operation(
					OPERATOR_MUL,vec![const_2, op.parents[0], grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_L1_1 => {
				let rows = try!(self.add_operation(OPERATOR_SIZE_1, vec![op.parents[0]]));
				let mut msg = try!(self.add_operation(
					OPERATOR_REPLICATEV,vec![grad,rows]));
				let msg_sign = try!(self.add_operation(OPERATOR_SIGN, vec![op.parents[0]]));
				msg = try!(self.add_operation(
					OPERATOR_MUL,vec![msg_sign, msg]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_L1_2 => {
				let cols = try!(self.add_operation(OPERATOR_SIZE_2, vec![op.parents[0]]));
				let mut msg = try!(self.add_operation(
					OPERATOR_REPLICATEH,vec![grad,cols]));
				let msg_sign = try!(self.add_operation(OPERATOR_SIGN, vec![op.parents[0]]));
				msg = try!(self.add_operation(
					OPERATOR_MUL,vec![msg_sign, msg]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_L1_ALL => {
				let msg = try!(self.add_operation(OPERATOR_SIGN, vec![op.parents[0]]));
				let msg = try!(self.add_operation(
					OPERATOR_MUL,vec![msg, grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_MAX => {
				if try!(self.is_dependable(op.parents[0])){
					let mut msg = try!(self.add_operation(
						OPERATOR_NEG,vec![op.parents[1]]));
					msg = try!(self.add_operation(OPERATOR_ADD,vec![op.parents[0],msg]));
					msg = try!(self.add_operation(OPERATOR_SIGN, vec![msg]));
					msg = try!(self.add_operation(OPERATOR_MUL,vec![msg, grad]));
					gradients.insert(op.parents[0], msg);
				}
				if try!(self.is_dependable(op.parents[1])){
					let mut msg = try!(self.add_operation(
						OPERATOR_NEG,vec![op.parents[0]]));
					msg = try!(self.add_operation(OPERATOR_ADD,vec![op.parents[1],msg]));
					msg = try!(self.add_operation(OPERATOR_SIGN, vec![msg]));
					msg = try!(self.add_operation(OPERATOR_MUL,vec![msg, grad]));
					gradients.insert(op.parents[1], msg);
				}
			},
			OPERATOR_MIN => {
				if try!(self.is_dependable(op.parents[0])){
					let mut msg = try!(self.add_operation(
						OPERATOR_NEG,vec![op.parents[0]]));
					msg = try!(self.add_operation(OPERATOR_ADD,vec![op.parents[1],msg]));
					msg = try!(self.add_operation(OPERATOR_SIGN, vec![msg]));
					msg = try!(self.add_operation(OPERATOR_MUL,vec![msg, grad]));
					gradients.insert(op.parents[0], msg);
				}
				if try!(self.is_dependable(op.parents[1])){
					let mut msg = try!(self.add_operation(
						OPERATOR_NEG,vec![op.parents[1]]));
					msg = try!(self.add_operation(OPERATOR_ADD,vec![op.parents[0],msg]));
					msg = try!(self.add_operation(OPERATOR_SIGN, vec![msg]));
					msg = try!(self.add_operation(OPERATOR_MUL,vec![msg, grad]));
					gradients.insert(op.parents[1], msg);
				}
			},
			OPERATOR_POW => {
				if try!(self.is_dependable(op.parents[0])){
					let mut msg = try!(self.add_operation(
						OPERATOR_DIV,vec![op.parents[0]]));
					msg = try!(self.add_operation(
						OPERATOR_MUL,vec![op.parents[1], child, msg, grad]));
					gradients.insert(op.parents[0], msg);
				}
				if try!(self.is_dependable(op.parents[1])){
					let mut msg = try!(self.add_operation(
						OPERATOR_LOG,vec![op.parents[0]]));
					msg = try!(self.add_operation(OPERATOR_MUL,vec![child, msg, grad]));
					gradients.insert(op.parents[1], msg);
				}
			},
			OPERATOR_QUAD => {
				if try!(self.is_dependable(op.parents[0])){
					let ptr = try!(self.add_operation(
						OPERATOR_TRANSPOSE,vec![op.parents[1]]));
					let msg_1 = try!(self.add_operation(
						OPERATOR_DOT,vec![ptr, op.parents[0], grad]));
					let gradtr = try!(self.add_operation(
						OPERATOR_TRANSPOSE,vec![grad]));
					let msg_2 = try!(self.add_operation(
						OPERATOR_DOT,vec![op.parents[1], op.parents[0], gradtr]));
					let msg = try!(self.add_operation(
						OPERATOR_ADD,vec![msg_1, msg_2]));
					gradients.insert(op.parents[0], msg);
				}
				if try!(self.is_dependable(op.parents[1])){
					let ptr = try!(self.add_operation(
						OPERATOR_TRANSPOSE,vec![op.parents[0]]));
						let msg = try!(self.add_operation(
							OPERATOR_DOT,vec![op.parents[0], grad, ptr]));
						gradients.insert(op.parents[1], msg);
				}
			},
			OPERATOR_ADD => {
				for i in op.parents.iter(){
					if try!(self.is_dependable(*i)){
						gradients.insert(*i, grad);
					}
				}
			},
			OPERATOR_MUL => {
				match op.parents.len(){
					0...1 => return Err(GraphError::Operator(OperatorError::InvalidNumberOfParents(OPERATOR_MUL, 2, op.parents.len()))),
					2 => {
						let p1 = op.parents[0];
						let p2 = op.parents[1];
						if try!(self.is_dependable(p1)){
							let msg = try!(self.add_operation(OPERATOR_MUL,vec![p2, grad]));
							gradients.insert(p1, msg);
						}
						if try!(self.is_dependable(p2)){
							let msg = try!(self.add_operation(OPERATOR_MUL,vec![p1, grad]));
							gradients.insert(p2, msg);
						}
					},
					_ => {
						for i in op.parents.iter(){
							if try!(self.is_dependable(*i)){
								let mut msg = try!(self.add_operation(
									OPERATOR_DIV,vec![*i]));
								msg = try!(self.add_operation(
										OPERATOR_MUL,vec![msg, child, grad]));
								gradients.insert(*i, msg);
							}
						}
					}
				}
			},
			OPERATOR_DOT => {
				let n = op.parents.len();
				match n {
					0...1 => return Err(GraphError::Operator(
						OperatorError::InvalidNumberOfParents(op.op_type,op.parents.len(),2))),
					_ => {
						// Left most parent
						let p1 = op.parents[0];
						if try!(self.is_dependable(p1)) {
							let right_msg =	if n == 2 {
								try!(self.add_operation(OPERATOR_TRANSPOSE, vec![op.parents[1]]))
							} else {
								let right_parents = op.parents[1..].to_owned();
								let msg = try!(self.add_operation(OPERATOR_DOT, right_parents));
								try!(self.add_operation(OPERATOR_TRANSPOSE, vec![msg]))
							};
							let msg = try!(self.add_operation(OPERATOR_DOT,vec![grad, right_msg]));
							gradients.insert(p1, msg);
						}
						// Right most parent
						let pend = op.parents[n-1];
						if try!(self.is_dependable(pend)) {
							let left_msg = if n == 2 {
								try!(self.add_operation(OPERATOR_TRANSPOSE, vec![op.parents[0]]))
							} else {
								let left_parents = op.parents[..n-1].to_owned();
								let msg = try!(self.add_operation(OPERATOR_DOT, left_parents));
								try!(self.add_operation(OPERATOR_TRANSPOSE, vec![msg]))
							};
							let msg = try!(self.add_operation(OPERATOR_DOT, vec![left_msg, grad]));
							gradients.insert(pend, msg);
						}
						if n > 2 {
							// Second from left to right
							let p = op.parents[1];
							if try!(self.is_dependable(p)) {
								let left_msg = try!(self.add_operation(OPERATOR_TRANSPOSE, vec![op.parents[0]]));
								let right_msg = if n == 3 {
									try!(self.add_operation(OPERATOR_TRANSPOSE, vec![op.parents[2]]))
								} else {
									let right_parents = op.parents[2..].to_owned();
									let msg = try!(self.add_operation(OPERATOR_DOT, right_parents));
									try!(self.add_operation(OPERATOR_TRANSPOSE, vec![msg]))
								};
								let msg = try!(self.add_operation(OPERATOR_DOT, vec![left_msg, grad, right_msg]));
								gradients.insert(p, msg);
							}
						}
						if n > 3 {
							// Second from right to left
							let p = op.parents[n-2];
							if try!(self.is_dependable(p)) {
								let right_msg = try!(self.add_operation(OPERATOR_TRANSPOSE, vec![op.parents[n-1]]));
								let left_parents = op.parents[..n-3].to_owned();
								let mut left_msg = try!(self.add_operation(OPERATOR_DOT, left_parents));
								left_msg = try!(self.add_operation(OPERATOR_TRANSPOSE, vec![left_msg]));
								let msg = try!(self.add_operation(OPERATOR_DOT,vec![left_msg, grad, right_msg]));
								gradients.insert(p, msg);
							}
						}
						if n > 4 {
							// Rest
							for i in 2..n-3{
								let p = op.parents[i];
								if try!(self.is_dependable(p)) {
									let left_parents = op.parents[..i-1].to_owned();
									let mut left_msg = try!(self.add_operation(OPERATOR_DOT, left_parents));
									left_msg = try!(self.add_operation(OPERATOR_TRANSPOSE, vec![left_msg]));
									let right_parents = op.parents[i+1..].to_owned();
									let mut right_msg = try!(self.add_operation(OPERATOR_DOT, right_parents));
									right_msg = try!(self.add_operation(OPERATOR_TRANSPOSE, vec![right_msg]));
									let msg = try!(self.add_operation(OPERATOR_DOT, vec![left_msg, grad, right_msg]));
									gradients.insert(p, msg);
								}
							}
						}
					}
				}
			},
			OPERATOR_HORZCAT => {
				let n = op.parents.len();
				match n {
					0...1 => return  Err(GraphError::Operator(
						OperatorError::InvalidNumberOfParents(op.op_type,op.parents.len(),2))),
					_ => {
						let mut last : usize = n;
						for (i,p) in op.parents.iter().enumerate().rev(){
							if try!(self.is_dependable(*p)) {
								last = i;
								break;
							}
						}
						if last < n {
							let const_0 = self.add_int(0);
							let rows =  try!(self.add_operation(OPERATOR_SIZE_1, 					vec![child]));
							let mut accum = self.add_int(0);
							for p in op.parents.iter().take(last+1) {
								let cols = try!(self.add_operation(OPERATOR_SIZE_2, vec![*p]));
								if try!(self.is_dependable(*p)) {
									let msg = try!(self.add_operation(OPERATOR_SUBINDEX, vec![grad,const_0,rows, accum, cols]));
									gradients.insert(*p, msg);
								}
								accum = try!(self.add_operation(OPERATOR_ADD, vec![accum, cols]));
							}
						}
					}
				}
			},
			OPERATOR_VERTCAT => {
				let n = op.parents.len();
				match n{
					0...1 => return  Err(GraphError::Operator(
						OperatorError::InvalidNumberOfParents(op.op_type,op.parents.len(),2))),
					_ => {
						let mut last : usize = n;
						for (i,p) in op.parents.iter().enumerate().rev(){
							if try!(self.is_dependable(*p)) {
								last = i;
								break;
							}
						}
						if last < n {
							let const_0 = self.add_int(0);
							let cols =  try!(self.add_operation(OPERATOR_SIZE_2, 					vec![child]));
							let mut accum = self.add_int(0);
							for p in op.parents.iter().take(last+1) {
								let rows = try!(self.add_operation(OPERATOR_SIZE_1, vec![*p]));
								if try!(self.is_dependable(*p)) {
									let msg = try!(self.add_operation(OPERATOR_SUBINDEX, vec![grad,accum,rows, const_0, cols]));
									gradients.insert(*p, msg);
								}
								accum = try!(self.add_operation(OPERATOR_ADD, vec![accum, rows]));
							}
						}
					}
				}
			},
			OPERATOR_SUBINDEX => {
				let mut new_parents = vec![grad];
				new_parents.extend(op.parents.iter().skip(1).cloned());
				let msg = try!(self.add_operation(
					OPERATOR_SUBASSIGN,new_parents));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_SUBASSIGN => {
				let mut new_parents = vec![grad];
				new_parents.extend(op.parents.iter().skip(1).cloned());
				let msg = try!(self.add_operation(
					OPERATOR_SUBINDEX,new_parents));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_RESHAPE => {
				let rows = try!(self.add_operation(
					OPERATOR_SIZE_1, vec![op.parents[0]]));
				let cols = try!(self.add_operation(
					OPERATOR_SIZE_2, vec![op.parents[0]]));
				let msg = try!(self.add_operation(
					OPERATOR_RESHAPE,vec![grad, rows, cols]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_REPLICATEH => {
				let msg = try!(self.add_operation(
					OPERATOR_SUM_2,vec![grad]));
				gradients.insert(op.parents[0], msg);
			},
			OPERATOR_REPLICATEV => {
				let msg = try!(self.add_operation(
					OPERATOR_SUM_1,vec![grad]));
				gradients.insert(op.parents[0], msg);
			}
		}
		Ok(gradients)
	}

	// #[inline(always)]
	pub fn get_mut_node(&mut self, index: usize) -> Result<&mut ComputeNode, GraphError>{
		let l = self.nodes.len();
		try!(self.nodes.get_mut(index).ok_or_else(
			|| GraphError::IndexOutOfBounds(index,l)))
			.as_mut().ok_or_else(
				|| GraphError::AccessNoneNode(index))
	}

	// #[inline(always)]
	pub fn get_node(&mut self, index: usize) -> Result<& ComputeNode, GraphError>{
		try!(self.nodes.get(index).ok_or_else(
			|| GraphError::IndexOutOfBounds(index,self.nodes.len())))
			.as_ref().ok_or_else(
				|| GraphError::AccessNoneNode(index))
	}

	// #[inline(always)]
	pub fn pop_node(&mut self, index: usize) -> Result<ComputeNode, GraphError> {
		self.nodes.push(None);
		self.nodes.swap_remove(index).ok_or_else(|| GraphError::AccessNoneNode(index))
	}

	// #[inline(always)]
	pub fn insert_node(&mut self, index: usize, node: Option<ComputeNode>) -> Option<ComputeNode> {
		self.nodes.push(node);
		self.nodes.swap_remove(index)
	}

	// #[inline(always)]
	pub fn is_dependable(&mut self, index: usize) -> Result<bool, GraphError> {
		match try!(self.get_node(index)).node_type.clone(){
			Type::Parameter | Type::ParameterDerived => Ok(true),
			_ => Ok(false)
		}
	}

	pub fn swap_child_connections(&mut self, old_parent: usize, new_parent: usize) -> Result<(), GraphError> {
		if old_parent == new_parent {
			return Ok(())
		}
		// Extract children
		let children = try!(self.get_node(old_parent)).children.clone();
		for child in children.iter(){
			try!(try!(self.get_mut_node(*child)).op.swap_parent_in_place(old_parent, new_parent));
		}
		// Add all children to the new parent
		try!(self.get_mut_node(new_parent)).children.extend(children.iter().cloned());
		Ok(())
	}

	pub fn string_to_operator(&mut self , name: String, args: Vec<usize>) -> Result<usize,GraphError>{
		match &name[..]{
			"const" => Ok(try!(self.add_operation(OPERATOR_CONST, args))),
			"eye" => Ok(try!(self.add_operation(OPERATOR_EYE, args))),
			"sign" => Ok(try!(self.add_operation(OPERATOR_SIGN, args))),
			"rows" => Ok(try!(self.add_operation(OPERATOR_SIZE_1, args))),
			"cols" => Ok(try!(self.add_operation(OPERATOR_SIZE_2, args))),
			"ones" => Ok(try!(self.add_operation(OPERATOR_ONES, args))),
			"zeros"  => Ok(try!(self.add_operation(OPERATOR_ZEROS, args))),
			"lt" => Ok(try!(self.add_operation(OPERATOR_LT, args))),
			"lte" => Ok(try!(self.add_operation(OPERATOR_LTE, args))),
			"gt" => Ok(try!(self.add_operation(OPERATOR_GT, args))),
			"gte" => Ok(try!(self.add_operation(OPERATOR_GTE, args))),
			"eq" => Ok(try!(self.add_operation(OPERATOR_EQ, args))),
			"neq" => Ok(try!(self.add_operation(OPERATOR_NEQ, args))),
			"neg" => Ok(try!(self.add_operation(OPERATOR_NEG, args))),
			"div" => Ok(try!(self.add_operation(OPERATOR_DIV, args))),
			"minv" => Ok(try!(self.add_operation(OPERATOR_MINV, args))),
			"tr" => Ok(try!(self.add_operation(OPERATOR_TRANSPOSE, args))),
			"mdiag" => Ok(try!(self.add_operation(OPERATOR_MDIAG, args))),
			"vdiag" => Ok(try!(self.add_operation(OPERATOR_VDIAG, args))),
			"cos" => Ok(try!(self.add_operation(OPERATOR_COS, args))),
			"sin" => Ok(try!(self.add_operation(OPERATOR_SIN, args))),
			"tan" => Ok(try!(self.add_operation(OPERATOR_TAN, args))),
			"cosh" => Ok(try!(self.add_operation(OPERATOR_COSH, args))),
			"sinh" => Ok(try!(self.add_operation(OPERATOR_SINH, args))),
			"tanh" => Ok(try!(self.add_operation(OPERATOR_TANH, args))),
			"abs" => Ok(try!(self.add_operation(OPERATOR_ABS, args))),
			"log" => Ok(try!(self.add_operation(OPERATOR_LOG, args))),
			"exp" => Ok(try!(self.add_operation(OPERATOR_EXP, args))),
			"sqrt" => Ok(try!(self.add_operation(OPERATOR_SQRT, args))),
			"square" => Ok(try!(self.add_operation(OPERATOR_SQUARE, args))),
			"sigm" => Ok(try!(self.add_operation(OPERATOR_SIGM, args))),
			"rect" => Ok(try!(self.add_operation(OPERATOR_RECT, args))),
			"sum" => match args.len(){
				2 => match try!(self.get_node(args[1])).node_type {
					Type::Integer(x) => {
						match x {
							0 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(OPERATOR_SUM_ALL, vec![args[0]]));
								Ok(result)
							},
							1 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									OPERATOR_SUM_1, vec![args[0]]));
								Ok(result)
							},
							2 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									OPERATOR_SUM_2, vec![args[0]]));
								Ok(result)
							},
							_ => return Err(GraphError::Operator(
								OperatorError::InvalidDimensionArgument("SUM".to_string(),x as usize, vec![0,1,2])))
						}
					},
					_ => return Err(GraphError::Operator(
						OperatorError::InvalidDimensionArgument("SUM".to_string(),999, vec![0,1,2])))
				},
				_ => return Err(GraphError::Operator(
					OperatorError::InvalidNumberOfParents(OPERATOR_SUM_ALL, args.len()-1, 1)))
			},
			"l2" => match args.len(){
				2 => match try!(self.get_node(args[1])).node_type {
					Type::Integer(x) => {
						match x {
							0 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									OPERATOR_L2_ALL, vec![args[0]]));
								Ok(result)
							},
							1 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									OPERATOR_L2_1, vec![args[0]]));
								Ok(result)
							},
							2 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									OPERATOR_L2_2, vec![args[0]]));
								Ok(result)
							},
							x => return Err(GraphError::Operator(
								OperatorError::InvalidDimensionArgument("L2".to_string(),x as usize, vec![0,1,2])))
						}
					},
					_ => return Err(GraphError::Operator(
						OperatorError::InvalidDimensionArgument("L2".to_string(), 999, vec![0,1,2])))
				},
				_ => return Err(GraphError::Operator(
					OperatorError::InvalidNumberOfParents(OPERATOR_L2_ALL, args.len()-1, 1)))
			},
			"l1" => match args.len(){
				2 => match try!(self.get_node(args[1])).node_type {
					Type::Integer(x) => {
						match x {
							0 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									OPERATOR_L1_ALL, vec![args[0]]));
								Ok(result)
							},
							1 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									OPERATOR_L1_1, vec![args[0]]));
								Ok(result)
							},
							2 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									OPERATOR_L1_2, vec![args[0]]));
								Ok(result)
							},
							x => return Err(GraphError::Operator(OperatorError::InvalidDimensionArgument("L1".to_string(),x as usize, vec![0,1,2])))
						}
					},
					_ => return Err(GraphError::Operator(
						OperatorError::InvalidDimensionArgument("L1".to_string(),999 , vec![0,1,2])))
				},
				_ => return Err(GraphError::Operator(
					OperatorError::InvalidNumberOfParents(OPERATOR_L1_ALL, args.len()-1, 1)))
			},
			"max" => Ok(try!(self.add_operation(OPERATOR_MAX, args))),
			"min" => Ok(try!(self.add_operation(OPERATOR_MIN, args))),
			"pow" => Ok(try!(self.add_operation(OPERATOR_POW, args))),
			"quad" => Ok(try!(self.add_operation(OPERATOR_QUAD, args))),
			"subind" => Ok(try!(self.add_operation(OPERATOR_SUBINDEX, args))),
			"subasign" => Ok(try!(self.add_operation(OPERATOR_SUBASSIGN, args))),
			"reshape" => Ok(try!(self.add_operation(OPERATOR_RESHAPE, args))),
			"replicateH" => Ok(try!(self.add_operation(OPERATOR_REPLICATEH, args))),
			"replicateV" => Ok(try!(self.add_operation(OPERATOR_REPLICATEV, args))),
			"add" => Ok(try!(self.add_operation(OPERATOR_ADD, args))),
			"mul" => Ok(try!(self.add_operation(OPERATOR_MUL, args))),
			"dot" => Ok(try!(self.add_operation(OPERATOR_DOT, args))),
			"horzcat" => Ok(try!(self.add_operation(OPERATOR_HORZCAT, args))),
			"vertcat" => Ok(try!(self.add_operation(OPERATOR_VERTCAT, args))),
			_ => Err(GraphError::UnknownFunction(name.clone()))
		}
	}
	pub fn is_function_name(name: &str) -> bool{
		match name{
			"const" | "eye" | "sign" | "rows" | "cols" | "ones" | "zeros"
			| "minv" | "mdiag" | "vdiag" | "cos" | "sin" | "tan" | "cosh" | "sinh" | "tanh"
			| "abs"| "log" | "exp" | "sqrt" | "square" | "sigm" | "rect" | "sum" | "l2" | "l1"
			| "max" | "min" | "pow" | "quad" | "reshape" | "replicateH"| "replicateV"
			| "horzcat" | "vertcat" | "dot" => true,
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

#[derive(Clone, Debug)]
pub enum GraphError {
	AccessNoneNode(usize),
	IndexOutOfBounds(usize,usize),
	UnknownFunction(String),
	LastHasChildren,
	GradientOfConstant(usize),
	NoGradientMessages(usize),
	Operator(OperatorError)
}

impl ::std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
			GraphError::AccessNoneNode(x) => write!(f, "AccessNoneNode: Trying to access node {},  but its none", x),
			GraphError::IndexOutOfBounds(n,l) => write!(f, "IndexOutOfBounds: Trying to access node {}, but currently the counter is {}", n,l),
			GraphError::UnknownFunction(ref name) => write!(f, "UnknownFunction: {}", name),
			GraphError::LastHasChildren => write!(f, "Last node already has children"),
			GraphError::GradientOfConstant(n) => write!(f, "Can not take a gradient with respect to a cosntant node - {}", n),
			GraphError::NoGradientMessages(n) => write!(f, "No gradient messages found for node {}", n),
            GraphError::Operator(ref err) => write!(f, "OperatorError: {}", err),
        }
    }
}

impl ::std::error::Error for GraphError {
    fn description(&self) -> &str {
        match *self {
			GraphError::AccessNoneNode(_) => "Trying to access a None node",
			GraphError::IndexOutOfBounds(_,_) => "Accessing node index out of bounds",
			GraphError::UnknownFunction(_) => "Trying to use an unkown function",
			GraphError::LastHasChildren => "Last node already has children",
			GraphError::GradientOfConstant(_) => "Taking gradient with respect to a constant",
			GraphError::NoGradientMessages(_) => "No gradient messages were send for a required node",
            GraphError::Operator(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
			GraphError::Operator(ref err) => Some(err),
			_ => None
        }
    }
}

impl ::std::convert::From<OperatorError> for GraphError {
    fn from(err: OperatorError) -> GraphError {
        GraphError::Operator(err)
    }
}
