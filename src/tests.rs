use crate::biguint::BigUInt;

#[test]
fn default_is_zero() {
    let result = BigUInt::default();
    assert_eq!(result.digits(), vec![0]);
}

#[test]
fn converts_123() {
    let result = BigUInt::from(123u32);
    assert_eq!(result.digits(), vec![1, 2, 3]);
}

#[test]
fn converts_60() {
    let result = BigUInt::from(60u32);
    assert_eq!(result.digits(), vec![6, 0]);
}

#[test]
fn converts_4235253() {
    let result = BigUInt::from(4235253u32);
    assert_eq!(result.digits(), vec![4,2,3,5,2,5,3]);
}

#[test]
fn converts_18446744073709551615() {
    let result = BigUInt::from(18446744073709551615u64);
    assert_eq!(result.digits(), vec![1,8,4,4,6,7,4,4,0,7,3,7,0,9,5,5,1,6,1,5]);
}

#[test]
fn converts_text_18446744551615() {
    let result = BigUInt::from("18446744551615");
    assert_eq!(result.digits(), vec![1,8,4,4,6,7,4,4,5,5,1,6,1,5]);
}

#[test]
fn adds_0_123(){
    let a = BigUInt::from(0u32);
    let b = BigUInt::from(123u32);
    let result = &a + &b;
    assert_eq!(result, BigUInt::from(123u32));
}

#[test]
fn adds_642_8537(){
    let a = BigUInt::from(642u32);
    let b = BigUInt::from(8537u32);
    let result = &a + &b;
    assert_eq!(result, BigUInt::from(9179u32));
}

#[test]
fn shifts_642_by_8(){
    let a = BigUInt::from(642u32);
    let result = a.times_ten_in_place(8);
    assert_eq!(result, BigUInt::from(64_200_000_000u64));
}

#[test]
fn multiplies_642_8537(){
    let a = BigUInt::from(642u32);
    let b = BigUInt::from(8537u32);
    let result = &a * &b;
    assert_eq!(result, BigUInt::from(5_480_754u32));
}

#[test]
fn multiplies_6428537_8537642(){
    let a = BigUInt::from(6428537u32);
    let b = BigUInt::from(8537642u32);
    let result = &a * &b;
    assert_eq!(result, BigUInt::from(54_884_547_489_754u64));
}

#[test]
fn factorial_153() {
  let a = 153;
  let result = BigUInt::factorial(a);
  assert_eq!(result, BigUInt::from("200634390509568239477828874698911718566246149616161171934231099284840946025238092339613294062603588435530393145048663047173051913507711632216305667129554900620296603188543122491838966881134795135997316305640071571629943041039657861120000000000000000000000000000000000000"));
}

#[test]
fn stringifies_64285378537642(){
    let a = BigUInt::from(64285378537642u64);
    let result = std::convert::Into::<String>::into(a);
    assert_eq!(result, "64285378537642");
}