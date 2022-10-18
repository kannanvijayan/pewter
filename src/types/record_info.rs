
use std::{
  fmt, mem,
  any::TypeId,
  collections::HashMap,
  marker::PhantomData,
  sync::{Arc, Mutex},
};
use super::{
  SharedName,
  DataTypeInfo,
  PrimType,
  RecordType,
  RecordTypeFieldSpecifier,
  DataType,
  RecordTypeInfo,
  intern_name,
  data_type_write,
  data_type_read,
};

/**
 * Information about a record type.
 */
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RecordInfo {
  name: SharedName,
  fields: Vec<RecordFieldInfo>,
  size: usize,
  align: usize,
}
impl RecordInfo {
  pub(crate) fn size(&self) -> usize { self.size }
  pub(crate) fn align(&self) -> usize { self.align }
  pub(crate) fn name(&self) -> &SharedName { &self.name }

  pub(crate) unsafe fn write_value<RT>(&self,
    rec: &RT,
    bytes_out: &mut [u8]
  ) where RT: RecordType {
    let rec_bytes: &u8 = mem::transmute(rec);
    for field in self.fields.iter() {
      (field.writer)(
        rec_bytes,
        &mut bytes_out[field.offset .. field.end_offset()]
      );
    }
  }
  pub(crate) unsafe fn read_value<RT>(&self,
    rec: &mut RT,
    bytes: &[u8]
  ) where RT: RecordType {
    let rec_bytes: &mut u8 = mem::transmute(rec);
    for field in self.fields.iter() {
      (field.reader)(
        rec_bytes,
        &bytes[field.offset .. field.end_offset()]
      );
    }
  }
}

/* Global cache of all `T: RecordType` to `SharedRecordInfo` */
lazy_static::lazy_static! {
  static ref SHARED_RECORD_INFOS
    : Arc<Mutex<HashMap<TypeId, SharedRecordInfo>>>
    = Arc::new(Mutex::new(HashMap::new()));
}

/**
 * Get the shared record information for a `RecordType`.
 */
pub(crate) fn record_info_for_type<RT: RecordType>()
  -> TypedRecordInfo<RT>
{
  let type_id = TypeId::of::<RT>();
  {
    let locked = SHARED_RECORD_INFOS.lock()
      .expect("Failed to lock SHARED_RECORD_INFOS");
    if let Some(record_info) = locked.get(&type_id) {
      return TypedRecordInfo {
        inner: record_info.clone(),
        _dummy: PhantomData,
      };
    }
  }

  // Build the record info outside of the lock acquire
  // because it may recursively call `record_info_for_type`.
  let record_info = SharedRecordInfo(
    Arc::new(build_record_info::<RT>())
  );

  {
    let mut locked = SHARED_RECORD_INFOS.lock()
      .expect("Failed to lock SHARED_RECORD_INFOS");
    let inner = if let Some(record_info) = locked.get(&type_id) {
      record_info.clone()
    } else {
      locked.insert(type_id, record_info.clone());
      record_info
    };
    return TypedRecordInfo { inner, _dummy: PhantomData };
  }
}

fn build_record_info<RT: RecordType>() -> RecordInfo {
  let mut builder = Builder {
    name: intern_name(std::any::type_name::<RT>()),
    fields: Vec::new(),
    cur_size: 0,
    max_align: 0,
  };
  RT::specify_fields(&mut builder);
  builder.finish()
}

pub(crate) struct RecordFieldInfo {
  name: SharedName,
  data_type: DataTypeInfo,
  offset: usize,

  writer: Box<dyn Send + Sync + Fn (&u8, &mut [u8])>,
  reader: Box<dyn Send + Sync + Fn (&mut u8, &[u8])>,
}
impl RecordFieldInfo {
  fn end_offset(&self) -> usize {
    self.offset + self.data_type.size()
  }
}
impl fmt::Debug for RecordFieldInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "RecordFieldInfo {{ name: {:?}, data_type: {:?}, offset: {:?} }}",
      &self.name, &self.data_type, self.offset)
  }
}
impl std::cmp::PartialEq for RecordFieldInfo {
  fn eq(&self, other: &Self) -> bool {
    (&self.name, &self.data_type, &self.offset)
      == (&other.name, &other.data_type, &other.offset)
  }
}
impl std::hash::Hash for RecordFieldInfo {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    (&self.name, &self.data_type, self.offset).hash(state);
  }
}
impl Eq for RecordFieldInfo {}

/**
 * Shared handle to a record type.
 */
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SharedRecordInfo(Arc<RecordInfo>);
impl SharedRecordInfo {
  pub(crate) fn into_data_type(self) -> DataTypeInfo {
    DataTypeInfo::Record(RecordTypeInfo::new(self))
  }
}
impl std::ops::Deref for SharedRecordInfo {
  type Target = RecordInfo;
  fn deref(&self) -> &RecordInfo { &*self.0 }
}

/**
 * Shared, typed handle to a record type.
 */
pub struct TypedRecordInfo<RT: RecordType> {
  inner: SharedRecordInfo,
  _dummy: PhantomData<RT>,
}
impl<RT: RecordType> TypedRecordInfo<RT> {
  pub(crate) fn into_untyped(self) -> SharedRecordInfo { self.inner }
  pub(crate) fn untyped(&self) -> &SharedRecordInfo { &self.inner }
  pub(crate) fn write_value(&self, rec: &RT, bytes_out: &mut [u8]) {
    unsafe { self.inner.write_value(rec, bytes_out) };
  }
  pub(crate) fn read_value(&self, rec: &mut RT, bytes: &[u8]) {
    unsafe { self.inner.read_value(rec, bytes) };
  }
}
impl<RT: RecordType> std::ops::Deref for TypedRecordInfo<RT> {
  type Target = SharedRecordInfo;
  fn deref(&self) -> &SharedRecordInfo { &self.inner }
}

/* Helper to build a `RecordInfo` from a `RecordType`. */
struct Builder {
  name: SharedName,
  fields: Vec<RecordFieldInfo>,
  cur_size: usize,
  max_align: usize,
}
impl Builder {
  fn add_field(&mut self,
    name: &str,
    data_type: DataTypeInfo,
    writer: Box<dyn Send + Sync + Fn (&u8, &mut [u8])>,
    reader: Box<dyn Send + Sync + Fn (&mut u8, &[u8])>,
  ) {
    let name = intern_name(name);
    let align = data_type.align();
    let offset = (self.cur_size + (align - 1)) & !(align - 1);
    let size = data_type.size();
    let field_info = RecordFieldInfo {
      name,
      data_type,
      offset,
      writer,
      reader 
    };
    self.fields.push(field_info);
    self.cur_size = offset + size;
    self.max_align = usize::max(self.max_align, align);
  }
  fn finish(self) -> RecordInfo {
    let max_align = self.max_align;
    RecordInfo {
      name: self.name,
      fields: self.fields,
      size: (self.cur_size + (max_align - 1)) & !(max_align - 1),
      align: max_align,
    }
  }
}
impl<RT: RecordType> RecordTypeFieldSpecifier<RT> for Builder {
  fn prim_field<FT, Get, Set>(&mut self, name: &str, get: Get, set: Set)
    where FT: PrimType,
          Get: 'static + Send + Sync + Fn (&RT) -> FT,
          Set: 'static + Send + Sync + Fn (&mut RT, FT)
  {
    self.add_field(
      name,
      FT::PRIM_INFO.into_data_type(),
      make_write_field(get),
      make_read_field(set),
    );
  }

  fn record_field<FT, Get, Set>(&mut self, name: &str, get: Get, set: Set)
    where FT: RecordType,
          Get: 'static + Send + Sync + Fn (&RT) -> FT,
          Set: 'static + Send + Sync + Fn (&mut RT, FT)
  {
    self.add_field(
      name,
      DataTypeInfo::Record(RecordTypeInfo::new(
        record_info_for_type::<FT>().untyped().clone()
      )),
      make_write_field(get),
      make_read_field(set),
    );
  }
}

fn make_write_field<RT, FT, Get>(get: Get) -> Box<dyn Send + Sync + Fn (&u8, &mut [u8])>
  where RT: RecordType,
        FT: DataType,
        Get: 'static + Send + Sync + Fn (&RT) -> FT,
{
  Box::new(move |f: &u8, bytes_out: &mut [u8]| {
    let p = unsafe { mem::transmute::<_, &RT>(f) };
    let field_value = get(p);
    data_type_write(&field_value, bytes_out);
  })
}
fn make_read_field<RT, FT, Set>(set: Set) -> Box<dyn Send + Sync + Fn (&mut u8, &[u8])>
  where RT: RecordType,
        FT: DataType,
        Set: 'static + Send + Sync + Fn (&mut RT, FT),
{
  Box::new(move |f: &mut u8, bytes: &[u8]| {
    let mut field_value: FT = Default::default();
    data_type_read(&mut field_value, bytes);
    let p = unsafe { mem::transmute::<_, &mut RT>(f) };
    set(p, field_value);
  })
}