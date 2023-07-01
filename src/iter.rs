use std::{
  iter::{FusedIterator, Step},
  ops::RangeBounds,
};

use crate::{get_real_bound, range::NRange};

pub enum NRangeIter<R, T: Step, const N: usize>
where
  R: RangeBounds<T>,
  R: ExactSizeIterator,
  Self: Sized,
{
  Iterating {
    nrange: NRange<R, T, N>,
    current: [T; N],
  },
  Done,
}

impl<R, T, const N: usize> Iterator for NRangeIter<R, T, N>
where
  R: RangeBounds<T>,
  R: ExactSizeIterator,
  T: Step,
{
  type Item = [T; N];

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      NRangeIter::Iterating {
        nrange: ndrange,
        current,
      } => {
        let ret = current.clone();
        for i in 0..current.len() {
          let next = T::forward(current[i].clone(), 1);
          if ndrange.bounds[i].contains(&next) {
            current[i] = next.clone();
            break;
          } else {
            current[i] = get_real_bound(ndrange.bounds[i].start_bound());
            if i == (N - 1) {
              *self = Self::Done;
              break;
            }
          }
        }
        Some(ret)
      },
      NRangeIter::Done => None,
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let mut size = 1;
    match self {
      NRangeIter::Iterating {
        nrange: ndrange, ..
      } => {
        for i in &ndrange.bounds {
          size *= i.len();
        }
        (size, Some(size))
      },
      NRangeIter::Done => (0, None),
    }
  }
}

impl<R, T, const N: usize> ExactSizeIterator for NRangeIter<R, T, N>
where
  R: RangeBounds<T>,
  R: ExactSizeIterator,
  T: Step,
{
  fn len(&self) -> usize {
    self.size_hint().0
  }
}

impl<R, T, const N: usize> FusedIterator for NRangeIter<R, T, N>
where
  R: RangeBounds<T>,
  R: ExactSizeIterator,
  R: FusedIterator,
  T: Step,
{
}
