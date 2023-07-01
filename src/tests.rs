use super::*;
extern crate test;
use std::hint::black_box;
use test::Bencher;

#[test]
fn eq() {
  let arr = [
    nrange!(1..3, 1..3),
    nrange!(1..3; 2),
    NRange::new([1..3, 1..3]),
  ];

  for a in &arr {
    for b in &arr {
      assert_eq!(a, b);
    }
  }
}

#[test]
fn order() {
  let mut nd = nrange!(0..2, 0..2).into_iter();
  assert_eq!(nd.next(), Some([0, 0]));
  assert_eq!(nd.next(), Some([1, 0]));
  assert_eq!(nd.next(), Some([0, 1]));
}

#[test]
fn emptiness() {
  let nd = nrange!(0..100, 0..99999, 0..0).into_iter();
  assert_eq!(nd.len(), 0);
}

#[test]
fn length() {
  let nd = nrange!(0..100, 0..100, 0..33).into_iter();
  assert_eq!(nd.len(), 100 * 100 * 33);
}

#[bench]
fn nd_benchmark(b: &mut Bencher) {
  b.iter(|| {
    for i in nrange!(0..100; 3).into_iter() {
      black_box(i);
    }
  });
}

#[bench]
fn linear_benchmark(b: &mut Bencher) {
  b.iter(|| {
    for x in (0..100).into_iter() {
      for y in (0..100).into_iter() {
        for z in (0..100).into_iter() {
          black_box(&[x, y, z]);
        }
      }
    }
  });
}
