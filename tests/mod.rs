#[macro_use]
mod macros;
mod core;
mod optimization;
mod linking;
// use tempdir::*;
// use std::process::Command;
// use std::io::Write;
// use std::dynamic_lib::DynamicLibrary;

// #[test]
// pub fn dynamic_linking_test() {
//     let name = "dylib";
//     let source = "
//     // dylib.rs
//     #[no_mangle]
//     pub fn minicall(x: f64) -> f64 {
//         1.0 / (1.0 + (-x).exp())
//     } ";
//
//     let dir = std::env::current_dir().unwrap();
//     // TempDir::new("rust_test").unwrap().into_path();
//     let mut source_loc = dir.clone();
//     source_loc.push(&name);
//     source_loc.set_extension("rs");
//     let file = std::fs::File::create(source_loc.as_path()).unwrap();
//     {
//         let mut writer = std::io::BufWriter::new(&file);
//         let _ = writer.write_fmt(format_args!("{}\n",source)).unwrap();
//     }
//     let mut target_loc = dir.clone();
//     target_loc.push(&name);
//     target_loc.set_extension("so");
//     println!("Generated source");
//
//     let cmd = format!("rustc --crate-type {} {} -o {}" ,name,
//         source_loc.into_os_string().to_str().unwrap(),
//         target_loc.clone().into_os_string().to_str().unwrap());
//     println!("Calling command: {}", cmd);
//     let  _ = Command::new("sh")
//         .arg("-c")
//         .arg(cmd)
//         .output()
//         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
//     let output = Command::new("sh")
//         .arg("-c")
//         .arg(format!("ls {}", dir.into_os_string().to_str().unwrap()))
//         .output()
//         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
//     println!("ls:\n {}",String::from_utf8(output.stdout).unwrap());
//
//     let (lib,func) = match DynamicLibrary::open(Some(target_loc.as_path())){
//         Err(err) => panic!("DL Error: {}", err),
//         Ok(lib) => {
//             println!("Unsafe block!");
//             let func: fn(f64) -> f64 = unsafe {
//                  match lib.symbol("minicall") {
//                     Err(err) => {panic!("ERROR: {}", err);}
//                     // And then cast that pointer a function
//                     Ok(f) => {::std::mem::transmute::<*mut f64, fn(f64) -> f64>(f)},
//                 }
//             };
//             (lib, func)
//             // let values = vec![-1.0,-0.75,-0.5,-0.25,0.0,0.25,0.5,0.75,1.0];
//             // for i in values.iter(){
//             //     println!("sigm({})={}",i,func(*i));
//             // }
//             // (func.clone()
//         }
//     };
//     let values = vec![-1.0,-0.75,-0.5,-0.25,0.0,0.25,0.5,0.75,1.0];
//     for i in values.iter(){
//         println!("sigm({})={}",i,func(*i));
//     }
//
//     assert!(false);
// }
