use core::*;

pub fn constant_folding(graph: &mut ComputeGraph) -> Result<bool, GraphError> {
    let mut outcome = false;
    let mut i = 0;
    loop {
        // Grab operator
        let node = match graph.nodes[i] {
            Some(_) => true,
            None => false
        };
        let result = if node {
            try!(single_fold(graph, i))
        } else {
            false
        };
        outcome = outcome || result;
        i += 1;
        if i == graph.nodes.len(){
            break;
        }
    }
    Ok(outcome)
}

fn single_fold(graph: &mut ComputeGraph, old: usize) -> Result<bool, GraphError>{
    let op = try!(graph.get_node(old)).op.clone();
    let mut created_nodes : Vec<usize> = Vec::new();
    let mut new_node : Option<usize> = None;
    let mut parents : Vec<usize> = Vec::new();
    match op.op_type{
        OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Const)) => {
            let parent = try!(graph.get_node(op.parents[0]));
            match parent.node_type {
                Type::Integer(_) | Type::Float(_) |
                Type::ConstInput | Type::ConstDerived => {
                    // Remove const(x) where x is itself a constant
                    if parent.grad_parents.len() > 0 || parent.grad_child.is_some() {
                        return Err(GraphError::GradientOfConstant(op.parents[0]))
                    }
                    else {
                        new_node = Some(op.parents[0]);
                        parents.push(op.parents[0]);
                    }
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Transpose)
        | OperatorType::Unary(UnaryOperatorType::MatrixDiag)
        | OperatorType::Unary(UnaryOperatorType::VectorDiag)
        | OperatorType::Unary(UnaryOperatorType::Sum(_)) => {

            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Integer(_) | Type::Float(_) => {
                    new_node = Some(op.parents[0]);
                    parents.push(op.parents[0]);
                }
                _ => ()
            }
        },
        OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Size(_))) => {
            match try!(graph.get_node(op.parents[0])).node_type{
                Type::Float(_) | Type::Integer(_) => {
                    let node = graph.add_int(1);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Sign)) =>{
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = if x == 0.0 {
                        graph.add_int(0)
                    } else if x < 0.0 {
                        graph.add_int(-1)
                    } else {
                        graph.add_int(1)
                    };
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = if x == 0 {
                        graph.add_int(0)
                    } else if x < 0 {
                        graph.add_int(-1)
                    } else {
                        graph.add_int(1)
                    };
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Neg) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(-x);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_int(-x);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Div)
        | OperatorType::Unary(UnaryOperatorType::MatrixInverse) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(1.0/x);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float(1.0/(x as f64));
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Cos) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x.cos());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float((x as f64).cos());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Sin) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x.sin());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float((x as f64).sin());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Tan) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x.tan());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float((x as f64).tan());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::CosH) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x.cosh());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float((x as f64).cosh());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::SinH) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x.sinh());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float((x as f64).sinh());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::TanH) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x.tanh());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float((x as f64).tanh());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Abs)
        | OperatorType::Unary(UnaryOperatorType::L1(_)) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    if x > 0.0{
                        new_node = Some(op.parents[0]);
                        parents.push(op.parents[0]);
                    } else {
                        let node = graph.add_float(-x);
                        created_nodes.push(node);
                        new_node = Some(node);
                        parents.push(op.parents[0]);
                    }
                },
                Type::Integer(x) => {
                    if x > 0{
                        new_node = Some(op.parents[0]);
                        parents.push(op.parents[0]);
                    } else {
                        let node = graph.add_int(-x);
                        created_nodes.push(node);
                        new_node = Some(node);
                        parents.push(op.parents[0]);
                    }
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::L2(_)) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x*x);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_int(x*x);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Log) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x.ln());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float((x as f64).ln());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Exp) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x.exp());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float((x as f64).exp());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Sqrt) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x.sqrt());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float((x as f64).sqrt());
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Square) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(x*x);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_int(x*x);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Unary(UnaryOperatorType::Sigmoid) => {
            match try!(graph.get_node(op.parents[0])).node_type {
                Type::Float(x) => {
                    let node = graph.add_float(1.0 / (1.0 + (-x).exp()));
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                Type::Integer(x) => {
                    let node = graph.add_float(1.0 / (1.0 + (-x as f64).exp()));
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(op.parents[0]);
                },
                _ => ()
            }
        },
        OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::LessThan)) => {
            let (_, values) = try!(extract_values(graph, &op.parents));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] < values[1] {1} else {0};
                    let node = graph.add_int(val);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.extend(op.parents.iter());
                },
                _ => unreachable!()
            }
        },
        OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::LessThanOrEqual)) => {
            let (_, values) = try!(extract_values(graph, &op.parents));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] <= values[1] {1} else {0};
                    let node = graph.add_int(val);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.extend(op.parents.iter());
                },
                _ => unreachable!()
            }
        },
        OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::GreaterThan)) => {
            let (_, values) = try!(extract_values(graph, &op.parents));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] > values[1] {1} else {0};
                    let node = graph.add_int(val);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.extend(op.parents.iter());
                },
                _ => unreachable!()
            }
        },
        OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::GreaterThanOrEqual)) => {
            let (_, values) = try!(extract_values(graph, &op.parents));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] >= values[1] {1} else {0};
                    let node = graph.add_int(val);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.extend(op.parents.iter());
                },
                _ => unreachable!()
            }
        },
        OperatorType::Binary(BinaryOperatorType::Max) => {
            let (_, values) = try!(extract_values(graph, &op.parents));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] > values[1] {values[0]} else {values[1]};
                    let node = if val.floor() == val {
                        graph.add_int(val as i64)
                    } else {
                        graph.add_float(val)
                    };
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.extend(op.parents.iter());
                },
                _ => unreachable!()
            }
        },
        OperatorType::Binary(BinaryOperatorType::Min) => {
            let (_, values) = try!(extract_values(graph, &op.parents));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] < values[1] {values[0]} else {values[1]};
                    let node = if val.floor() == val {
                        graph.add_int(val as i64)
                    } else {
                        graph.add_float(val)
                    };
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.extend(op.parents.iter());
                },
                _ => unreachable!()
            }
        },
        OperatorType::Binary(BinaryOperatorType::Pow) => {
            let (indexes, values) = try!(extract_values(graph, &op.parents));
            match values.len() {
                0 => (),
                1 => {
                    match indexes[0] {
                        0 => match values[0] {
                            0.0 => {
                                let node = graph.add_int(0);
                                created_nodes.push(node);
                                new_node = Some(node);
                                parents.extend(op.parents.iter());
                            },
                            1.0 => {
                                let node = graph.add_int(1);
                                created_nodes.push(node);
                                new_node = Some(node);
                                parents.extend(op.parents.iter());
                            },
                            _ => ()
                        },
                        1 => match values[0]{
                            0.0 => {
                                let node = graph.add_int(1);
                                created_nodes.push(node);
                                new_node = Some(node);
                                parents.extend(op.parents.iter());
                            },
                            1.0 => {
                                new_node = Some(op.parents[0]);
                                parents.extend(op.parents.iter());
                            },
                            2.0 => {
                                let node = try!(graph.add_operation(
                                    OperatorType::Unary(UnaryOperatorType::Square), vec![op.parents[0]]));
                                created_nodes.push(node);
                                new_node = Some(node);
                                parents.extend(op.parents.iter());
                            },
                            _ => ()
                        },
                        _ => unreachable!()
                    }
                },
                2 => {
                    let val = values[0].powf(values[1]);
                    let node = if val.floor() == val {
                        graph.add_int(val as i64)
                    } else {
                        graph.add_float(val)
                    };
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.extend(op.parents.iter());
                },
                _ => unreachable!()
            }
        },
        OperatorType::Binary(BinaryOperatorType::Quadratic) => {
            let po = try!(graph.get_node(op.parents[0])).op.clone();
            // Check first operand for Zeros and Eye
            match po.op_type {
                OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Zeros))   => {
                    let node = graph.add_int(0);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.extend(op.parents.iter());
                },
                OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Eye))       => {
                    new_node = Some(op.parents[1]);
                    parents.extend(op.parents.iter());
                },
                _ => ()
            }
            if new_node == None {
                // Check second operand for Zeros and Eye
                let po = try!(graph.get_node(op.parents[1])).op.clone();
                match po.op_type {
                    OperatorType::Constant(ConstantOperatorType::Binary(ConstantBinaryOperatorType::Zeros)) => {
                        let node = graph.add_int(0);
                        created_nodes.push(node);
                        new_node = Some(node);
                        parents.extend(op.parents.iter());
                    },
                    OperatorType::Constant(ConstantOperatorType::Unary(ConstantUnaryOperatorType::Eye)) => {
                        let node = try!(graph.add_operation(
                            OperatorType::Nary(NaryOperatorType::Dot), vec![op.parents[0], op.parents[0]]));
                        created_nodes.push(node);
                        new_node = Some(node);
                        parents.extend(op.parents.iter());
                    },
                    _ => ()
                }
            }
            if new_node == None{
                let (_, values) = try!(extract_values(graph, &op.parents));
                match values.len() {
                    0...1 => (),
                    2 => {
                        let val = if values[0] < values[1] {values[0]} else {values[1]};
                        let node = if val.floor() == val {
                                graph.add_int(val as i64)
                            } else {
                                graph.add_float(val)
                            };
                        created_nodes.push(node);
                        new_node = Some(node);
                        parents.extend(op.parents.iter());
                    },
                    _ => unreachable!()
                }
            }
        },
        OperatorType::Special(SpecialUnaryOperatorType::ReplicateHorz)
        | OperatorType::Special(SpecialUnaryOperatorType::ReplicateVert) => {
            match try!(graph.get_node(op.args[0])).node_type {
                Type::Float(_) => return Err(::std::convert::From::from(
                    InvalidOperatorError::new(op.op_type, 1, 1, 0, 0))),
                Type::Integer(x) => match x{
                    0 => return Err(::std::convert::From::from(
                        InvalidOperatorError::new(op.op_type, 1, 1, 0, 0))),
                    1 => {
                        new_node = Some(op.parents[0]);
                        parents.push(op.parents[0]);
                        parents.push(op.args[0]);
                    },
                    _ => ()
                },
                _ => ()
            }
        },
        // OpeartorType::Nary(NaryOperatorType::Add)
        // | OpeartorType::Nary(NaryOperatorType::Mul)
        // | OpeartorType::Nary(NaryOperatorType::Dot) => {
        //     let (indexes, values) = try!(extract_values(graph, &node.parents));
        //     match values.len(){
        //         0...1 => (),
        //         _ => {
        //             let x = match node.op.op_type {
        //                 OpeartorType::Nary(NaryOperatorType::Add)  =>  values.iter().fold(0.0, |acc, &x| acc + x),
        //                 _ => values.iter().fold(1.0, |acc, &x| acc * x),
        //             };
        //             if p.len() == values.len() {
        //                 let node = if x == x.floor() {
        //                     graph.add_int(x as i64)
        //                 } else {
        //                     graph.add_float(x)
        //                 };
        //                 created_nodes.push(node);
        //                 new_node = Some(node);
        //                 parents.extend(p.iter().cloned());
        //             } else {
        //                 // Combine all constants
        //                 let combined = if x == x.floor() {graph.add_int(x as i64)}
        //                     else {graph.add_float(x)};
        //                 created_nodes.push(combined);
        //                 // All parents which were not constants + combined
        //                 let mut new_parents  = p.iter().enumerate().filter(|&(i,_)| !indexes.contains(&i)).map(|(_,v)| v.clone()).collect::<Vec<_>>();
        //                 new_parents.push(combined);
        //                 // println!("Parents:{}, {:?} - {:?}", new_parents.len(), indexes, p);
        //                 let new_op = try!(op.recreate(new_parents));
        //                 let node = try!(graph.add_operation(new_op));
        //                 created_nodes.push(node);
        //                 new_node = Some(node);
        //                 parents.extend(p.iter().cloned());
        //             }
        //         }
        //     }
        // },
        _ => ()
    }
    match new_node{
        Some(node) => {
            // Swap connection of the children to point to the new node
            try!(graph.swap_child_connections(old, node));
            // Delete the node from the parent's children
            for i in parents {
                let children : &mut Vec<usize> = &mut try!(graph.get_mut_node(i)).children;
                children.iter().position(|&x| x == old).map(|x| children.remove(x));
            }
            // Remove node from the graph
            graph.insert_node(old, None);
            graph.outputs.iter().position(|&x| x == old).map(|x| {graph.outputs.push(node); graph.outputs.swap_remove(x);});
            // Remove node from the ordering and put all created nodes
            let order:usize = graph.ordering.iter().position(|&x| x == old).unwrap();
            println!("Ordering: {:?}",graph.ordering);
            println!("Removing {} from position {}",old,order);
            let _ = graph.ordering.remove(order);
            for _ in created_nodes.iter(){
                let i = graph.ordering.pop().unwrap();
                graph.ordering.insert(order, i);
            }
            Ok(true)
        },
        None => Ok(false)
    }
}

fn extract_values(graph: &mut ComputeGraph, nodes: &Vec<usize>)
    -> Result<(Vec<usize>, Vec<f64>), GraphError> {
    let mut values : Vec<f64> = Vec::new();
    let mut indexes : Vec<usize> = Vec::new();
    for (index, node) in nodes.iter().enumerate(){
        match try!(graph.get_node(*node)).node_type{
            Type::Float(x) => {
                values.push(x);
                indexes.push(index);
            },
            Type::Integer(x) => {
                indexes.push(index);
                values.push(x as f64);
            },
            _ => ()
        }
    }
    Ok((indexes, values))
}
