use std::fmt::{Display, Formatter, Error};
use std::string::ToString;
use std::collections::HashMap;
// use std::collections::vec_deque::VecDeque;
use super::operator::*;
use super::node::*;



/// The core of this module - this structure contains the full compute graph
#[derive(Clone, Debug)]
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
	pub fn add_operation<T>(&mut self, op_type: T, args: Vec<usize>)
		-> Result<usize, GraphError>
		where T: ::std::convert::Into<OperatorType> {
		let mut node_type = Type::ConstDerived;
		let id = self.counter;
		let converted = ::std::convert::Into::into(op_type);
		let op = match converted {
			OperatorType::Special(_) =>  {
				if args.len() == 0 {
					try!(Operator::new(converted,vec![],vec![]))
				} else {
					try!(Operator::new(converted,vec![args[0]],args[1..].to_vec()))
				}
			}
			OperatorType::Constant(_) => {
				try!(Operator::new(converted,args,Vec::new()))
			},
			_ => {
				for i in args.iter(){
					// TODO try! instead of unwrap
					match self.get_mut_node(*i).unwrap().node_type {
						Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
						_ => ()
					}
				}
				try!(Operator::new(converted,args,Vec::new()))
			}
		};
		for i in op.get_ancestors(){
			// TODO try! instead of unwrap
			self.get_mut_node(*i).unwrap().children.push(id);
		}
		// 	 => Operator::new(op, args, vec![]),
		//
		// 	Operator::Const(parent_id) |  Operator::Eye(parent_id) | Operator::Size(parent_id, _)
		// 	| Operator::Sign(parent_id)=> {
		// 		try!(self.get_mut_node(parent_id)).children.push(new_id);
		// 	},
		// 	Operator::Ones(parent_id1, parent_id2) | Operator::Zeros(parent_id1, parent_id2)
		// 	| Operator::LessThan(parent_id1, parent_id2) | Operator::LessThanOrEqual(parent_id1, parent_id2)
		// 	| Operator::GreaterThan(parent_id1, parent_id2) | Operator::GreaterThanOrEqual(parent_id1, parent_id2) => {
		// 		try!(self.get_mut_node(parent_id1)).children.push(new_id);
		// 		try!(self.get_mut_node(parent_id2)).children.push(new_id);
		// 	},
		// 	Operator::Neg(parent_id) | Operator::Div(parent_id) | Operator::MatrixInverse(parent_id)
		// 	| Operator::Transpose(parent_id) | Operator::MatrixDiag(parent_id) | Operator::VectorDiag(parent_id)
		// 	| Operator::Cos(parent_id) | Operator::Sin(parent_id) | Operator::Tan(parent_id)
		// 	| Operator::CosH(parent_id) | Operator::SinH(parent_id) | Operator::TanH(parent_id)
		// 	| Operator::Abs(parent_id) | Operator::Log(parent_id) | Operator::Exp(parent_id)
		// 	| Operator::Sqrt(parent_id) | Operator::Square(parent_id)
		// 	| Operator::Sigmoid(parent_id) | Operator::Sum(parent_id,_)
		// 	| Operator::L2(parent_id,_) | Operator::L1(parent_id,_) => {
		// 		//| Operator::Broadcast(parent_id,_) =>
		// 		try!(self.get_mut_node(parent_id)).children.push(new_id);
		// 		match try!(self.get_mut_node(parent_id)).node_type {
		// 			Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
		// 			_ => ()
		// 		}
		// 	},
		// 	Operator::Pow(parent_id1, parent_id2) | Operator::Quadratic(parent_id1, parent_id2)
		// 	| Operator::Max(parent_id1, parent_id2) | Operator::Min(parent_id1, parent_id2) => {
		// 		try!(self.get_mut_node(parent_id1)).children.push(new_id);
		// 		match try!(self.get_mut_node(parent_id1)).node_type {
		// 			Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
		// 			_ => ()
		// 		}
		// 		try!(self.get_mut_node(parent_id2)).children.push(new_id);
		// 		match try!(self.get_mut_node(parent_id2)).node_type {
		// 			Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
		// 			_ => ()
		// 		}
		// 	},
		// 	Operator::Add(ref parent_ids) | Operator::Mul(ref parent_ids) | Operator::Dot(ref parent_ids)
		// 	| Operator::HorzCat(ref parent_ids) | Operator::VertCat(ref parent_ids) => {
		// 		for parent_id in parent_ids.iter().cloned(){
		// 			try!(self.get_mut_node(parent_id)).children.push(new_id);
		// 			match try!(self.get_mut_node(parent_id)).node_type {
		// 				Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
		// 				_ => ()
		// 			}
		// 		}
		// 	},
		// 	Operator::SubIndex(parent_id, arg_id1, arg_id2, arg_id3, arg_id4) |
		// 	Operator::SubAssign(parent_id, arg_id1, arg_id2, arg_id3, arg_id4) => {
		// 		try!(self.get_mut_node(parent_id)).children.push(new_id);
		// 		match try!(self.get_mut_node(parent_id)).node_type {
		// 			Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
		// 			_ => ()
		// 		}
		// 		let args = vec![arg_id1, arg_id2, arg_id3, arg_id4];
		// 		for arg_id in args{
		// 			try!(self.get_mut_node(arg_id)).children.push(new_id);
		// 		}
		// 	},
		// 	Operator::Reshape(parent_id, arg_id1, arg_id2) => {
		// 		try!(self.get_mut_node(parent_id)).children.push(new_id);
		// 		match try!(self.get_mut_node(parent_id)).node_type {
		// 			Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
		// 			_ => ()
		// 		}
		// 		let args = vec![arg_id1, arg_id2];
		// 		for arg_id in args{
		// 			try!(self.get_mut_node(arg_id)).children.push(new_id);
		// 		}
		// 	},
		// 	Operator::ReplicateHorz(parent_id, arg_id) | Operator::ReplicateVert(parent_id, arg_id) => {
		// 		try!(self.get_mut_node(parent_id)).children.push(new_id);
		// 		match try!(self.get_mut_node(parent_id)).node_type {
		// 			Type::Parameter | Type::ParameterDerived => node_type = Type::ParameterDerived,
		// 			_ => ()
		// 		}
		// 		try!(self.get_mut_node(arg_id)).children.push(new_id);
		// 	}
		// }
		let node = ComputeNode::new(0, node_type, self.grad_level, op);
		Ok(self.insert_new(node))
	}

	/// Generates an ordering of computation
	pub fn generate_ordering(&mut self, mut targets: Vec<usize>) -> Result<Vec<usize>,GraphError> {
		let mut spanning_tree = vec![false; self.counter];
		// Generate spanning tree for the target
		// let mut stack : Vec<usize> = Vec::new();
		while targets.len() > 0 {
			// println!("S1");
			let node = targets.pop().unwrap();
			// println!("S2");
			// println!("{}-{}",spanning_tree.len(),node);
			if !spanning_tree[node] {
				for p in try!(self.get_node(node)).op.get_ancestors() {
						targets.push(*p);
				}
			}
			spanning_tree[node] = true;
		}
		Ok(self.ordering.iter().cloned().filter(|&x| spanning_tree[x]).collect::<Vec<usize>>())

		// let mut ordering : Vec<usize> = Vec::new();
		// let mut processed : HashSet<usize> = HashSet::new();
		// let _ = self.nodes.iter().enumerate().map(|x| if x.1.is_none(){processed.insert(x.0);});
		// let mut change = true;
		// //let n = self.nodes.len();
		// while change {
		// 	change = false;
		// 	for (i,node) in self.nodes.iter().enumerate() {
		// 		if !processed.contains(&i) {
		// 			match *node {
		// 				Some(ref n) => {
		// 					match n.op{
		// 						Some(ref operator) => {
		// 							if operator.get_ancestors().iter()
		// 								.fold(true, |acc, x| acc && processed.contains(x)){
		// 									ordering.push(i);
		// 									processed.insert(i);
		// 									change = true;
		// 								}
		// 						},
		// 						None => {
		// 							ordering.push(i);
		// 							processed.insert(i);
		// 							change = true;
		// 						}
		// 					}
		// 				},
		// 				None => (),
		// 			}
		// 		}
		// 	}
		// }
		// ordering
		// Ok(vec![1])
	}

	/// Applies gradient operator with the target held in the graph
	pub fn direct_gradient(&mut self) -> Result<(),GraphError>{
		let target = self.outputs[0];
		self.gradient(target)
	}

	/// Applies a gradient operator to the graph given the target
	pub fn gradient(&mut self, target: usize) -> Result<(),GraphError>{
		match self.nodes[target]{
			Some(ref node) => match node.node_type {
				Type::Parameter | Type::ParameterDerived => (),
				_ => return Ok(())
			},
			None => return Err(GraphError::AccessNoneNode(target))
		}
		self.grad_level += 1;
		let mut messages : HashMap<usize, Vec<usize>> = HashMap::new();
		// let mut span : Vec<bool> = self.nodes.iter().cloned().map(|_| false).collect::<Vec<bool>>();
		let ordering = try!(self.generate_ordering(vec![target]));
		println!("Ordering: {:?}",ordering);
		// let mut stack = VecDeque::new();
		// stack.push_back(target);

		// span.push(true);
		// span.swap_remove(self.target);

		messages.insert(target, vec![self.add_int(1)]);
		for i in ordering.iter().rev(){
		// while stack.len() > 0 {
		// for i in (0..self.target + 1).rev(){
			// Skip if the node is not in the spanning tree of the target
			// if !span[i] {
				// continue;
			// }

			// let i = stack.pop_front().unwrap();
			// println!("Poping {}",i);
			// println!("Messages: {:?}", messages);
			// Get the gradient of the current node
			let gradient = match messages.remove(&i) {
				Some(vec) => match vec.len() {
					0 => return Err(GraphError::NoGradientMessages(*i)),
					1 => vec[0],
					// TODO change unwrap to try
					_ => self.add_operation(OperatorType::Nary(NaryOperatorType::Add), vec).unwrap()
				},
				None => continue//return Err(format!("No incoming messages found for node {}", i))
			};
			// Connect the gradient info and the parent
			try!(self.get_mut_node(gradient)).grad_parents.push(*i);
			try!(self.get_mut_node(*i)).grad_child = Some(gradient);
			let gradient = 0;
			// Generate gradient messages
			let grad_msgs = try!(self.op_gradient(*i, gradient));

			for (parent, msg) in grad_msgs{
				// Mark that that the parent is in the sapnning tree
				// span.push(true);
				// span.swap_remove(parent);
				// Add message to his incomings
				let mut mine = if messages.contains_key(&parent) {
					messages.get_mut(&parent).unwrap()
				} else {
					// println!("no found for parent {} sending form {}", parent, i);
					// stack.push_back(parent);
					messages.insert(parent, Vec::new());
					messages.get_mut(&parent).unwrap()
				};
				mine.push(msg);
				// println!("Inserting message from {} to {} with id {},{}", i, parent, msg, mine.len());
				// messages.insert(parent,mine);
				// let mut mine = match messages.remove(0).unrwap();
				// println!("One : {}", mine.len());
				// messages.insert(parent,mine);
			}
			// println!("Messages: {:?}", messages);
			// println!("Finishing {}", i);
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
	fn op_gradient(&mut self, child: usize, grad: usize) -> Result<HashMap<usize,usize>, GraphError>{
		let mut gradients : HashMap<usize,usize> = HashMap::new();
		let op = try!(self.get_node(child)).op.clone();
		if op.op_type == OperatorType::Constant(ConstantOperatorType::None) {
			return Ok(gradients)
		}
		match op.op_type{
			OperatorType::Constant(_) => return Err(GraphError::GradientOfConstant(child)),
			OperatorType::Unary(UnaryOperatorType::Neg) => {
				let msg = try!(self.add_operation(UnaryOperatorType::Neg,vec![grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Div) => {
				let mut msg = try!(self.add_operation(UnaryOperatorType::Square,op.parents.clone()));
				msg = try!(self.add_operation(UnaryOperatorType::Div,vec![msg]));
				msg = try!(self.add_operation(UnaryOperatorType::Neg,vec![msg]));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::MatrixInverse) => {
				let mut msg = try!(self.add_operation(UnaryOperatorType::Transpose,vec![child]));
				msg = try!(self.add_operation(NaryOperatorType::Dot,vec![msg,grad,msg]));
				msg = try!(self.add_operation(UnaryOperatorType::Neg,vec![msg]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Transpose) => {
				let msg = try!(self.add_operation(UnaryOperatorType::Transpose,vec![grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::MatrixDiag) => {
				let msg = try!(self.add_operation(UnaryOperatorType::VectorDiag,vec![grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::VectorDiag) => {
				let msg = try!(self.add_operation(UnaryOperatorType::MatrixDiag,vec![grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Cos) => {
				let mut msg = try!(self.add_operation(UnaryOperatorType::Sin,op.parents.clone()));
				msg = try!(self.add_operation(UnaryOperatorType::Neg,vec![msg]));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Sin) => {
				let mut msg = try!(self.add_operation(UnaryOperatorType::Cos,op.parents.clone()));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Tan) => {
				let mut msg = try!(self.add_operation(UnaryOperatorType::Cos,op.parents.clone()));
				msg = try!(self.add_operation(UnaryOperatorType::Square,vec![msg]));
				msg = try!(self.add_operation(UnaryOperatorType::Div,vec![msg]));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::CosH) => {
				let mut msg = try!(self.add_operation(UnaryOperatorType::SinH,op.parents.clone()));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::SinH) => {
				let mut msg = try!(self.add_operation(UnaryOperatorType::CosH,op.parents.clone()));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::TanH) => {
				let mut msg = try!(self.add_operation(UnaryOperatorType::Square,vec![child]));
				msg = try!(self.add_operation(UnaryOperatorType::Neg,vec![msg]));
				let const_1 = self.add_int(1);
				msg = try!(self.add_operation(NaryOperatorType::Add,vec![msg,const_1]));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Abs) => {
				let mut msg = try!(self.add_operation(ConstantUnaryOperatorType::Sign,op.parents.clone()));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Log) => {
				let mut msg = try!(self.add_operation(UnaryOperatorType::Div,op.parents.clone()));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Exp) => {
				let msg = try!(self.add_operation(NaryOperatorType::Mul,vec![child,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Sqrt) => {
				let const_half = self.add_float(0.5);
				let mut msg = try!(self.add_operation(UnaryOperatorType::Div,vec![child]));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![const_half,msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Square) => {
				let const_2 = self.add_int(2);
				let msg = try!(self.add_operation(
					NaryOperatorType::Mul,vec![const_2,op.parents[0],grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Sigmoid) => {
				let const_1 = self.add_int(1);
				let mut msg = try!(self.add_operation(UnaryOperatorType::Neg,vec![child]));
				msg = try!(self.add_operation(NaryOperatorType::Add,vec![const_1,msg]));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,child,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Rectifier) => {
				let const_0 = self.add_int(0);
				let mut msg = try!(self.add_operation(
					ConstantBinaryOperatorType::GreaterThan,vec![op.parents[0],const_0]));
				msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg,grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Unary(UnaryOperatorType::Sum(dim)) => {
				match dim {
					Dimension::First => {
						let rows = try!(self.add_operation(ConstantUnaryOperatorType::Size(dim), vec![op.parents[0]]));
						let msg = try!(self.add_operation(
							SpecialUnaryOperatorType::ReplicateVert,vec![grad,rows]));
						gradients.insert(op.parents[0], msg);
					},
					Dimension::Second => {
						let cols = try!(self.add_operation(ConstantUnaryOperatorType::Size(dim), vec![op.parents[0]]));
						let msg = try!(self.add_operation(
							SpecialUnaryOperatorType::ReplicateHorz,vec![grad,cols]));
						gradients.insert(op.parents[0], msg);
					},
					Dimension::All => {
						let rows = try!(self.add_operation(
							ConstantUnaryOperatorType::Size(Dimension::First), vec![op.parents[0]]));
						let cols = try!(self.add_operation(
							ConstantUnaryOperatorType::Size(Dimension::Second), vec![op.parents[0]]));
						let mut msg = try!(self.add_operation(
							SpecialUnaryOperatorType::ReplicateVert,vec![grad,rows]));
						msg = try!(self.add_operation(
							SpecialUnaryOperatorType::ReplicateHorz,vec![msg,cols]));
						gradients.insert(op.parents[0], msg);
					}
				}
			},
			OperatorType::Unary(UnaryOperatorType::L2(dim)) => {
				let const_2 = self.add_int(2);
				match dim {
					Dimension::First => {
						let rows = try!(self.add_operation(ConstantUnaryOperatorType::Size(dim), vec![op.parents[0]]));
						let mut msg = try!(self.add_operation(
							SpecialUnaryOperatorType::ReplicateVert,vec![grad,rows]));
						msg = try!(self.add_operation(
							NaryOperatorType::Mul,vec![const_2, op.parents[0], msg]));
						gradients.insert(op.parents[0], msg);
					},
					Dimension::Second => {
						let cols = try!(self.add_operation(ConstantUnaryOperatorType::Size(dim), vec![op.parents[0]]));
						let mut msg = try!(self.add_operation(
							SpecialUnaryOperatorType::ReplicateHorz,vec![grad,cols]));
						msg = try!(self.add_operation(
							NaryOperatorType::Mul,vec![const_2, op.parents[0], msg]));
						gradients.insert(op.parents[0], msg);
					},
					Dimension::All => {
						let msg = try!(self.add_operation(
							NaryOperatorType::Mul,vec![const_2, op.parents[0], grad]));
						gradients.insert(op.parents[0], msg);
					}
				}
			},
			OperatorType::Unary(UnaryOperatorType::L1(dim)) => {
				match dim {
					Dimension::First => {
						let rows = try!(self.add_operation(ConstantUnaryOperatorType::Size(dim), vec![op.parents[0]]));
						let mut msg = try!(self.add_operation(
							SpecialUnaryOperatorType::ReplicateVert,vec![grad,rows]));
						let msg_sign = try!(self.add_operation(ConstantUnaryOperatorType::Sign, vec![op.parents[0]]));
						msg = try!(self.add_operation(
							NaryOperatorType::Mul,vec![msg_sign, msg]));
						gradients.insert(op.parents[0], msg);
					},
					Dimension::Second => {
						let cols = try!(self.add_operation(ConstantUnaryOperatorType::Size(dim), vec![op.parents[0]]));
						let mut msg = try!(self.add_operation(
							SpecialUnaryOperatorType::ReplicateHorz,vec![grad,cols]));
						let msg_sign = try!(self.add_operation(ConstantUnaryOperatorType::Sign, vec![op.parents[0]]));
						msg = try!(self.add_operation(
							NaryOperatorType::Mul,vec![msg_sign, msg]));
						gradients.insert(op.parents[0], msg);
					},
					Dimension::All => {
						let msg = try!(self.add_operation(ConstantUnaryOperatorType::Sign, vec![op.parents[0]]));
						let msg = try!(self.add_operation(
							NaryOperatorType::Mul,vec![msg, grad]));
						gradients.insert(op.parents[0], msg);
					}
				}
			},
			OperatorType::Binary(BinaryOperatorType::Max) => {
				if try!(self.is_dependable(op.parents[0])){
					let mut msg = try!(self.add_operation(
						UnaryOperatorType::Neg,vec![op.parents[1]]));
					msg = try!(self.add_operation(NaryOperatorType::Add,vec![op.parents[0],msg]));
					msg = try!(self.add_operation(ConstantUnaryOperatorType::Sign, vec![msg]));
					msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg, grad]));
					gradients.insert(op.parents[0], msg);
				}
				if try!(self.is_dependable(op.parents[1])){
					let mut msg = try!(self.add_operation(
						UnaryOperatorType::Neg,vec![op.parents[0]]));
					msg = try!(self.add_operation(NaryOperatorType::Add,vec![op.parents[1],msg]));
					msg = try!(self.add_operation(ConstantUnaryOperatorType::Sign, vec![msg]));
					msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg, grad]));
					gradients.insert(op.parents[1], msg);
				}
			},
			OperatorType::Binary(BinaryOperatorType::Min) => {
				if try!(self.is_dependable(op.parents[0])){
					let mut msg = try!(self.add_operation(
						UnaryOperatorType::Neg,vec![op.parents[0]]));
					msg = try!(self.add_operation(NaryOperatorType::Add,vec![op.parents[1],msg]));
					msg = try!(self.add_operation(ConstantUnaryOperatorType::Sign, vec![msg]));
					msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg, grad]));
					gradients.insert(op.parents[0], msg);
				}
				if try!(self.is_dependable(op.parents[1])){
					let mut msg = try!(self.add_operation(
						UnaryOperatorType::Neg,vec![op.parents[1]]));
					msg = try!(self.add_operation(NaryOperatorType::Add,vec![op.parents[0],msg]));
					msg = try!(self.add_operation(ConstantUnaryOperatorType::Sign, vec![msg]));
					msg = try!(self.add_operation(NaryOperatorType::Mul,vec![msg, grad]));
					gradients.insert(op.parents[1], msg);
				}
			},
			OperatorType::Binary(BinaryOperatorType::Pow) => {
				if try!(self.is_dependable(op.parents[0])){
					let mut msg = try!(self.add_operation(
						UnaryOperatorType::Div,vec![op.parents[0]]));
					msg = try!(self.add_operation(
						NaryOperatorType::Mul,vec![op.parents[1], child, msg, grad]));
					gradients.insert(op.parents[0], msg);
				}
				if try!(self.is_dependable(op.parents[1])){
					let mut msg = try!(self.add_operation(
						UnaryOperatorType::Log,vec![op.parents[0]]));
					msg = try!(self.add_operation(NaryOperatorType::Mul,vec![child, msg, grad]));
					gradients.insert(op.parents[1], msg);
				}
			},
			OperatorType::Binary(BinaryOperatorType::Quadratic) => {
				if try!(self.is_dependable(op.parents[0])){
					let ptr = try!(self.add_operation(
						UnaryOperatorType::Transpose,vec![op.parents[1]]));
					let msg_1 = try!(self.add_operation(
						NaryOperatorType::Dot,vec![ptr, op.parents[0], grad]));
					let gradtr = try!(self.add_operation(
						UnaryOperatorType::Transpose,vec![grad]));
					let msg_2 = try!(self.add_operation(
						NaryOperatorType::Dot,vec![op.parents[1], op.parents[0], gradtr]));
					let msg = try!(self.add_operation(
						NaryOperatorType::Add,vec![msg_1, msg_2]));
					gradients.insert(op.parents[0], msg);
				}
				if try!(self.is_dependable(op.parents[1])){
					let ptr = try!(self.add_operation(
						UnaryOperatorType::Transpose,vec![op.parents[0]]));
						let msg = try!(self.add_operation(
							NaryOperatorType::Dot,vec![op.parents[0], grad, ptr]));
						gradients.insert(op.parents[1], msg);
				}
			},
			OperatorType::Nary(NaryOperatorType::Add) => {
				for i in op.parents.iter(){
					if try!(self.is_dependable(*i)){
						gradients.insert(*i, grad);
					}
				}
			},
			OperatorType::Nary(NaryOperatorType::Mul) => {
				match op.parents.len(){
					0...1 => return Err(::std::convert::From::from(
						InvalidOperatorError::new(OperatorType::Nary(NaryOperatorType::Mul), 2, 0, op.parents.len(),0))),
					2 => {
						let p1 = op.parents[0];
						let p2 = op.parents[1];
						if try!(self.is_dependable(p1)){
							let msg = try!(self.add_operation(
								NaryOperatorType::Mul,vec![p2, grad]));
							gradients.insert(p1, msg);
						}
						if try!(self.is_dependable(p2)){
							let msg = try!(self.add_operation(
								NaryOperatorType::Mul,vec![p1, grad]));
							gradients.insert(p2, msg);
						}
					},
					_ => {
						for i in op.parents.iter(){
							if try!(self.is_dependable(*i)){
								let mut msg = try!(self.add_operation(
									UnaryOperatorType::Div,vec![*i]));
								msg = try!(self.add_operation(
										NaryOperatorType::Mul,vec![msg, child, grad]));
								gradients.insert(*i, msg);
							}
						}
					}
				}
			},
			OperatorType::Nary(NaryOperatorType::Dot) => {
				// TODO
			},
			// Operator::Dot(ref parents) => {
			// 	match parents.len(){
			// 		0...1 => return Err("Multiplication with less than 2 parents".to_string()),
			// 		_ => {
			// 			// Left most parent
			// 			let p1 = parents[0];
			// 			if try!(self.is_dependable(p1)) {
			// 				let mut right_msg : usize;
			// 				if parents.len() == 2{
			// 					right_msg = try!(self.add_operation(Operator::Transpose(parents[1])));
			// 				}
			// 				else {
			// 					let right_parents = parents[1..].to_owned();
			// 					right_msg = try!(self.add_operation(Operator::Dot(right_parents)));
			// 					right_msg = try!(self.add_operation(Operator::Transpose(right_msg)));
			// 				}
			// 				let msg = try!(self.add_operation(Operator::Dot(vec![grad, right_msg])));
			// 				gradients.insert(p1, msg);
			// 			}
			// 			// Right most parent
			// 			let last = parents.len()-1;
			// 			let pend = parents[last];
			// 			if try!(self.is_dependable(pend)) {
			// 				let mut left_msg : usize;
			// 				if parents.len() == 2{
			// 					left_msg = try!(self.add_operation(Operator::Transpose(parents[0])));
			// 				}
			// 				else {
			// 					let left_parents = parents[..last].to_owned();
			// 					left_msg = try!(self.add_operation(Operator::Dot(left_parents)));
			// 					left_msg = try!(self.add_operation(Operator::Transpose(left_msg)));
			// 				}
			// 				let msg = try!(self.add_operation(Operator::Dot(vec![left_msg, grad])));
			// 				gradients.insert(pend, msg);
			// 			}
			// 			if parents.len() > 2 {
			// 				// Second from left to right
			// 				let p = parents[1];
			// 				if try!(self.is_dependable(p)) {
			// 					let left_msg = try!(self.add_operation(Operator::Transpose(parents[0])));
			// 					let mut right_msg : usize;
			// 					if parents.len() == 3 {
			// 						right_msg = try!(self.add_operation(Operator::Transpose(parents[2])));
			// 					}
			// 					else {
			// 						let right_parents = parents[2..].to_owned();
			// 						right_msg = try!(self.add_operation(Operator::Dot(right_parents)));
			// 						right_msg = try!(self.add_operation(Operator::Transpose(right_msg)));
			// 					}
			// 					let msg = try!(self.add_operation(Operator::Dot(vec![left_msg, grad, right_msg])));
			// 					gradients.insert(p, msg);
			// 				}
			// 			}
			// 			if parents.len() > 3 {
			// 				// Second from right to left
			// 				let p = parents[last-1];
			// 				if try!(self.is_dependable(p)) {
			// 					let right_msg = try!(self.add_operation(Operator::Transpose(parents[last])));
			// 					let left_parents = parents[..last-1].to_owned();
			// 					let mut left_msg = try!(self.add_operation(Operator::Dot(left_parents)));
			// 					left_msg = try!(self.add_operation(Operator::Transpose(left_msg)));
			// 					let msg = try!(self.add_operation(Operator::Dot(vec![left_msg, grad, right_msg	])));
			// 					gradients.insert(p, msg);
			// 				}
			// 				// Rest
			// 				for i in 2..last-1{
			// 					let p = parents[i];
			// 					if try!(self.is_dependable(p)) {
			// 						let left_parents = parents[..last-1].to_owned();
			// 						let mut left_msg = try!(self.add_operation(Operator::Dot(left_parents)));
			// 						left_msg = try!(self.add_operation(Operator::Transpose(left_msg)));
			// 						let right_parents = parents[2..].to_owned();
			// 						let mut right_msg = try!(self.add_operation(Operator::Dot(right_parents)));
			// 						right_msg = try!(self.add_operation(Operator::Transpose(right_msg)));
			// 						let msg = try!(self.add_operation(Operator::Dot(vec![left_msg, grad, right_msg])));
			// 						gradients.insert(p, msg);
			// 					}
			// 				}
			// 			}
			// 		}
			// 	}
			// },
			OperatorType::Nary(NaryOperatorType::HorzCat) => {
				// TODO
			},
			// Operator::HorzCat(ref parents) => {
			// 	match parents.len(){
			// 		0...1 => return Err("Multiplication with less than 2 parents".to_string()),
			// 		_ => {
			// 			let mut last : usize = parents.len() + 1;
			// 			for i in (0..parents.len()).rev(){
			// 				if try!(self.is_dependable(parents[i])) {
			// 					last = i;
			// 					break;
			// 				}
			// 			}
			// 			if last < parents.len(){
			// 				let const_0 = self.add_int(0);
			// 				let size_x =  try!(self.add_operation(Operator::Size(child, Dimension::First)));
			// 				let mut accum = self.add_int(0);
			// 				for i in 0..last+1{
			// 					let p = parents[i];
			// 					let size_y = try!(self.add_operation(Operator::Size(p, Dimension::Second)));
			// 					if try!(self.is_dependable(p)) {
			// 						let start_y = accum;
			// 						let msg = try!(self.add_operation(Operator::SubIndex(grad, const_0, size_x, start_y, size_y)));
			// 						gradients.insert(p, msg);
			// 					}
			// 					if i < last{
			// 						accum = try!(self.add_operation(Operator::Add(vec![accum, size_y])));
			// 					}
			// 				}
			// 			}
			// 		}
			// 	}
			// },
			OperatorType::Nary(NaryOperatorType::VertCat) => {
				// TODO
			},
			// Operator::VertCat(ref parents) => {
			// 	match parents.len(){
			// 		0...1 => return Err("Multiplication with less than 2 parents".to_string()),
			// 		_ => {
			// 			let mut last : usize = parents.len() + 1;
			// 			for i in (0..parents.len()).rev(){
			// 				if try!(self.is_dependable(parents[i])) {
			// 					last = i;
			// 					break;
			// 				}
			// 			}
			// 			if last < parents.len(){
			// 				let const_0 = self.add_int(0);
			// 				let size_y =  try!(self.add_operation(Operator::Size(child, Dimension::Second)));
			// 				let mut accum = self.add_int(0);
			// 				for i in 0..last+1{
			// 					let p = parents[i];
			// 					let size_x = try!(self.add_operation(Operator::Size(p, Dimension::First)));
			// 					if try!(self.is_dependable(p)) {
			// 						let start_x = accum;
			// 						let msg = try!(self.add_operation(Operator::SubIndex(grad, start_x, size_x, const_0, size_y)));
			// 						gradients.insert(p, msg);
			// 					}
			// 					if i < last{
			// 						accum = try!(self.add_operation(Operator::Add(vec![accum, size_x])));
			// 					}
			// 				}
			// 			}
			// 		}
			// 	}
			// },
			OperatorType::Special(SpecialUnaryOperatorType::SubIndex) => {
				let mut new_parents = vec![grad];
				new_parents.extend(op.parents.iter().skip(1).cloned());
				let msg = try!(self.add_operation(
					SpecialUnaryOperatorType::SubAssign,new_parents));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Special(SpecialUnaryOperatorType::SubAssign) => {
				let mut new_parents = vec![grad];
				new_parents.extend(op.parents.iter().skip(1).cloned());
				let msg = try!(self.add_operation(
					SpecialUnaryOperatorType::SubIndex,new_parents));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Special(SpecialUnaryOperatorType::Reshape) => {
				let rows = try!(self.add_operation(
					ConstantUnaryOperatorType::Size(Dimension::First), vec![op.parents[0]]));
				let cols = try!(self.add_operation(
					ConstantUnaryOperatorType::Size(Dimension::Second), vec![op.parents[0]]));
				let msg = try!(self.add_operation(
					SpecialUnaryOperatorType::Reshape,vec![grad, rows, cols]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Special(SpecialUnaryOperatorType::ReplicateHorz) => {
				let msg = try!(self.add_operation(
					UnaryOperatorType::Sum(Dimension::Second),vec![grad]));
				gradients.insert(op.parents[0], msg);
			},
			OperatorType::Special(SpecialUnaryOperatorType::ReplicateVert) => {
				let msg = try!(self.add_operation(
					UnaryOperatorType::Sum(Dimension::First),vec![grad]));
				gradients.insert(op.parents[0], msg);
			}
		}
		Ok(gradients)
	}

	#[inline(always)]
	pub fn get_mut_node(&mut self, index: usize) -> Result<&mut ComputeNode, GraphError>{
		let l = self.nodes.len();
		try!(self.nodes.get_mut(index).ok_or_else(
			|| GraphError::IndexOutOfBounds(index,l)))
			.as_mut().ok_or_else(
				|| GraphError::AccessNoneNode(index))
	}

	#[inline(always)]
	pub fn get_node(&mut self, index: usize) -> Result<& ComputeNode, GraphError>{
		try!(self.nodes.get(index).ok_or_else(
			|| GraphError::IndexOutOfBounds(index,self.nodes.len())))
			.as_ref().ok_or_else(
				|| GraphError::AccessNoneNode(index))
	}

	#[inline(always)]
	pub fn pop_node(&mut self, index: usize) -> Result<ComputeNode, GraphError> {
		self.nodes.push(None);
		self.nodes.swap_remove(index).ok_or_else(|| GraphError::AccessNoneNode(index))
	}

	#[inline(always)]
	pub fn insert_node(&mut self, index: usize, node: Option<ComputeNode>) -> Option<ComputeNode> {
		self.nodes.push(node);
		self.nodes.swap_remove(index)
	}

	#[inline(always)]
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
			"const" => Ok(try!(self.add_operation(ConstantUnaryOperatorType::Const, args))),
			"eye" => Ok(try!(self.add_operation(ConstantUnaryOperatorType::Eye, args))),
			"sign" => Ok(try!(self.add_operation(ConstantUnaryOperatorType::Sign, args))),
			"rows" => Ok(try!(self.add_operation(ConstantUnaryOperatorType::Size(Dimension::First), args))),
			"cols" => Ok(try!(self.add_operation(ConstantUnaryOperatorType::Size(Dimension::Second), args))),
			"ones" => Ok(try!(self.add_operation(ConstantBinaryOperatorType::Ones, args))),
			"zeros"  => Ok(try!(self.add_operation(ConstantBinaryOperatorType::Zeros, args))),
			"lt" => Ok(try!(self.add_operation(ConstantBinaryOperatorType::LessThan, args))),
			"lte" => Ok(try!(self.add_operation(ConstantBinaryOperatorType::LessThanOrEqual, args))),
			"gt" => Ok(try!(self.add_operation(ConstantBinaryOperatorType::GreaterThan, args))),
			"gte" => Ok(try!(self.add_operation(ConstantBinaryOperatorType::GreaterThanOrEqual, args))),
			"eq" => Ok(try!(self.add_operation(ConstantBinaryOperatorType::Equals, args))),
			"neq" => Ok(try!(self.add_operation(ConstantBinaryOperatorType::NotEquals, args))),
			"neg" => Ok(try!(self.add_operation(UnaryOperatorType::Neg, args))),
			"div" => Ok(try!(self.add_operation(UnaryOperatorType::Div, args))),
			"minv" => Ok(try!(self.add_operation(UnaryOperatorType::MatrixInverse, args))),
			"tr" => Ok(try!(self.add_operation(UnaryOperatorType::Transpose, args))),
			"mdiag" => Ok(try!(self.add_operation(UnaryOperatorType::MatrixDiag, args))),
			"vdiag" => Ok(try!(self.add_operation(UnaryOperatorType::VectorDiag, args))),
			"cos" => Ok(try!(self.add_operation(UnaryOperatorType::Cos, args))),
			"sin" => Ok(try!(self.add_operation(UnaryOperatorType::Sin, args))),
			"tan" => Ok(try!(self.add_operation(UnaryOperatorType::Tan, args))),
			"cosh" => Ok(try!(self.add_operation(UnaryOperatorType::CosH, args))),
			"sinh" => Ok(try!(self.add_operation(UnaryOperatorType::SinH, args))),
			"tanh" => Ok(try!(self.add_operation(UnaryOperatorType::TanH, args))),
			"abs" => Ok(try!(self.add_operation(UnaryOperatorType::Abs, args))),
			"log" => Ok(try!(self.add_operation(UnaryOperatorType::Log, args))),
			"exp" => Ok(try!(self.add_operation(UnaryOperatorType::Exp, args))),
			"sqrt" => Ok(try!(self.add_operation(UnaryOperatorType::Sqrt, args))),
			"square" => Ok(try!(self.add_operation(UnaryOperatorType::Square, args))),
			"sigm" => Ok(try!(self.add_operation(UnaryOperatorType::Sigmoid, args))),
			"rect" => Ok(try!(self.add_operation(UnaryOperatorType::Rectifier, args))),
			"sum" => match args.len(){
				2 => match try!(self.get_node(args[1])).node_type {
					Type::Integer(x) => {
						match x {
							0 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									UnaryOperatorType::Sum(Dimension::All), vec![args[0]]));
								Ok(result)
							},
							1 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									UnaryOperatorType::Sum(Dimension::First), vec![args[0]]));
								Ok(result)
							},
							2 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									UnaryOperatorType::Sum(Dimension::Second), vec![args[0]]));
								Ok(result)
							},
							_ => return Err(::std::convert::From::from(
								 InvalidOperatorError::new(::std::convert::From::from(
									UnaryOperatorType::Sum(Dimension::All)), 1, 0, args.len(), 1)))
						}
					},
					_ => return Err(::std::convert::From::from(
						InvalidOperatorError::new(::std::convert::From::from(
							UnaryOperatorType::Sum(Dimension::All)), 1, 0, args.len()-1, 0)))
				},
				_ => return Err(::std::convert::From::from(
					InvalidOperatorError::new(::std::convert::From::from(
						UnaryOperatorType::Sum(Dimension::All)), 1, 0, args.len()-1, 0)))
			},
			"l2" => match args.len(){
				2 => match try!(self.get_node(args[1])).node_type {
					Type::Integer(x) => {
						match x {
							0 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									UnaryOperatorType::L2(Dimension::All), vec![args[0]]));
								Ok(result)
							},
							1 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									UnaryOperatorType::L2(Dimension::First), vec![args[0]]));
								Ok(result)
							},
							2 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									UnaryOperatorType::L2(Dimension::Second), vec![args[0]]));
								Ok(result)
							},
							_ => return Err(::std::convert::From::from(
								InvalidOperatorError::new(::std::convert::From::from(
									UnaryOperatorType::L2(Dimension::All)), 1, 0, args.len(), 1)))
						}
					},
					_ => return Err(::std::convert::From::from(
						InvalidOperatorError::new(::std::convert::From::from(
							UnaryOperatorType::L2(Dimension::All)), 1, 0, args.len()-1, 0)))
				},
				_ => return Err(::std::convert::From::from(
					InvalidOperatorError::new(::std::convert::From::from(
						UnaryOperatorType::L2(Dimension::All)), 1, 0, args.len()-1, 0)))
			},
			"l1" => match args.len(){
				2 => match try!(self.get_node(args[1])).node_type {
					Type::Integer(x) => {
						match x {
							0 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									UnaryOperatorType::L1(Dimension::All), vec![args[0]]));
								Ok(result)
							},
							1 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									UnaryOperatorType::L1(Dimension::First), vec![args[0]]));
								Ok(result)
							},
							2 => {
								try!(self.remove_last());
								let result = try!(self.add_operation(
									UnaryOperatorType::L1(Dimension::Second), vec![args[0]]));
								Ok(result)
							},
							_ => return Err(::std::convert::From::from(
								InvalidOperatorError::new(::std::convert::From::from(
									UnaryOperatorType::L1(Dimension::All)), 1, 0, args.len(), 1)))
						}
					},
					_ => return Err(::std::convert::From::from(
						InvalidOperatorError::new(::std::convert::From::from(
							UnaryOperatorType::L1(Dimension::All)), 1, 0, args.len()-1, 0)))
				},
				_ => return Err(::std::convert::From::from(
					InvalidOperatorError::new(::std::convert::From::from(
						UnaryOperatorType::L1(Dimension::All)), 1, 0, args.len()-1, 0)))
			},
			"max" => Ok(try!(self.add_operation(BinaryOperatorType::Max, args))),
			"min" => Ok(try!(self.add_operation(BinaryOperatorType::Min, args))),
			"pow" => Ok(try!(self.add_operation(BinaryOperatorType::Pow, args))),
			"quad" => Ok(try!(self.add_operation(BinaryOperatorType::Quadratic, args))),
			"subind" => Ok(try!(self.add_operation(SpecialUnaryOperatorType::SubIndex, args))),
			"subasign" => Ok(try!(self.add_operation(SpecialUnaryOperatorType::SubAssign, args))),
			"reshape" => Ok(try!(self.add_operation(SpecialUnaryOperatorType::Reshape, args))),
			"replicate" => match args.len() {
					_ => Ok(1)
			},
			"add" => Ok(try!(self.add_operation(NaryOperatorType::Add, args))),
			"mul" => Ok(try!(self.add_operation(NaryOperatorType::Mul, args))),
			"dot" => Ok(try!(self.add_operation(NaryOperatorType::Dot, args))),
			"horzcat" => Ok(try!(self.add_operation(NaryOperatorType::HorzCat, args))),
			"vertcat" => Ok(try!(self.add_operation(NaryOperatorType::VertCat, args))),
			// "sum" => match args.len() {
			// 	2 => {
			// 		let val : i64;
			// 		let ch;
			// 		match self.nodes[args[1]] {
			// 			Some(ComputeNode{node_type: Type::Integer(x), id:_, name:_, ref children , op:_
			// 				, grad_level: _, inline:_, grad_child: _, grad_parents: _}) if x == 0 || x == 1 || x == 2 => {
			// 				val = x;
			// 				ch = children.len();
			// 			}
			// 			_ => return Err("The second argument for Sum is missing from the graph or is not 0,1 or 2.".to_string())
			// 		}
			// 		if self.counter - 1 == args[1] && ch == 0 {
			// 			println!("1");
			// 			self.remove_last();
			// 		}
			// 		match val  {
			// 			0 => self.add_operation(Operator::Sum(args[0], Dimension::All)),
			// 			1 => self.add_operation(Operator::Sum(args[0], Dimension::First)),
			// 			2 => self.add_operation(Operator::Sum(args[0], Dimension::Second)),
			// 			_ => Err("Sum takes as an argument only 0,1 or 2".to_string())
			// 		}
			// 	},
			// 	_ => Err("Sum takes exactly two arguments".to_string())
			// },
			// "l2" => match args.len() {
			// 	2 => {
			// 		let val : i64;
			// 		let ch;
			// 		match self.nodes[args[1]] {
			// 			Some(ComputeNode{node_type: Type::Integer(x), id:_, name:_, ref children , op:_
			// 				, grad_level: _, inline:_, grad_child: _, grad_parents: _}) if x == 0 || x == 1 || x == 2 => {
			// 				val = x;
			// 				ch = children.len();
			// 			}
			// 			_ => return Err("The second argument for L2 is missing from the graph or is not 0,1 or 2.".to_string())
			// 		}
			// 		if self.counter - 1 == args[1] && ch == 0 {
			// 			println!("2");
			// 			self.remove_last();
			// 		}
			// 		match val  {
			// 			0 => self.add_operation(Operator::L2(args[0], Dimension::All)),
			// 			1 => self.add_operation(Operator::L2(args[0], Dimension::First)),
			// 			2 => self.add_operation(Operator::L2(args[0], Dimension::Second)),
			// 			_ => Err("L2 takes as an argument only 0,1 or 2".to_string())
			// 		}
			// 	},
			// 	_ => Err("L2 takes exactly two arguments".to_string())
			// },
			// "l1" => match args.len() {
			// 	2 => {
			// 		let val : i64;
			// 		let ch;
			// 		match self.nodes[args[1]] {
			// 			Some(ComputeNode{node_type: Type::Integer(x), id:_, name:_, ref children , op:_
			// 				, grad_level: _, inline:_, grad_child: _, grad_parents: _}) if x == 0 || x == 1 || x == 2 => {
			// 				val = x;
			// 				ch = children.len();
			// 			}
			// 			_ => return Err("The second argument for L1 is missing from the graph or is not 0,1 or 2.".to_string())
			// 		}
			// 		if self.counter - 1 == args[1] && ch == 0 {
			// 			println!("3");
			// 			self.remove_last();
			// 		}
			// 		match val  {
			// 			0 => self.add_operation(Operator::L1(args[0], Dimension::All)),
			// 			1 => self.add_operation(Operator::L1(args[0], Dimension::First)),
			// 			2 => self.add_operation(Operator::L1(args[0], Dimension::Second)),
			// 			_ => Err("L1 takes as an argument only 0,1 or 2".to_string())
			// 		}
			// 	}
			// 	_ => Err("L1 takes exactly two arguments".to_string())
			// },
			// "dot" => match args.len() {
			// 	0 ... 1 => Err("Dot takes at least two arguments".to_string()),
			// 	_ => self.add_operation(Operator::Dot(args.clone())),
			// },
			// "horzcat" => match args.len() {
			// 	0 ... 1 => Err("HorzCat takes at least two arguments".to_string()),
			// 	_ => self.add_operation(Operator::HorzCat(args.clone())),
			// },
			// "vertcat" => match args.len() {
			// 	0 ... 1 => Err("VertCat takes at least two arguments".to_string()),
			// 	_ => self.add_operation(Operator::VertCat(args.clone())),
			// },
			// "reshape" => match args.len() {
			// 	3 => self.add_operation(Operator::Reshape(args[0], args[1], args[2])),
			// 	_ => Err("Reshape takes exactly three arguments".to_string())
			// },
			// "replicateH" => match args.len() {
			// 	2 => self.add_operation(Operator::ReplicateHorz(args[0], args[1])),
			// 	_ => Err("ReplicateHorz takes exactly two arguments".to_string())
			// },
			// "replicateV" => match args.len() {
			// 	2 => self.add_operation(Operator::ReplicateVert(args[0], args[1])),
			// 	_ => Err("ReplicateHorz takes exactly two arguments".to_string())
			// },
			_ => Err(GraphError::UnknownFunction(name.clone()))
		}
	}
	pub fn is_function_name(name: &str) -> bool{
		match name{
			"const" | "eye" | "sign" | "rows" | "cols" | "ones" | "zeros"
			| "minv" | "mdiag" | "vdiag" | "cos" | "sin" | "tan" | "cosh" | "sinh" | "tanh"
			| "abs"| "log" | "exp" | "sqrt" | "square" | "sigm" | "rect" | "sum" | "l2" | "l1"
			| "max" | "min" | "pow" | "quad" | "reshape" | "replicate"| "horzcat" | "vertcat" |
			"dot" => true,
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

impl ::std::convert::From<InvalidOperatorError> for GraphError {
    fn from(err: InvalidOperatorError) -> GraphError {
        GraphError::Operator(::std::convert::From::from(err))
    }
}

impl ::std::convert::From<NotFoundError> for GraphError {
    fn from(err: NotFoundError) -> GraphError {
        GraphError::Operator(::std::convert::From::from(err))
    }
}
