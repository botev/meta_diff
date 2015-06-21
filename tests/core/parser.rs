extern crate autodiff;

#[test]
fn parse_success_1() {
    let source_code = "
    function [a] = mat(a,b)
	a= a + b;
	d = a + b;
	end";
	let result = autodiff::core::parser::metaFile(source_code);
	match result {
		Ok(graph) => {
			assert!(graph.nodes.len() == 4)
		}
		Err(msg) => {
			assert!(false, msg)
		}
	}
}


fn parse_success_2() {
    let source_code = "
    function [a] = mat(a,b)
	a= a + b;
	d = a + b;
	end";
	let result = autodiff::core::parser::metaFile(source_code);
	match result {
		Ok(graph) => {
			assert!(graph.nodes.len() == 4)
		}
		Err(msg) => {
			assert!(false, msg)
		}
	}
}

fn parse_success_3() {
    let source_code = "
    function [a] = mat(a,b)
	a= a + b;
	d = a + b;
	end";
	let result = autodiff::core::parser::metaFile(source_code);
	match result {
		Ok(graph) => {
			assert!(graph.nodes.len() == 4)
		}
		Err(msg) => {
			assert!(false, msg)
		}
	}
}