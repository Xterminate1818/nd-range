#![feature(test)]
#![feature(step_trait)]

//! The `nrange` crate provides an abstraction of the
//! standard range types over *n*-dimensions
//! While the traditional range types cover a set of linear
//! values, the [`NRange`](crate::range::NRange) type
//! cover a region in vector space. `NRange` can represent
//! the points in a 2D rectangle, 3D cube, 4D hypercube, or
//! extend to any number of dimensions using rust's const
//! generics.
//! # Getting started
//! The preferred way of initializing `NRange` is using
//! the provided macro. The syntax is similar to the `vec!`
//! macro:
//! ```
//! use nrange::*;
//! let range_a = nrange!(0..3, 0..3);
//! let range_b = nrange!(0..3; 2);
//! assert_eq!(range_a, range_b);
//!
//! assert!(range_a.contains(&[0, 2]));
//!
//! for v in range_a {
//!   print!("{v:?}, "); // [0, 0], [1, 0], [2, 0], [0, 1], ...
//! }
//! ```

pub(crate) fn get_real_bound<T>(bound: Bound<&T>) -> T
where
  T: Step,
  T: Clone,
{
  match bound {
    Bound::Included(s) => s.clone(),
    Bound::Excluded(s) => Step::forward(s.clone(), 1),
    Bound::Unbounded => panic!("Cannot get real bound"),
  }
}

pub mod bounds_ext;
pub mod iter;
pub mod range;

use std::{iter::Step, ops::Bound};

pub use range::*;

#[cfg(test)]
mod tests;
