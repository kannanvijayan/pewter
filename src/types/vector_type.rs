
use super::{
  DataType,
  PrimType,
  PrimTypeInfo,
  ScalarType,
  DataTypeSpecialize,
  DataTypeSpecializeRef,
  DataTypeSpecializeMut,
};

pub trait VectorType: PrimType {
  const NAME: &'static str;
  const VECTOR_INFO: VectorTypeInfo;
  const SIZE: usize;
  type Field: ScalarType;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VectorTypeInfo {
  U32x2, U32x3, U32x4,
  I32x2, I32x3, I32x4,
  F32x2, F32x3, F32x4,
}
impl VectorTypeInfo {
  pub const fn into_prim(self) -> PrimTypeInfo {
    PrimTypeInfo::Vector(self)
  }
  pub const fn size(&self) -> usize {
    match *self {
      VectorTypeInfo::U32x2 => 8,
      VectorTypeInfo::U32x3 => 12,
      VectorTypeInfo::U32x4 => 16,

      VectorTypeInfo::I32x2 => 8,
      VectorTypeInfo::I32x3 => 12,
      VectorTypeInfo::I32x4 => 16,

      VectorTypeInfo::F32x2 => 8,
      VectorTypeInfo::F32x3 => 12,
      VectorTypeInfo::F32x4 => 16,
    }
  }
  pub const fn align(&self) -> usize {
    match *self {
      VectorTypeInfo::U32x2 => 8,
      VectorTypeInfo::U32x3 => 16,
      VectorTypeInfo::U32x4 => 16,

      VectorTypeInfo::I32x2 => 8,
      VectorTypeInfo::I32x3 => 16,
      VectorTypeInfo::I32x4 => 16,

      VectorTypeInfo::F32x2 => 8,
      VectorTypeInfo::F32x3 => 16,
      VectorTypeInfo::F32x4 => 16,
    }
  }
  pub const fn name(&self) -> &'static str {
    match *self {
      VectorTypeInfo::U32x2 => "vec2<u32>",
      VectorTypeInfo::U32x3 => "vec3<u32>",
      VectorTypeInfo::U32x4 => "vec4<u32>",

      VectorTypeInfo::I32x2 => "vec2<i32>",
      VectorTypeInfo::I32x3 => "vec3<i32>",
      VectorTypeInfo::I32x4 => "vec4<i32>",

      VectorTypeInfo::F32x2 => "vec2<f32>",
      VectorTypeInfo::F32x3 => "vec3<f32>",
      VectorTypeInfo::F32x4 => "vec4<f32>",
    }
  }
}

// U32
impl DataType for [u32;2] {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<[u32;2]>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<[u32;2]>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<[u32;2]>(self);
  }
}
impl PrimType for [u32;2] {
  const PRIM_INFO: PrimTypeInfo = Self::VECTOR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out[0..4].copy_from_slice(&self[0].to_le_bytes());
    bytes_out[4..8].copy_from_slice(&self[1].to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [[u8; 4]; 2] = Default::default();
    fixed[0].copy_from_slice(&bytes_in[0..4]);
    fixed[1].copy_from_slice(&bytes_in[4..8]);
    *self = [
      u32::from_le_bytes(fixed[0]),
      u32::from_le_bytes(fixed[1])
    ];
  }
}
impl VectorType for [u32;2] {
  const NAME: &'static str = "vec2<u32>";
  const VECTOR_INFO: VectorTypeInfo = VectorTypeInfo::U32x2;
  const SIZE: usize = 2;
  type Field = u32;
}

impl DataType for [u32;3] {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<[u32;3]>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<[u32;3]>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<[u32;3]>(self);
  }
}
impl PrimType for [u32;3] {
  const PRIM_INFO: PrimTypeInfo = Self::VECTOR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out[0..4].copy_from_slice(&self[0].to_le_bytes());
    bytes_out[4..8].copy_from_slice(&self[1].to_le_bytes());
    bytes_out[8..12].copy_from_slice(&self[2].to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [[u8; 4]; 3] = Default::default();
    fixed[0].copy_from_slice(&bytes_in[0..4]);
    fixed[1].copy_from_slice(&bytes_in[4..8]);
    fixed[2].copy_from_slice(&bytes_in[8..12]);
    *self = [
      u32::from_le_bytes(fixed[0]),
      u32::from_le_bytes(fixed[1]),
      u32::from_le_bytes(fixed[2])
    ];
  }
}
impl VectorType for [u32;3] {
  const NAME: &'static str = "vec3<u32>";
  const VECTOR_INFO: VectorTypeInfo = VectorTypeInfo::U32x3;
  const SIZE: usize = 3;
  type Field = u32;
}

impl DataType for [u32;4] {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<[u32;4]>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<[u32;4]>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<[u32;4]>(self);
  }
}
impl PrimType for [u32;4] {
  const PRIM_INFO: PrimTypeInfo = Self::VECTOR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out[0..4].copy_from_slice(&self[0].to_le_bytes());
    bytes_out[4..8].copy_from_slice(&self[1].to_le_bytes());
    bytes_out[8..12].copy_from_slice(&self[2].to_le_bytes());
    bytes_out[12..16].copy_from_slice(&self[3].to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [[u8; 4]; 4] = Default::default();
    fixed[0].copy_from_slice(&bytes_in[0..4]);
    fixed[1].copy_from_slice(&bytes_in[4..8]);
    fixed[2].copy_from_slice(&bytes_in[8..12]);
    fixed[3].copy_from_slice(&bytes_in[12..16]);
    *self = [
      u32::from_le_bytes(fixed[0]),
      u32::from_le_bytes(fixed[1]),
      u32::from_le_bytes(fixed[2]),
      u32::from_le_bytes(fixed[3]),
    ];
  }
}
impl VectorType for [u32;4] {
  const NAME: &'static str = "vec4<u32>";
  const VECTOR_INFO: VectorTypeInfo = VectorTypeInfo::U32x4;
  const SIZE: usize = 4;
  type Field = u32;
}

// I32
impl DataType for [i32;2] {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<[i32;2]>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<[i32;2]>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<[i32;2]>(self);
  }
}
impl PrimType for [i32;2] {
  const PRIM_INFO: PrimTypeInfo = Self::VECTOR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out[0..4].copy_from_slice(&self[0].to_le_bytes());
    bytes_out[4..8].copy_from_slice(&self[1].to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [[u8; 4]; 2] = Default::default();
    fixed[0].copy_from_slice(&bytes_in[0..4]);
    fixed[1].copy_from_slice(&bytes_in[4..8]);
    *self = [
      i32::from_le_bytes(fixed[0]),
      i32::from_le_bytes(fixed[1])
    ];
  }
}
impl VectorType for [i32;2] {
  const NAME: &'static str = "vec2<i32>";
  const VECTOR_INFO: VectorTypeInfo = VectorTypeInfo::I32x2;
  const SIZE: usize = 2;
  type Field = i32;
}

impl DataType for [i32;3] {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<[i32;3]>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<[i32;3]>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<[i32;3]>(self);
  }
}
impl PrimType for [i32;3] {
  const PRIM_INFO: PrimTypeInfo = Self::VECTOR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out[0..4].copy_from_slice(&self[0].to_le_bytes());
    bytes_out[4..8].copy_from_slice(&self[1].to_le_bytes());
    bytes_out[8..12].copy_from_slice(&self[2].to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [[u8; 4]; 3] = Default::default();
    fixed[0].copy_from_slice(&bytes_in[0..4]);
    fixed[1].copy_from_slice(&bytes_in[4..8]);
    fixed[2].copy_from_slice(&bytes_in[8..12]);
    *self = [
      i32::from_le_bytes(fixed[0]),
      i32::from_le_bytes(fixed[1]),
      i32::from_le_bytes(fixed[2])
    ];
  }
}
impl VectorType for [i32;3] {
  const NAME: &'static str = "vec3<i32>";
  const VECTOR_INFO: VectorTypeInfo = VectorTypeInfo::I32x3;
  const SIZE: usize = 3;
  type Field = i32;
}

impl DataType for [i32;4] {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<[i32;4]>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<[i32;4]>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<[i32;4]>(self);
  }
}
impl PrimType for [i32;4] {
  const PRIM_INFO: PrimTypeInfo = Self::VECTOR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out[0..4].copy_from_slice(&self[0].to_le_bytes());
    bytes_out[4..8].copy_from_slice(&self[1].to_le_bytes());
    bytes_out[8..12].copy_from_slice(&self[2].to_le_bytes());
    bytes_out[12..16].copy_from_slice(&self[3].to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [[u8; 4]; 4] = Default::default();
    fixed[0].copy_from_slice(&bytes_in[0..4]);
    fixed[1].copy_from_slice(&bytes_in[4..8]);
    fixed[2].copy_from_slice(&bytes_in[8..12]);
    fixed[3].copy_from_slice(&bytes_in[12..16]);
    *self = [
      i32::from_le_bytes(fixed[0]),
      i32::from_le_bytes(fixed[1]),
      i32::from_le_bytes(fixed[2]),
      i32::from_le_bytes(fixed[3]),
    ];
  }
}
impl VectorType for [i32;4] {
  const NAME: &'static str = "vec4<i32>";
  const VECTOR_INFO: VectorTypeInfo = VectorTypeInfo::I32x4;
  const SIZE: usize = 4;
  type Field = i32;
}

// F32
impl DataType for [f32;2] {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<[f32;2]>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<[f32;2]>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<[f32;2]>(self);
  }
}
impl PrimType for [f32;2] {
  const PRIM_INFO: PrimTypeInfo = Self::VECTOR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out[0..4].copy_from_slice(&self[0].to_le_bytes());
    bytes_out[4..8].copy_from_slice(&self[1].to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [[u8; 4]; 2] = Default::default();
    fixed[0].copy_from_slice(&bytes_in[0..4]);
    fixed[1].copy_from_slice(&bytes_in[4..8]);
    *self = [
      f32::from_le_bytes(fixed[0]),
      f32::from_le_bytes(fixed[1])
    ];
  }
}
impl VectorType for [f32;2] {
  const NAME: &'static str = "vec2<f32>";
  const VECTOR_INFO: VectorTypeInfo = VectorTypeInfo::F32x2;
  const SIZE: usize = 2;
  type Field = f32;
}

impl DataType for [f32;3] {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<[f32;3]>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<[f32;3]>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<[f32;3]>(self);
  }
}
impl PrimType for [f32;3] {
  const PRIM_INFO: PrimTypeInfo = Self::VECTOR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out[0..4].copy_from_slice(&self[0].to_le_bytes());
    bytes_out[4..8].copy_from_slice(&self[1].to_le_bytes());
    bytes_out[8..12].copy_from_slice(&self[2].to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [[u8; 4]; 3] = Default::default();
    fixed[0].copy_from_slice(&bytes_in[0..4]);
    fixed[1].copy_from_slice(&bytes_in[4..8]);
    fixed[2].copy_from_slice(&bytes_in[8..12]);
    *self = [
      f32::from_le_bytes(fixed[0]),
      f32::from_le_bytes(fixed[1]),
      f32::from_le_bytes(fixed[2])
    ];
  }
}
impl VectorType for [f32;3] {
  const NAME: &'static str = "vec3<f32>";
  const VECTOR_INFO: VectorTypeInfo = VectorTypeInfo::F32x3;
  const SIZE: usize = 3;
  type Field = f32;
}

impl DataType for [f32;4] {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<[f32;4]>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<[f32;4]>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<[f32;4]>(self);
  }
}
impl PrimType for [f32;4] {
  const PRIM_INFO: PrimTypeInfo = Self::VECTOR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out[0..4].copy_from_slice(&self[0].to_le_bytes());
    bytes_out[4..8].copy_from_slice(&self[1].to_le_bytes());
    bytes_out[8..12].copy_from_slice(&self[2].to_le_bytes());
    bytes_out[12..16].copy_from_slice(&self[3].to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [[u8; 4]; 4] = Default::default();
    fixed[0].copy_from_slice(&bytes_in[0..4]);
    fixed[1].copy_from_slice(&bytes_in[4..8]);
    fixed[2].copy_from_slice(&bytes_in[8..12]);
    fixed[3].copy_from_slice(&bytes_in[12..16]);
    *self = [
      f32::from_le_bytes(fixed[0]),
      f32::from_le_bytes(fixed[1]),
      f32::from_le_bytes(fixed[2]),
      f32::from_le_bytes(fixed[3]),
    ];
  }
}
impl VectorType for [f32;4] {
  const NAME: &'static str = "vec4<f32>";
  const VECTOR_INFO: VectorTypeInfo = VectorTypeInfo::F32x4;
  const SIZE: usize = 4;
  type Field = f32;
}