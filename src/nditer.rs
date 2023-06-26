use std::{
  iter::{FusedIterator, Step},
  ops::RangeBounds,
};

use crate::{get_bound_start, ndrange::NdRange};

pub enum NdIter<R, T: Step, const N: usize>
where
  R: RangeBounds<T>,
  R: ExactSizeIterator,
  Self: Sized,
{
  Iterating {
    ndrange: NdRange<R, T, N>,
    current: [T; N],
  },
  Done,
}

impl<R, T, const N: usize> Iterator for NdIter<R, T, N>
where
  R: RangeBounds<T>,
  R: ExactSizeIterator,
  T: Step,
{
  type Item = [T; N];

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      NdIter::Iterating { ndrange, current } => {
        let ret = current.clone();
        for i in 0..current.len() {
          let next = T::forward(current[i].clone(), 1);
          if ndrange.ranges[i].contains(&next) {
            current[i] = next.clone();
            break;
          } else {
            current[i] = get_bound_start(&ndrange.ranges[i]);
            if i == (current.len() - 1) {
              *self = Self::Done;
              break;
            }
          }
        }
        Some(ret)
      },
      NdIter::Done => None,
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let mut size = 1;
    match self {
      NdIter::Iterating { ndrange, .. } => {
        for i in &ndrange.ranges {
          size *= i.len();
        }
        (size, Some(size))
      },
      NdIter::Done => (0, None),
    }
  }
}

impl<R, T, const N: usize> ExactSizeIterator for NdIter<R, T, N>
where
  R: RangeBounds<T>,
  R: ExactSizeIterator,
  T: Step,
{
  fn len(&self) -> usize {
    self.size_hint().0
  }
}

impl<R, T, const N: usize> FusedIterator for NdIter<R, T, N>
where
  R: RangeBounds<T>,
  R: ExactSizeIterator,
  R: FusedIterator,
  T: Step,
{
}
