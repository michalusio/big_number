use std::{iter::{self, Sum}, ops::{Add, Mul}};

use crate::{utils::log10_floor, times::times};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BigUInt(Vec<u8>);

impl Default for BigUInt {
  fn default() -> Self {
    BigUInt(vec![0])
  }
}

impl From<u64> for BigUInt {
  fn from(a: u64) -> Self {
    let digits = (log10_floor(a) + 1) as usize;
    let mut result = Vec::with_capacity(digits);
    let mut a = a;
    loop {
        let digit = a % 10;
        result.push(digit as u8);
        a /= 10;
        if a == 0 { break; }
    }
    result.reverse();
    BigUInt(result)
  }
}

impl From<u32> for BigUInt {
  fn from(a: u32) -> Self {
    BigUInt::from(a as u64)
  }
}

impl From<usize> for BigUInt {
  fn from(a: usize) -> Self {
    BigUInt::from(a as u64)
  }
}

impl From<&str> for BigUInt {
  fn from(text: &str) -> Self {
    let result = text.chars()
      .inspect(|c| {
        if !c.is_ascii_digit() && *c != '_' {
          panic!("The string can contain only digits from 0 to 9, got {}.", c);
        }
      })
      .filter_map(|c| c.to_digit(10))
      .map(|d| d as u8)
      .collect();
    BigUInt(result)
  }
}

impl From<BigUInt> for String {
  fn from(n: BigUInt) -> Self {
    let mut str = String::with_capacity(n.0.len());
    str.extend(n.0.into_iter().map(|d| (d + b'0') as char));
    str
  }
}

impl<'a, 'b> Add<&'b BigUInt> for &'a BigUInt {
  type Output = BigUInt;

  fn add(self, rhs: &'b BigUInt) -> Self::Output {
    self.add_shifted(rhs, 0)
  }
}

impl<'a> Sum<&'a BigUInt> for BigUInt {
  fn sum<I: Iterator<Item = &'a BigUInt>>(iter: I) -> Self {
    iter.fold(BigUInt::default(), |x, y| &x + y)
  }
}

impl Sum<BigUInt> for BigUInt {
  fn sum<I: Iterator<Item = BigUInt>>(iter: I) -> Self {
    iter.fold(BigUInt::default(), |x, y| &x + &y)
  }
}

impl<'a, 'b> Mul<&'b BigUInt> for &'a BigUInt {
  type Output = BigUInt;

  fn mul(self, rhs: &'b BigUInt) -> Self::Output {
    let min = if self.0.len() > rhs.0.len() { rhs } else { self };
    let max = if self.0.len() > rhs.0.len() { self } else { rhs };
    let times_table = setup_times(max.clone());
    min.0.iter()
      .rev()
      .enumerate()
      .map(|(i, d)| (&times_table[*d as usize], i))
      .fold(BigUInt::default(), |total, (p, i)| total.add_shifted(p, i))
  }
}

fn setup_times(x: BigUInt) -> [BigUInt; 10] {
  let x0 = BigUInt::default();
  let x2 = &x + &x;
  let x3 = &x2 + &x;
  let x1 = x;
  let x4 = &x2 + &x2;
  let x5 = &x3 + &x2;
  let x6 = &x3 + &x3;
  let x7 = &x4 + &x3;
  let x8 = &x4 + &x4;
  let x9 = &x5 + &x4;
  [
    x0,
    x1,
    x2,
    x3,
    x4,
    x5,
    x6,
    x7,
    x8,
    x9
  ]
}

impl BigUInt {

  pub fn digits(self) -> Vec<u8> {
    self.0
  }

  pub fn factorial(a: u64) -> BigUInt {
    if a == 0 { return BigUInt(vec![1]); }
    let mut result = BigUInt::from(1u32);
    let mut next = BigUInt::from(2u32);
    let one = BigUInt::from(1u32);
    for _ in 1..a {
      result = &result * &next;
      next = &next + &one;
    }
    result
  }

  pub fn times_ten(this: &BigUInt, shift: usize) -> BigUInt {
    this.clone().times_ten_in_place(shift)
  }

  pub fn times_ten_in_place(self: BigUInt, shift: usize) -> BigUInt {
    let mut new_number = self.0;
    new_number.resize(new_number.len() + shift, 0);
    BigUInt(new_number)
  }

  /// Adds two numbers, with the second being shifted left (multiplied by 10) `shift` times.
  /// 
  /// The shift does not modify the second number.
  pub fn add_shifted<'a, 'b>(self: &'a BigUInt, rhs: &'b BigUInt, shift: usize) -> BigUInt {
    let capacity = self.0.len().max(rhs.0.len() + shift) + 1;
    let mut result = Vec::with_capacity(capacity);

    let temp = if self.0.len() > (rhs.0.len() + shift) { &self.0 } else { &rhs.0 };
    let (b, a) = if std::ptr::eq(temp, &rhs.0) {
      (
        self.0.iter().chain(times(0).map(zero_closure)),
        temp.iter().chain(times(shift).map(zero_closure))
      )
    } else {
      (
        rhs.0.iter().chain(times(shift).map(zero_closure)),
        temp.iter().chain(times(0).map(zero_closure))
      )
    };

    let mut carry = 0;

    let digit_adder = a
    .rev()
    .zip(b.rev().chain(iter::repeat(&0))).map(|(digit_a, digit_b)| {
        let sum = *digit_a + *digit_b + carry;
        carry = ((sum << 3) + (sum << 2) + sum) >> 7u8;
        sum.wrapping_sub((carry << 3) + (carry << 1))
    });
    result.extend(digit_adder);
    if carry > 0 {
        result.push(carry)
    }
    result.reverse();
    debug_assert!(capacity >= result.len());
    BigUInt(result)
  }
}

fn zero_closure<'a>(_: usize) -> &'a u8 {
  &0u8
}