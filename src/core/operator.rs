/// Represents all possible dimensionality arguments, used in the operators
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dimension{
	First,
	Second,
	All
}

impl ::std::convert::From<usize> for Dimension {
    fn from(err: usize) ->  Dimension {
		match err {
			1 => Dimension::First,
			2 => Dimension::Second,
			_ => Dimension::All
		}
    }
}

/// An enum for operators which take a single parent node and produce a constant
///
/// The operator should have a single parent and no argumetns
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstantUnaryOperatorType{
	/// Transforms the parent node to a constant one, regardless if whether it is dependable on any parameters or not
	Const,
	/// Creates a new identity matrix with dimensions given by the parent node
	Eye,
	/// Returns the size of the node along the selected dimension. Should never be used with `Dimension::All`!
	Size(Dimension),
	/// Represents elementwise `sign(x)`
	Sign,
}

/// An enum for operators which take two parent nodes and produce a constant.
///
/// The operator should have two parents and no arguments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstantBinaryOperatorType {
	/// Creates a new matrix of ones with dimensions given by its parents
	Ones,
	/// Creates a new matrix of zeros with dimensions given by its parents
	Zeros,
	/// Represents elemtwise `x < y`
	LessThan,
	/// Represents elemtwise `x <= y`
	LessThanOrEqual,
	/// Represents elemtwise `x > y`
	GreaterThan,
	/// Represents elemtwise `x >= y`
	GreaterThanOrEqual,
	/// Represents elementwise `x == y`
	Equals,
	/// Represents elementiwse `x != y`
	NotEquals
}

/// An enum for operators which  produce a constant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstantOperatorType {
	Unary(ConstantUnaryOperatorType),
	Binary(ConstantBinaryOperatorType),
	None
}

impl ::std::convert::From<ConstantUnaryOperatorType> for ConstantOperatorType {
    fn from(err: ConstantUnaryOperatorType) ->  ConstantOperatorType {
         ConstantOperatorType::Unary(err)
    }
}

impl ::std::convert::From<ConstantBinaryOperatorType> for ConstantOperatorType {
    fn from(err: ConstantBinaryOperatorType) ->  ConstantOperatorType {
         ConstantOperatorType::Binary(err)
    }
}

/// An enum for operators which take a single parent node.
///
/// The operator should have one parent and no arguments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnaryOperatorType {
	/// Represents elemtwise `-x`
	Neg,
	/// Represents elemntwise `x^-1`
	Div,
	/// Represents matrix inversion `M^-1`
	MatrixInverse,
	/// Represents matrix transpose `M^T`
	Transpose,
	/// Takes the diagonal of a matrix as a column vector
	MatrixDiag,
	/// Takes a vector to a matrix, whose diagonal is equal to that vector
	VectorDiag,
	/// Represents elementwise `cos(x)`
	Cos,
	/// Represents elementwise `sin(x)`
	Sin,
	/// Represents elementwise `tan(x)`
	Tan,
	/// Represents elementwise `cosh(x)`
	CosH,
	/// Represents elementwise `sinh(x)`
	SinH,
	/// Represents elementwise `tanh(x)`
	TanH,
	/// Represents elementwise `abs(x)`
	Abs,
	/// Represents elementwise `ln(x)`
	Log,
	/// Represents elementwise `e^x`
	Exp,
	/// Represents elementwise `sqrt(x)`
	Sqrt,
	/// Represents elementwise `x^2`
	Square,
	/// Represents elementwise `1 / (1 + exp(-x))`
	Sigmoid,
	/// Repersents elementwise `max(x,0)`
	Rectifier,
	/// Takes the sum of the elements along the given dimension.
	Sum(Dimension),
	/// Takes the L2 squared norm along the given dimension. This is defuned as `sum(x_i^2)`
	L2(Dimension),
	/// Takes the L1 norm along the given dimension. This is defined as `sum(abs(x_i))`
	L1(Dimension)
}

/// An enum for special operators which take a single parent node.
///
/// The operator should have one parent and several arguments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpecialUnaryOperatorType {
	/// Takes the sub block of the parent node described by the following 4 arguments in the following sense: (parent, start_x, sizeOfBlockX, start_y, sizeOfBlockY)
	SubIndex,
	/// Represents subassignment. This will produce a matrix for which the sub block described by the operator will be equal to the first argument. E.g. this means that the result is a matrix of zeros, whose subblock is equal to the parent.The arguments are presented in the form  - (parent, start_x, sizeOfBlockX, start_y, sizeOfBlockY)
	SubAssign,
	/// Represents reshaping the argument to a given size
	Reshape,
	/// Represents the replication of the parent node horizontally. It is assumed that it is a scalar or column vector.
	ReplicateHorz,
	/// Represents the replication of the parent node vertically. It is assumed that it is a scalar or row vector.
	ReplicateVert
}

impl SpecialUnaryOperatorType{
	/// Returns the number of required arguments for the operator
	pub fn required_num_of_args(&self) -> usize {
		match *self{
			SpecialUnaryOperatorType::SubIndex => 4,
			SpecialUnaryOperatorType::SubAssign => 4,
			SpecialUnaryOperatorType::Reshape => 2,
			SpecialUnaryOperatorType::ReplicateHorz | SpecialUnaryOperatorType::ReplicateVert=> 1,
		}
	}
}

/// An enum for operators which take a two parent nodes
///
/// The operator should have two parents and no arguments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOperatorType {
	/// Represents elementwise `max(x,y)`
	Max,
	/// Represents elementwise `min(x,y)`
	Min,
	/// Represents elemntwise power `x^y`
	Pow,
	/// Represents the matrix quadratic form `M_1^T M_2 M_1'
	Quadratic
}

/// An enum for operators which are applied to several parent nodes
///
/// The operator should have at least two parents and no arguments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NaryOperatorType {
	/// Represents `a + b + ... + z`
	Add,
	/// Represents elementwise `a * b * ... * z`
	Mul,
	/// Represents the linear algebra matrix multiplication `M_a M_b ... M_z`
	Dot,
	/// Concatenates horizontally all of its arguments
	HorzCat,
	/// Concatenates vertically all of its arguments
	VertCat
}

/// An enum that represents all supported mathematical operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OperatorType {
	Constant(ConstantOperatorType),
	Unary(UnaryOperatorType),
	Special(SpecialUnaryOperatorType),
	Binary(BinaryOperatorType),
	Nary(NaryOperatorType)
}

impl ::std::convert::From<ConstantOperatorType> for OperatorType {
    fn from(op: ConstantOperatorType) ->  OperatorType {
         OperatorType::Constant(op)
    }
}

impl ::std::convert::From<ConstantUnaryOperatorType> for OperatorType {
    fn from(op: ConstantUnaryOperatorType) ->  OperatorType {
         OperatorType::Constant(ConstantOperatorType::Unary(op))
    }
}

impl ::std::convert::From<ConstantBinaryOperatorType> for OperatorType {
    fn from(op: ConstantBinaryOperatorType) ->  OperatorType {
         OperatorType::Constant(ConstantOperatorType::Binary(op))
    }
}

impl ::std::convert::From<UnaryOperatorType> for OperatorType {
    fn from(op: UnaryOperatorType) ->  OperatorType {
         OperatorType::Unary(op)
    }
}

impl ::std::convert::From<SpecialUnaryOperatorType> for OperatorType {
    fn from(op: SpecialUnaryOperatorType) ->  OperatorType {
         OperatorType::Special(op)
    }
}

impl ::std::convert::From<BinaryOperatorType> for OperatorType {
    fn from(err: BinaryOperatorType) ->  OperatorType {
         OperatorType::Binary(err)
    }
}

impl ::std::convert::From<NaryOperatorType> for OperatorType {
    fn from(err: NaryOperatorType) ->  OperatorType {
         OperatorType::Nary(err)
    }
}

// impl OperatorType {
// 	pub fn from_string(name: &str) -> OperatorType {
// 		match name {
// 			"const" => OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Const)),
// 			"eye" => OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Eye)),
// 			"sign" => OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Sign)),
// 			"rows" => OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Size(Dimension::First))),
// 			"cols" => OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Size(Dimension::Second))),
// 			"ones" => OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Ones)),
// 			"zeros" => OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Zeros)),
// 			"lt" => OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::LessThan)),
// 			"lte" => OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::LessThanOrEqual)),
// 			"gt" => OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::GreaterThan)),
// 			"gte" => OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::GreaterThanOrEqual)),
// 			"eq" => OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Equals)),
// 			"neq" => OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::NotEquals)),
// 			"neg" => OperatorType::Unary(UnaryOperatorType::Neg),
// 			"div" => OperatorType::Unary(UnaryOperatorType::Div),
// 			"minv" => OperatorType::Unary(UnaryOperatorType::MatrixInverse),
// 			"tr" => OperatorType::Unary(UnaryOperatorType::Transpose),
// 			"mdiag" => OperatorType::Unary(UnaryOperatorType::MatrixDiag),
// 			"vdiag" => OperatorType::Unary(UnaryOperatorType::VectorDiag),
// 			"cos" => OperatorType::Unary(UnaryOperatorType::Cos),
// 			"sin" => OperatorType::Unary(UnaryOperatorType::Sin),
// 			"tan" => OperatorType::Unary(UnaryOperatorType::Tan),
// 			"cosh" => OperatorType::Unary(UnaryOperatorType::CosH),
// 			"sinh" => OperatorType::Unary(UnaryOperatorType::SinH),
// 			"tanh" => OperatorType::Unary(UnaryOperatorType::TanH),
// 			"abs" => OperatorType::Unary(UnaryOperatorType::Abs),
// 			"log" => OperatorType::Unary(UnaryOperatorType::Log),
// 			"exp" => OperatorType::Unary(UnaryOperatorType::Exp),
// 			"sqrt" => OperatorType::Unary(UnaryOperatorType::Sqrt),
// 			"square" => OperatorType::Unary(UnaryOperatorType::Square),
// 			"sigm" => OperatorType::Unary(UnaryOperatorType::Sigmoid),
// 			"sum_0" => OperatorType::Unary(UnaryOperatorType::Sum(Dimension::All)),
// 			"sum_1" => OperatorType::Unary(UnaryOperatorType::Sum(Dimension::First)),
// 			"sum_2" => OperatorType::Unary(UnaryOperatorType::Sum(Dimension::Second)),
// 			"l2_0" => OperatorType::Unary(UnaryOperatorType::L2(Dimension::All)),
// 			"l2_1" => OperatorType::Unary(UnaryOperatorType::L2(Dimension::First)),
// 			"l2_2" => OperatorType::Unary(UnaryOperatorType::L2(Dimension::Second)),
// 			"l1_0" => OperatorType::Unary(UnaryOperatorType::L1(Dimension::All)),
// 			"l1_1" => OperatorType::Unary(UnaryOperatorType::L1(Dimension::First)),
// 			"l1_2" => OperatorType::Unary(UnaryOperatorType::L1(Dimension::Second)),
// 			_ => OperatorType::Constant(ConstantOperatorType::None)
// 		}
// 	}
// }


/// A struct that captures all supported mathematical operations with their metadata
#[derive(Clone, Debug)]
pub struct Operator {
		pub op_type: OperatorType,
		pub parents: Vec<usize>,
		pub args: Vec<usize>,
}

impl Operator {
	/// Creates a new `Operator` with the given arguments
	/// Returns an error if the given arguments or parents do not match the type of operator
	pub fn new(op_type: OperatorType, parents: Vec<usize>, args: Vec<usize> ) -> Result<Self,InvalidOperatorError> {
		if super::super::VERIFICATION {
			match op_type {
				OperatorType::Constant(ConstantOperatorType::None) => {
					if parents.len() != 0 || args.len() != 0 {
						return Err(InvalidOperatorError::new(op_type, 0, 0, parents.len(), args.len()))
					};
				},
				OperatorType::Constant(ConstantOperatorType::Unary(_)) | OperatorType::Unary(_) => {
					if parents.len() != 1 || args.len() != 0 {
						return Err(InvalidOperatorError::new(op_type, 1, 0, parents.len(), args.len()))
					};
				},
				OperatorType::Constant(ConstantOperatorType::Binary(_)) | OperatorType::Binary(_) => if parents.len() != 2 || args.len() != 0 {
						return Err(InvalidOperatorError::new(op_type, 2, 0, parents.len(), args.len()))
				},
				OperatorType::Nary(_) => if parents.len() < 2 || args.len() != 0 {
					return Err(InvalidOperatorError::new(op_type, ::std::usize::MAX, 0, parents.len(), args.len()))
				},
				OperatorType::Special(ref sp) =>  if parents.len() != 1 || args.len() != sp.required_num_of_args() {
					return Err(InvalidOperatorError::new(op_type, 1, 0, sp.required_num_of_args(), args.len()))
				}
			}
		}
		Ok(Operator{op_type: op_type, parents: parents, args: args})
	}

	pub fn get_ancestors(&self) -> ::std::iter::Chain<::std::slice::Iter<usize>, ::std::slice::Iter<usize>> {
		self.parents.iter().chain(self.args.iter())
	}

	/// Swaps the given parents of the operator
	/// Returns an error if `old_parent` was not in the parents' list
	pub fn swap_parent(&self, old_parent: usize, new_parent: usize) -> Result<Self, OperatorError> {
		let position = self.parents.iter().position(|&x| x == old_parent);
		match position {
			Some(_) => {
				let new_parents = self.parents.iter().map(
					|&x| if x == old_parent {new_parent} else {x}).collect::<Vec<usize>>();
				Ok(try!(Operator::new(self.op_type, new_parents, self.args.clone())))
			}
			None => try!(Err(NotFoundError::new(self.op_type, NotFoundErrorType::Parent, old_parent, self.parents.clone(), self.args.clone())))
		}
	}

	/// Swaps the given the parents of the operator in place
	/// Returns an error if `old_parent` was not in the parents' list
	pub fn swap_parent_in_place(&mut self, old_parent: usize, new_parent: usize) -> Result<(), NotFoundError> {
		let position = self.parents.iter().position(|&x| x == old_parent);
		match position {
			Some(p) => {
				self.parents.push(new_parent);
				let _  = self.parents.swap_remove(p);
				Ok(())
			}
			None => Err(NotFoundError::new(self.op_type, NotFoundErrorType::Parent, old_parent, self.parents.clone(), self.args.clone()))
		}
	}

	/// Swaps the given the arguments of the operator
	/// Returns an error if `old_arg` was not in the arguments' list
	pub fn swap_argument(&self, old_arg: usize, new_arg: usize) -> Result<Self, OperatorError>{
		let position = self.args.iter().position(|&x| x == old_arg);
		match position {
			Some(_) => {
				let new_args = self.args.iter().map(
					|&x| if x == old_arg {new_arg} else {x}).collect::<Vec<usize>>();
					Ok(try!(Operator::new(self.op_type, self.parents.clone(), new_args)))
				},
			None => try!(Err(NotFoundError::new(self.op_type, NotFoundErrorType::Argument, old_arg, self.parents.clone(), self.args.clone())))
		}
	}

	/// Swaps the given the arguments of the operator in place
	/// Returns an error if `old_arg` was not in the arguments' list
	pub fn swap_argument_in_place(&mut self, old_arg: usize, new_arg: usize) -> Result<(), NotFoundError> {
		let position = self.args.iter().position(|&x| x == old_arg);
		match position {
			Some(p) => {
				self.args.push(new_arg);
				let _  = self.args.swap_remove(p);
				Ok(())
			}
			None => Err(NotFoundError::new(self.op_type, NotFoundErrorType::Argument, old_arg, self.parents.clone(), self.args.clone()))
		}
	}

	/// Swaps the given the ancestors of the operator, in all occurences both in the parents' and arguments' list
	/// Returns an error if `old_anc` was not in the parents' or in the arguments' list
	pub fn swap_ancestor(&self, old_anc: usize, new_anc: usize) -> Result<Self, OperatorError> {
		let position_p = self.args.iter().position(|&x| x == old_anc);
		let position_a = self.args.iter().position(|&x| x == old_anc);
		match position_p {
			Some(_) => {
				let new_parents = self.parents.iter().map(
					|&x| if x == old_anc {new_anc} else {x}).collect::<Vec<usize>>();
				match position_a {
					Some(_) => {
						let new_args = self.args.iter().map(
							|&x| if x == old_anc {new_anc} else {x}).collect::<Vec<usize>>();
						Ok(try!(Operator::new(self.op_type, new_parents, new_args)))
					},
					None => Ok(try!(Operator::new(self.op_type, new_parents, self.args.clone())))
				}
			},
			None => match position_a {
				Some(_) => {
					let new_args = self.args.iter().map(
						|&x| if x == old_anc {new_anc} else {x}).collect::<Vec<usize>>();
					Ok(try!(Operator::new(self.op_type, self.parents.clone(), new_args)))
				},
				None => try!(Err(NotFoundError::new(self.op_type, NotFoundErrorType::Ancestor, old_anc, self.parents.clone(), self.args.clone())))
			}
		}
	}

	/// Swaps the given the ancestors of the operator in place, in all occurences both in the parents' and arguments' list
	/// Returns an error if `old_anc` was not in the parents' or in the arguments' list
	pub fn swap_ancestor_in_place(&mut self, old_anc: usize, new_anc: usize) -> Result<(), NotFoundError> {
		let result_p = self.swap_parent_in_place(old_anc, new_anc);
		let result_a = self.swap_argument_in_place(old_anc, new_anc);
		if result_p.is_ok() || result_a.is_ok() {
			Ok(())
		} else {
			Err(NotFoundError::new(self.op_type, NotFoundErrorType::Ancestor, old_anc, self.parents.clone(), self.args.clone()))
		}
	}

	/// Creates an operator of the same type, but with different parents and argumetns
	pub fn recreate(&self, parents: Vec<usize>, args: Vec<usize>) -> Result<Self, InvalidOperatorError> {
		// try!(self.verify_operator());
		Operator::new(self.op_type, parents, args)
		// try!(result.verify_operator());
		// Ok(result)
	}

	// /// Verifies the the number of parents and arguments in the coresponding list of the struct are correct based on the `OperatorType`.
	// /// It is switched on and off by the top level flag `VERIFICATION`.
	// #[inline(always)]
	// fn verify_operator(&self) -> Result<(),InvalidOperatorError>{
		// if super::super::VERIFICATION {
		// 	match self.op_type {
		// 		OperatorType::Constant(ConstantOperatorType::None) => {
		// 			if self.parents.len() != 0 || self.args.len() != 0 {
		// 				return Err(InvalidOperatorError::new(self.op_type, 0, 0, self.parents.len(), self.args.len()))
		// 			};
		// 		},
		// 		OperatorType::Constant(ConstantOperatorType::Unary(_)) | OperatorType::Unary(_) => {
		// 			if self.parents.len() != 1 || self.args.len() != 0 {
		// 				return Err(InvalidOperatorError::new(self.op_type, 1, 0, self.parents.len(), self.args.len()))
		// 			};
		// 		},
		// 		OperatorType::Constant(ConstantOperatorType::Binary(_)) | OperatorType::Binary(_) => if self.parents.len() != 2 || self.args.len() != 0 {
		// 				return Err(InvalidOperatorError::new(self.op_type, 2, 0, self.parents.len(), self.args.len()))
		// 		},
		// 		OperatorType::Nary(_) => if self.parents.len() < 2 || self.args.len() != 0 {
		// 			return Err(InvalidOperatorError::new(self.op_type, ::std::usize::MAX, 0, self.parents.len(), self.args.len()))
		// 		},
		// 		OperatorType::Special(ref sp) =>  if self.parents.len() != 1 || self.args.len() != sp.required_num_of_args() {
		// 			return Err(InvalidOperatorError::new(self.op_type, 1, 0, sp.required_num_of_args(), self.args.len()))
		// 		}
		// 	}
		// }
	// 	Ok(())
	// }
}

//
// impl SwapNode for ConstantBinaryOperator {
// 	fn swap_parent(&self, old_parent: usize, new_parent: usize)
// 		-> Result<Self, NotFoundError> {
// 		if self.parents[0] == old_parent {
// 			Ok(ConstantUnaryOperator{parents: [old_parent, self.parents[1]], op_type: self.op_type})
// 		} else if self.parents[1] == old_parent {
// 			Ok(ConstantUnaryOperator{parents: [self.parents[0], old_parent], op_type: self.op_type})
// 		}
// 		else {
// 			Err(NotFoundError::new(self.op_type, NotFoundErrorType::Parent,
// 				old_parent, vec![self.parents], vec![]))
// 		}
// 	}
// 	fn swap_parent_in_place(&mut self, old_parent: usize, new_parent: usize)
// 		-> Result<Self, NotFoundError> {
// 		if self.parents[0] == old_parent {
// 			self.parents[0] = new_parent;
// 		} else if self.parents[1] == old_parent {
// 			self.parents[1] = new_parent;
// 		} else {
// 			Err(NotFoundError::new(self.op_type, NotFoundErrorType::Parent,
// 				old_parent, vec![self.parent], vec![]))
// 		}
// 	}
//
// 	fn swap_argument(&self, old_parent: usize, new_parent: usize)
// 		-> Result<Self, NotFoundError> {
// 		Err(NotFoundError::new(self.op_type, NotFoundErrorType::Argument,
// 			old_parent, vec![self.parent], vec![]))
// 	}
//
// 	fn swap_argument_in_place(&mut self, old_parent: usize, new_parent: usize)
// 		-> Result<Self, NotFoundError> {
// 		Err(NotFoundError::new(self.op_type, NotFoundErrorType::Argument,
// 			old_parent, vec![self.parent], vec![]))
// 	}
//
// 	fn swap_ancestor(&self, old_parent: usize, new_parent: usize)
// 		-> Result<Self, NotFoundError> {
// 		self.swap_parent(old_parent, new_parent)
// 	}
//
// 	fn swap_ancestor_in_place(&self, old_parent: usize, new_parent: usize)
// 		-> Result<Self, NotFoundError> {
// 		self.swap_parent(old_parent, new_parent_in_place)
// 	}
// }


// impl Operator{
// 	pub fn correct_num_of_parents(&self, suggested: usize) -> bool {
// 		match *self {
// 			Self::Nary => suggested > 1,
// 			Self::Constant(Binary(_)) | Self::Binary(_) => suggested == 2,
// 			_ => suggested == 1
// 		}
// 	}
//
// 	pub fn correct_num_of_arguments(&self, suggested: usize) -> bool {
// 		match *self {
// 			Self::Special(ref s)  => suggested == s.required_num_of_args(),
// 			_ => suggested == 0
// 		}
// 	}
//
// 	pub fn correct_num_of_ancestors(&self, suggested: usize) -> bool {
// 		match *self {
// 			Self::Nary => suggested > 1,
// 			Self::Constant(Binary(_)) | Self::Binary(_) => suggested == 2,
// 			Self::Special(ref s)  => suggested == s.required_num_of_args() + 1,
// 			_ => suggested == 1
// 		}
// 	}
//
//
// 	/// Returns a new `Operator` having swapped the `old_parent` with the `new_parent`. If the `old_parent` is not in the parent(s) an error is returned.
// 	pub fn swap_parent(&self, old_parent: usize, new_parent: usize) -> Result<Self, String> {
//
//         match *self {
//             Operator::Const(p) => match p == old_parent{
//                 true => Ok(Operator::Const(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Eye(p) => match p == old_parent{
//                 true => Ok(Operator::Eye(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Size(p,dim) => match p == old_parent{
//                 true => Ok(Operator::Size(new_parent,dim)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Sign(p) => match p == old_parent{
//                 true => Ok(Operator::Sign(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Neg(p) => match p == old_parent{
//                 true => Ok(Operator::Neg(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Div(p) => match p == old_parent{
//                 true => Ok(Operator::Div(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::MatrixInverse(p) => match p == old_parent{
//                 true => Ok(Operator::MatrixInverse(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Transpose(p) => match p == old_parent{
//                 true => Ok(Operator::Transpose(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::MatrixDiag(p) => match p == old_parent{
//                 true => Ok(Operator::MatrixDiag(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::VectorDiag(p) => match p == old_parent{
//                 true => Ok(Operator::VectorDiag(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Cos(p) => match p == old_parent{
//                 true => Ok(Operator::Cos(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Sin(p) => match p == old_parent{
//                 true => Ok(Operator::Sin(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Tan(p) => match p == old_parent{
//                 true => Ok(Operator::Tan(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::CosH(p) => match p == old_parent{
//                 true => Ok(Operator::CosH(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::SinH(p) => match p == old_parent{
//                 true => Ok(Operator::SinH(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::TanH(p) => match p == old_parent{
//                 true => Ok(Operator::TanH(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Abs(p) => match p == old_parent{
//                 true => Ok(Operator::Abs(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Log(p) => match p == old_parent{
//                 true => Ok(Operator::Log(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Exp(p) => match p == old_parent{
//                 true => Ok(Operator::Exp(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Sqrt(p) => match p == old_parent{
//                 true => Ok(Operator::Sqrt(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Square(p) => match p == old_parent{
//                 true => Ok(Operator::Square(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Sigmoid(p) => match p == old_parent{
//                 true => Ok(Operator::Sigmoid(new_parent)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Sum(p,dim) => match p == old_parent{
//                 true => Ok(Operator::Sum(new_parent,dim)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::L2(p,dim) => match p == old_parent{
//                 true => Ok(Operator::L2(new_parent,dim)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::L1(p,dim) => match p == old_parent{
//                 true => Ok(Operator::L1(new_parent,dim)),
//                 false => Err("The old parent was not found in this operator!".to_string())
//             },
//             Operator::Ones(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::Ones(new_parent, p_2)),
//                     2 => Ok(Operator::Ones(p_1, new_parent)),
//                     3 => Ok(Operator::Ones(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::Zeros(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::Zeros(new_parent, p_2)),
//                     2 => Ok(Operator::Zeros(p_1, new_parent)),
//                     3 => Ok(Operator::Zeros(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::LessThan(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::LessThan(new_parent, p_2)),
//                     2 => Ok(Operator::LessThan(p_1, new_parent)),
//                     3 => Ok(Operator::LessThan(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::LessThanOrEqual(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::LessThanOrEqual(new_parent, p_2)),
//                     2 => Ok(Operator::LessThanOrEqual(p_1, new_parent)),
//                     3 => Ok(Operator::LessThanOrEqual(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::GreaterThan(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::GreaterThan(new_parent, p_2)),
//                     2 => Ok(Operator::GreaterThan(p_1, new_parent)),
//                     3 => Ok(Operator::GreaterThan(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::GreaterThanOrEqual(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::GreaterThanOrEqual(new_parent, p_2)),
//                     2 => Ok(Operator::GreaterThanOrEqual(p_1, new_parent)),
//                     3 => Ok(Operator::GreaterThanOrEqual(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::Max(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::Max(new_parent, p_2)),
//                     2 => Ok(Operator::Max(p_1, new_parent)),
//                     3 => Ok(Operator::Max(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::Min(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::Min(new_parent, p_2)),
//                     2 => Ok(Operator::Min(p_1, new_parent)),
//                     3 => Ok(Operator::Min(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::Pow(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::Pow(new_parent, p_2)),
//                     2 => Ok(Operator::Pow(p_1, new_parent)),
//                     3 => Ok(Operator::Pow(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::Quadratic(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::Quadratic(new_parent, p_2)),
//                     2 => Ok(Operator::Quadratic(p_1, new_parent)),
//                     3 => Ok(Operator::Quadratic(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::ReplicateHorz(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::ReplicateHorz(new_parent, p_2)),
//                     2 => Ok(Operator::ReplicateHorz(p_1, new_parent)),
//                     3 => Ok(Operator::ReplicateHorz(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::ReplicateVert(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
//                     0 => Err("The old parent was not found in this operator!".to_string()),
//                     1 => Ok(Operator::ReplicateVert(new_parent, p_2)),
//                     2 => Ok(Operator::ReplicateVert(p_1, new_parent)),
//                     3 => Ok(Operator::ReplicateVert(new_parent, new_parent)),
//                     _ => unreachable!()
//             },
//             Operator::Add(ref parents) => {
//                 let mut check = false;
//                 let res = parents.iter().map(|&x|
//                     if x == old_parent {check=true; new_parent} else {x})
//                 .collect::<Vec<usize>>();
//                 if check {
//                     Ok(Operator::Add(res))
//                 }
//                 else {
//                     Err("The old parent was not found in this operator!".to_string())
//                 }
//             },
//             Operator::Mul(ref parents) => {
//                 let mut check = false;
//                 let res = parents.iter().map(|&x|
//                     if x == old_parent {check=true; new_parent} else {x})
//                 .collect::<Vec<usize>>();
//                 if check {
//                     Ok(Operator::Mul(res))
//                 }
//                 else {
//                     Err("The old parent was not found in this operator!".to_string())
//                 }
//             },
//             Operator::Dot(ref parents) => {
//                 let mut check = false;
//                 let res = parents.iter().map(|&x|
//                     if x == old_parent {check=true; new_parent} else {x})
//                 .collect::<Vec<usize>>();
//                 if check {
//                     Ok(Operator::Dot(res))
//                 }
//                 else {
//                     Err("The old parent was not found in this operator!".to_string())
//                 }
//             },
//             Operator::HorzCat(ref parents) => {
//                 let mut check = false;
//                 let res = parents.iter().map(|&x|
//                     if x == old_parent {check=true; new_parent} else {x})
//                 .collect::<Vec<usize>>();
//                 if check {
//                     Ok(Operator::HorzCat(res))
//                 }
//                 else {
//                     Err("The old parent was not found in this operator!".to_string())
//                 }
//             },
//             Operator::VertCat(ref parents) => {
//                 let mut check = false;
//                 let res = parents.iter().map(|&x|
//                     if x == old_parent {check=true; new_parent} else {x})
//                 .collect::<Vec<usize>>();
//                 if check {
//                     Ok(Operator::VertCat(res))
//                 }
//                 else {
//                     Err("The old parent was not found in this operator!".to_string())
//                 }
//             },
//             Operator::SubIndex(p_1,p_2,p_3,p_4,p_5) => {
//                 let mut parents = vec![p_1, p_2, p_3, p_4, p_5];
//                 let mut check = false;
//                 parents = parents.iter().map(|&x|
//                     if x == old_parent {check=true; new_parent} else {x})
//                 .collect::<Vec<usize>>();
//                 if check {
//                     Ok(Operator::SubIndex(parents[0],parents[1],parents[2],parents[3],parents[4]))
//                 }
//                 else {
//                     Err("The old parent was not found in this operator!".to_string())
//                 }
//             },
//             Operator::SubAssign(p_1,p_2,p_3,p_4,p_5) => {
//                 let mut parents = vec![p_1, p_2, p_3, p_4, p_5];
//                 let mut check = false;
//                 parents = parents.iter().map(|&x|
//                     if x == old_parent {check=true; new_parent} else {x})
//                 .collect::<Vec<usize>>();
//                 if check {
//                     Ok(Operator::SubAssign(parents[0],parents[1],parents[2],parents[3],parents[4]))
//                 }
//                 else {
//                     Err("The old parent was not found in this operator!".to_string())
//                 }
//             },
//             Operator::Reshape(p_1,p_2,p_3) => {
//                 let mut parents = vec![p_1, p_2, p_3];
//                 let mut check = false;
//                 parents = parents.iter().map(|&x|
//                     if x == old_parent {check=true; new_parent} else {x})
//                 .collect::<Vec<usize>>();
//                 if check {
//                     Ok(Operator::Reshape(parents[0],parents[1],parents[2]))
//                 }
//                 else {
//                     Err("The old parent was not found in this operator!".to_string())
//                 }
//             }
//         }
// 	}
//
// 	/// Creates the same `Operator` type, but with the given parents, but with the same arguments.
// 	/// If the number of parents is wrong returns an error.
// 	pub fn recreate(&self, parents: Vec<usize>) -> Result<Self, InvalidNumberOfArguments> {
// 		match *self{
// 			Operator::Const(_) => match parents.len() {
// 				1 => Ok(Operator::Const(parents[0])),
// 				x =>  InvalidNumberOfParents{"Const".to_string(), 1, x}
//             },
//             Operator::Eye(_) => match parents.len() {
// 				1 => Ok(Operator::Eye(parents[0])),
// 				x =>  InvalidNumberOfParents{"Eye".to_string(), 1, x}
//             },
//             Operator::Size(_,dim) => match parents.len() {
// 				1 => Ok(Operator::Size(parents[0],dim)),
// 				x =>  InvalidNumberOfParents{"Size".to_string(), 1, x}
//             },
//             Operator::Sign(_) => match parents.len() {
// 				1 => Ok(Operator::Sign(parents[0])),
// 				x =>  InvalidNumberOfParents{"Sign".to_string(), 1, x}
//             },
//             Operator::Neg(_) => match parents.len() {
// 				1 => Ok(Operator::Neg(parents[0])),
// 				x =>  InvalidNumberOfParents{"Neg".to_string(), 1, x}
//             },
//             Operator::Div(_) =>match parents.len() {
// 				1 => Ok(Operator::Div(parents[0])),
// 				x =>  InvalidNumberOfParents{"Div".to_string(), 1, x}
//             },
//             Operator::MatrixInverse(_) => match parents.len() {
// 				1 => Ok(Operator::MatrixInverse(parents[0])),
// 				x =>  InvalidNumberOfParents{"MatrixInverse".to_string(), 1, x}
//             },
//             Operator::Transpose(_) => match parents.len() {
// 				1 => Ok(Operator::Transpose(parents[0])),
// 				_ =>  Err("Transpose takes only 1 parent!".to_string())
//             },
//             Operator::MatrixDiag(_) => match parents.len() {
// 				1 => Ok(Operator::MatrixDiag(parents[0])),
// 				_ =>  Err("MatrixDiag takes only 1 parent!".to_string())
//             },
//             Operator::VectorDiag(_) => match parents.len() {
// 				1 => Ok(Operator::VectorDiag(parents[0])),
// 				_ =>  Err("VectorDiag takes only 1 parent!".to_string())
//             },
//             Operator::Cos(_) => match parents.len() {
// 				1 => Ok(Operator::Cos(parents[0])),
// 				_ =>  Err("Cos takes only 1 parent!".to_string())
//             },
//             Operator::Sin(_) => match parents.len() {
// 				1 => Ok(Operator::Sin(parents[0])),
// 				_ =>  Err("Sin takes only 1 parent!".to_string())
//             },
//             Operator::Tan(_) => match parents.len() {
// 				1 => Ok(Operator::Tan(parents[0])),
// 				_ =>  Err("Tan takes only 1 parent!".to_string())
//             },
//             Operator::CosH(_) => match parents.len() {
// 				1 => Ok(Operator::CosH(parents[0])),
// 				_ =>  Err("CosH takes only 1 parent!".to_string())
//             },
//             Operator::SinH(_) => match parents.len() {
// 				1 => Ok(Operator::SinH(parents[0])),
// 				_ =>  Err("SinH takes only 1 parent!".to_string())
//             },
//             Operator::TanH(_) => match parents.len() {
// 				1 => Ok(Operator::TanH(parents[0])),
// 				_ =>  Err("TanH takes only 1 parent!".to_string())
//             },
//             Operator::Abs(_) => match parents.len() {
// 				1 => Ok(Operator::Abs(parents[0])),
// 				_ =>  Err("Abs takes only 1 parent!".to_string())
//             },
//             Operator::Log(_) => match parents.len() {
// 				1 => Ok(Operator::Log(parents[0])),
// 				_ =>  Err("Log takes only 1 parent!".to_string())
//             },
//             Operator::Exp(_) => match parents.len() {
// 				1 => Ok(Operator::Exp(parents[0])),
// 				_ =>  Err("Exp takes only 1 parent!".to_string())
//             },
//             Operator::Sqrt(_) => match parents.len() {
// 				1 => Ok(Operator::Sqrt(parents[0])),
// 				_ =>  Err("Sqrt takes only 1 parent!".to_string())
//             },
//             Operator::Square(_) => match parents.len() {
// 				1 => Ok(Operator::Square(parents[0])),
// 				_ =>  Err("Square takes only 1 parent!".to_string())
//             },
//             Operator::Sigmoid(_) => match parents.len() {
// 				1 => Ok(Operator::Sigmoid(parents[0])),
// 				_ =>  Err("Sigmoid takes only 1 parent!".to_string())
//             },
//             Operator::Sum(_,dim) => match parents.len() {
// 				1 => Ok(Operator::Sum(parents[0],dim)),
// 				_ =>  Err("Sum takes only 1 parent!".to_string())
//             },
//             Operator::L2(_,dim) => match parents.len() {
// 				1 => Ok(Operator::L2(parents[0],dim)),
// 				_ =>  Err("L2 takes only 1 parent!".to_string())
//             },
//             Operator::L1(_,dim) => match parents.len() {
// 				1 => Ok(Operator::L1(parents[0],dim)),
// 				_ =>  Err("L1 takes only 1 parent!".to_string())
//             },
//             Operator::Ones(_,_) => match parents.len() {
// 				2 => Ok(Operator::Ones(parents[0],parents[1])),
// 				_ =>  Err("Ones takes only 2 parents!".to_string())
//             },
//             Operator::Zeros(_,_) => match parents.len() {
// 				2 => Ok(Operator::Zeros(parents[0],parents[1])),
// 				_ =>  Err("Zeros takes only 2 parents!".to_string())
//             },
//             Operator::LessThan(_,_) => match parents.len() {
// 				2 => Ok(Operator::LessThan(parents[0],parents[1])),
// 				_ =>  Err("LessThan takes only 2 parents!".to_string())
//             },
//             Operator::LessThanOrEqual(_,_) => match parents.len() {
// 				2 => Ok(Operator::LessThanOrEqual(parents[0],parents[1])),
// 				_ =>  Err("LessThanOrEqual takes only 2 parents!".to_string())
//             },
//             Operator::GreaterThan(_,_) => match parents.len() {
// 				2 => Ok(Operator::GreaterThan(parents[0],parents[1])),
// 				_ =>  Err("GreaterThan takes only 2 parents!".to_string())
//             },
//             Operator::GreaterThanOrEqual(_,_) => match parents.len() {
// 				2 => Ok(Operator::GreaterThanOrEqual(parents[0],parents[1])),
// 				_ =>  Err("GreaterThanOrEqual takes only 2 parents!".to_string())
//             },
//             Operator::Max(_,_) => match parents.len() {
// 				2 => Ok(Operator::Max(parents[0],parents[1])),
// 				_ =>  Err("Max takes only 2 parents!".to_string())
//             },
//             Operator::Min(_,_) =>  match parents.len() {
// 				2 => Ok(Operator::Min(parents[0],parents[1])),
// 				_ =>  Err("Min takes only 2 parents!".to_string())
//             },
//             Operator::Pow(_,_) =>  match parents.len() {
// 				2 => Ok(Operator::Pow(parents[0],parents[1])),
// 				_ =>  Err("Pow takes only 2 parents!".to_string())
//             },
//             Operator::Quadratic(_,_) =>  match parents.len() {
// 				2 => Ok(Operator::Quadratic(parents[0],parents[1])),
// 				_ =>  Err("Quadratic takes only 2 parents!".to_string())
//             },
//             Operator::ReplicateHorz(_,_) =>  match parents.len() {
// 				2 => Ok(Operator::ReplicateHorz(parents[0],parents[1])),
// 				_ =>  Err("ReplicateHorz takes only 2 parents!".to_string())
//             },
//             Operator::ReplicateVert(_,_) =>  match parents.len() {
// 				2 => Ok(Operator::ReplicateVert(parents[0],parents[1])),
// 				_ =>  Err("ReplicateVert takes only 2 parents!".to_string())
//             },
//             Operator::Add(_) =>  match parents.len() {
// 				0...1 => Err("Add takes at least 2 parents!".to_string()),
// 				_ =>  Ok(Operator::Add(parents))
//             },
//             Operator::Mul(_) =>  match parents.len() {
// 				0...1 => Err("Mul takes at least 2 parents!".to_string()),
// 				_ =>  Ok(Operator::Mul(parents))
//             },
//             Operator::Dot(_) =>  match parents.len() {
// 				0...1 => Err("Dot takes at least 2 parents!".to_string()),
// 				_ =>  Ok(Operator::Dot(parents))
//             },
//             Operator::HorzCat(_) =>  match parents.len() {
// 				0...1 => Err("HorzCat takes at least 2 parents!".to_string()),
// 				_ =>  Ok(Operator::HorzCat(parents))
//             },
//             Operator::VertCat(_) =>  match parents.len() {
// 				0...1 => Err("VertCat takes at least 2 parents!".to_string()),
// 				_ =>  Ok(Operator::VertCat(parents))
//             },
//             Operator::SubIndex(_,_,_,_,_) =>  match parents.len() {
// 				5 =>  Ok(Operator::SubIndex(parents[0],parents[1],parents[2],parents[3],parents[4])),
// 				_ =>  Err("SubIndex takes only 5 parents!".to_string())
//             },
//             Operator::SubAssign(_,_,_,_,_) =>  match parents.len() {
// 				5 =>  Ok(Operator::SubAssign(parents[0],parents[1],parents[2],parents[3],parents[4])),
// 				_ =>  Err("SubAssign takes only 5 parents!".to_string())
//             },
//             Operator::Reshape(_,_,_) => match parents.len() {
// 				3 =>  Ok(Operator::Reshape(parents[0],parents[1],parents[2])),
// 				_ =>  Err("SubAssign takes only 3 parents!".to_string())
//             },
// 		}
// 	}
//
// 	/// Returns a `Vec` of the ancestors of the `Operator`
// 	pub fn get_ancestors(&self) -> Vec<usize> {
// 		match *self {
// 			Operator::Const(ref p) | Operator::Eye(ref p) | Operator::Size(ref p,_) | Operator::Sign(ref p)
// 			| Operator::Neg(ref p)| Operator::Div(ref p)| Operator::MatrixInverse(ref p)
// 			| Operator::Transpose(ref p)| Operator::MatrixDiag(ref p) | Operator::VectorDiag(ref p)
// 			| Operator::Cos(ref p) | Operator::Sin(ref p)| Operator::Tan(ref p)
// 			| Operator::CosH(ref p)| Operator::SinH(ref p)| Operator::TanH(ref p)
// 			| Operator::Abs(ref p) | Operator::Log(ref p)| Operator::Exp(ref p)| Operator::Sqrt(ref p)
// 			| Operator::Square(ref p) | Operator::Sigmoid(ref p) | Operator::Sum(ref p,_)
// 			| Operator::L2(ref p,_)| Operator::L1(ref p,_) => vec![p.clone()],
// 			Operator::Ones(ref p1, ref p2) | Operator::Zeros(ref p1, ref p2)| Operator::LessThan(ref p1, ref p2)
// 			| Operator::LessThanOrEqual(ref p1, ref p2) | Operator::GreaterThan(ref p1, ref p2)
// 			| Operator::GreaterThanOrEqual(ref p1, ref p2)| Operator::Max(ref p1, ref p2)
// 			| Operator::Min(ref p1, ref p2) | Operator::Pow(ref p1, ref p2) | Operator::Quadratic(ref p1, ref p2)
// 			| Operator::ReplicateHorz(ref p1, ref p2) | Operator::ReplicateVert(ref p1, ref p2)
// 				=> vec![p1.clone(),p2.clone()],
// 			Operator::Add(ref p)| Operator::Mul(ref p) | Operator::Dot(ref p)
// 			| Operator::HorzCat(ref p) | Operator::VertCat(ref p) => p.clone(),
// 			Operator::SubIndex(ref p,ref a1,ref a2,ref a3,ref a4)
// 			| Operator::SubAssign(ref p,ref a1,ref a2,ref a3,ref a4)
// 				=> vec![p.clone(),a1.clone(),a2.clone(),a3.clone(),a4.clone()],
// 			Operator::Reshape(ref p,ref a1,ref a2) => vec![p.clone(),a1.clone(),a2.clone()]
// 		}
// 	}
//
// 	/// Returns true if the `Operator` is unary - check description of the enum
// 	pub fn unary(&self) -> bool {
// 		match *self {
//             Operator::Const(_) | Operator::Eye(_) | Operator::Size(_,_) | Operator::Sign(_)
// 			| Operator::Neg(_)| Operator::Div(_)| Operator::MatrixInverse(_)
// 			| Operator::Transpose(_)| Operator::MatrixDiag(_) | Operator::VectorDiag(_)
// 			| Operator::Cos(_) | Operator::Sin(_)| Operator::Tan(_)
// 			| Operator::CosH(_)| Operator::SinH(_)| Operator::TanH(_)
// 			| Operator::Abs(_) | Operator::Log(_)| Operator::Exp(_)| Operator::Sqrt(_)
// 			| Operator::Square(_) | Operator::Sigmoid(_) | Operator::Sum(_,_)
// 			| Operator::L2(_,_)| Operator::L1(_,_) => true,
// 			_ => false
// 		}
// 	}
//
// 	/// Returns true if the `Operator` is binary - check description of the enum
// 	pub fn binary(&self) -> bool {
// 		match *self{
// 			Operator::Ones(_,_) | Operator::Zeros(_,_)| Operator::LessThan(_,_)
// 			| Operator::LessThanOrEqual(_,_) | Operator::GreaterThan(_,_)
// 			| Operator::GreaterThanOrEqual(_,_)| Operator::Max(_,_)
// 			| Operator::Min(_,_) | Operator::Pow(_,_) | Operator::Quadratic(_,_)
// 			| Operator::ReplicateHorz(_,_) | Operator::ReplicateVert(_,_) => true,
// 			_ => false
// 		}
// 	}
//
// 	/// Returns true if the `Operator` is nary - check description of the enum
// 	pub fn nary(&self) -> bool {
// 		match *self{
// 			Operator::Add(_)| Operator::Mul(_) | Operator::Dot(_)
// 			| Operator::HorzCat(_) | Operator::VertCat(_)
// 			| Operator::SubIndex(_,_,_,_,_) | Operator::SubAssign(_,_,_,_,_)
// 			| Operator::Reshape(_,_,_) => true,
// 			_ => false
// 		}
// 	}
// }


/// Represents the possible NotFoundErrorType
#[derive(Clone, Copy, Debug)]
pub enum NotFoundErrorType{
	/// The node is a parent
	Parent,
	/// The node is an argument
	Argument,
	/// The node is either a parent or an argument
	Ancestor
}

#[derive(Clone, Debug)]
pub struct NotFoundError {
	operator_type: OperatorType,
	request_type: NotFoundErrorType,
	requested: usize,
	parents: Vec<usize>,
	args: Vec<usize>
}

impl NotFoundError{
	pub fn new(operator_type: OperatorType, request_type: NotFoundErrorType, requested: usize,
		parents: Vec<usize>, args: Vec<usize>) -> Self {
		NotFoundError{operator_type: operator_type, request_type: request_type,
		requested:requested, parents: parents, args:args}
	}
}

impl ::std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		write!(f, "A request in an operator {:?}, for an {:?} with id {} was made, but it was not found. Parents: {:?}, Arguments:{:?}",
			self.operator_type, self.request_type, self.requested, self.parents, self.args)
    }
}

impl ::std::error::Error for NotFoundError {
	fn description(&self) -> &str {
		"Requested ancestor was not found in this operator"
	}
}


#[derive(Clone, Debug)]
pub struct InvalidOperatorError {
	operator_type: OperatorType,
	expexcted_parents : usize,
	expected_args: usize,
	actual_parents: usize,
	actual_args: usize
}

impl InvalidOperatorError{
	#[inline(always)]
	pub fn new(operator_type: OperatorType, expexcted_parents : usize, 	expected_args: usize, actual_parents: usize, actual_args: usize) -> Self {
		InvalidOperatorError{operator_type: operator_type, expexcted_parents : expexcted_parents, expected_args: expected_args, actual_parents: actual_parents, actual_args: actual_args}
	}
}

impl ::std::fmt::Display for InvalidOperatorError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		write!(f, "The operator {:?} should take {} parents and {} arguments, but was found to have {} parents and {} arguments.",
			self.operator_type, self.expexcted_parents, self.expected_args, self.actual_parents, self.actual_args)
    }
}

impl ::std::error::Error for InvalidOperatorError {
	fn description(&self) -> &str {
		"The operator action is invalid."
	}
}

#[derive(Clone, Debug)]
pub enum OperatorError {
	NotFound(NotFoundError),
	Invalid(InvalidOperatorError)
}

impl ::std::fmt::Display for OperatorError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            OperatorError::NotFound(ref err) => write!(f, "NotFoundError: {}", err),
            OperatorError::Invalid(ref err) => write!(f, "InvalidOperatorError: {}", err),
        }
    }
}

impl ::std::error::Error for OperatorError {
    fn description(&self) -> &str {
        match *self {
			OperatorError::NotFound(ref err) => err.description(),
            OperatorError::Invalid(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
			OperatorError::NotFound(ref err) => Some(err),
            OperatorError::Invalid(ref err) => Some(err)
        }
    }
}

impl ::std::convert::From<NotFoundError> for OperatorError {
    fn from(err: NotFoundError) -> OperatorError {
        OperatorError::NotFound(err)
    }
}

impl ::std::convert::From<InvalidOperatorError> for OperatorError {
    fn from(err: InvalidOperatorError) -> OperatorError {
        OperatorError::Invalid(err)
    }
}

impl ::std::convert::From<InvalidOperatorError> for String{
	fn from(err: InvalidOperatorError) -> String {
        format!("{}", err)
    }
}

impl ::std::convert::From<NotFoundError> for String{
	fn from(err: NotFoundError) -> String {
        format!("{}", err)
    }
}
