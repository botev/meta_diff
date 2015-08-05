use std::collections::{HashMap, HashSet};
use super::operator::*;
use super::node::*;
use super::graph::*;

#[context]
graph_res -> Result<ComputeGraph, ParseError> = {Ok(ComputeGraph::new())}
#[context]
variable_table -> HashMap<String,usize> = {HashMap::new()}

// ==================================================================
// PARSER RULES
//

#[pub]
metaFile -> Result<ComputeGraph, ParseError> = functionDefinition {graph_res.clone()}

/// Set the name of the graph add all outputs and set target to the first output
functionDefinition = (eol / __)* FUNCTION __ outputs: functionReturn __? EQ __? main:ID __? mainParamList eol statementList END (eol / __)* {
	let mut result : Result<(),ParseError> = Ok(());
	match *graph_res{
		Ok(ref mut graph) => {
			graph.name = main;
			for output in outputs.iter(){
				match variable_table.get(output){
					Some(id) => {
						// println!("Target {} is {}", output, *id);
						graph.outputs.push(*id);
					},
					None => {
						result = result_err!(input, state,
						format!("Output variable \'{}\' has not been defined", output));
						// println!("{:?}", result);
						break;
					}
				}
			}
		},
		Err(ref msg) => {result = Err(msg.clone());}
	}
	match result {
		Ok(_) => (),
		Err(msg) => {*graph_res = Err(msg);}
	}
}

/// Return all of the outputs names
functionReturn -> Vec<String> =  LSBRACE __? ids: ID ++ (__? COMMA) __? RSBRACE {ids}

/// Only match pattern
mainParamList = LPAREN __? inputVar ++ (__? COMMA) __? RPAREN

/// Add all of the inputs to the variable table and in to the graph
inputVar = param: AT? name:ID {
	if ComputeGraph::is_function_name(&name) {
		*graph_res = result_err!(input, state,
					format!("Can not have a variable with name \'{}\' since it is a built in function", name))
	}
	else{
		match *graph_res{
			Ok(ref mut graph) => match param {
				Some(_) => {variable_table.insert(name.clone(),graph.add_parameter(name));()},
				None => {variable_table.insert(name.clone(),graph.add_const_input(name));()}
			},
			Err(_) => (),
		}
	}
}

/// Only match pattern
statementList = ((eol / comment/ __)* statement)* (eol / comment/ __)*

/// Insert the variable on the left to represent the graph node of the expression on the right
statement = name: ID __? EQ __? id:expression __? SEMI {
	let result = match *graph_res{
		Ok(ref mut graph) => {
			if ComputeGraph::is_function_name(&name) {
				result_err!(input, state,
					format!("Can not have a variable with name \'{}\' since it is a built in function", name))
			}
			else{
				variable_table.insert(name,id);
				Ok(())
			}
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg);}
	}
}

/// Logical operators
g1 -> OperatorType =
	GTE {OPERATOR_GTE}
	/ GT {OPERATOR_GT}
	/ LTE {OPERATOR_LTE}
	/ LT {OPERATOR_LT}
 	/ NEQ {OPERATOR_NEQ}
	/ DOUBLE_EQ {OPERATOR_EQ}

/// Plus or Minus operators
g2	-> bool
	= PLUS {true}
	/ MINUS {false}

/// Multiplication and division operators
g3	-> bool
	= TIMES {true}
	/ DIVISION {false}

/// Currently the logical operators are not supported
expression	-> usize = first: e1  second:(__? op:g1 __? var:expression{(op,var)})? {
	let result = match *graph_res{
		Ok(ref mut graph) => match second {
			Some(tuple) => match graph.add_operation(tuple.0, vec![first, tuple.1]) {
				Ok(var) => Ok(var),
				Err(err) => result_err!(input, state, format!("{}",err))
			},
			None => Ok(first)
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Addition or subtraction, however subtraction in the graph is represented as
/// addition and unary negation
e1	-> usize = first: e2 rest:( __? op:g2 __? var:e2 {
	let result = match *graph_res{
		Ok(ref mut graph) => {
			if op {
				Ok(var)
			} else {
				match graph.add_operation(OPERATOR_NEG, vec![var]) {
					Ok(var) => Ok(var),
					Err(err) => result_err!(input, state, format!("{}",err))
				}
			}
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
})* {
	let result = match *graph_res{
		Ok(ref mut graph) => match rest.len() {
			0 => Ok(first),
			_ => {
				let mut vars = vec![first];
				vars.extend(rest);
				match graph.add_operation(OPERATOR_ADD, vars) {
					Ok(var) => Ok(var),
					Err(err) => result_err!(input, state, format!("{}",err))
				}
			}
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Multiplication and division, however division in the graph is represented as
/// multiplication and unary division
e2	-> usize = first: e3 rest:( __? op:g3 __? var:e3  {
	let result = match *graph_res{
		Ok(ref mut graph) => {
			if op {
				Ok(var)
			} else {
				match graph.add_operation(OPERATOR_DIV, vec![var]) {
					Ok(var) => Ok(var),
					Err(err) => result_err!(input, state, format!("{}",err))
				}
			}
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
})* {
	let result = match *graph_res{
		Ok(ref mut graph) => match rest.len() {
			0 => Ok(first),
			_ => {
				let mut vars = vec![first];
				vars.extend(rest);
				match graph.add_operation(OPERATOR_MUL, vars) {
					Ok(var) => Ok(var),
					Err(err) => result_err!(input, state, format!("{}",err))
				}
			}
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Matrix multiplication - having higher precedence than normal
e3	-> usize = vars: e4 ++ (__? DOT_PRODUCT __?) {
	let result = match *graph_res{
		Ok(ref mut graph) => match vars.len() {
			0 => unreachable!(),
			1 => Ok(vars[0]),
			_ => match graph.add_operation(OPERATOR_DOT, vars) {
				Ok(var) => Ok(var),
				Err(err) => result_err!(input, state, format!("{}",err))
			}
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Unary negation
e4	-> usize = m:MINUS? __? var: e5 {
	let result = match *graph_res{
		Ok(ref mut graph) => match m {
			Some(_) => match graph.add_operation(OPERATOR_NEG,vec![var]) {
				Ok(var) => Ok(var),
				Err(err) => result_err!(input, state, format!("{}",err))
			},
			None => Ok(var)
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Powering one value by another
e5  -> usize = first: e6 second: (__? EXP __? var:e6 {var})? {
	let result = match *graph_res{
		Ok(ref mut graph) => match second{
			Some(id) => match graph.add_operation(OPERATOR_POW,vec![first,id]) {
				Ok(var) => Ok(var),
				Err(err) => result_err!(input, state, format!("{}",err))
			},
			None => Ok(first)
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Transpose
e6	-> usize = var: unaryExpression tr: TRANSPOSE? {
	let mut result = match *graph_res{
		Ok(ref mut graph) => match tr{
			Some(_) => match graph.add_operation(OPERATOR_TRANSPOSE,vec![var]) {
				Ok(var) => Ok(var),
				Err(err) => result_err!(input, state, format!("{}",err))
			},
			None => Ok(var)
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Unary expression
unaryExpression -> usize =  baseExpression / (LPAREN __? e:expression __? RPAREN {e})

/// Only if this is an ID we have to check if it is in the variable table
baseExpression -> usize = NUMBER /  indexedVar / varDotFunc / funcCall / name:ID {
	let result = match *graph_res{
		Ok(ref mut graph) => match variable_table.get(&name) {
			Some(id) => Ok(*id),
			None => result_err!(input, state, format!("Use of undefined variable \'{}\'", name))
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Index a variable - var[arg1,arg2,arg3,arg4]
indexedVar -> usize =  name: ID LSBRACE __? arg1: expression __? COMMA __? arg2:expression __? COMMA __?
arg3:expression __? COMMA __? arg4:expression __? RSBRACE {
	let result = match *graph_res{
		Ok(ref mut graph) => match variable_table.get(&name) {
			Some(id) => match graph.add_operation(
				OPERATOR_SUBINDEX,vec![*id,arg1,arg2,arg3,arg4]) {
				Ok(var) => Ok(var),
				Err(err) => result_err!(input, state, format!("{}",err))
			},
			None => result_err!(input, state, format!("Use of undefined variable \'{}\'", name))
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Call on a variable an unary function - var.func(args)
varDotFunc -> usize = name:ID DOT func:ID args:paramList {
	let result = match *graph_res{
		Ok(ref mut graph) => match variable_table.get(&name){
			Some(id) => {
				let mut newargs = args.clone();
				newargs.insert(0,*id);
				if ComputeGraph::is_function_name(&func) {
					match graph.string_to_operator(func, newargs) {
						Ok(var) => Ok(var),
						Err(err) => result_err!(input, state, format!("{}",err))
					}
				} else {
					result_err!(input, state, format!("Use of undefined function \'{}\'", func))
				}
			},
			None => result_err!(input, state, format!("Use of undefined variable \'{}\'", name))
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Function call
funcCall -> usize = func:ID args:paramList {
	let result = match *graph_res{
		Ok(ref mut graph) => {
			if ComputeGraph::is_function_name(&func) {
				match graph.string_to_operator(func, args) {
					Ok(var) => Ok(var),
					Err(err) => result_err!(input, state, format!("{}",err))
				}
			} else {
				result_err!(input, state, format!("Use of undefined function \'{}\'", func))
			}
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

/// Parameter list
paramList -> Vec<usize> =  LPAREN __? vars: expression ** (__? COMMA __?) __? RPAREN{
	vars
}

// ==================================================================
// LEXER RULES
//

//
// language keywords
//

// BREAK	= "break";
// CASE	= "case";
// CATCH	= "catch";
// CONTINUE= "continue";
// ELSE	= "else";
// ELSEIF	= "elseif";
END	= "end";
// FOR	= "for";
FUNCTION= "function";
// GLOBAL	= "global";
// IF	= "if";
// OTHERWISE= "otherwise";
// PERSISTENT= "persistent";
// RETURNS	= "return";
// SWITCH	= "switch";
// TRY	= "try";
// WHILE	= "while";
// CLEAR	= "clear";
// VARARGIN= "varargin";
// NARGIN= "nargin";
// VARARGOUT= "varargout";
// NARGOUT= "nargout";
// HOLDON = "hold on";
// PAUSE = "pause";
// PARAMS= "params";

// Operators and assignments
//


// // Binary - Not supported
// BIN_OR	= "|";
// BIN_AND	= "&";

// Logical - Not supported
// LOG_OR	= "||";
// LOG_AND	= "&&";

// Comparison - Not supported
DOUBLE_EQ	= "==";
NEQ	= "~=";
LTE = "<=";
GTE = ">=";
LT	= "<";
GT	= ">";
EQ	= "=";

// Standard arithmetic
PLUS	= "+";
MINUS	= "-";
TIMES	= "*";
DIVISION	= "/";
EXP	= "^";
TRANSPOSE	= "\'";
DOT_PRODUCT = "dot";

//
// Other useful language snippets
//

SEMI	= ";";
LPAREN	= "(";
	RPAREN	= ")";
LBRACE	= "{";
RBRACE	= "}";
LSBRACE	= "[";
RSBRACE	= "]";
AT	= "@";
DOT	= ".";
COMMA	= ",";


//
// Identifiers, numbers, strings, linecomennts and  whitespace
//

ID -> String = [a-zA-Z] [a-zA-Z0-9_]* {  match_str.to_string() }

NUMBER -> usize = [0-9]+ ('.' [0-9]+)? {
	let result  = match *graph_res{
		Ok(ref mut graph) => match match_str.parse::<i64>(){
			Ok(value) => Ok(graph.add_int(value)),
			Err(_) => match match_str.parse::<f64>(){
				Ok(value) => Ok(graph.add_float(value)),
				Err(_) => unreachable!()
			}
		},
		Err(ref msg) => Err(msg.clone())
	};
	match result {
		Ok(var) => var,
		Err(msg) => {*graph_res = Err(msg); 0}
	}
}

comment = "%" (!eolChar .)*

/* Modeled after ECMA-262, 5th ed., 7.3. */
eol
= "\n"
/ "\r\n"
/ "\r"
/ "\u{2028}"
/ "\u{2029}"

eolChar
= [\n\r\u2028\u2029]

/* Modeled after ECMA-262, 5th ed., 7.2.  - whitespace */
__  = [ \t\u{00A0}\u{FEFF}\u{1680}\u{180E}\u{2000}-\u{200A}\u{202F}\u{205F}\u{3000}] // \v\f removed


// pub fn metaFile<'input>(input: &'input str)
//  -> ParseResult<ComputeGraph> {
//     let mut state = ParseState::new();
//     match parse_metaFile(input, &mut state, 0) {
//         Matched(pos, value) => { if pos == input.len() { return value } }
//         _ => { }
//     }
//     let (line, col) = pos_to_line(input, state.max_err_pos);
//     Err(ParseError{line: line,
//                    column: col,
//                    offset: state.max_err_pos,
//                    expected: state.expected,
//                    msg: None,})
// }
//
//
// macro_rules! result_err {
// 	($input:ident , $state:ident , $msg:expr) => {
// 		{
// 			let (line, col) = pos_to_line($input, $state.max_err_pos);
// 			Err(ParseError{line: line, column: col, offset: $state.max_err_pos,
// 				expected: HashSet::new(), msg: Some($msg)})
// 		}
// 	};
// }
