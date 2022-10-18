use super::{
  PrimType,
  DataType,
  PrimTypeInfo,
  DataTypeSpecialize, DataTypeSpecializeRef, DataTypeSpecializeMut,
};

pub trait ScalarType: PrimType {
  const NAME: &'static str;
  const SCALAR_INFO: ScalarTypeInfo;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ScalarTypeInfo {
  U32, I32, F32,
}
impl ScalarTypeInfo {
  pub const fn into_prim(self) -> PrimTypeInfo {
    PrimTypeInfo::Scalar(self)
  }
  pub const fn size(&self) -> usize { 4 }
  pub const fn align(&self) -> usize { 4 }
  pub const fn name(&self) -> &'static str {
    match *self {
      ScalarTypeInfo::F32 => "f32",
      ScalarTypeInfo::U32 => "u32",
      ScalarTypeInfo::I32 => "i32",
    }
  }
}

impl DataType for u32 {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<u32>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<u32>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<u32>(self);
  }
}
impl PrimType for u32 {
  const PRIM_INFO: PrimTypeInfo = Self::SCALAR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out.copy_from_slice(&self.to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [u8; 4] = Default::default();
    fixed.copy_from_slice(bytes_in);
    *self = u32::from_le_bytes(fixed);
  }
}
impl ScalarType for u32 {
  const NAME: &'static str = &"u32";
  const SCALAR_INFO: ScalarTypeInfo = ScalarTypeInfo::U32;
}

impl DataType for i32 {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<i32>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<i32>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<i32>(self);
  }
}
impl PrimType for i32 {
  const PRIM_INFO: PrimTypeInfo = Self::SCALAR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out.copy_from_slice(&self.to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [u8; 4] = Default::default();
    fixed.copy_from_slice(bytes_in);
    *self = i32::from_le_bytes(fixed);
  }
}
impl ScalarType for i32 {
  const NAME: &'static str = &"i32";
  const SCALAR_INFO: ScalarTypeInfo = ScalarTypeInfo::I32;
}

impl DataType for f32 {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.prim::<f32>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.prim::<f32>(self);
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.prim::<f32>(self);
  }
}
impl PrimType for f32 {
  const PRIM_INFO: PrimTypeInfo = Self::SCALAR_INFO.into_prim();
  fn write_bytes(&self, bytes_out: &mut [u8]) {
    bytes_out.copy_from_slice(&self.to_le_bytes());
  }
  fn read_bytes(&mut self, bytes_in: &[u8]) {
    let mut fixed: [u8; 4] = Default::default();
    fixed.copy_from_slice(bytes_in);
    *self = f32::from_le_bytes(fixed);
  }
}
impl ScalarType for f32 {
  const NAME: &'static str = &"f32";
  const SCALAR_INFO: ScalarTypeInfo = ScalarTypeInfo::F32;
}