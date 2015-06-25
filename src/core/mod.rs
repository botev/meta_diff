#[macro_use]
pub mod macros;

pub mod graph;
pub mod parser;
pub mod symbolic;

// use std::result::Result;

// pub trait apply_copy<T: Clone,F,E> where F:  Fn(T) -> Result<(),E> {
// 	fn apply_copy(&self, f: F) -> Result<(),E> ;
// }

// pub trait apply_ref<T,F,E> where F:  Fn(&T) -> Result<(),E>  {
// 	fn apply_ref(&self, f: F) -> Result<(),E> ;
// }

// impl <T:Clone,F,E> apply_copy<T,F,E> for Vec<Option<T>> where F:  Fn(T) -> Result<(),E>  {
// 	fn apply_copy(&self, f: F) -> Result<(),E> {
// 		for option in self.iter(){
// 			match *option{
// 				None => (),
// 				Some(ref value) => match f(value.clone()) {
// 					Ok(()) => (),
// 					Err(msg) => return Err(msg)
// 				}
// 			}
// 		}
// 		Ok(())
// 	}
// }

// impl <T,F,E> apply_ref<T,F,E> for Vec<Option<T>> where F:  Fn(&T) -> Result<(),E>  {
// 	fn apply_ref(&self, f: F) -> Result<(),E> {
// 		for option in self.iter(){
// 			match *option{
// 				None => (),
// 				Some(ref value) => match f(value) {
// 					Ok(()) => (),
// 					Err(msg) => return Err(msg)
// 				}
// 			}
// 		}
// 		Ok(())
// 	}
// }