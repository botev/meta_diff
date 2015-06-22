//! This is a module for manipulation of symbolic polynomials overs the integers.
//! Unfortunately, at this stage since `Rust` does not support integers as Generic types,
//! the only possible implementation of `SymMonomials` is with a vector. This in terms means
//! that it can not implement the `Copy` trait, for which reason currently all operations are
//! defined for references to the actual structures. If this changes in the future I will
//! update the module accordingly.
//!
//! Any examples of usage can be found under the relevant methods in each structure documentation.

use std::ops::{Add, Sub, Neg, Mul, Div};
use std::fmt::{Display, Formatter, Error};
use std::cmp::Ordering;

/// The default number of symbolic variables for now is set 10
const DEFAULT_VARIABLES : usize = 10;

/// An implementation for a monomial over the integers. 
#[derive(Clone, Debug)]
pub struct SymMonomial{
	vars : usize,
	pub coefficient : i32,
	pub powers : Vec<usize>
}

impl SymMonomial{
	/// Checks if the monomial is a constant
	pub fn is_const(&self) -> bool {
		for power in self.powers.iter(){
			if *power > 0 {
				return false
			}
		}
		return true
	}

	/// Creates a new monomial with the given number of symbolic variables. 
	/// Coefficient is initialised to 1.
	pub fn new(vars: usize) -> Self{
		let mut result = SymMonomial{vars: vars, coefficient : 1, powers: Vec::new()};
		result.powers = vec![0; result.vars];
		result
	}
}

/// The default implementation creates a new monomial using the defaultVariables. 
/// Coefficient is initialised to 1.
impl Default for SymMonomial{
	fn default() -> Self{ 
		return SymMonomial::new(DEFAULT_VARIABLES);
	}
}

/// PartialEq for a monomial takes in to account only the powers of the symbolic variables.
/// # Panics
/// The function will panic if you pass tow monomials with different number of symbolic variables.
///
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::SymMonomial;
/// //-5x^2 == 2x^2
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// mon1.coefficient = -5;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 2;
/// mon2.coefficient = 2;
/// assert!(mon1 == mon2);
/// ```
/// ```
/// # use meta_diff::core::symbolic::SymMonomial;
/// //x^2y != xy^2
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// mon1.powers[1] = 1;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 1;
/// mon2.powers[1] = 2;
/// assert!(mon1 != mon2);
/// ```
/// ```should_panic
/// # use meta_diff::core::symbolic::SymMonomial;
/// // Panic example
/// let mon1 = SymMonomial::new(1);
/// let mon2 = SymMonomial::new(2);
/// println!("{}",mon1 == mon2);
/// ```
impl PartialEq for SymMonomial{
	fn eq(&self, other: &SymMonomial) -> bool{
		if self.vars != other.vars {
			panic!("Comparing monomials with different number of variables.")
		}
		if self.coefficient == 0 && other.coefficient == 0{
			return true
		}
		for (my_power, other_power) in self.powers.iter().zip(other.powers.iter()){
			if my_power != other_power {
				return false
			}
		}
		return true 
	}
}

/// Directly uses `PartialEq`
impl Eq for SymMonomial{}

/// The partial ordering of the monomials is based on which monomial is of greatest total power. 
/// Coefficients are ignored. Will return `None` only if you pass two monomials with different 
/// number of symbolic variables
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::SymMonomial;
/// // x^5 < x^3y^3
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 5;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 3;
/// mon2.powers[1] = 3;
/// assert!(mon1 < mon2);
/// ```
/// ```
/// # use meta_diff::core::symbolic::SymMonomial;
/// # use std::cmp::Ordering;
/// // x^2y^3 == x^3y^2
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// mon1.powers[1] = 3;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 3;
/// mon2.powers[1] = 2;
/// assert!(mon1.partial_cmp(&mon2).unwrap() == Ordering::Equal);
/// ```
/// ```
/// # use meta_diff::core::symbolic::SymMonomial;
/// # use std::cmp::Ordering;
/// // Monomials with differnet number of variables are incomparable
/// let mut mon1 = SymMonomial::new(1);
/// let mut mon2 = SymMonomial::new(2);
/// assert!(mon1.partial_cmp(&mon2) == None);
/// ```
impl PartialOrd for SymMonomial{
	fn partial_cmp(&self, other: &SymMonomial) -> Option<Ordering> {
		if self.vars != other.vars {
			return None
		}
		let sum1 = self.powers.iter().fold(0, |acc, x| acc + x);
		let sum2 = other.powers.iter().fold(0, |acc, x| acc + x);
		if sum1 > sum2 {
			return Some(Ordering::Greater)
		}
		if sum1 < sum2 {
			return Some(Ordering::Less)
		}
		return Some(Ordering::Equal)
	}
}

/// The oredring derives directly from the `PartialOrd`
/// # Panics
/// If the partial ordering returns `None`
impl Ord for SymMonomial{
	fn cmp(&self, other: &Self) -> Ordering{
		match self.partial_cmp(other){
			Some(ordering) => ordering,
			None => panic!("Comparing monomials with different number of variables.")
		}
	}
}

impl Display for SymMonomial {
	fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
		let powers = self.powers.iter().map(|p| p.to_string()).collect::<Vec<String>>().connect(", ");
		write!(f, "{}*[{}]", self.coefficient, powers)
	}
}

/// A symbolic polynomial over the integers.
#[derive(Clone, Debug)]
pub struct SymPolynomial{
	vars: usize,
	pub monomials: Vec<SymMonomial>
}

/// The default implementation creates a new polynoimal using the default_variables. 
impl Default for SymPolynomial{
	fn default() -> Self{ 
		return SymPolynomial::new(DEFAULT_VARIABLES)
	}
}

impl SymPolynomial{
	/// Checks if the monomial is a constant
	pub fn is_const(&self) -> bool {
		for monomial in self.monomials.iter(){
			if !monomial.is_const() {
				return false
			}
		}
		return true
	}

	/// Createse a new symbolic polynomial with the given number of variables
	pub fn new(vars: usize) -> Self{
		SymPolynomial{vars: vars, monomials: Vec::<SymMonomial>::new()}
	}

	/// Returns a symolic polynomial representing the `var` variable in order to the power 1.
	pub fn get_first_order(var: usize, vars: usize) -> Self{
		if var >= vars{
			panic!("Asking for a first order variable {}, when the polynomial is defined for only {} variables", var, vars)
		}
		let mut monomial = SymMonomial::new(vars);
		monomial.powers[var] = 1;
		SymPolynomial{vars: vars, monomials: vec![monomial]}
	}

	/// Returns a symolic polynomial representing the constant value
	pub fn get_constant(value: i32, vars:usize) -> Self{
		let mut monomial = SymMonomial::new(vars);
		monomial.coefficient = value;
		SymPolynomial{vars: vars, monomials: vec![monomial]}
	}

	/// The method "simplifies" the polynomial to its minimal form.	
	/// It combines monomials which have the same powers of each symbolic variables and 
	/// removing any monomials which are equal to 0. Before you perform any methods,
	/// such as addition, mutliplication and etc. you should always call simplify.
	/// # Examples
	/// ```
	/// # use meta_diff::core::symbolic::*;
	/// // simplify(2x^2y^3 + x^2y^3) = 3x^2y^3
	/// let mut mon1 = SymMonomial::new(2);
	/// mon1.coefficient = 2;
	/// mon1.powers[0] = 2;
	/// mon1.powers[1] = 3;
	/// let mut mon2 = SymMonomial::new(2);
	/// mon2.powers[0] = 2;
	/// mon2.powers[1] = 3;
	/// let mut mon3 = SymMonomial::new(2);
	/// mon3.coefficient = 0;
	/// let mut poly = SymPolynomial::new(2);
	/// poly.monomials.push(mon1);
	/// poly.monomials.push(mon2);
	/// poly.monomials.push(mon3);
	/// poly.simplify();
	/// assert!(poly.monomials.len() == 1);
	/// assert!(poly.monomials[0].coefficient == 3);
	/// assert!(poly.monomials[0].powers.len() == 2);
	/// assert!(poly.monomials[0].powers[0] == 2);
	/// assert!(poly.monomials[0].powers[1] == 3);
	/// ```
	pub fn simplify(&mut self) {
		let mut consumed = vec![false; self.monomials.len()];
		for i in 0..self.monomials.len(){
			for j in (i+1)..self.monomials.len(){
				if !consumed[j] && (self.monomials[i] == self.monomials[j]) {
					self.monomials[i].coefficient += self.monomials[j].coefficient;
					consumed[j] = true;
				}			
			}
		}
		for i in (0..consumed.len()).rev(){
			if consumed[i] {
				self.monomials.remove(i);
			}
		}
		for i in (0..self.monomials.len()).rev(){
			if self.monomials[i].coefficient == 0 {
				self.monomials.remove(i);
			}
		}
		// self.monomials.sort();
	}
}

/// Two polynomials are considered equal only if their monomials are equal (including the coefficients).
/// This is a slight disctinction between the monomials equality. 
/// # Panics
/// If the number of symbolic variables in the two polynomials is different
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// //-5x^2 == 2x^2 for a monomials, but -5x^2 != 2x^2 for a polynomial
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// mon1.coefficient = -5;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 2;
/// mon2.coefficient = 2;
/// assert!(mon1 == mon2);
/// let mut poly1 = SymPolynomial::new(2);
/// poly1.monomials.push(mon1);
/// let mut poly2 = SymPolynomial::new(2);
/// poly2.monomials.push(mon2);
/// assert!(poly1 != poly2);
/// ```
/// ```
/// # use meta_diff::core::symbolic::SymMonomial;
/// //x^2y + 1 != x^2 + 1
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// mon1.powers[1] = 1;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 1;
/// mon2.powers[1] = 2;
/// // assert!(mon1 + 1 != mon2 + 1);
/// ```
impl PartialEq for SymPolynomial{
	fn eq(&self, other: &SymPolynomial) -> bool{
		if self.vars != other.vars {
			panic!("Comparing two polynomials with different number of symbolic variables - {} and {}", self.vars, other.vars)
		}
		if self.monomials.len() != other.monomials.len() {
			return false
		}
		for monomial in self.monomials.iter(){
			let mut is_common = false;
			for other_monomial in other.monomials.iter(){
				if monomial == other_monomial && monomial.coefficient == other_monomial.coefficient {
					is_common = true;
					break;
				}
			}
			if !is_common {
				return false
			}
		}
		return true
	}
}

/// Directly uses `PartialEq`
impl Eq for SymPolynomial{}


impl Display for SymPolynomial {
	fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
		if self.monomials.len() == 0 {
			return write!(f, "{}",0)
		}
		for monomial in self.monomials.iter(){
			let current =  write!(f, "{}+", monomial);
			if !current.is_ok() {
				return current
			}
		}
		return Ok(())
	}
}

impl<'a> Neg for &'a SymMonomial{
	type Output = SymMonomial;

	fn neg(self) -> Self::Output{
		return SymMonomial{vars: self.vars, coefficient: -self.coefficient, powers: self.powers.clone()}
	}
}

impl<'a> Neg for &'a SymPolynomial{
	type Output = SymPolynomial;

	fn neg(self) -> Self::Output{
		return SymPolynomial{vars: self.vars, monomials: self.monomials.iter().map(|x| -x).collect::<Vec<SymMonomial>>()}
	}
}

/// # Examples
/// # Panics
/// If the number of symbolic variables in the two monomials is different
/// ```
/// # use meta_diff::core::symbolic::*;
/// // x + x^2 = x^2 + x
/// let mut mon1 = SymMonomial::new(1);
/// mon1.powers[0] = 1;
/// let mut mon2 = SymMonomial::new(1);
/// mon2.powers[0] = 2;
/// let poly = &mon1 + &mon2;
/// assert!(poly.monomials.len() == 2);
/// assert!(poly.monomials[0].powers[0] == 1);
/// assert!(poly.monomials[1].powers[0] == 2);
/// ```
impl<'a, 'b> Add<&'b SymMonomial> for &'a SymMonomial{
	type Output = SymPolynomial;

	fn add(self, rhs: &'b SymMonomial) -> Self::Output{
		if self.vars != rhs.vars {
			panic!("Adding two monomials with different number of symbolic variables - {} and {}", self.vars, rhs.vars)
		}
		let mut result = SymPolynomial{vars: self.vars, monomials: Vec::with_capacity(2)};
		result.monomials.push(self.clone());
		result.monomials.push(rhs.clone());
		result.simplify();
		return result
	}
}

/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (x + 1) + 1 = x + 2
/// let mut mon1 = SymMonomial::new(1);
/// mon1.powers[0] = 1;
/// let poly = &mon1 + 1;
/// assert!(poly.monomials.len() == 2);
/// assert!(poly.monomials[0].powers[0] == 1);
/// assert!(poly.monomials[1].powers[0] == 0);
/// assert!(poly.monomials[1].is_const());
/// ```
impl<'a> Add<i32> for &'a SymMonomial{
	type Output = SymPolynomial;

	fn add(self, rhs: i32) -> Self::Output{
		let mut result = SymPolynomial{vars: self.vars, monomials: Vec::with_capacity(2)};
		if self.is_const() {
			result.monomials.push(self.clone());
			result.monomials[0].coefficient += rhs;
		}
		else{
			result.monomials.push(self.clone());
			result.monomials.push(SymMonomial::new(self.vars));
			result.monomials[1].coefficient = rhs;
		}
		result.simplify();
		return result
	}
}

impl<'a> Add<&'a SymMonomial> for i32{
	type Output = SymPolynomial;

	fn add(self, rhs: &'a SymMonomial) -> Self::Output{
		return rhs + self;
	}
}

/// # Panics
/// If the number of symbolic variables in the monomial and polynomial are different
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (x + 1) + x = 2x + 1
/// let mut mon1 = SymMonomial::new(1);
/// mon1.powers[0] = 1;
/// let poly = &mon1 + 1;
/// let poly2 = &poly + &mon1;
/// assert!(poly2.monomials.len() == 2);
/// assert!(poly2.monomials[0].powers[0] == 1);
/// assert!(poly2.monomials[0].coefficient == 2);
/// assert!(poly2.monomials[1].is_const());
/// assert!(poly2.monomials[1].coefficient == 1);
/// ```
impl<'a, 'b> Add<&'b SymMonomial> for &'a SymPolynomial{
	type Output = SymPolynomial;

	fn add(self, rhs: &'b SymMonomial) -> Self::Output{
		if self.vars != rhs.vars {
			panic!("Adding two monomials with different number of symbolic variables - {} and {}", self.vars, rhs.vars)
		}
		let mut result = SymPolynomial{vars: self.vars,monomials: Vec::with_capacity(self.monomials.len() + 1)};
		result.monomials.push(rhs.clone());
		result.monomials.extend(self.monomials.iter().cloned());
		result.simplify();
		return result
	}
}


/// # Panics
/// If the number of symbolic variables in the monomial and polynomial are different
impl<'a, 'b> Add<&'b SymPolynomial> for &'a SymMonomial{
	type Output = SymPolynomial;

	fn add(self, rhs: &'b SymPolynomial) -> Self::Output{
		return rhs + self;
	}
}
/// # Panics
/// If the number of symbolic variables in the two polynomials are different
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (x+1) + (y+2) = x + y + 3
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 1;
/// let poly1 = &mon1 + 1;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[1] = 1;
/// let poly2 = &mon2 + 2;
/// let polySum = &poly1 + &poly2;
/// assert!(polySum.monomials.len() == 3);
/// assert!(polySum.monomials[0].powers[0] == 1);
/// assert!(polySum.monomials[1].is_const());
/// assert!(polySum.monomials[1].coefficient == 3);
/// assert!(polySum.monomials[2].powers[1] == 1);
/// ```
impl<'a, 'b> Add<&'b SymPolynomial> for &'a SymPolynomial{
	type Output = SymPolynomial;

	fn add(self, rhs: &'b SymPolynomial) -> Self::Output{
		if self.vars != rhs.vars {
			panic!("Adding two monomials with different number of symbolic variables - {} and {}", self.vars, rhs.vars)
		}
		let mut result = SymPolynomial{vars: self.vars, monomials: Vec::with_capacity(self.monomials.len() + rhs.monomials.len())};
		result.monomials.extend(self.monomials.iter().cloned());
		result.monomials.extend(rhs.monomials.iter().cloned());
		result.simplify();
		return result
	}
}

/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (x + 1) + 2 = x + 3
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 1;
/// let poly1 = &mon1 + 1;
/// let poly_sum = &poly1 + 2;
/// assert!(poly_sum.monomials.len() == 2);
/// assert!(poly_sum.monomials[0].powers[0] == 1);
/// assert!(poly_sum.monomials[1].is_const());
/// assert!(poly_sum.monomials[1].coefficient == 3);
/// ```
impl<'a> Add<i32> for &'a SymPolynomial{
	type Output = SymPolynomial;

	fn add(self, rhs: i32) -> Self::Output{
		let mut result = SymPolynomial{vars: self.vars, monomials: Vec::with_capacity(self.monomials.len() + 1)};
		result.monomials.extend(self.monomials.iter().cloned());
		if self.monomials.len() == 0 {
			result.monomials.push(SymMonomial::new(DEFAULT_VARIABLES));
		}
		else{
			result.monomials.push(SymMonomial::new(self.monomials[0].vars));
		}
		result.monomials[self.monomials.len()].coefficient = rhs;
		result.simplify();
		return result
	}
}

impl<'a> Add<&'a SymPolynomial> for i32{
	type Output = SymPolynomial;

	fn add(self, rhs: &'a SymPolynomial) -> Self::Output{
		return rhs + self;
	}
}

/// # Panics
/// If the number of symbolic variables in the two monomials are different
impl<'a, 'b> Sub<&'b SymMonomial> for &'a SymMonomial{
	type Output = SymPolynomial;

	fn sub(self, rhs: &'b SymMonomial) -> Self::Output{
		return self + &(-rhs);
	}
}

impl<'a> Sub<i32> for &'a SymMonomial{
	type Output = SymPolynomial;

	fn sub(self, rhs: i32) -> Self::Output{
		return self + (-rhs);
	}
}

impl<'a> Sub<&'a SymMonomial> for i32{
	type Output = SymPolynomial;

	fn sub(self, rhs: &'a SymMonomial) -> Self::Output{
		return self + &(-rhs);
	}
}

/// # Panics
/// If the number of symbolic variables in the monomial and polynomial are different
impl<'a, 'b> Sub<&'b SymPolynomial> for &'a SymMonomial{
	type Output = SymPolynomial;

	fn sub(self, rhs: &'b SymPolynomial) -> Self::Output{
		return self + &(-rhs);
	}
}

/// # Panics
/// If the number of symbolic variables in the monomial and polynomial are different
impl<'a, 'b> Sub<&'b SymMonomial> for &'a SymPolynomial{
	type Output = SymPolynomial;

	fn sub(self, rhs: &'b SymMonomial) -> Self::Output{
		return self + &(-rhs);
	}
}

/// # Panics
/// If the number of symbolic variables in the two polynomials are different
impl<'a, 'b> Sub<&'b SymPolynomial> for &'a SymPolynomial{
	type Output = SymPolynomial;

	fn sub(self, rhs: &'b SymPolynomial) -> Self::Output{
		return self + &(-rhs);
	}
}

impl<'a> Sub<i32> for &'a SymPolynomial{
	type Output = SymPolynomial;

	fn sub(self, rhs: i32) -> Self::Output{
		return self + (-rhs);
	}
}

impl<'a> Sub<&'a SymPolynomial> for i32{
	type Output = SymPolynomial;

	fn sub(self, rhs: &'a SymPolynomial) -> Self::Output{
		return self + &(-rhs);
	}
}

/// # Panics
/// If the number of symbolic variables in the two monomials are different
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // 2xy * y^2 = 2xy^3
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 1;
/// mon1.powers[1] = 1;
/// mon1.coefficient = 2;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[1] = 2;
/// let mon_prod = &mon1 * &mon2;
/// assert!(mon_prod.coefficient == 2);
/// assert!(mon_prod.powers[0] == 1);
/// assert!(mon_prod.powers[1] == 3);
/// ```
impl<'a, 'b> Mul<&'b SymMonomial> for &'a SymMonomial{
	type Output = SymMonomial;

	fn mul(self, rhs:  &'b SymMonomial) -> Self::Output{
		if self.vars != rhs.vars {
			panic!("Multiplying monomials with different number of variables.")
		}
		let powers = self.powers.iter().zip(rhs.powers.iter()).map(|s| {let (a,b) = s; a+b}).collect::<Vec<usize>>();
		SymMonomial{vars: self.vars, coefficient: self.coefficient * rhs.coefficient, powers: powers}
	}
}

/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// //2xy * 3 = 6xy
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 1;
/// mon1.powers[1] = 1;
/// mon1.coefficient = 2;
/// let mon_prod = &mon1 * 3;
/// assert!(mon_prod.coefficient == 6);
/// assert!(mon_prod.powers[0] == 1);
/// assert!(mon_prod.powers[1] == 1);
/// ```
impl<'a> Mul<i32> for &'a SymMonomial{
	type Output = SymMonomial;

	fn mul(self, rhs:  i32) -> Self::Output{
		return SymMonomial{vars: self.vars, coefficient: self.coefficient * rhs, powers: self.powers.clone()}
	}
}

impl<'a> Mul<&'a SymMonomial> for i32{
	type Output = SymMonomial;

	fn mul(self, rhs:  &'a SymMonomial) -> Self::Output{
		return rhs * self;
	}
}

/// # Panics
/// If the number of symbolic variables in the monomial and polynomial are different
impl<'a, 'b> Mul<&'b SymPolynomial> for &'a SymMonomial{
	type Output = SymPolynomial;

	fn mul(self, rhs: &'b SymPolynomial) -> Self::Output{
		return rhs * self;
	}
}

/// # Panics
/// If the number of symbolic variables in the monomial and polynomial are different
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (2xy+1) * 3y^2 = 6xy^3 + 3y^2
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 1;
/// mon1.powers[1] = 1;
/// mon1.coefficient = 2;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[1] = 2;
/// mon2.coefficient = 3;
/// let poly = &mon1 + 1;
/// let poly_prod = &poly * &mon2;
/// assert!(poly_prod.monomials.len() == 2);
/// assert!(poly_prod.monomials[0].coefficient == 6);
/// assert!(poly_prod.monomials[0].powers[0] == 1);
/// assert!(poly_prod.monomials[0].powers[1] == 3);
/// assert!(poly_prod.monomials[1].coefficient == 3);
/// assert!(poly_prod.monomials[1].powers[0] == 0);
/// assert!(poly_prod.monomials[1].powers[1] == 2);
/// ```
impl<'a, 'b> Mul<&'b SymMonomial> for &'a SymPolynomial{
	type Output = SymPolynomial;

	fn mul(self, rhs: &'b SymMonomial) -> Self::Output{
		if self.vars != rhs.vars {
			panic!("Adding two monomials with different number of symbolic variables - {} and {}", self.vars, rhs.vars)
		}
		let mut result = SymPolynomial{vars: self.vars, monomials: Vec::with_capacity(self.monomials.len())};
		for my_monomial in self.monomials.iter(){
			result.monomials.push(my_monomial * rhs);
		}
		result.simplify();
		return result
	}
}

/// # Panics
/// If the number of symbolic variables in the two polynomials are different
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (2xy+1) * (3y^2 + 2) = 6xy^3 + 4xy + 3y^2 + 2
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 1;
/// mon1.powers[1] = 1;
/// mon1.coefficient = 2;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[1] = 2;
/// mon2.coefficient = 3;
/// let poly1 = &mon1 + 1;
/// let poly2 = &mon2 + 2;
/// let poly_prod = &poly1 * &poly2;
/// assert!(poly_prod.monomials.len() == 4);
/// assert!(poly_prod.monomials[0].coefficient == 6);
/// assert!(poly_prod.monomials[0].powers[0] == 1);
/// assert!(poly_prod.monomials[0].powers[1] == 3);
/// assert!(poly_prod.monomials[1].coefficient == 4);
/// assert!(poly_prod.monomials[1].powers[0] == 1);
/// assert!(poly_prod.monomials[1].powers[1] == 1);
/// assert!(poly_prod.monomials[2].coefficient == 3);
/// assert!(poly_prod.monomials[2].powers[0] == 0);
/// assert!(poly_prod.monomials[2].powers[1] == 2);
/// assert!(poly_prod.monomials[3].coefficient == 2);
/// assert!(poly_prod.monomials[3].is_const());
/// ```
impl<'a, 'b> Mul<&'b SymPolynomial> for &'a SymPolynomial{
	type Output = SymPolynomial;

	fn mul(self, rhs: &'b SymPolynomial) -> Self::Output{
		if self.vars != rhs.vars {
			panic!("Adding two monomials with different number of symbolic variables - {} and {}", self.vars, rhs.vars)
		}
		let mut result = SymPolynomial{vars: self.vars, monomials: Vec::with_capacity(self.monomials.len() * rhs.monomials.len())};
		for my_monomial in self.monomials.iter(){
			for other_monomial in rhs.monomials.iter(){
				result.monomials.push(my_monomial * other_monomial);
			}
		}
		result.simplify();
		return result
	}
}

impl<'a> Mul<i32> for &'a SymPolynomial{
	type Output = SymPolynomial;

	fn mul(self, rhs: i32) -> Self::Output{
		let mut result = SymPolynomial{vars: self.vars, 
			monomials: self.monomials.iter().map(|x| x * rhs).collect::<Vec<SymMonomial>>()};
			result.simplify();
			return result
		}
	}

	impl<'a> Mul<&'a SymPolynomial> for i32{
		type Output = SymPolynomial;

		fn mul(self, rhs: &'a SymPolynomial) -> Self::Output{
			return rhs * self;
		}
	}

/// Returns None if the monomials are non divisible.
/// # Panics
/// If the number of symbolic variables in the two monomials are different
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // 4x^2y^3 / 2xy = 2xy^2
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// mon1.powers[1] = 3;
/// mon1.coefficient = 4;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 1;
/// mon2.powers[1] = 1;
/// mon2.coefficient = 2;
/// let option = &mon1 / &mon2;
/// assert!(option.is_some());
/// let poly = option.unwrap();
/// assert!(poly.coefficient == 2);
/// assert!(poly.powers[0] == 1);
/// assert!(poly.powers[1] == 2);
/// ```
/// ```
/// # use meta_diff::core::symbolic::*;
/// // 2x^2y^3 / 4xy = None
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// mon1.powers[1] = 3;
/// mon1.coefficient = 2;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 1;
/// mon2.powers[1] = 4;
/// mon2.coefficient = 2;
/// let option = &mon1 / &mon2;
/// assert!(option.is_none());
/// ```
/// ```
/// # use meta_diff::core::symbolic::*;
/// // 2x^2y^3 / 4x^3y = None
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// mon1.powers[1] = 3;
/// mon1.coefficient = 4;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 3;
/// mon2.powers[1] = 2;
/// mon2.coefficient = 2;
/// let option = &mon1 / &mon2;
/// assert!(option.is_none());
/// ```
impl<'a, 'b> Div<&'b SymMonomial> for &'a SymMonomial{
	type Output = Option<SymMonomial>;

	fn div(self, rhs:  &'b SymMonomial) -> Self::Output{
		if self.vars != rhs.vars {
			panic!("Multiplying monomials with different number of variables.")
		}
		if rhs.coefficient == 0 || self.coefficient % rhs.coefficient != 0 {
			return None
		}
		let mut result = SymMonomial{vars: self.vars, coefficient: self.coefficient / rhs.coefficient, powers: Vec::with_capacity(self.vars)};
		for (my_power, other_power) in self.powers.iter().zip(rhs.powers.iter()){
			if my_power < other_power {
				return None
			}
			result.powers.push(my_power - other_power);
		}
		return Some(result)
	}
}

/// Returns None if the monomial is not divisible by the integer
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // 4x^2y^3 / 2 = 2x^2y^3
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// mon1.powers[1] = 3;
/// mon1.coefficient = 4;
/// let option = &mon1 / 2;
/// assert!(option.is_some());
/// let poly = option.unwrap();
/// assert!(poly.coefficient == 2);
/// assert!(poly.powers[0] == 2);
/// assert!(poly.powers[1] == 3);
/// ```
/// ```
/// # use meta_diff::core::symbolic::*;
/// // 2x^2y^3 / 3 = None
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// mon1.powers[1] = 3;
/// mon1.coefficient = 2;
/// let option = &mon1 / 3;
/// assert!(option.is_none());
/// ```
impl<'a> Div<i32> for &'a SymMonomial{
	type Output = Option<SymMonomial>;

	fn div(self, rhs:  i32) -> Self::Output{
		if rhs == 0 || self.coefficient % rhs != 0 {
			return None
		}
		return Some(SymMonomial{vars: self.vars, coefficient: self.coefficient / rhs, powers: self.powers.clone()})
	}
}

/// Returns None if the polynomial is not divisible by the monomial
/// # Panics
/// If the number of symbolic variables in the monomial and polynomial are different
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (2x^2y + x^3y + xy) / xy = 2x + x^2 + 1
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 1;
/// mon1.powers[1] = 1;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 1;
/// mon2.coefficient = 2;
/// let mut mon3 = SymMonomial::new(2);
/// mon3.powers[0] = 2;
/// let poly1 = &(&mon2 + &mon3) + 1;
/// let poly_prod = &poly1 * &mon1;
/// let option = &poly_prod / &mon1;
/// assert!(option.is_some());
/// assert!(option.unwrap()== poly1);
/// ```
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (2x^2y + x^3y + xy + 1) / xy = None
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 1;
/// mon1.powers[1] = 1;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[0] = 1;
/// mon2.coefficient = 2;
/// let mut mon3 = SymMonomial::new(2);
/// mon3.powers[0] = 2;
/// let poly1 = &(&mon2 + &mon3) + 1;
/// let poly_prod = &(&poly1 * &mon1) + 1;
/// let option = &poly_prod / &mon1;
/// assert!(option.is_none());
/// ```
impl<'a, 'b> Div<&'b SymMonomial> for &'a SymPolynomial{
	type Output = Option<SymPolynomial>;

	fn div(self, rhs: &'b SymMonomial) -> Self::Output{
		if self.vars != rhs.vars {
			panic!("Adding two monomials with different number of symbolic variables - {} and {}", self.vars, rhs.vars)
		}
		if rhs.coefficient == 0 {
			return None
		}
		if self.monomials.len() == 0 {
			return Some(SymPolynomial::new(self.vars));
		}
		let mut result = SymPolynomial{vars: self.vars, monomials: Vec::with_capacity(self.monomials.len())};
		for monomial in self.monomials.iter(){
			match monomial / rhs {
				Some(valid) => result.monomials.push(valid),
				None => return None
			}
		}
		result.simplify();
		return Some(result)
	}
}

/// Returns None if the polynomial is not divisible by the other polynomial
/// # Panics
/// If the number of symbolic variables in the two polynomials are different
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (2x^4 + x^3y + 3x^2y + 2x^2 + xy^2 + xy + y^2 + y) / (x^2 + y + 1) = 2x^2 + y + xy
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[1] = 1;
/// let mut mon3 = SymMonomial::new(2);
/// mon3.powers[0] = 1;
/// mon3.powers[1] = 1;
/// let poly_divisor = &mon1 + &(&mon2 +1);
/// let poly_result = &(2 * &mon1) + &(&mon2 + &mon3);
/// let poly_prod = &poly_divisor * &poly_result;
/// let option = &poly_prod / &poly_divisor;
/// assert!(option.is_some());
/// assert!(option.unwrap()== poly_result);
/// ```
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (2x^4 + x^3y + 3x^2y + 2x^2 + xy^2 + xy + y^2 + y) / (x^2 + y + 2) = None
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[1] = 1;
/// let mut mon3 = SymMonomial::new(2);
/// mon3.powers[0] = 1;
/// mon3.powers[1] = 1;
/// let poly_divisor = &mon1 + &(&mon2 +1);
/// let poly_result = &(2 * &mon1) + &(&mon2 + &mon3);
/// let poly_prod = &poly_divisor * &poly_result;
/// let poly_newdivisor = &poly_divisor + 1;
/// let option = &poly_prod / &poly_newdivisor;
/// assert!(option.is_none());
/// ```
impl<'a, 'b> Div<&'b SymPolynomial> for &'a SymPolynomial{
	type Output = Option<SymPolynomial>;

	fn div(self, rhs: &'b SymPolynomial) -> Self::Output{
		if self.vars != rhs.vars {
			panic!("Adding two monomials with different number of symbolic variables - {} and {}", self.vars, rhs.vars)
		}
		if rhs.monomials.len() == 0 {
			return None
		}
		if self.monomials.len() == 0 {
			return Some(SymPolynomial::new(self.vars));
		}
		let mut result = SymPolynomial{vars: self.vars, monomials: Vec::with_capacity(self.monomials.len())};
		let mut remainder = self.clone();
		let mut main_divisor = &rhs.monomials[0];
		for monomial in rhs.monomials.iter(){
			if monomial > main_divisor {
				main_divisor = monomial;
			}
		}
		let mut some_division = true;
		while remainder.monomials.len() > 0 {
			if !some_division {
				return None
			}
			some_division = false;
			for monomial in remainder.monomials.iter(){
				match monomial / main_divisor {
					Some(mon) => {
						result.monomials.push(mon);
						some_division = true;
					},
					None => {}
				}
			}
			result.simplify();
			remainder = self - &(&result * rhs);
		}
		return Some(result)
	}
}

/// Returns None if the polynomial is not divisible by the other polynomial
/// # Examples
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (2x^2 + 2xy) / 2 = x^2 + xy
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[1] = 1;
/// mon2.powers[1] = 1;
/// let poly_result = &mon1 + &mon2;
/// let poly_prod = 2 * &poly_result;
/// let option = &poly_prod / 2;
/// assert!(option.is_some());
/// assert!(option.unwrap()== poly_result);
/// ```
/// ```
/// # use meta_diff::core::symbolic::*;
/// // (2x^2 + 2xy + 1) / 2 = None
/// let mut mon1 = SymMonomial::new(2);
/// mon1.powers[0] = 2;
/// let mut mon2 = SymMonomial::new(2);
/// mon2.powers[1] = 1;
/// mon2.powers[1] = 1;
/// let poly_result = &mon1 + &mon2;
/// let poly_prod = &(2 * &poly_result) + 1;
/// let option = &poly_prod / 2;
/// assert!(option.is_none());
/// ```
impl<'a> Div<i32> for &'a SymPolynomial{
	type Output = Option<SymPolynomial>;

	fn div(self, rhs: i32) -> Self::Output{
		if rhs == 0 {
			return None
		}
		if self.monomials.len() == 0 {
			return Some(SymPolynomial::new(self.vars));
		}
		let mut result = SymPolynomial{vars: self.vars, monomials: Vec::with_capacity(self.monomials.len())};
		for monomial in self.monomials.iter(){
			match monomial / rhs {
				Some(valid) => result.monomials.push(valid),
				None => return None
			}
		}
		result.simplify();
		return Some(result)
	}
}