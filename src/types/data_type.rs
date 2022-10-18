
use std::fmt::Debug;
use super::{
  PrimTypeInfo,
  PrimType,
  RecordType,
  RecordTypeInfo,
  record_info_for_type,
};


/**
 * Unification of record types and primitive types.
 */
pub trait DataType: 'static + Sized + Clone + Copy + Debug + Default {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result;
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S);
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S);
}

/**
 * Callback types to use with `specialize...` methods on `DataType`.
 */
pub trait DataTypeSpecialize {
  type Result;
  fn record<RT: RecordType>(self) -> Self::Result;
  fn prim<PT: PrimType>(self) -> Self::Result;
}
pub trait DataTypeSpecializeRef {
  fn record<RT: RecordType>(self, rec: &RT);
  fn prim<PT: PrimType>(self, prim: &PT);
}
pub trait DataTypeSpecializeMut {
  fn record<RT: RecordType>(self, rec: &mut RT);
  fn prim<PT: PrimType>(self, prim: &mut PT);
}



/**
 * Write a data value out to a GPU-formatted byte buffer.
 */
pub fn data_type_write<DT: DataType>(dt: &DT, bytes_out: &mut [u8]) {
  dt.specialize_ref(WriteSpecializer { bytes_out });
}
struct WriteSpecializer<'a> {
  bytes_out: &'a mut [u8],
}
impl<'a> DataTypeSpecializeRef for WriteSpecializer<'a> {
  fn prim<PT: PrimType>(self, prim: &PT) {
    prim.write_bytes(self.bytes_out);
  }
  fn record<RT: RecordType>(self, rec: &RT) {
    record_info_for_type::<RT>().write_value(rec, self.bytes_out);
  }
}

/**
 * Read a data value from a GPU-formatted byte buffer.
 */
pub fn data_type_read<DT: DataType>(dt_out: &mut DT, bytes: &[u8]) {
  dt_out.specialize_mut(ReadSpecializer { bytes });
}

struct ReadSpecializer<'a> {
  bytes: &'a [u8],
}
impl<'a> DataTypeSpecializeMut for ReadSpecializer<'a> {
  fn prim<PT: PrimType>(self, prim: &mut PT) {
    prim.read_bytes(self.bytes);
  }
  fn record<RT: RecordType>(self, rec: &mut RT) {
    record_info_for_type::<RT>().read_value(rec, self.bytes);
  }
}

/**
 * Runtime type-information for `DataType`.
 */
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataTypeInfo {
  Prim(PrimTypeInfo),
  Record(RecordTypeInfo),
}
impl DataTypeInfo {
  pub fn size(&self) -> usize {
    match self {
      &DataTypeInfo::Prim(ref prim) => prim.size(),
      &DataTypeInfo::Record(ref record) => record.size(),
    }
  }
  pub fn align(&self) -> usize {
    match self {
      &DataTypeInfo::Prim(ref prim) => prim.align(),
      &DataTypeInfo::Record(ref record) => record.align(),
    }
  }
  pub fn name(&self) -> &str {
    match self {
      &DataTypeInfo::Prim(info) => info.name(),
      &DataTypeInfo::Record(ref record_type) => record_type.name(),
    }
  }
}

pub(crate) fn data_type_info_for<DT: DataType>() -> DataTypeInfo {
  DT::specialize(InfoSpecializer)
}
struct InfoSpecializer;
impl DataTypeSpecialize for InfoSpecializer {
  type Result = DataTypeInfo;
  fn prim<PT: PrimType>(self) -> DataTypeInfo {
    PT::PRIM_INFO.into_data_type()
  }
  fn record<RT: RecordType>(self) -> DataTypeInfo {
    record_info_for_type::<RT>().into_untyped().into_data_type()
  }
}