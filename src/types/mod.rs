
mod shared_name;
mod prim_type;
mod scalar_type;
mod vector_type;
mod record_type;
mod record_info;

mod data_type;
mod data_type_tuple;

pub(crate) use self::{
  shared_name::{
    intern_name,
  },
  data_type::{
    DataType,
    DataTypeInfo,
    DataTypeSpecialize,
    DataTypeSpecializeMut,
    DataTypeSpecializeRef,
    data_type_info_for,
    data_type_read,
    data_type_write,
  },
  record_info::{
    SharedRecordInfo,
    record_info_for_type,
  },
  record_type::{
    RecordTypeInfo,
  }
};
pub use self::{
  shared_name::{
    SharedName,
  },
  record_type::{
    RecordType,
    RecordTypeFieldSpecifier,
  },
  record_info::{
    RecordInfo,
    TypedRecordInfo,
  },
  data_type_tuple::{
    DataTypeTuple,
  },
  scalar_type::{
    ScalarType,
    ScalarTypeInfo,
  },
  vector_type::{
    VectorType,
    VectorTypeInfo,
  },
  prim_type::{
    PrimType,
    PrimTypeInfo,
  },
};