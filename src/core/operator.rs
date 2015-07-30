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
	/// `Constant` logical operator
	LessThan(usize, usize),
	/// `Constant` logical operator
	LessThanOrEqual(usize, usize),
	/// `Constant` logical operator
	GreaterThan(usize, usize),
	/// `Constant` logical operator
	GreaterThanOrEqual(usize, usize),
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
	Sigmoid(usize),
	/// `Unary` operator representing elementwise max
	Max(usize, usize),
	/// `Unary` operator representing elementwise max
	Min(usize, usize),
	// /// `Unary` operator that represents the broadcasting of the first argument
	// /// along the dimension of the second argument with respect to the third.
	// Broadcast(usize, Dimension),
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
	/// `Binary` operator that represents the matrix quadratic form A' B A
	Quadratic(usize,usize),
	/// `Nary` operator that concatenates horizontally all of its arguments
	HorzCat(Vec<usize>),
	/// `Nary` operator that concatenates vertically all of its arguments
	VertCat(Vec<usize>),
	/// `Unary` operator that takes the sub block of the first argument described by the others in the sense
	/// (parent, start_x, sizeOfBlockX, start_y, sizeOfBlockY)
	SubIndex(usize,usize,usize,usize,usize),
	/// `Unary` operator that represent the oppoiste of subindexing - subassignment. E.g. this means that
	/// the result is a matrix of zeros, whose subblock is equal to the parent.
	/// The arguments are in the same format as subindex - (parent, start_x, sizeOfBlockX, start_y, sizeOfBlockY)
	SubAssign(usize, usize, usize, usize, usize),
	/// `Unary` operator that takes the reshapes the first argument to a matrix of dimensions (2nd,3rd)
	Reshape(usize,usize,usize),
	/// `Unary` operator that replicates the first argument horizontally. It is assumed that it is a scalar or column vector.
	ReplicateHorz(usize,usize),
	/// `Unary` operator that replicates the first argument vertically. It is assumed that it is a scalar or row vector.
	ReplicateVert(usize,usize)
}

impl ToString for Operator{
	fn to_string(&self) -> String {format!("{:?}", self)}
}

impl Operator{
	pub fn swap_parent(&self, old_parent: usize, new_parent: usize) -> Result<Self, String> {
        match *self {
            Operator::Const(p) => match p == old_parent{
                true => Ok(Operator::Const(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Eye(p) => match p == old_parent{
                true => Ok(Operator::Eye(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Size(p,dim) => match p == old_parent{
                true => Ok(Operator::Size(new_parent,dim)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Sign(p) => match p == old_parent{
                true => Ok(Operator::Sign(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Neg(p) => match p == old_parent{
                true => Ok(Operator::Neg(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Div(p) => match p == old_parent{
                true => Ok(Operator::Div(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::MatrixInverse(p) => match p == old_parent{
                true => Ok(Operator::MatrixInverse(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Transpose(p) => match p == old_parent{
                true => Ok(Operator::Transpose(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::MatrixDiag(p) => match p == old_parent{
                true => Ok(Operator::MatrixDiag(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::VectorDiag(p) => match p == old_parent{
                true => Ok(Operator::VectorDiag(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Cos(p) => match p == old_parent{
                true => Ok(Operator::Cos(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Sin(p) => match p == old_parent{
                true => Ok(Operator::Sin(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Tan(p) => match p == old_parent{
                true => Ok(Operator::Tan(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::CosH(p) => match p == old_parent{
                true => Ok(Operator::CosH(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::SinH(p) => match p == old_parent{
                true => Ok(Operator::SinH(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::TanH(p) => match p == old_parent{
                true => Ok(Operator::TanH(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Abs(p) => match p == old_parent{
                true => Ok(Operator::Abs(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Log(p) => match p == old_parent{
                true => Ok(Operator::Log(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Exp(p) => match p == old_parent{
                true => Ok(Operator::Exp(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Sqrt(p) => match p == old_parent{
                true => Ok(Operator::Sqrt(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Square(p) => match p == old_parent{
                true => Ok(Operator::Square(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Sigmoid(p) => match p == old_parent{
                true => Ok(Operator::Sigmoid(new_parent)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Sum(p,dim) => match p == old_parent{
                true => Ok(Operator::Sum(new_parent,dim)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::L2(p,dim) => match p == old_parent{
                true => Ok(Operator::L2(new_parent,dim)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::L1(p,dim) => match p == old_parent{
                true => Ok(Operator::L1(new_parent,dim)),
                false => Err("The old parent was not found in this operator!".to_string())
            },
            Operator::Ones(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::Ones(new_parent, p_2)),
                    2 => Ok(Operator::Ones(p_1, new_parent)),
                    3 => Ok(Operator::Ones(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::Zeros(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::Zeros(new_parent, p_2)),
                    2 => Ok(Operator::Zeros(p_1, new_parent)),
                    3 => Ok(Operator::Zeros(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::LessThan(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::LessThan(new_parent, p_2)),
                    2 => Ok(Operator::LessThan(p_1, new_parent)),
                    3 => Ok(Operator::LessThan(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::LessThanOrEqual(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::LessThanOrEqual(new_parent, p_2)),
                    2 => Ok(Operator::LessThanOrEqual(p_1, new_parent)),
                    3 => Ok(Operator::LessThanOrEqual(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::GreaterThan(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::GreaterThan(new_parent, p_2)),
                    2 => Ok(Operator::GreaterThan(p_1, new_parent)),
                    3 => Ok(Operator::GreaterThan(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::GreaterThanOrEqual(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::GreaterThanOrEqual(new_parent, p_2)),
                    2 => Ok(Operator::GreaterThanOrEqual(p_1, new_parent)),
                    3 => Ok(Operator::GreaterThanOrEqual(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::Max(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::Max(new_parent, p_2)),
                    2 => Ok(Operator::Max(p_1, new_parent)),
                    3 => Ok(Operator::Max(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::Min(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::Min(new_parent, p_2)),
                    2 => Ok(Operator::Min(p_1, new_parent)),
                    3 => Ok(Operator::Min(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::Pow(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::Pow(new_parent, p_2)),
                    2 => Ok(Operator::Pow(p_1, new_parent)),
                    3 => Ok(Operator::Pow(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::Quadratic(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::Quadratic(new_parent, p_2)),
                    2 => Ok(Operator::Quadratic(p_1, new_parent)),
                    3 => Ok(Operator::Quadratic(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::ReplicateHorz(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::ReplicateHorz(new_parent, p_2)),
                    2 => Ok(Operator::ReplicateHorz(p_1, new_parent)),
                    3 => Ok(Operator::ReplicateHorz(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::ReplicateVert(p_1,p_2) => match (p_1 == old_parent) as i8 + 2*(p_2 == old_parent) as i8{
                    0 => Err("The old parent was not found in this operator!".to_string()),
                    1 => Ok(Operator::ReplicateVert(new_parent, p_2)),
                    2 => Ok(Operator::ReplicateVert(p_1, new_parent)),
                    3 => Ok(Operator::ReplicateVert(new_parent, new_parent)),
                    _ => unreachable!()
            },
            Operator::Add(ref parents) => {
                let mut check = false;
                let res = parents.iter().map(|&x|
                    if x == old_parent {check=true; new_parent} else {x})
                .collect::<Vec<usize>>();
                if check {
                    Ok(Operator::Add(res))
                }
                else {
                    Err("The old parent was not found in this operator!".to_string())
                }
            },
            Operator::Mul(ref parents) => {
                let mut check = false;
                let res = parents.iter().map(|&x|
                    if x == old_parent {check=true; new_parent} else {x})
                .collect::<Vec<usize>>();
                if check {
                    Ok(Operator::Mul(res))
                }
                else {
                    Err("The old parent was not found in this operator!".to_string())
                }
            },
            Operator::Dot(ref parents) => {
                let mut check = false;
                let res = parents.iter().map(|&x|
                    if x == old_parent {check=true; new_parent} else {x})
                .collect::<Vec<usize>>();
                if check {
                    Ok(Operator::Dot(res))
                }
                else {
                    Err("The old parent was not found in this operator!".to_string())
                }
            },
            Operator::HorzCat(ref parents) => {
                let mut check = false;
                let res = parents.iter().map(|&x|
                    if x == old_parent {check=true; new_parent} else {x})
                .collect::<Vec<usize>>();
                if check {
                    Ok(Operator::HorzCat(res))
                }
                else {
                    Err("The old parent was not found in this operator!".to_string())
                }
            },
            Operator::VertCat(ref parents) => {
                let mut check = false;
                let res = parents.iter().map(|&x|
                    if x == old_parent {check=true; new_parent} else {x})
                .collect::<Vec<usize>>();
                if check {
                    Ok(Operator::VertCat(res))
                }
                else {
                    Err("The old parent was not found in this operator!".to_string())
                }
            },
            Operator::SubIndex(p_1,p_2,p_3,p_4,p_5) => {
                let mut parents = vec![p_1, p_2, p_3, p_4, p_5];
                let mut check = false;
                parents = parents.iter().map(|&x|
                    if x == old_parent {check=true; new_parent} else {x})
                .collect::<Vec<usize>>();
                if check {
                    Ok(Operator::SubIndex(parents[0],parents[1],parents[2],parents[3],parents[4]))
                }
                else {
                    Err("The old parent was not found in this operator!".to_string())
                }
            },
            Operator::SubAssign(p_1,p_2,p_3,p_4,p_5) => {
                let mut parents = vec![p_1, p_2, p_3, p_4, p_5];
                let mut check = false;
                parents = parents.iter().map(|&x|
                    if x == old_parent {check=true; new_parent} else {x})
                .collect::<Vec<usize>>();
                if check {
                    Ok(Operator::SubAssign(parents[0],parents[1],parents[2],parents[3],parents[4]))
                }
                else {
                    Err("The old parent was not found in this operator!".to_string())
                }
            },
            Operator::Reshape(p_1,p_2,p_3) => {
                let mut parents = vec![p_1, p_2, p_3];
                let mut check = false;
                parents = parents.iter().map(|&x|
                    if x == old_parent {check=true; new_parent} else {x})
                .collect::<Vec<usize>>();
                if check {
                    Ok(Operator::Reshape(parents[0],parents[1],parents[2]))
                }
                else {
                    Err("The old parent was not found in this operator!".to_string())
                }
            }
        }
	}

	pub fn recreate(&self, parents: Vec<usize>) -> Result<Self, String> {
		match *self{
			Operator::Const(_) => match parents.len() {
				1 => Ok(Operator::Const(parents[0])),
				_ =>  Err("Const takes only 1 parent!".to_string())
            },
            Operator::Eye(_) => match parents.len() {
				1 => Ok(Operator::Eye(parents[0])),
				_ =>  Err("Eye takes only 1 parent!".to_string())
            },
            Operator::Size(_,dim) => match parents.len() {
				1 => Ok(Operator::Size(parents[0],dim)),
				_ =>  Err("Size takes only 1 parent!".to_string())
            },
            Operator::Sign(_) => match parents.len() {
				1 => Ok(Operator::Sign(parents[0])),
				_ =>  Err("Sign takes only 1 parent!".to_string())
            },
            Operator::Neg(_) => match parents.len() {
				1 => Ok(Operator::Neg(parents[0])),
				_ =>  Err("Neg takes only 1 parent!".to_string())
            },
            Operator::Div(_) =>match parents.len() {
				1 => Ok(Operator::Div(parents[0])),
				_ =>  Err("Div takes only 1 parent!".to_string())
            },
            Operator::MatrixInverse(_) => match parents.len() {
				1 => Ok(Operator::MatrixInverse(parents[0])),
				_ =>  Err("MatrixInverse takes only 1 parent!".to_string())
            },
            Operator::Transpose(_) => match parents.len() {
				1 => Ok(Operator::Transpose(parents[0])),
				_ =>  Err("Transpose takes only 1 parent!".to_string())
            },
            Operator::MatrixDiag(_) => match parents.len() {
				1 => Ok(Operator::MatrixDiag(parents[0])),
				_ =>  Err("MatrixDiag takes only 1 parent!".to_string())
            },
            Operator::VectorDiag(_) => match parents.len() {
				1 => Ok(Operator::VectorDiag(parents[0])),
				_ =>  Err("VectorDiag takes only 1 parent!".to_string())
            },
            Operator::Cos(_) => match parents.len() {
				1 => Ok(Operator::Cos(parents[0])),
				_ =>  Err("Cos takes only 1 parent!".to_string())
            },
            Operator::Sin(_) => match parents.len() {
				1 => Ok(Operator::Sin(parents[0])),
				_ =>  Err("Sin takes only 1 parent!".to_string())
            },
            Operator::Tan(_) => match parents.len() {
				1 => Ok(Operator::Tan(parents[0])),
				_ =>  Err("Tan takes only 1 parent!".to_string())
            },
            Operator::CosH(_) => match parents.len() {
				1 => Ok(Operator::CosH(parents[0])),
				_ =>  Err("CosH takes only 1 parent!".to_string())
            },
            Operator::SinH(_) => match parents.len() {
				1 => Ok(Operator::SinH(parents[0])),
				_ =>  Err("SinH takes only 1 parent!".to_string())
            },
            Operator::TanH(_) => match parents.len() {
				1 => Ok(Operator::TanH(parents[0])),
				_ =>  Err("TanH takes only 1 parent!".to_string())
            },
            Operator::Abs(_) => match parents.len() {
				1 => Ok(Operator::Abs(parents[0])),
				_ =>  Err("Abs takes only 1 parent!".to_string())
            },
            Operator::Log(_) => match parents.len() {
				1 => Ok(Operator::Log(parents[0])),
				_ =>  Err("Log takes only 1 parent!".to_string())
            },
            Operator::Exp(_) => match parents.len() {
				1 => Ok(Operator::Exp(parents[0])),
				_ =>  Err("Exp takes only 1 parent!".to_string())
            },
            Operator::Sqrt(_) => match parents.len() {
				1 => Ok(Operator::Sqrt(parents[0])),
				_ =>  Err("Sqrt takes only 1 parent!".to_string())
            },
            Operator::Square(_) => match parents.len() {
				1 => Ok(Operator::Square(parents[0])),
				_ =>  Err("Square takes only 1 parent!".to_string())
            },
            Operator::Sigmoid(_) => match parents.len() {
				1 => Ok(Operator::Sigmoid(parents[0])),
				_ =>  Err("Sigmoid takes only 1 parent!".to_string())
            },
            Operator::Sum(_,dim) => match parents.len() {
				1 => Ok(Operator::Sum(parents[0],dim)),
				_ =>  Err("Sum takes only 1 parent!".to_string())
            },
            Operator::L2(_,dim) => match parents.len() {
				1 => Ok(Operator::L2(parents[0],dim)),
				_ =>  Err("L2 takes only 1 parent!".to_string())
            },
            Operator::L1(_,dim) => match parents.len() {
				1 => Ok(Operator::L1(parents[0],dim)),
				_ =>  Err("L1 takes only 1 parent!".to_string())
            },
            Operator::Ones(_,_) => match parents.len() {
				2 => Ok(Operator::Ones(parents[0],parents[1])),
				_ =>  Err("Ones takes only 2 parents!".to_string())
            },
            Operator::Zeros(_,_) => match parents.len() {
				2 => Ok(Operator::Zeros(parents[0],parents[1])),
				_ =>  Err("Zeros takes only 2 parents!".to_string())
            },
            Operator::LessThan(_,_) => match parents.len() {
				2 => Ok(Operator::LessThan(parents[0],parents[1])),
				_ =>  Err("LessThan takes only 2 parents!".to_string())
            },
            Operator::LessThanOrEqual(_,_) => match parents.len() {
				2 => Ok(Operator::LessThanOrEqual(parents[0],parents[1])),
				_ =>  Err("LessThanOrEqual takes only 2 parents!".to_string())
            },
            Operator::GreaterThan(_,_) => match parents.len() {
				2 => Ok(Operator::GreaterThan(parents[0],parents[1])),
				_ =>  Err("GreaterThan takes only 2 parents!".to_string())
            },
            Operator::GreaterThanOrEqual(_,_) => match parents.len() {
				2 => Ok(Operator::GreaterThanOrEqual(parents[0],parents[1])),
				_ =>  Err("GreaterThanOrEqual takes only 2 parents!".to_string())
            },
            Operator::Max(_,_) => match parents.len() {
				2 => Ok(Operator::Max(parents[0],parents[1])),
				_ =>  Err("Max takes only 2 parents!".to_string())
            },
            Operator::Min(_,_) =>  match parents.len() {
				2 => Ok(Operator::Min(parents[0],parents[1])),
				_ =>  Err("Min takes only 2 parents!".to_string())
            },
            Operator::Pow(_,_) =>  match parents.len() {
				2 => Ok(Operator::Pow(parents[0],parents[1])),
				_ =>  Err("Pow takes only 2 parents!".to_string())
            },
            Operator::Quadratic(_,_) =>  match parents.len() {
				2 => Ok(Operator::Quadratic(parents[0],parents[1])),
				_ =>  Err("Quadratic takes only 2 parents!".to_string())
            },
            Operator::ReplicateHorz(_,_) =>  match parents.len() {
				2 => Ok(Operator::ReplicateHorz(parents[0],parents[1])),
				_ =>  Err("ReplicateHorz takes only 2 parents!".to_string())
            },
            Operator::ReplicateVert(_,_) =>  match parents.len() {
				2 => Ok(Operator::ReplicateVert(parents[0],parents[1])),
				_ =>  Err("ReplicateVert takes only 2 parents!".to_string())
            },
            Operator::Add(_) =>  match parents.len() {
				0...1 => Err("Add takes at least 2 parents!".to_string()),
				_ =>  Ok(Operator::Add(parents))
            },
            Operator::Mul(_) =>  match parents.len() {
				0...1 => Err("Mul takes at least 2 parents!".to_string()),
				_ =>  Ok(Operator::Mul(parents))
            },
            Operator::Dot(_) =>  match parents.len() {
				0...1 => Err("Dot takes at least 2 parents!".to_string()),
				_ =>  Ok(Operator::Dot(parents))
            },
            Operator::HorzCat(_) =>  match parents.len() {
				0...1 => Err("HorzCat takes at least 2 parents!".to_string()),
				_ =>  Ok(Operator::HorzCat(parents))
            },
            Operator::VertCat(_) =>  match parents.len() {
				0...1 => Err("VertCat takes at least 2 parents!".to_string()),
				_ =>  Ok(Operator::VertCat(parents))
            },
            Operator::SubIndex(_,_,_,_,_) =>  match parents.len() {
				5 =>  Ok(Operator::SubIndex(parents[0],parents[1],parents[2],parents[3],parents[4])),
				_ =>  Err("SubIndex takes only 5 parents!".to_string())
            },
            Operator::SubAssign(_,_,_,_,_) =>  match parents.len() {
				5 =>  Ok(Operator::SubAssign(parents[0],parents[1],parents[2],parents[3],parents[4])),
				_ =>  Err("SubAssign takes only 5 parents!".to_string())
            },
            Operator::Reshape(_,_,_) => match parents.len() {
				3 =>  Ok(Operator::Reshape(parents[0],parents[1],parents[2])),
				_ =>  Err("SubAssign takes only 3 parents!".to_string())
            },
		}
	}

	pub fn unary(&self) -> bool {
		match *self {
            Operator::Const(_) | Operator::Eye(_) | Operator::Size(_,_) | Operator::Sign(_)
			| Operator::Neg(_)| Operator::Div(_)| Operator::MatrixInverse(_)
			| Operator::Transpose(_)| Operator::MatrixDiag(_) | Operator::VectorDiag(_)
			| Operator::Cos(_) | Operator::Sin(_)| Operator::Tan(_)
			| Operator::CosH(_)| Operator::SinH(_)| Operator::TanH(_)
			| Operator::Abs(_) | Operator::Log(_)| Operator::Exp(_)| Operator::Sqrt(_)
			| Operator::Square(_) | Operator::Sigmoid(_) | Operator::Sum(_,_)
			| Operator::L2(_,_)| Operator::L1(_,_) => true,
			_ => false
		}
	}

	pub fn binary(&self) -> bool {
		match *self{
			Operator::Ones(_,_) | Operator::Zeros(_,_)| Operator::LessThan(_,_)
			| Operator::LessThanOrEqual(_,_) | Operator::GreaterThan(_,_)
			| Operator::GreaterThanOrEqual(_,_)| Operator::Max(_,_)
			| Operator::Min(_,_) | Operator::Pow(_,_) | Operator::Quadratic(_,_)
			| Operator::ReplicateHorz(_,_) | Operator::ReplicateVert(_,_) => true,
			_ => false
		}
	}

	pub fn nary(&self) -> bool {
		match *self{
			Operator::Add(_)| Operator::Mul(_) | Operator::Dot(_)
			| Operator::HorzCat(_) | Operator::VertCat(_)
			| Operator::SubIndex(_,_,_,_,_) | Operator::SubAssign(_,_,_,_,_)
			| Operator::Reshape(_,_,_) => true,
			_ => false
		}
	}
}
