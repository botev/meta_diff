extern crate meta_diff;

fn constant_folding_some(nodes_before: usize, nodes_after: usize, source: &str){
	let result = meta_diff::core::parseMetaFile(source);
    let mut graph = match result {
		Ok(g) => g,
		Err(msg) => {return assert!(false, "{}", msg);}
	};
    assert!(graph.len() == nodes_before, "Number of the initial graph nodes expected: {}, was: {}", nodes_before, graph.len());
	//println!("Initial: {:?}",graph.ordering);
    match meta_diff::optimization::constant_folding(&mut graph) {
        Ok(b) => {
			assert!(b, "Did not return true for folding");
			if graph.len() == nodes_after {
				assert!(true);
			} else {
				match meta_diff::codegen::write_graphviz(&mut ::std::io::stdout() , &graph){
					Ok(_) => (),
					Err(msg) => assert!(false, "{}", msg)
				}
				//println!("{:?}",graph.ordering);
				println!("{}",graph);
				assert!(false, "Number of the optimized graph nodes expected: {}, was: {}", nodes_after, graph.len());
			}
		},
        Err(msg) => assert!(false, "{}", msg)
    }
}

fn constant_folding_none(nodes: usize, source: &str){
	let result = meta_diff::core::parseMetaFile(source);
    let mut graph = match result {
		Ok(g) => g,
		Err(msg) => {return assert!(false, "{}", msg);}
	};
    assert!(graph.len() == nodes, "Number of the initial graph nodes expected: {}, was: {}", nodes, graph.len());

    match meta_diff::optimization::constant_folding(&mut graph) {
        Ok(b) => {
			assert!(!b, "Did not return true for folding");
			if graph.len() == nodes {
				assert!(true);
			} else {
				match meta_diff::codegen::write_graphviz(&mut ::std::io::stdout() , &graph){
					Ok(_) => (),
					Err(msg) => assert!(false, "{}", msg)
				}
				println!("{}",graph);
				assert!(false, "Number of the optimized graph nodes expected: {}, was: {}", nodes, graph.len());
			}
		},
        Err(msg) => assert!(false, "{}", msg)
    }
}

parametarise_test!(constant_folding_some,
	[11, 9,
	"function [d] = mat(a,b)
	c = 1 + 2 * 3';
	d = l2(c,0) * l1(c,0);
	end"],
	[17, 16,
	"function [L] = mat(@w,x,y)
	h = tanh(w*const(vertcat(x,1)));
	h = tanh(w*const(vertcat(h,1)));
	L = quad(const(h),eye(3));
	end"],
	[18, 16,
	"function [L] = mat(@w,x,y)
	h = tanh(w*const(vertcat(x,1)));
	s = sinh(w*horzcat(h,1));
	L = quad(eye(3),l1(h-y,0));
	end"],
	[14, 13,
	"function [L] = mat(@w,x,y,@z)
	h = replicateH(w,1) + x dot y * sin(2);
	L = sum(h^2,0);
	end"],
	[16, 16,
	"function [L] = mat(@w,x,y)
	h = const(w*-vertcat(x,cols(5)));
	s = diagV(w*horzcat(h,1));
	L = l1(s-h,0);
	end"]
);

parametarise_test!(constant_folding_none,
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
	L = sum(h^3,0);
	end"],
	[15,
	"function [L] = mat(@w,x,y)
	h = const(w*-vertcat(x,1));
	s = diagV(w*horzcat(h,1));
	L = l1(s-h,0);
	end"]
);
