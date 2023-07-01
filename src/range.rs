use std::{iter::Step, marker::PhantomData, ops::RangeBounds};

use crate::{get_real_bound, iter::NRangeIter};

/// N-dimensional range
/// R: any range type implementing RangeBounds<Idx>
/// Idx: range bound type (see [`std::ops::Range`])
/// N: dimensionality of the range (the *N* in `NRange`)
#[derive(Clone, Eq, Hash)]
pub struct NRange<R, Idx, const N: usize>
where
  R: RangeBounds<Idx>,
{
  pub bounds: [R; N],
  pub(crate) _phantom: PhantomData<Idx>,
}

impl<R, Idx, const N: usize> NRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
{
  pub fn new(ranges: [R; N]) -> Self {
    Self {
      bounds: ranges,
      _phantom: PhantomData,
    }
  }
}

impl<R, Idx, const N: usize> std::fmt::Debug for NRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
  R: std::fmt::Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.bounds.fmt(f)?;
    Ok(())
  }
}

impl<R, Idx, const N: usize> PartialEq for NRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
  R: PartialEq<R>,
{
  fn eq(&self, other: &Self) -> bool {
    for i in 0..N {
      if self.bounds[i] != other.bounds[i] {
        return false;
      }
    }
    return true;
  }
}

impl<R, Idx, const N: usize> NRange<R, Idx, N>
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
      if !self.bounds[i].contains(&other[i]) {
        return false;
      }
    }
    return true;
  }
}

impl<R, Idx, const N: usize> NRange<R, Idx, N>
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

impl<R, Idx, const N: usize> IntoIterator for NRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
  R: ExactSizeIterator,
  Idx: Step,
{
  type IntoIter = NRangeIter<R, Idx, N>;
  type Item = [Idx; N];

  fn into_iter(self) -> Self::IntoIter {
    let mut empty = N == 0;
    for i in 0..N {
      if self.bounds[i].len() == 0 {
        empty = true;
        break;
      }
    }

    match empty {
      true => NRangeIter::Done,
      false => NRangeIter::Iterating {
        current: std::array::from_fn(|i| {
          get_real_bound(self.bounds[i].start_bound())
        }),
        nrange: self,
      },
    }
  }
}

impl<R, Idx, const N: usize> Default for NRange<R, Idx, N>
where
  R: RangeBounds<Idx>,
  R: Default,
{
  fn default() -> Self {
    Self {
      bounds: std::array::from_fn(|_| R::default()),
      _phantom: PhantomData,
    }
  }
}
#[macro_use]
pub mod create_macro {
  #[macro_export]
  macro_rules! nrange {
    () => {compile_error!("NRange cannot have 0 dimensions!")};

    ($( $x:expr ), + $(,)?) => {
      $crate::range::NRange::new([ $($x,)* ])
    };

    ($elem:expr; $n:expr) => (
      {
        let arr: [_; $n] = std::array::from_fn(|_| {$elem});
        $crate::range::NRange::new(arr)
      }
    );
  }
}
