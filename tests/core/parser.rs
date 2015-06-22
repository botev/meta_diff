extern crate meta_diff;

fn parse_ok(nodes: usize, source: &str){
	let result = meta_diff::core::parser::metaFile(source);
	match result {
		Ok(graph) => {
			assert!(graph.nodes.len() == nodes, "Number of nodes expected: {}, was: {}", nodes, graph.nodes.len());
		}
		Err(msg) => {
			assert!(false, "{}", msg);
		}
	}
}

fn parse_fail(fail_msg: &str, source: &str){
	let result = meta_diff::core::parser::metaFile(source);
	match result {
		Ok(_) => {
			assert!(false, "Fail parsed, but should have failed.");
		}
		Err(msg) => {
			let text = format!("{}",msg);
			assert!(text == fail_msg,format!("Parser failed message expected: {}, was: {}", fail_msg, msg));
		}
	}
}

parametarise_test!(parse_ok,
	[8,
	"function [d] = mat(a,b)
	c = a + b * a';
	d = l2(c,0) * l1(c,0);
	end" ],
	[14,
	"function [L] = mat(@w,x,y)
	h = tanh(w*vertcat(x,1));
	h = tanh(w*vertcat(h,1));
	L = l2(h-y,0);
	end"],
	[14,
	"function [L] = mat(@w,x,y)
	h = tanh(w*vertcat(x,1));
	s = sinh(w*horzcat(h,1));
	L = l1(h-y,0);
	end"],
	[10,
	"function [L] = mat(@w,x,y,@z)
	h = w + x dot y * z;
	L = sum(h^2,0);
	end"],
	[15,
	"function [L] = mat(@w,x,y)
	h = const(w*-vertcat(x,1));
	s = diagV(w*horzcat(h,1));
	L = l1(s-h,0);
	end"]
);

parametarise_test!(parse_fail,
	["Error at 2:7: Use of undefined variable \'d\'",
	"function [d] = mat(a,b)
	c = d + b * a';
	d = l2(c,0) * l1(c,0);
	end" ],
	["Error at 3:28: Can not have a variable with name \'sin\' since it is a built in function",
	"function [L] = mat(@w,x,y)
	h = tanh(w*vertcat(x,1));
	sin = tanh(w*vertcat(h,1));
	L = l2(h-y,0);
	end"],
	["Error at 4:13: Comparison operators not supported!",
	"function [L] = mat(@w,x,y)
	h = tanh(w*vertcat(x,1));
	s = sinh(w*horzcat(h,1));
	L = l1(h>=y,0);
	end"],
	["Error at 4:5: Output variable \'k\' has not been defined",
	"function [L,k] = mat(@w,x,y,@z)
	h = w + x dot y * z;
	L = sum(h^2,0);
	end"],
	["Error at 2:29: HorzCat takes at least two arguments",
	"function [L] = mat(@w,x,y)
	h = horzcat(w*-vertcat(x,1));
	s = diagV(w*horzcat(h,1));
	L = l1(s-h,0);
	end"]
);