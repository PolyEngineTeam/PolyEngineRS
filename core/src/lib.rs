#[allow(unused_imports)]
#[macro_use]
pub extern crate static_assertions;

#[allow(unused_imports)]
#[macro_use]
pub extern crate approx; // For the macro relative_eq!
pub extern crate nalgebra as na;

mod types;

pub mod math;
pub use types::*;
