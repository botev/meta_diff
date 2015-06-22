#[macro_use]

// #[macro_export]
// macro_rules! parametarise_test { 
// 	($func: ident, $([$name: ident, $($args: expr),*]),+) => {
// 		$(
// 			#[test]
// 			fn $name(){$func($($args),*);}
// 			)*
// 	};
// }

// #[macro_export]
// macro_rules! count_exprs {
//     () => { 0 };
//     ([ $($args: expr),* ]) => { 1 };
//     ([ $($args: expr),* ], $([ $($tail: expr),* ]),+) => { 1 + count_exprs!($([$($tail),*]),*) };
// }

// #[macro_export]
// macro_rules! test_sources {
// 	($func:ident, $n: expr) => {};
// 	($func:ident, $n: expr, [ $($args: expr),* ])  => {
// 		#[test]
// 		fn [test_ $n] (){super::$func($($args),*);}
// 	};
// 	($func:ident, $n: expr, [ $($args: expr),* ], $([ $($tail: expr),* ]),+)  => {
// 		#[test]
// 		fn [test_ $n] (){super::$func($($args),*);}
// 		test_sources!($func, $n - 1, $([$($tail),*]),*);
// 	};
// }

// #[macro_export]
// macro_rules! parametarise_t { 
// 	($func: ident, $([$($args: expr),*]),+) => {
// 	mod $func{
// 		test_sources!($func, count_exprs!($([$($args),*]),*), $([$($args),*]),*);
// 	}
// 	};
// }



#[macro_export]
macro_rules! result_err {
	($input:ident , $state:ident , $msg:expr) => {
		{
			let (line, col) = pos_to_line($input, $state.max_err_pos);	
			Err(ParseError{line: line, column: col, offset: $state.max_err_pos, 
				expected: HashSet::new(), msg: Some($msg)})
		}
	};
}

// #[macro_export]
// macro_rules! graph_match {
// 	($graph_res:ident, $res: ty, $def: expr, $code: block) => {
// 		{
// 		let mut result : Result<$res,ParseError> = Ok($def);
// 		match *$graph_res{
// 			Ok(ref mut graph) => $code,
// 			Err(ref msg) => {result = Err(msg.clone());}
// 		}
// 		match result {
// 			Ok(var) => var,
// 			Err(msg) => {*$graph_res = Err(msg); 0}
// 		}
// 	}
// 	};
// }

// #[macro_export]
// macro_rules! count_exprs {
//     () => { 0 };
//     ($e:expr) => { 1 };
//     ($e:expr, $($es:expr),+) => { 1 + count_exprs!($($es),*) };
// }

#[macro_export]
macro_rules! parametarise_test { 
	($func: ident, [$($args0: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
		}
	};
	($func: ident, [$($args0: expr),*], [$($args1: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
			#[test]
			fn test_1 (){super::$func($($args1),*);}
		}
	};
	($func: ident, [$($args0: expr),*], [$($args1: expr),*], [$($args2: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
			#[test]
			fn test_1 (){super::$func($($args1),*);}
			#[test]
			fn test_2 (){super::$func($($args2),*);}
		}
	};
	($func: ident, [$($args0: expr),*], [$($args1: expr),*], [$($args2: expr),*], [$($args3: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
			#[test]
			fn test_1 (){super::$func($($args1),*);}
			#[test]
			fn test_2 (){super::$func($($args2),*);}
			#[test]
			fn test_3 (){super::$func($($args3),*);}
		}
	};
	($func: ident, [$($args0: expr),*], [$($args1: expr),*], [$($args2: expr),*], [$($args3: expr),*], [$($args4: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
			#[test]
			fn test_1 (){super::$func($($args1),*);}
			#[test]
			fn test_2 (){super::$func($($args2),*);}
			#[test]
			fn test_3 (){super::$func($($args3),*);}
			#[test]
			fn test_4 (){super::$func($($args4),*);}
		}
	};
	($func: ident, [$($args0: expr),*], [$($args1: expr),*], [$($args2: expr),*], [$($args3: expr),*], [$($args4: expr),*], [$($args5: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
			#[test]
			fn test_1 (){super::$func($($args1),*);}
			#[test]
			fn test_2 (){super::$func($($args2),*);}
			#[test]
			fn test_3 (){super::$func($($args3),*);}
			#[test]
			fn test_4 (){super::$func($($args4),*);}
			#[test]
			fn test_5 (){super::$func($($args5),*);}
		}
	};
	($func: ident, [$($args0: expr),*], [$($args1: expr),*], [$($args2: expr),*], [$($args3: expr),*], [$($args4: expr),*], [$($args5: expr),*],
		[$($args6: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
			#[test]
			fn test_1 (){super::$func($($args1),*);}
			#[test]
			fn test_2 (){super::$func($($args2),*);}
			#[test]
			fn test_3 (){super::$func($($args3),*);}
			#[test]
			fn test_4 (){super::$func($($args4),*);}
			#[test]
			fn test_5 (){super::$func($($args5),*);}
			#[test]
			fn test_6 (){super::$func($($args5),*);}
		}
	};
	($func: ident, [$($args0: expr),*], [$($args1: expr),*], [$($args2: expr),*], [$($args3: expr),*], [$($args4: expr),*], [$($args5: expr),*],
		[$($args6: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
			#[test]
			fn test_1 (){super::$func($($args1),*);}
			#[test]
			fn test_2 (){super::$func($($args2),*);}
			#[test]
			fn test_3 (){super::$func($($args3),*);}
			#[test]
			fn test_4 (){super::$func($($args4),*);}
			#[test]
			fn test_5 (){super::$func($($args5),*);}
			#[test]
			fn test_6 (){super::$func($($args6),*);}
		}
	};
	($func: ident, [$($args0: expr),*], [$($args1: expr),*], [$($args2: expr),*], [$($args3: expr),*], [$($args4: expr),*], [$($args5: expr),*],
		[$($args6: expr),*], [$($args7: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
			#[test]
			fn test_1 (){super::$func($($args1),*);}
			#[test]
			fn test_2 (){super::$func($($args2),*);}
			#[test]
			fn test_3 (){super::$func($($args3),*);}
			#[test]
			fn test_4 (){super::$func($($args4),*);}
			#[test]
			fn test_5 (){super::$func($($args5),*);}
			#[test]
			fn test_6 (){super::$func($($args6),*);}
			#[test]
			fn test_7 (){super::$func($($args7),*);}
		}
	};
	($func: ident, [$($args0: expr),*], [$($args1: expr),*], [$($args2: expr),*], [$($args3: expr),*], [$($args4: expr),*], [$($args5: expr),*],
		[$($args6: expr),*], [$($args7: expr),*], , [$($args8: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
			#[test]
			fn test_1 (){super::$func($($args1),*);}
			#[test]
			fn test_2 (){super::$func($($args2),*);}
			#[test]
			fn test_3 (){super::$func($($args3),*);}
			#[test]
			fn test_4 (){super::$func($($args4),*);}
			#[test]
			fn test_5 (){super::$func($($args5),*);}
			#[test]
			fn test_6 (){super::$func($($args6),*);}
			#[test]
			fn test_7 (){super::$func($($args7),*);}
			#[test]
			fn test_8 (){super::$func($($args8),*);}
		}
	};
	($func: ident, [$($args0: expr),*], [$($args1: expr),*], [$($args2: expr),*], [$($args3: expr),*], [$($args4: expr),*], [$($args5: expr),*],
		[$($args6: expr),*], [$($args7: expr),*], , [$($args8: expr),*], [$($args9: expr),*]) => {
		mod $func{
			#[test]
			fn test_0 (){super::$func($($args0),*);}
			#[test]
			fn test_1 (){super::$func($($args1),*);}
			#[test]
			fn test_2 (){super::$func($($args2),*);}
			#[test]
			fn test_3 (){super::$func($($args3),*);}
			#[test]
			fn test_4 (){super::$func($($args4),*);}
			#[test]
			fn test_5 (){super::$func($($args5),*);}
			#[test]
			fn test_6 (){super::$func($($args6),*);}
			#[test]
			fn test_7 (){super::$func($($args7),*);}
			#[test]
			fn test_8 (){super::$func($($args8),*);}
			#[test]
			fn test_9 (){super::$func($($args9),*);}
		}
	};
}