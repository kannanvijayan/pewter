use super::{
  DataType,
  ScalarTypeInfo,
  VectorTypeInfo, DataTypeInfo
};

/**
 * Unification of record types and primitive types.
 */
pub trait PrimType: DataType {
  const PRIM_INFO: PrimTypeInfo;
  fn write_bytes(&self, bytes_out: &mut [u8]);
  fn read_bytes(&mut self, bytes_in: & [u8]);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PrimTypeInfo {
  Scalar(ScalarTypeInfo),
  Vector(VectorTypeInfo),
}
impl PrimTypeInfo {
  pub(crate) fn into_data_type(self) -> DataTypeInfo {
    DataTypeInfo::Prim(self)
  }
  pub(crate) fn name(self) -> &'static str {
    match self {
      PrimTypeInfo::Scalar(info) => info.name(),
      PrimTypeInfo::Vector(info) => info.name(),
    }
  }
}
impl PrimTypeInfo {
  pub const fn size(&self) -> usize {
    match self {
      &PrimTypeInfo::Scalar(ref scalar_info) => scalar_info.size(),
      &PrimTypeInfo::Vector(ref vector_info) => vector_info.size(),
    }
  }
  pub const fn align(&self) -> usize {
    match self {
      &PrimTypeInfo::Scalar(ref scalar_info) => scalar_info.align(),
      &PrimTypeInfo::Vector(ref vector_info) => vector_info.align(),
    }
  }
}