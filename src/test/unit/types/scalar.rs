
use crate::{
  types::{
    PrimType,
  },
};

#[test]
fn test_scalar_u32() {
  test_scalar_u32_value(0x3f901527_u32);
  test_scalar_u32_value(u32::max_value());
  test_scalar_u32_value(u32::min_value());
  test_scalar_u32_value(0b10101010_10101010_10101010_u32);
}

fn test_scalar_u32_value(value: u32) {
  let mut buf: [u8; 4] = Default::default();
  let mut read_value = 0_u32;

  value.write_bytes(&mut buf);
  read_value.read_bytes(&buf);
  assert!(read_value == value);
}

#[test]
fn test_scalar_i32() {
  test_scalar_i32_value(0x3f901527_i32);
  test_scalar_i32_value(-99_i32);
  test_scalar_i32_value(i32::max_value());
  test_scalar_i32_value(i32::min_value());
  test_scalar_i32_value(0b10101010_10101010_10101010_i32);
}

fn test_scalar_i32_value(value: i32) {
  let mut buf: [u8; 4] = Default::default();
  let mut read_value = 0_i32;

  value.write_bytes(&mut buf);
  read_value.read_bytes(&buf);
  assert!(read_value == value);
}

#[test]
fn test_scalar_f32() {
  test_scalar_f32_value(-0.0);
  test_scalar_f32_value(0.0);
  test_scalar_f32_value(1.0);
  test_scalar_f32_value(-1.0);
  test_scalar_f32_value(f32::NEG_INFINITY);
  test_scalar_f32_value(f32::INFINITY);
  test_scalar_f32_value(f32::MIN_POSITIVE);
  test_scalar_f32_value(f32::MAX);
  test_scalar_f32_value(f32::MIN);
  test_scalar_f32_value(2099.2251);
  test_scalar_f32_value(-41.7);
}

fn test_scalar_f32_value(value: f32) {
  let mut buf: [u8; 4] = Default::default();
  let mut read_value = 0_f32;

  value.write_bytes(&mut buf);
  read_value.read_bytes(&buf);
  assert!(read_value == value);
}