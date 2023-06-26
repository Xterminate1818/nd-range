use std::{iter::Step, marker::PhantomData, ops::RangeBounds};

use crate::nditer::NdIter;

pub(crate) fn get_bound_start<R, T>(s: &R) -> T
where
  R: RangeBounds<T>,
  R: ExactSizeIterator,
  T: Step,
  T: Clone,
{
  match s.start_bound() {
    std::ops::Bound::Included(s) => s.clone(),
    std::ops::Bound::Excluded(s) => Step::forward(s.clone(), 1),
    std::ops::Bound::Unbounded => unreachable!(
      "Implementing exact size iterator means the range cannot be unbounded \
       at the start"
    ),
  }
}

/// A N-Dimensional extension to Range and its derivatives
///
/// # Examples
/// ```
/// assert_eq!(ndrange!(0..5, 0..5), NdRange::new([0..5, 0..5]));
/// assert_eq!(ndrange!(0..3, 0..3).len(), 9);
///
/// let mut iter = ndrange!(0..2, 0..2).into_iter();
/// assert_eq!(iter.next(), Some([0, 0]));
/// assert_eq!(iter.next(), Some([1, 0]));
/// assert_eq!(iter.next(), Some([0, 1]));
/// assert_eq!(iter.next(), Some([1, 1]));
/// assert_eq!(iter.next(), None);
///
/// assert!(ndrange!(0..10, 0..10).contains(&[2, 4]));
///
/// assert_eq!(ndrange!().len(), 0);
/// assert_eq!(ndrange!(0..7, 0..0).len(), 0);
/// assert!(!ndrange!(0..3, 0..2, 0..0).contains(&[2, 1, 0]));
///
/// for i in NdRange::new([0..5, 0..5]) {
///   println!("{:?}", i);
/// }
///
/// for _ in ndrange!() {
///   panic!("Unreachable, empty NdRange returns an empty
/// iterator") }
///
/// for i in ndrange!(0..5, 0..5) {
///   println!("{i:?}");
/// }
/// ```
#[derive(Clone, Eq, Hash)]
pub struct NdRange<R, Idx, const N: usize>
where
  R: RangeBounds<Idx>,
{
  pub(crate) ranges: [R; N],
  pub(crate) _phantom: PhantomData<Idx>,
}

impl<R, Idx, const N: usize> std::fmt::Debug for NdRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
  R: std::fmt::Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[ ")?;
    for r in &self.ranges {
      r.fmt(f)?;
      write!(f, ", ")?;
    }
    write!(f, "]")?;
    Ok(())
  }
}

impl<R, Idx, const N: usize> PartialEq for NdRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
  R: PartialEq<R>,
{
  fn eq(&self, other: &Self) -> bool {
    for i in 0..N {
      if self.ranges[i] != other.ranges[i] {
        return false;
      }
    }
    return true;
  }
}

impl<R, Idx, const N: usize> NdRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
{
  pub fn new(ranges: [R; N]) -> Self {
    Self {
      ranges,
      _phantom: PhantomData,
    }
  }
}

impl<R, Idx, const N: usize> NdRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
  Idx: PartialOrd<Idx>,
{
  pub fn contains<U>(&self, other: &[U; N]) -> bool
  where
    Idx: PartialOrd<U>,
    U: PartialOrd<Idx>,
  {
    for i in 0..N {
      if !self.ranges[i].contains(&other[i]) {
        return false;
      }
    }
    return true;
  }
}

impl<R, Idx, const N: usize> NdRange<R, Idx, N>
where
  Self: Sized,
  R: RangeBounds<Idx>,
  R: ExactSizeIterator,
  R: Clone,
  Idx: Step,
{
  pub fn len(&self) -> usize {
    self.clone().into_iter().len()
  }
}

impl<R, Idx, const N: usize> IntoIterator for NdRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
  R: ExactSizeIterator,
  Idx: Step,
{
  type IntoIter = NdIter<R, Idx, N>;
  type Item = [Idx; N];

  fn into_iter(self) -> Self::IntoIter {
    let mut empty = N == 0;
    for i in 0..N {
      if self.ranges[i].len() == 0 {
        empty = true;
        break;
      }
    }

    match empty {
      true => NdIter::Done,
      false => NdIter::Iterating {
        current: std::array::from_fn(|i| get_bound_start(&self.ranges[i])),
        ndrange: self,
      },
    }
  }
}

impl<R, Idx, const N: usize> Default for NdRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
  R: Default,
{
  fn default() -> Self {
    Self {
      ranges: std::array::from_fn(|_| R::default()),
      _phantom: PhantomData,
    }
  }
}

#[macro_export]
macro_rules! ndrange {

  ( $( $x:expr ), + $(,)? ) => {
    NdRange::new([ $($x,)* ])
  };

}
