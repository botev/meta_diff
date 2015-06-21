use std::fmt::{Display, Formatter, Error};

/// The `Dimension` type which represents any dimension up to the number of supported ones
#[derive(Copy, Clone, Debug)]
pub enum Dimension{
	First,
	Second,
	All
}

/// The `Operator` type of any node resulting from an algebric operation
#[derive(Clone, Debug)]
pub enum Operator {
	/// `Constant` operator that transforms a ParameterNode or ParameterDerivedNode to a ConstantDerivedNode.
	/// Other types of nodes remain unchanged
	Const(usize),
	/// `Constant` operator that creates a new matrix of ones with dimensions given by its arguments.
	/// Those need to be either a SymbolicNode or their dimensions should be (1,1)
	Ones(usize,usize),
	/// `Constant` operator that creates a new matrix of zeros with dimensions given by its arguments.
	/// Those need to be either a SymbolicNode or their dimensions should be (1,1)
	Zeros(usize,usize),
	/// `Constant` operator that creates a new identity matrix with dimensions given by its argument.
	/// That need to be either a SymbolicNode or their dimensions should be (1,1)
	Eye(usize),
	/// `Constant` operator that returns a new SymbolicNode representing the size of the argument among the given dimension.
	Size(usize, Dimension),
	/// `Constant` operator that represents elementwise sign
	Sign(usize),
	/// `Unary` operator representing negation operation : -x
	Neg(usize),
	/// `Unary` operator representing division operation : x^-1
	Div(usize),
	/// `Unary` operator representing matrix inversion : M^-1
	MatrixInverse(usize),
	/// `Unary` operator representing matrix transpose : M^T
	Transpose(usize),
	/// `Unary` operator taking the diagonal of a matrix as a column vector
	MatrixDiag(usize),
	/// `Unary` operator taking a vector to a matrix, whose diagonal is equal to that vector
	VectorDiag(usize),
	/// `Unary` operator representing elementwise cosine
	Cos(usize),
	/// `Unary` operator representing elementwise sine
	Sin(usize),
	/// `Unary` operator representing elementwise tangent
	Tan(usize),
	/// `Unary` operator representing elementwise hyperbolic cosine
	CosH(usize),
	/// `Unary` operator representing elementwise hyperbolic sine
	SinH(usize),
	/// `Unary` operator representing elementwise hyperbolic tangent
	TanH(usize),
	/// `Unary` operator representing elementwise absolute value
	Abs(usize),
	/// `Unary` operator representing elementwise natural logarithm
	Log(usize),
	/// `Unary` operator representing elementwise exponential
	Exp(usize),
	/// `Unary` operator representing elementwise square root
	Sqrt(usize),
	/// `Unary` operator representing elementwise square
	Square(usize),
	/// `Unary` operator representing elementwise rectifier : max(x,0)
	Rect(usize),
	/// `Unary` operator representing elementwise sigmoid : (1+exp(-x))^-1
	Sigmoid(usize),
	/// `Unary` operator that represents the broadcasting of the first argument
	/// along the dimension of the second argument with respect to the third.
	Broadcast(usize, Dimension, usize),
	/// `Unary` operator that takes the sum of the elements among the given dimension.
	/// If the dimension is none takes the sum of all elements.
	Sum(usize, Dimension),
	/// `Unary` operator that takes the L2 squared norm among the given dimension.
	/// If the dimension is none takes the L2 of all elements
	L2(usize, Dimension),
	/// `Unary` operator that takes the L2 squared norm among the given dimension.
	/// If the dimension is none takes the L1 of all elements
	L1(usize, Dimension),
	/// `Nary` operator that represents the summation of all arguments
	Add(Vec<usize>),
	/// `Nary` operator that represents the multiplication of all arguments elementwise
	Mul(Vec<usize>),
	/// `Nary` operator that represents the linear algebra multiplication of all arguments
	Dot(Vec<usize>),
	/// `Binary` operator that represents the first argument elementwise to the power of the second argument
	Pow(usize,usize),
	/// `Nary` operator that concatenates horizontally all of its arguments
	HorzCat(Vec<usize>),
	/// `Nary` operator that concatenates vertically all of its arguments
	VertCat(Vec<usize>),
	/// `Unary` operator that takes the sub block of the first argument described by the others in the sense
	/// (matrix, startX, sizeOfBlockX, startY, sizeOfBlockY)
	SubIndex(usize,usize,usize,usize,usize),
	/// `Unary` operator that takes the reshapes the first argument to a matrix of dimensions (2nd,3rd)
	Reshape(usize,usize,usize),
	/// `Unary` operator that replicates the first argument along the dimensions as many times as given by 2nd and 3r argument
	Replicate(usize,usize,usize),
}

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

/// The main data structure of the `ComputeGraph`
#[derive(Clone, Debug)]
pub struct ComputeNode{
	id: usize,
	node_type: Type,
	name: String,
	children: Vec<usize>,
	grad_level: u8,
	inline: bool, 
	// dims: Pair<SymPolynomial>,
	grad_child: Option<usize>,
	grad_parent: Option<usize>,
	op: Option<Operator>
}

impl Display for ComputeNode{
	fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
		write!(f, concat!("********{}[{}]********\n", "Type:{:?}\n"
			, "Operator: {:?}\n", "Children:{:?}")
		, self.name, self.id, self.node_type, self.op, self.children)
	}
}

impl ComputeNode{
	/// Creates a new empty `ComputeNode`, its name depends on the input type and gradient level
	fn new(id: usize, node_type: Type, grad_level: u8, op: Option<Operator>) -> Self{
		let name: &str;
		if grad_level > 0{
			name = "AutoGrad"; 
		}
		else {
			match node_type {
				Type::Float(_) => name = "Float",
				Type::Integer(_) => name = "Int",
				Type::ConstInput => name = "ConstIn",
				Type::Parameter => name = "Param",
				Type::ConstDerived => name = "ConstDer",
				Type::ParameterDerived => name = "ParamDer"
			}
		}
		ComputeNode{id: id, node_type: node_type, name: name.to_string(), children: Vec::new(), grad_level: grad_level, inline: false, grad_child: None, grad_parent: None, op:op}
	}
}

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
		match write!(f, concat!("============Graph {}============\n", "Number of nodes:{}\n"
			,"Target: {}\n", "Outputs:{:?}\n" , "============= Nodes =============\n")
		, self.name, self.counter, self.target, self.outputs) {
			Ok(_) => {},
			Err(msg) => {return Err(msg)}
		}
		match self.nodes[0] {
			Some(_) => {},
			None => {}
		}
		for node_opt in self.nodes.iter(){
			match *node_opt {
				Some(ref node) => { match write!(f,"{}\n",node){
					Ok(_) => {},
					Err(msg) => {return Err(msg)}
				};},
				None => {}
			}
		}
		write!(f, "============Graph End============")
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
			Operator::Ones(parent_id1, parent_id2) | Operator::Zeros(parent_id1, parent_id2) => {
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
			| Operator::Sqrt(parent_id) | Operator::Square(parent_id) | Operator::Rect(parent_id) 
			| Operator::Sigmoid(parent_id) | Operator::Sum(parent_id,_)  
			| Operator::L2(parent_id,_) | Operator::L1(parent_id,_) => {
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
			Operator::Broadcast(parent_id1,_, parent_id2) | Operator::Pow(parent_id1, parent_id2) => {
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
			Operator::SubIndex(parent_id, arg_id1, arg_id2, arg_id3, arg_id4) => {
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
			Operator::Reshape(parent_id, arg_id1, arg_id2) | Operator::Replicate(parent_id, arg_id1, arg_id2) => {
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
		}
		let node = ComputeNode::new(self.counter, node_type, self.grad_level, Some(op));
		self.nodes.push(Some(node));
		self.counter += 1;
		return Ok(self.counter-1);
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
				1 => self.add_operation(Operator::Rect(args[0])),
				_ => Err("Rect takes exactly one argument".to_string())
			},
			"sigm" => match args.len() {
				1 => self.add_operation(Operator::Sigmoid(args[0])),
				_ => Err("Sigmoid takes exactly one argument".to_string())
			},
			"br" => match args.len() {
				2 => {
					let val : i64;
					let ch;
					match self.nodes[args[1]] {
						Some(ComputeNode{node_type: Type::Integer(x), id:_, name:_, ref children , op:_
							, grad_level: _, inline:_, grad_child: _, grad_parent: _}) if x == 1 || x == 2 => {
							val = x;
							ch = children.len();
						}
						_ => return Err("The second argument for Broadcast is missing from the graph or is not 1 or 2.".to_string())
					}
					if self.counter - 1 == args[1] && ch == 0 {
						self.counter -= 1;
						self.nodes.remove(self.counter);
						println!("Removing integer variable {} with id {}", val, self.counter);
					}
					match val  {
						1 => self.add_operation(Operator::Broadcast(args[0], Dimension::First, 0)),
						2 => self.add_operation(Operator::Broadcast(args[0], Dimension::Second, 0)),
						_ => return Err("Broadcast takes as an argument only 1 or 2".to_string())
					}
				},			
				_ => return Err("Broadcast takes exactly two arguments".to_string())
			},
			"sum" => match args.len() {
				2 => {
					let val : i64;
					let ch;
					match self.nodes[args[1]] {
						Some(ComputeNode{node_type: Type::Integer(x), id:_, name:_, ref children , op:_
							, grad_level: _, inline:_, grad_child: _, grad_parent: _}) if x == 1 || x == 2 => {
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
							, grad_level: _, inline:_, grad_child: _, grad_parent: _}) if x == 1 || x == 2 => {
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
							, grad_level: _, inline:_, grad_child: _, grad_parent: _}) if x == 1 || x == 2 => {
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
			"replicate" => match args.len() {
				3 => self.add_operation(Operator::Replicate(args[0], args[1], args[2])),
				_ => Err("Replicate takes exactly three arguments".to_string())
			},
			_ => Err("Unrecognised function name".to_string())
		}
	}
}
