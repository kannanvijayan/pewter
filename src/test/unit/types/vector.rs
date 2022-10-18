
use crate::{
  types::{
    PrimType,
  },
};

#[test]
fn test_vector_u32x2() {
  test_vector_u32x2_value([0x3f901527_u32, 0x101_u32]);
  test_vector_u32x2_value([u32::max_value(), u32::min_value()]);
}

fn test_vector_u32x2_value(value: [u32; 2]) {
  let mut buf: [u8; 8] = Default::default();
  let mut read_value = [0_u32, 0_u32];

  value.write_bytes(&mut buf);
  read_value.read_bytes(&buf);
  assert!(read_value == value);
}

#[test]
fn test_vector_u32x3() {
  test_vector_u32x3_value([0x3f901527_u32, 0x101_u32, 0x5009915_u32]);
  test_vector_u32x3_value([u32::max_value(), u32::max_value() / 2, 0]);
}

fn test_vector_u32x3_value(value: [u32; 3]) {
  let mut buf: [u8; 12] = Default::default();
  let mut read_value = [0_u32, 0_u32, 0_u32];

  value.write_bytes(&mut buf);
  read_value.read_bytes(&buf);
  assert!(read_value == value);
}

#[test]
fn test_vector_u32x4() {
  test_vector_u32x4_value(
    [0x3f901527_u32, 0x101_u32, 0x5009915_u32, 0x181A5B3_u32]
  );
  test_vector_u32x4_value([
    u32::max_value(),
    u32::max_value() / 2,
    u32::max_value() / 3,
    u32::min_value()
  ]);
}

fn test_vector_u32x4_value(value: [u32; 4]) {
  let mut buf: [u8; 16] = Default::default();
  let mut read_value = [0_u32, 0_u32, 0_u32, 0_u32];

  value.write_bytes(&mut buf);
  read_value.read_bytes(&buf);
  assert!(read_value == value);
}