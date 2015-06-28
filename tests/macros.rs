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