use std::{iter::{self, Sum}, ops::{Add, Mul}};

use crate::utils::log10_floor;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BigUInt(Vec<u8>);

impl Default for BigUInt {
    fn default() -> Self {
        BigUInt(vec![0])
    }
}

impl From<u64> for BigUInt {
    fn from(a: u64) -> Self {
        let mut result = Vec::with_capacity((log10_floor(a) + 1) as usize);
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
  fn from(a: &str) -> Self {
    BigUInt::from_str(a, 10)
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
        let mut result = Vec::with_capacity(self.0.len().max(rhs.0.len()) + 1);

        let temp = if self.0.len() > rhs.0.len() { &self.0 } else { &rhs.0 };
        let b = if temp == &rhs.0 { &self.0 } else { &rhs.0 };
        let a = temp;

        let mut carry = 0;

        for digits in a.iter()
        .rev()
        .zip(b.iter().rev().chain(iter::repeat(&0))) {
            let sum = *digits.0 + *digits.1 + carry;
            carry = sum / 10;
            let digit = sum % 10;
            result.push(digit);
        }
        if carry > 0 {
            result.push(carry)
        }
        result.reverse();
        BigUInt(result)
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
        let mut times_table: [Option<BigUInt>; 10] = Default::default();
        let min = if self.0.len() > rhs.0.len() { rhs } else { self };
        let max = if self.0.len() > rhs.0.len() { self } else { rhs };
        min.0.iter().rev().enumerate().map(|(i, d)| {
          let timed = get_times(&mut times_table, max, (*d).into());
          timed.times_ten_in_place(i)
        }).sum()
    }
}

fn get_times<'a, 'b: 'a>(table: &'b mut [Option<BigUInt>; 10], x: &'a BigUInt, times: usize) -> BigUInt {
    if table[times].is_none() {
        table[times] = Some(match times {
            0 => BigUInt::default(),
            1 => x.to_owned(),
            2 => x + x,
            3 => &get_times(table, x, 2) + x,
            4 => &get_times(table, x, 2) + &get_times(table, x, 2),
            5 => &get_times(table, x, 3) + &get_times(table, x, 2),
            6 => &get_times(table, x, 3) + &get_times(table, x, 3),
            7 => &get_times(table, x, 3) + &get_times(table, x, 4),
            8 => &get_times(table, x, 4) + &get_times(table, x, 4),
            _ => &get_times(table, x, 5) + &get_times(table, x, 4),
        });
    }
    table[times].clone().unwrap()
}

impl BigUInt {

  pub fn digits(self) -> Vec<u8> {
    self.0
  }

  fn from_str(text: &str, radix: u32) -> BigUInt {
    let result = text.chars()
      .inspect(|c| {
        if !c.is_digit(radix) && *c != '_' {
          panic!("The string can contain only digits and underscores, got {}.", c);
        }
      })
      .filter_map(|c| c.to_digit(radix))
      .map(|d| d as u8)
      .collect();
    BigUInt(result)
  }

  pub fn factorial(a: u64) -> BigUInt {
      if a == 0 { return BigUInt(vec![1]); }
      &Self::factorial(a - 1) * &BigUInt::from(a)
  }

  pub fn times_ten(this: &BigUInt, shift: usize) -> BigUInt {
      this.clone().times_ten_in_place(shift)
  }

  pub fn times_ten_in_place(self: BigUInt, shift: usize) -> BigUInt {
      let mut new_number = self.0;
      new_number.resize(new_number.len() + shift, 0);
      BigUInt(new_number)
  }
}