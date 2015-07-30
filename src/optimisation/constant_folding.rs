use core::*;

pub fn constant_folding(graph: &mut ComputeGraph) -> Result<(), String> {
    for i in (0..graph.nodes.len()-1) {
        // Grab operator
        let option = match graph.nodes[i]{
            Some(ref node) => node.op.clone(),
            None => None
        };
        let result: Result<(),String> = match option{
            Some(op) => single_fold(graph, i, op),
            None => Ok(()),
        };
        try!(result);
    }
    Ok(())
}

// fn fold_nary(graph: &mut ComputeGraph, old: usize, op: Operator) -> Result<(), String>{
//     let mut new_node : Option<usize> = None;
//     let mut parents : Vec<usize> = Vec::new();
//     match op {
//
//         // Operator::Mul(ref p) => {
//         //     let (indexes, values) = try!(extract_values(graph, p));
//         //     match values.len(){
//         //         0...1 => (),
//         //         _ => {
//         //             let x = values.iter().fold(1.0, |acc, &x| acc * x);
//         //             if parents.len() == values.len() {
//         //                 if x == x.floor() {
//         //                     new_node = Some(graph.add_int(x as i64));
//         //                 } else {
//         //                     new_node = Some(graph.add_float(x));
//         //                 }
//         //                 parents.push(p[0]);
//         //                 parents.push(p[1]);
//         //             } else {
//         //                 // Combine all constants
//         //                 let combined = if x == x.floor() {graph.add_int(x as i64)}
//         //                     else {graph.add_float(x)};
//         //                 // All parents which were not constants + combined
//         //                 let mut new_parents  = p.iter().enumerate().filter(|&(i,_)| !indexes.contains(&i)).map(|(_,v)| v.clone()).collect::<Vec<_>>();
//         //                 new_parents.push(combined);
//         //                 new_node = Some(try!(graph.add_operation(Operator::Mul(new_parents))));
//         //                 parents.extend(p.iter().cloned());
//         //             }
//         //         }
//         //     }
//         // },
//         _ => return Err("Operator is not nary".to_string())
//     }
//     Ok(())
// }

fn single_fold(graph: &mut ComputeGraph, old: usize, op: Operator) -> Result<(), String>{
    let mut new_node : Option<usize> = None;
    let mut parents : Vec<usize> = Vec::new();
    match op{
        Operator::Const(p) => match try!(graph.get_node(p)).node_type{
                Type::Integer(_) | Type::Float(_) |
                Type::ConstInput | Type::ConstDerived => {
                    // Remove const(x) where x is itself a constant
                    if try!(graph.get_node(old)).grad_parents.len() > 0 {
                        return Err(format!("The constant node {} has gradient parents?", p))
                    }
                    else {
                        new_node = Some(p);
                        parents.push(p);
                    }
                },
                _ => ()
        },
        Operator::Transpose(p) | Operator::MatrixDiag(p)
        | Operator::VectorDiag(p) | Operator::Sum(p,_) =>  match try!(graph.get_node(p)).node_type{
            Type::Integer(_) | Type::Float(_) => {
                new_node = Some(p);
                parents.push(p);
            }
            _ => ()
        },
        Operator::Size(p, _) => match try!(graph.get_node(p)).node_type{
            Type::Float(_) | Type::Integer(_) => {
                new_node = Some(graph.add_int(1));
                parents.push(p);
            },
            _ => ()
        },
        Operator::Sign(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = if x == 0.0 {
                    Some(graph.add_int(0))
                } else if x < 0.0 {
                    Some(graph.add_int(-1))
                } else {
                    Some(graph.add_int(1))
                };
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = if x == 0 {
                    Some(graph.add_int(0))
                } else if x < 0 {
                    Some(graph.add_int(-1))
                } else {
                    Some(graph.add_int(1))
                };
                parents.push(p);
            },
            _ => ()
        },
        Operator::Neg(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(-x));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_int(-x));
                parents.push(p);
            },
            _ => ()
        },
        Operator::Div(p) | Operator::MatrixInverse(p)
        => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(1.0/x));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float(1.0/(x as f64)));
                parents.push(p);
            },
            _ => ()
        },
        Operator::Cos(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(x.cos()));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float((x as f64).cos()));
                parents.push(p);
            },
            _ => ()
        },
        Operator::Sin(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(x.sin()));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float((x as f64).sin()));
                parents.push(p);
            },
            _ => ()
        },
        Operator::Tan(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(x.tan()));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float((x as f64).tan()));
                parents.push(p);
            },
            _ => ()
        },
        Operator::CosH(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(x.cosh()));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float((x as f64).cosh()));
                parents.push(p);
            },
            _ => ()
        },
        Operator::SinH(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(x.sinh()));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float((x as f64).sinh()));
                parents.push(p);
            },
            _ => ()
        },
        Operator::TanH(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(x.tanh()));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float((x as f64).tanh()));
                parents.push(p);
            },
            _ => ()
        },
        Operator::Abs(p) | Operator::L2(p,_)| Operator::L1(p,_)
        => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                if x > 0.0{
                    new_node = Some(p);
                    parents.push(p);
                } else {
                    new_node = Some(graph.add_float(-x));
                    parents.push(p);
                }
            },
            Type::Integer(x) => {
                if x > 0{
                    new_node = Some(p);
                    parents.push(p);
                } else {
                    new_node = Some(graph.add_int(-x));
                    parents.push(p);
                }
            },
            _ => ()
        },
        Operator::Log(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(x.ln()));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float((x as f64).ln()));
                parents.push(p);
            },
            _ => ()
        },
        Operator::Exp(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(x.exp()));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float((x as f64).exp()));
                parents.push(p);
            },
            _ => ()
        },
        Operator::Sqrt(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(x.sqrt()));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float((x as f64).sqrt()));
                parents.push(p);
            },
            _ => ()
        },
        Operator::Square(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(x*x));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_int(x*x));
                parents.push(p);
            },
            _ => ()
        },
        Operator::Sigmoid(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                new_node = Some(graph.add_float(1.0 / (1.0 + (-x).exp())));
                parents.push(p);
            },
            Type::Integer(x) => {
                new_node = Some(graph.add_float(1.0 / (1.0 + (-x as f64).exp())));
                parents.push(p);
            },
            _ => ()
        },
        Operator::LessThan(p_1,p_2) => {
            let (_, values) = try!(extract_values(graph, &vec![p_1, p_2]));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] < values[1] {1} else {0};
                    new_node = Some(graph.add_int(val));
                    parents.push(p_1);
                    parents.push(p_2);
                },
                _ => unreachable!()
            }
        },
        Operator::LessThanOrEqual(p_1,p_2) => {
            let (_, values) = try!(extract_values(graph, &vec![p_1, p_2]));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] <= values[1] {1} else {0};
                    new_node = Some(graph.add_int(val));
                    parents.push(p_1);
                    parents.push(p_2);
                },
                _ => unreachable!()
            }
        },
        Operator::GreaterThan(p_1,p_2) => {
            let (_, values) = try!(extract_values(graph, &vec![p_1, p_2]));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] > values[1] {1} else {0};
                    new_node = Some(graph.add_int(val));
                    parents.push(p_1);
                    parents.push(p_2);
                },
                _ => unreachable!()
            }
        },
        Operator::GreaterThanOrEqual(p_1,p_2) => {
            let (_, values) = try!(extract_values(graph, &vec![p_1, p_2]));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] >= values[1] {1} else {0};
                    new_node = Some(graph.add_int(val));
                    parents.push(p_1);
                    parents.push(p_2);
                },
                _ => unreachable!()
            }
        }
        Operator::Max(p_1, p_2) => {
            let (_, values) = try!(extract_values(graph, &vec![p_1, p_2]));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] > values[1] {values[0]} else {values[1]};
                    new_node = if val.floor() == val {
                        Some(graph.add_int(val as i64))
                    } else {
                        Some(graph.add_float(val))
                    };
                    parents.push(p_1);
                    parents.push(p_2);
                },
                _ => unreachable!()
            }
        },
        Operator::Min(p_1, p_2) => {
            let (_, values) = try!(extract_values(graph, &vec![p_1, p_2]));
            match values.len() {
                0...1 => (),
                2 => {
                    let val = if values[0] < values[1] {values[0]} else {values[1]};
                    new_node = if val.floor() == val {
                            Some(graph.add_int(val as i64))
                        } else {
                            Some(graph.add_float(val))
                        };
                    parents.push(p_1);
                    parents.push(p_2);
                },
                _ => unreachable!()
            }
        },
        Operator::Pow(p_1, p_2) => {
            let (indexes, values) = try!(extract_values(graph, &vec![p_1, p_2]));
            match values.len() {
                0 => (),
                1 => {
                    match indexes[0] {
                        0 => match values[0] {
                            0.0 => {
                                new_node = Some(graph.add_int(0));
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            1.0 => {
                                new_node = Some(graph.add_int(1));
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            _ => ()
                        },
                        1 => match values[0]{
                            0.0 => {
                                new_node = Some(graph.add_int(1));
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            1.0 => {
                                new_node = Some(p_1);
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            2.0 => {
                                new_node = Some(try!(graph.add_operation(Operator::Square(p_1))));
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            _ => ()
                        },
                        _ => unreachable!()
                    }
                },
                2 => {
                    let val = values[0].powf(values[1]);
                    new_node = if val.floor() == val {
                        Some(graph.add_int(val as i64))
                    } else {
                        Some(graph.add_float(val))
                    };
                    parents.push(p_1);
                    parents.push(p_2);
                },
                _ => unreachable!()
            }
        },
        Operator::Quadratic(p_1, p_2) => {
            // Check first operand for Zeros and Eye
            let op = try!(graph.get_node(p_1)).op.clone();
            match op {
                Some(ref operator) => {
                    match *operator {
                        Operator::Zeros(_,_) => {
                            new_node = Some(graph.add_int(0));
                            parents.push(p_1);
                            parents.push(p_2);
                        },
                        Operator::Eye(_) => {
                            new_node = Some(p_2);
                            parents.push(p_1);
                            parents.push(p_2);
                        }
                        _ => ()
                    }
                },
                None => ()
            }
            if new_node == None {
                // Check second operand for Zeros and Eye
                let op = try!(graph.get_node(p_2)).op.clone();
                match op {
                    Some(ref operator) => {
                        match *operator {
                            Operator::Zeros(_,_) => {
                                new_node = Some(graph.add_int(0));
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            Operator::Eye(_) => {
                                new_node = Some(try!(graph.add_operation(Operator::Dot(vec![p_1, p_1]))));
                                parents.push(p_1);
                                parents.push(p_2);
                            }
                            _ => ()
                        }
                    },
                    None => ()
                }
            }
            if new_node == None{
                let (_, values) = try!(extract_values(graph, &vec![p_1, p_2]));
                match values.len() {
                    0...1 => (),
                    2 => {
                        let val = if values[0] < values[1] {values[0]} else {values[1]};
                        new_node = if val.floor() == val {
                                Some(graph.add_int(val as i64))
                            } else {
                                Some(graph.add_float(val))
                            };
                        parents.push(p_1);
                        parents.push(p_2);
                    },
                    _ => unreachable!()
                }
            }
        },
        Operator::ReplicateHorz(p_1, p_2) => {
            match try!(graph.get_node(p_2)).node_type {
                Type::Float(_) => return Err("HorzCat with floating point second argument".to_string()),
                Type::Integer(x) => match x{
                    0 => return Err("HorzCat with 0 as second argument".to_string()),
                    1 => {
                        new_node = Some(p_1);
                        parents.push(p_1);
                        parents.push(p_2);
                    },
                    _ => ()
                },
                _ => ()
            }
        },
        Operator::ReplicateVert(p_1, p_2) => {
            match try!(graph.get_node(p_2)).node_type {
                Type::Float(_) => return Err("HorzCat with floating point second argument".to_string()),
                Type::Integer(x) => match x{
                    0 => return Err("HorzCat with 0 as second argument".to_string()),
                    1 => {
                        new_node = Some(p_1);
                        parents.push(p_1);
                        parents.push(p_2);
                    },
                    _ => ()
                },
                _ => ()
            }
        },
        Operator::Add(ref p) | Operator::Mul(ref p) | Operator::Dot(ref p)=> {
            let (indexes, values) = try!(extract_values(graph, p));
            match values.len(){
                0...1 => return Err("Nary operator with 0 or 1 arguments".to_string()),
                _ => {
                    let x = match op {
                        Operator::Add(_) =>  values.iter().fold(0.0, |acc, &x| acc + x),
                        Operator::Mul(_) | Operator::Dot (_)
                            => values.iter().fold(1.0, |acc, &x| acc * x),
                        _ => unreachable!()
                    };
                    if parents.len() == values.len() {
                        if x == x.floor() {
                            new_node = Some(graph.add_int(x as i64));
                        } else {
                            new_node = Some(graph.add_float(x));
                        }
                        parents.push(p[0]);
                        parents.push(p[1]);
                    } else {
                        // Combine all constants
                        let combined = if x == x.floor() {graph.add_int(x as i64)}
                            else {graph.add_float(x)};
                        // All parents which were not constants + combined
                        let mut new_parents  = p.iter().enumerate().filter(|&(i,_)| !indexes.contains(&i)).map(|(_,v)| v.clone()).collect::<Vec<_>>();
                        new_parents.push(combined);
                        let new_op = try!(op.recreate(new_parents));
                        new_node = Some(try!(graph.add_operation(new_op)));
                        parents.extend(p.iter().cloned());
                    }
                }
            }
        },
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
            Ok(())
        },
        None => Ok(())
    }
}

fn extract_values(graph: &mut ComputeGraph, nodes: &Vec<usize>)
    -> Result<(Vec<usize>, Vec<f64>), String> {
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
