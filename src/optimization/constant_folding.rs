use core::*;

pub fn constant_folding(graph: &mut ComputeGraph) -> Result<bool, String> {
    let mut outcome = false;
    let mut i = 0;
    loop {
        // Grab operator
        let option = match graph.nodes[i]{
            Some(ref node) => node.op.clone(),
            None => None
        };
        let result = match option{
            Some(op) => single_fold(graph, i, op),
            None => Ok(false),
        };
        outcome = outcome || try!(result);
        i += 1;
        if i == graph.nodes.len(){
            break;
        }
    }
    Ok(outcome)
}

fn single_fold(graph: &mut ComputeGraph, old: usize, op: Operator) -> Result<bool, String>{
    let mut created_nodes : Vec<usize> = Vec::new();
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
                let node = graph.add_int(1);
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Sign(p) => match try!(graph.get_node(p)).node_type{
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
                parents.push(p);
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
                parents.push(p);
            },
            _ => ()
        },
        Operator::Neg(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(-x);
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_int(-x);
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Div(p) | Operator::MatrixInverse(p)
        => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(1.0/x);
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float(1.0/(x as f64));
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Cos(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x.cos());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float((x as f64).cos());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Sin(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x.sin());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float((x as f64).sin());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Tan(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x.tan());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float((x as f64).tan());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::CosH(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x.cosh());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float((x as f64).cosh());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::SinH(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x.sinh());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float((x as f64).sinh());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::TanH(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x.tanh());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float((x as f64).tanh());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Abs(p) | Operator::L1(p,_)
        => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                if x > 0.0{
                    new_node = Some(p);
                    parents.push(p);
                } else {
                    let node = graph.add_float(-x);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(p);
                }
            },
            Type::Integer(x) => {
                if x > 0{
                    new_node = Some(p);
                    parents.push(p);
                } else {
                    let node = graph.add_int(-x);
                    created_nodes.push(node);
                    new_node = Some(node);
                    parents.push(p);
                }
            },
            _ => ()
        },
        Operator::L2(p,_)
        => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x*x);
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_int(x*x);
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Log(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x.ln());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float((x as f64).ln());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Exp(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x.exp());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float((x as f64).exp());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Sqrt(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x.sqrt());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float((x as f64).sqrt());
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Square(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(x*x);
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_int(x*x);
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            _ => ()
        },
        Operator::Sigmoid(p) => match try!(graph.get_node(p)).node_type{
            Type::Float(x) => {
                let node = graph.add_float(1.0 / (1.0 + (-x).exp()));
                created_nodes.push(node);
                new_node = Some(node);
                parents.push(p);
            },
            Type::Integer(x) => {
                let node = graph.add_float(1.0 / (1.0 + (-x as f64).exp()));
                created_nodes.push(node);
                new_node = Some(node);
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
                    let node = graph.add_int(val);
                    created_nodes.push(node);
                    new_node = Some(node);
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
                    let node = graph.add_int(val);
                    created_nodes.push(node);
                    new_node = Some(node);
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
                    let node = graph.add_int(val);
                    created_nodes.push(node);
                    new_node = Some(node);
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
                    let node = graph.add_int(val);
                    created_nodes.push(node);
                    new_node = Some(node);
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
                    let node = if val.floor() == val {
                        graph.add_int(val as i64)
                    } else {
                        graph.add_float(val)
                    };
                    created_nodes.push(node);
                    new_node = Some(node);
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
                    let node = if val.floor() == val {
                            graph.add_int(val as i64)
                        } else {
                            graph.add_float(val)
                        };
                        created_nodes.push(node);
                        new_node = Some(node);
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
                                let node = graph.add_int(0);
                                created_nodes.push(node);
                                new_node = Some(node);
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            1.0 => {
                                let node = graph.add_int(1);
                                created_nodes.push(node);
                                new_node = Some(node);
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            _ => ()
                        },
                        1 => match values[0]{
                            0.0 => {
                                let node = graph.add_int(1);
                                created_nodes.push(node);
                                new_node = Some(node);
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            1.0 => {
                                new_node = Some(p_1);
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            2.0 => {
                                let node = try!(graph.add_operation(Operator::Square(p_1)));
                                created_nodes.push(node);
                                new_node = Some(node);
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
                    let node = if val.floor() == val {
                        graph.add_int(val as i64)
                    } else {
                        graph.add_float(val)
                    };
                    created_nodes.push(node);
                    new_node = Some(node);
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
                            let node = graph.add_int(0);
                            created_nodes.push(node);
                            new_node = Some(node);
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
                                let node = graph.add_int(0);
                                created_nodes.push(node);
                                new_node = Some(node);
                                parents.push(p_1);
                                parents.push(p_2);
                            },
                            Operator::Eye(_) => {
                                println!("2");
                                let node = try!(graph.add_operation(Operator::Dot(vec![p_1, p_1])));
                                created_nodes.push(node);
                                new_node = Some(node);
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
                        let node = if val.floor() == val {
                                graph.add_int(val as i64)
                            } else {
                                graph.add_float(val)
                            };
                        created_nodes.push(node);
                        new_node = Some(node);
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
                    0 => return Err("ReplicateHorz with 0 as second argument".to_string()),
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
                Type::Float(_) => return Err("ReplicateVert with floating point second argument".to_string()),
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
                0...1 => (),
                _ => {
                    let x = match op {
                        Operator::Add(_) =>  values.iter().fold(0.0, |acc, &x| acc + x),
                        Operator::Mul(_) | Operator::Dot (_)
                            => values.iter().fold(1.0, |acc, &x| acc * x),
                        _ => unreachable!()
                    };
                    if p.len() == values.len() {
                        let node = if x == x.floor() {
                            graph.add_int(x as i64)
                        } else {
                            graph.add_float(x)
                        };
                        created_nodes.push(node);
                        new_node = Some(node);
                        parents.extend(p.iter().cloned());
                    } else {
                        // Combine all constants
                        let combined = if x == x.floor() {graph.add_int(x as i64)}
                            else {graph.add_float(x)};
                        created_nodes.push(combined);
                        // All parents which were not constants + combined
                        let mut new_parents  = p.iter().enumerate().filter(|&(i,_)| !indexes.contains(&i)).map(|(_,v)| v.clone()).collect::<Vec<_>>();
                        new_parents.push(combined);
                        // println!("Parents:{}, {:?} - {:?}", new_parents.len(), indexes, p);
                        let new_op = try!(op.recreate(new_parents));
                        let node = try!(graph.add_operation(new_op));
                        created_nodes.push(node);
                        new_node = Some(node);
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
            if graph.target == old{
                graph.target = node;
            }
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
