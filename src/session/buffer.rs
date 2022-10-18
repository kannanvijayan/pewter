use std::marker::PhantomData;

use crate::{
  types::{DataType, DataTypeInfo},
  session::DeclareSession,
};


/**
 * Represents a buffer abstraction during a shader function
 * session.
 */
pub struct SessionBuffer<'a, DT: DataType> {
  data_type: DataTypeInfo,
  idx: usize,
  _dummy: PhantomData<&'a DT>,
}
impl<'a, DT: DataType> SessionBuffer<'a, DT> {
  pub(crate) fn new(
    data_type: DataTypeInfo, 
    idx: usize,
  ) -> Self {
    SessionBuffer { data_type, idx, _dummy: PhantomData }
  }
}
impl<'a, DT: DataType> Clone for SessionBuffer<'a, DT> {
  fn clone(&self) -> Self {
    SessionBuffer {
      data_type: self.data_type.clone(),
      idx: self.idx,
      _dummy: PhantomData,
    }
  }
}