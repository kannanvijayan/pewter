use super::{DataType, PrimType, SharedRecordInfo, SharedName};


/**
 * Rust type that identifies to a wgpu struct type.
 */
pub trait RecordType: DataType {
  /** Iterate over all fields of the type. */
  fn specify_fields<FS: RecordTypeFieldSpecifier<Self>>(fs: &mut FS);
}

pub trait RecordTypeFieldSpecifier<RT: RecordType> {
  fn prim_field<FT, Get, Set>(&mut self, name: &str, get: Get, set: Set)
    where FT: PrimType,
          Get: 'static + Send + Sync + Fn (&RT) -> FT,
          Set: 'static + Send + Sync + Fn (&mut RT, FT);

  fn record_field<FT, Get, Set>(&mut self, name: &str, get: Get, set: Set)
    where FT: RecordType,
          Get: 'static + Send + Sync + Fn (&RT) -> FT,
          Set: 'static + Send + Sync + Fn (&mut RT, FT);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RecordTypeInfo {
  record_info: SharedRecordInfo,
}
impl RecordTypeInfo {
  pub(crate) fn new(record_info: SharedRecordInfo) -> Self {
    RecordTypeInfo { record_info }
  }
  pub fn size(&self) -> usize {
    self.record_info.size()
  }
  pub fn align(&self) -> usize {
    self.record_info.align()
  }
  pub fn name(&self) -> &SharedName {
    &self.record_info.name()
  }
}