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
	/// Represents elementwise sign(x)
	Sign,
	/// Returns the size of the node along the selected dimension. Should never be used with `Dimension::All`!
	Size(Dimension)
}

/// An enum for operators which take two parent nodes and produce a constant
///
/// The operator should have two parents and no arguments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstantBinaryOperatorType {
	/// Creates a new matrix of zeros with dimensions given by its parents
	Zeros,
	/// Creates a new matrix of ones with dimensions given by its parents
	Ones,
	/// Represents elemtwise x < y
	LessThan,
	/// Represents elemtwise x <= y
	LessThanOrEqual,
	/// Represents elemtwise x > y
	GreaterThan,
	/// Represents elemtwise x >= y
	GreaterThanOrEqual,
	/// Represents elementwise x == y
	Equals,
	/// Represents elementiwse x != y
	NotEquals
}

/// An enum for all operators which  produce a constant
///
/// The operator should have either one or two parents and no arguments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstantOperatorType {
	/// A `ConstantUnaryOperatorType`
	Unary(ConstantUnaryOperatorType),
	/// A `ConstantBinaryOperatorType`
	Binary(ConstantBinaryOperatorType),
	/// An empty operator, used for inputs or parameters
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
	/// Represents elemtwise -x
	Neg,
	/// Represents elemntwise x^-1
	Div,
	/// Represents matrix inversion M^-1
	MatrixInverse,
	/// Represents matrix transpose M^T
	Transpose,
	/// Takes the diagonal of a matrix as a column vector
	MatrixDiag,
	/// Takes a vector to a matrix, whose diagonal is equal to that vector
	VectorDiag,
	/// Represents elementwise cos(x)
	Cos,
	/// Represents elementwise sin(x)
	Sin,
	/// Represents elementwise tan(x)
	Tan,
	/// Represents elementwise cosh(x)
	CosH,
	/// Represents elementwise sinh(x)
	SinH,
	/// Represents elementwise tanh(x)
	TanH,
	/// Represents elementwise abs(x)
	Abs,
	/// Represents elementwise ln(x)
	Log,
	/// Represents elementwise e^x
	Exp,
	/// Represents elementwise sqrt(x)
	Sqrt,
	/// Represents elementwise x^2
	Square,
	/// Represents elementwise 1 / (1 + exp(-x))
	Sigmoid,
	/// Repersents elementwise max(x,0)
	Rectifier,
	/// Takes the sum of the elements along the given dimension.
	Sum(Dimension),
	/// Takes the L2 squared norm along the given dimension. This is defuned as sum(x_i^2)
	L2(Dimension),
	/// Takes the L1 norm along the given dimension. This is defined as sum(abs(x_i))
	L1(Dimension)
}

/// An enum for operators which take a two parent nodes
///
/// The operator should have two parents and no arguments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOperatorType {
	/// Represents elementwise max(x,y)
	Max,
	/// Represents elementwise min(x,y)
	Min,
	/// Represents elemntwise power x^y
	Pow,
	/// Represents the matrix quadratic form M_1^T M_2 M_1
	Quadratic
}

/// An enum for operators which are applied to several parent nodes
///
/// The operator should have at least two parents and no arguments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NaryOperatorType {
	/// Represents a + b + ... + z
	Add,
	/// Represents elementwise a * b * ... * z
	Mul,
	/// Represents the linear algebra matrix multiplication M_a M_b ... M_z
	Dot,
	/// Concatenates horizontally all of its arguments
	HorzCat,
	/// Concatenates vertically all of its arguments
	VertCat
}

/// An enum for special operators which take a single parent node.
///
/// The operator should have one parent and several arguments
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpecialUnaryOperatorType {
	/// Takes the sub block of the parent node described by the following 4 arguments.
	/// The operator has the following syntax: (parent, start_x, sizeOfBlockX, start_y, sizeOfBlockY)
	SubIndex,
	/// Represents subassignment. This will produce a matrix for which the sub block described by the 4 argument will be equal to the parent node. E.g. this means that the result is a matrix of zeros, whose subblock is equal to the parent.
	/// The operator has the following syntax: (parent, start_x, sizeOfBlockX, start_y, sizeOfBlockY)
	SubAssign,
	/// Represents reshaping the parent node to a size given by its arguments.
	/// The operator has the following syntax: (parent, dim_1, dim_2)
	Reshape,
	/// Represents the replication of the parent node horizontally. It is assumed that it is a scalar or column vector.
	/// The operator has the following syntax: (parent, times)
	ReplicateHorz,
	/// Represents the replication of the parent node vertically. It is assumed that it is a scalar or row vector.
	/// The operator has the following syntax: (parent, times)
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

/// An enum that represents all supported mathematical operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OperatorType {
	/// A `ConstantOperatorType`
	Constant(ConstantOperatorType),
	/// A `UnaryOperatorType`
	Unary(UnaryOperatorType),
	/// A `BinaryOperatorType`
	Binary(BinaryOperatorType),
	/// A `NaryOperatorType`
	Nary(NaryOperatorType),
	/// A `SpecialUnaryOperatorType`
	Special(SpecialUnaryOperatorType)
}

impl ::std::fmt::Display for OperatorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		match *self {
			OperatorType::Constant(ConstantOperatorType::None) => Ok(()),
			OperatorType::Constant(ConstantOperatorType::Unary(op)) => write!(f, "{:?}", op),
			OperatorType::Constant(ConstantOperatorType::Binary(op)) => write!(f, "{:?}", op),
			OperatorType::Unary(op) => write!(f, "{:?}", op),
			OperatorType::Binary(op) => write!(f, "{:?}", op),
			OperatorType::Nary(op) => write!(f, "{:?}", op),
			OperatorType::Special(op) => write!(f, "{:?}", op)
		}
    }
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

/// A struct that captures all supported mathematical operations with their metadata
#[derive(Clone, Debug, PartialEq)]
pub struct Operator {
		/// The type of this operator
		pub op_type: OperatorType,
		/// List of parent nodes
		pub parents: Vec<usize>,
		/// List of argument nodes
		pub args: Vec<usize>,
}

impl ::std::fmt::Display for Operator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		match self.op_type{
			OPERATOR_NONE => Ok(()),
			_ => {
				if self.parents.len() > 0 && self.args.len() > 0 {
					write!(f, "{}[{:?}|{:?}]", self.op_type, self.parents, self.args)
				} else if self.parents.len() > 0 {
					write!(f, "{}[{:?}]", self.op_type, self.parents)
				} else if self.args.len() > 0 {
					write!(f, "{}[{:?}]", self.op_type, self.args)
				} else {
					write!(f, "{}", self.op_type)
				}
			}
		}
    }
}

impl Operator {
	/// Creates a new `Operator` with the given ancestors
	/// Returns an error if the given ancestors do not match the type of operator
	pub fn new(op_type: OperatorType, parents: Vec<usize>, args: Vec<usize> ) -> Result<Self,OperatorError> {
		if super::super::VERIFICATION {
			match op_type {
				OperatorType::Constant(ConstantOperatorType::None) => {
					if parents.len() != 0 {
						return Err(OperatorError::InvalidNumberOfParents(op_type,parents.len(),0))
					}
					if args.len() != 0 {
						return Err(OperatorError::InvaludNumberOfArguments(op_type,args.len(),0))
					}
				},
				OperatorType::Constant(ConstantOperatorType::Unary(_)) | OperatorType::Unary(_) => {
					if parents.len() != 1 {
						return Err(OperatorError::InvalidNumberOfParents(op_type,parents.len(),1))
					}
					if args.len() != 0 {
						return Err(OperatorError::InvaludNumberOfArguments(op_type,args.len(),0))
					}
				},
				OperatorType::Constant(ConstantOperatorType::Binary(_)) | OperatorType::Binary(_) =>  {
					if parents.len() != 2 {
						return Err(OperatorError::InvalidNumberOfParents(op_type,parents.len(),2))
					}
					if args.len() != 0 {
						return Err(OperatorError::InvaludNumberOfArguments(op_type,args.len(),0))
					}
				},
				OperatorType::Nary(_) => {
					if parents.len() < 2 {
						return Err(OperatorError::InvalidNumberOfParents(op_type,parents.len(),2))
					}
					if args.len() != 0 {
						return Err(OperatorError::InvaludNumberOfArguments(op_type,args.len(),0))
					}
				},
				OperatorType::Special(ref sp) =>  {
					if parents.len() != 1 {
						return Err(OperatorError::InvalidNumberOfParents(op_type,parents.len(),1))
					}
					if args.len() != sp.required_num_of_args() {
						return Err(OperatorError::InvaludNumberOfArguments(op_type, args.len(), sp.required_num_of_args()))
					}
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
			None => Err(OperatorError::ParentNotFound(self.op_type, old_parent, self.parents.clone()))
		}
	}

	/// Swaps the given the parents of the operator in place
	/// Returns an error if `old_parent` was not in the parents' list
	pub fn swap_parent_in_place(&mut self, old_parent: usize, new_parent: usize) -> Result<(), OperatorError> {
		let position = self.parents.iter().position(|&x| x == old_parent);
		match position {
			Some(p) => {
				self.parents.push(new_parent);
				let _  = self.parents.swap_remove(p);
				Ok(())
			}
			None => Err(OperatorError::ParentNotFound(self.op_type, old_parent, self.parents.clone()))
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
			None => Err(OperatorError::ArgumentNotFound(self.op_type, old_arg, self.args.clone()))
		}
	}

	/// Swaps the given the arguments of the operator in place
	/// Returns an error if `old_arg` was not in the arguments' list
	pub fn swap_argument_in_place(&mut self, old_arg: usize, new_arg: usize) -> Result<(), OperatorError> {
		let position = self.args.iter().position(|&x| x == old_arg);
		match position {
			Some(p) => {
				self.args.push(new_arg);
				let _  = self.args.swap_remove(p);
				Ok(())
			}
			None => Err(OperatorError::ArgumentNotFound(self.op_type, old_arg, self.args.clone()))
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
				None => Err(OperatorError::AncestorNotFound(self.op_type, old_anc, self.parents.clone(), self.args.clone()))
			}
		}
	}

	/// Swaps the given the ancestors of the operator in place, in all occurences both in the parents' and arguments' list
	/// Returns an error if `old_anc` was not in the parents' or in the arguments' list
	pub fn swap_ancestor_in_place(&mut self, old_anc: usize, new_anc: usize) -> Result<(), OperatorError> {
		let result_p = self.swap_parent_in_place(old_anc, new_anc);
		let result_a = self.swap_argument_in_place(old_anc, new_anc);
		if result_p.is_ok() || result_a.is_ok() {
			Ok(())
		} else {
			Err(OperatorError::AncestorNotFound(self.op_type, old_anc, self.parents.clone(), self.args.clone()))
		}
	}

	/// Creates an operator of the same type, but with different parents and argumetns
	// #[inline(always)]
	pub fn recreate(&self, parents: Vec<usize>, args: Vec<usize>) -> Result<Self, OperatorError> {
		Operator::new(self.op_type, parents, args)
	}
}

/// A `OperatorType::Constant(ConstantOperatorType::None)`
pub const OPERATOR_NONE: OperatorType = OperatorType::Constant(ConstantOperatorType::None);
/// A `OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Const))`
pub const OPERATOR_CONST: OperatorType = OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Const));
/// A `OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Eye))`
pub const OPERATOR_EYE: OperatorType = OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Eye));
/// A `OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Sign))`
pub const OPERATOR_SIGN: OperatorType = OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Sign));
/// A `OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Size(Dimension::First)))`
pub const OPERATOR_SIZE_1: OperatorType = OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Size(Dimension::First)));
/// A `OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Size(Dimension::Second)))`
pub const OPERATOR_SIZE_2: OperatorType = OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Size(Dimension::Second)));
/// A `OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Ones))`
pub const OPERATOR_ONES: OperatorType = OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Ones));
/// A `OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Zeros))`
pub const OPERATOR_ZEROS: OperatorType = OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Zeros));
/// A `OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::LessThan))`
pub const OPERATOR_LT: OperatorType = OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::LessThan));
/// A `OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::LessThanOrEqual))`
pub const OPERATOR_LTE: OperatorType = OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::LessThanOrEqual));
/// A `OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::GreaterThan))`
pub const OPERATOR_GT: OperatorType = OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::GreaterThan));
/// A `OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::GreaterThanOrEqual))`
pub const OPERATOR_GTE: OperatorType = OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::GreaterThanOrEqual));
/// A `OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Equals))`
pub const OPERATOR_EQ: OperatorType = OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Equals));
/// A `OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::NotEquals))`
pub const OPERATOR_NEQ: OperatorType = OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::NotEquals));
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Neg)`
pub const OPERATOR_NEG: OperatorType = OperatorType::Unary(UnaryOperatorType::Neg);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Div)`
pub const OPERATOR_DIV: OperatorType = OperatorType::Unary(UnaryOperatorType::Div);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::MatrixInverse)`
pub const OPERATOR_MINV: OperatorType = OperatorType::Unary(UnaryOperatorType::MatrixInverse);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Transpose)`
pub const OPERATOR_TRANSPOSE: OperatorType = OperatorType::Unary(UnaryOperatorType::Transpose);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::MatrixDiag)`
pub const OPERATOR_MDIAG: OperatorType = OperatorType::Unary(UnaryOperatorType::MatrixDiag);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::VectorDiag)`
pub const OPERATOR_VDIAG: OperatorType = OperatorType::Unary(UnaryOperatorType::VectorDiag);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Cos)`
pub const OPERATOR_COS: OperatorType = OperatorType::Unary(UnaryOperatorType::Cos);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Sin)`
pub const OPERATOR_SIN: OperatorType = OperatorType::Unary(UnaryOperatorType::Sin);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Tan)`
pub const OPERATOR_TAN: OperatorType = OperatorType::Unary(UnaryOperatorType::Tan);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::CosH)`
pub const OPERATOR_COSH: OperatorType = OperatorType::Unary(UnaryOperatorType::CosH);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::SinH)`
pub const OPERATOR_SINH: OperatorType = OperatorType::Unary(UnaryOperatorType::SinH);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::TanH)`
pub const OPERATOR_TANH: OperatorType = OperatorType::Unary(UnaryOperatorType::TanH);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Abs)`
pub const OPERATOR_ABS: OperatorType = OperatorType::Unary(UnaryOperatorType::Abs);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Log)`
pub const OPERATOR_LOG: OperatorType = OperatorType::Unary(UnaryOperatorType::Log);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Exp)`
pub const OPERATOR_EXP: OperatorType = OperatorType::Unary(UnaryOperatorType::Exp);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Sqrt)`
pub const OPERATOR_SQRT: OperatorType = OperatorType::Unary(UnaryOperatorType::Sqrt);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Square)`
pub const OPERATOR_SQUARE: OperatorType = OperatorType::Unary(UnaryOperatorType::Square);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Sigmoid)`
pub const OPERATOR_SIGM: OperatorType = OperatorType::Unary(UnaryOperatorType::Sigmoid);
/// A `OperatorType = OperatorType::Unary(UnaryOperatorType::Rectifier)`
pub const OPERATOR_RECT: OperatorType = OperatorType::Unary(UnaryOperatorType::Rectifier);
/// A `OperatorType::Unary(UnaryOperatorType::Sum(Dimension::All))`
pub const OPERATOR_SUM_ALL: OperatorType = OperatorType::Unary(UnaryOperatorType::Sum(Dimension::All));
/// A `OperatorType::Unary(UnaryOperatorType::Sum(Dimension::First))`
pub const OPERATOR_SUM_1: OperatorType = OperatorType::Unary(UnaryOperatorType::Sum(Dimension::First));
/// A `OperatorType::Unary(UnaryOperatorType::Sum(Dimension::Second))`
pub const OPERATOR_SUM_2: OperatorType = OperatorType::Unary(UnaryOperatorType::Sum(Dimension::Second));
/// A `OperatorType::Unary(UnaryOperatorType::L2(Dimension::All))`
pub const OPERATOR_L2_ALL: OperatorType = OperatorType::Unary(UnaryOperatorType::L2(Dimension::All));
/// A `OperatorType::Unary(UnaryOperatorType::L2(Dimension::First))`
pub const OPERATOR_L2_1: OperatorType = OperatorType::Unary(UnaryOperatorType::L2(Dimension::First));
/// A `OperatorType::Unary(UnaryOperatorType::L2(Dimension::Second))`
pub const OPERATOR_L2_2: OperatorType = OperatorType::Unary(UnaryOperatorType::L2(Dimension::Second));
/// A `OperatorType::Unary(UnaryOperatorType::L1(Dimension::All))`
pub const OPERATOR_L1_ALL: OperatorType = OperatorType::Unary(UnaryOperatorType::L1(Dimension::All));
/// A `OperatorType::Unary(UnaryOperatorType::L1(Dimension::First))`
pub const OPERATOR_L1_1: OperatorType = OperatorType::Unary(UnaryOperatorType::L1(Dimension::First));
/// A `OperatorType::Unary(UnaryOperatorType::L1(Dimension::Second))`
pub const OPERATOR_L1_2: OperatorType = OperatorType::Unary(UnaryOperatorType::L1(Dimension::Second));
/// A `OperatorType::Binary(BinaryOperatorType::Max))`
pub const OPERATOR_MAX: OperatorType = OperatorType::Binary(BinaryOperatorType::Max);
/// A `OperatorType::Binary(BinaryOperatorType::Min))`
pub const OPERATOR_MIN: OperatorType = OperatorType::Binary(BinaryOperatorType::Min);
/// A `OperatorType::Binary(BinaryOperatorType::Pow))`
pub const OPERATOR_POW: OperatorType = OperatorType::Binary(BinaryOperatorType::Pow);
/// A `OperatorType::Binary(BinaryOperatorType::Quadratic))`
pub const OPERATOR_QUAD: OperatorType = OperatorType::Binary(BinaryOperatorType::Quadratic);
/// A `OperatorType::Nary(NaryOperatorType::Add)`
pub const OPERATOR_ADD: OperatorType = OperatorType::Nary(NaryOperatorType::Add);
/// A `OperatorType::Nary(NaryOperatorType::Mul)`
pub const OPERATOR_MUL: OperatorType = OperatorType::Nary(NaryOperatorType::Mul);
/// A `OperatorType::Nary(NaryOperatorType::Dot)`
pub const OPERATOR_DOT: OperatorType = OperatorType::Nary(NaryOperatorType::Dot);
/// A `OperatorType::Nary(NaryOperatorType::HorzCat)`
pub const OPERATOR_HORZCAT: OperatorType = OperatorType::Nary(NaryOperatorType::HorzCat);
/// A `OperatorType::Nary(NaryOperatorType::VertCat)`
pub const OPERATOR_VERTCAT: OperatorType = OperatorType::Nary(NaryOperatorType::VertCat);
/// A `OperatorType::Special(SpecialUnaryOperatorType::SubIndex)`
pub const OPERATOR_SUBINDEX: OperatorType = OperatorType::Special(SpecialUnaryOperatorType::SubIndex);
/// A `OperatorType::Special(SpecialUnaryOperatorType::SubAssign)`
pub const OPERATOR_SUBASSIGN: OperatorType = OperatorType::Special(SpecialUnaryOperatorType::SubAssign);
/// A `OperatorType::Special(SpecialUnaryOperatorType::Reshape)`
pub const OPERATOR_RESHAPE: OperatorType = OperatorType::Special(SpecialUnaryOperatorType::Reshape);
/// A `OperatorType::Special(SpecialUnaryOperatorType::ReplicateHorz)`
pub const OPERATOR_REPLICATEH: OperatorType = OperatorType::Special(SpecialUnaryOperatorType::ReplicateHorz);
/// A `OperatorType::Special(SpecialUnaryOperatorType::ReplicateVert)`
pub const OPERATOR_REPLICATEV: OperatorType = OperatorType::Special(SpecialUnaryOperatorType::ReplicateVert);



// /// Represents the possible NotFoundErrorType
// #[derive(Clone, Copy, Debug)]
// pub enum NotFoundErrorType{
// 	/// The node is a parent
// 	Parent,
// 	/// The node is an argument
// 	Argument,
// 	/// The node is either a parent or an argument
// 	Ancestor
// }

/// An error when trying to create or manipulate an operator in in consistent way
#[derive(Clone, Debug)]
pub enum OperatorError{
	/// When trying to swap a prent which is not in the parents' list
	/// Fields are (operator, requested parent, parents' list)
	ParentNotFound(OperatorType,usize, Vec<usize>),
	/// When trying to swap an argument which is not in the parents' list
	/// Fields are (operator, requested argument, arguments' list)
	ArgumentNotFound(OperatorType,usize, Vec<usize>),
	/// When trying to swap an ancestor which is not in the parents' list
	/// Fields are (operator, requested ancestor, parents' list, arguments' list)
	AncestorNotFound(OperatorType,usize, Vec<usize>, Vec<usize>),
	/// When trying to create an operator with the wrong number of parents
	/// Fields are (operator, given number of parents, expected number of parents)
	InvalidNumberOfParents(OperatorType, usize, usize),
	/// When trying to create an operator with the wrong number of arguments
	/// Fields are (operator, given number of parents, expected number of parents)
	InvaludNumberOfArguments(OperatorType, usize, usize),
	/// When trying to create an operator with the wrong number of ancesotrs
	/// Fields are (operator, given number of parents, expected number of parents)
	InvalidNumberOfAncestors(OperatorType, usize, usize),
	/// When trying to create an operator with wrongly specified dimension. These are in in integer format where 0 is `Dimension::All`
	/// Fields are (operator name, dimension provided, possible dimensions)
	InvalidDimensionArgument(String, usize, Vec<usize>)
}

impl ::std::fmt::Display for OperatorError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		match *self {
			OperatorError::ParentNotFound(op,req,ref actual) => write!(f, "Requested parent {} for operator {} does not exist. Parents: {:?}", req, op, actual),
			OperatorError::ArgumentNotFound(op,req,ref actual) => write!(f, "Requested argument {} for operator {} does not exist. Arguments: {:?}", req, op, actual),
			OperatorError::AncestorNotFound(op,req,ref p_list,ref arg_list) => write!(f, "Requested ancestor {} for operator {} does not exist. Parents: {:?}, Arguments: {:?}", req, op, p_list, arg_list),
			OperatorError::InvalidNumberOfParents(op, given, expected) => write!(f, "Can not create an operator {} with {} parents, when {} are required", op, given, expected),
			OperatorError::InvaludNumberOfArguments(op, given, expected) => write!(f, "Can not create an operator {} with {} arguments, when {} are required", op, given, expected),
			OperatorError::InvalidNumberOfAncestors(op, given, expected) => write!(f, "Can not create an operator {} with {} ancesotrs, when {} are required", op, given, expected),
			OperatorError::InvalidDimensionArgument(ref name, given, ref expected) => write!(f, "Can not create an operator {} with dimension {}, when {:?} are possible", name, given, expected),
		}
    }
}

impl ::std::error::Error for OperatorError {
    fn description(&self) -> &str {
        match *self {
			OperatorError::ParentNotFound(_,_,_) => "Attempted swapping of a parent which is not present in the parents' list",
			OperatorError::ArgumentNotFound(_,_,_) => "Attempted swapping of an argument which is not present in the arguments' list",
			OperatorError::AncestorNotFound(_,_,_,_) => "Attempted swapping of an ancestor which is not present in either the parertns' or the arguments' list",
			OperatorError::InvalidNumberOfParents(_,_,_) => "Attempted to create an operator with wrong number of parents",
			OperatorError::InvaludNumberOfArguments(_,_,_) =>  "Attempted to create an operator with wrong number of arguments",
			OperatorError::InvalidNumberOfAncestors(_,_,_) =>  "Attempted to create an operator with wrong number of acnestors",
			OperatorError::InvalidDimensionArgument(_,_,_) =>  "Attempted to create an operator with wrong dimensionality argument"
        }
    }
}
