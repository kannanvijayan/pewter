
use crate::{
  types::{
    DataType,
    DataTypeInfo,
    data_type_info_for,
  },
};


/**
 * A trait capturing data-type tuples, useful for statically
 * mapping to tuples of typed buffers, and tuples of other
 * wrapped datatypes.
 */
pub trait DataTypeTuple {
  fn info_vec() -> Vec<DataTypeInfo>;
}

impl DataTypeTuple for () {
  fn info_vec() -> Vec<DataTypeInfo> {
    Vec::new()
  }
}

impl<T0> DataTypeTuple for T0
  where T0: DataType,
{
  fn info_vec() -> Vec<DataTypeInfo> {
    Vec::from(&[data_type_info_for::<T0>()] as &[_])
  }
}

impl<T0, T1> DataTypeTuple for (T0, T1)
  where T0: DataType, T1: DataType,
{
  fn info_vec() -> Vec<DataTypeInfo> {
    Vec::from(&[
      data_type_info_for::<T0>(), data_type_info_for::<T1>(),
    ] as &[_])
  }
}

impl<T0, T1, T2> DataTypeTuple for (T0, T1, T2)
  where T0: DataType, T1: DataType, T2: DataType,
{
  fn info_vec() -> Vec<DataTypeInfo> {
    Vec::from(&[
      data_type_info_for::<T0>(), data_type_info_for::<T1>(),
      data_type_info_for::<T2>(),
    ] as &[_])
  }
}

impl<T0, T1, T2, T3> DataTypeTuple for (T0, T1, T2, T3)
  where T0: DataType, T1: DataType, T2: DataType, T3: DataType,
{
  fn info_vec() -> Vec<DataTypeInfo> {
    Vec::from(&[
      data_type_info_for::<T0>(), data_type_info_for::<T1>(),
      data_type_info_for::<T2>(), data_type_info_for::<T3>(),
    ] as &[_])
  }
}

impl<T0, T1, T2, T3, T4> DataTypeTuple for (T0, T1, T2, T3, T4)
  where T0: DataType, T1: DataType, T2: DataType, T3: DataType,
        T4: DataType,
{
  fn info_vec() -> Vec<DataTypeInfo> {
    Vec::from(&[
      data_type_info_for::<T0>(), data_type_info_for::<T1>(),
      data_type_info_for::<T2>(), data_type_info_for::<T3>(),
      data_type_info_for::<T4>(),
    ] as &[_])
  }
}

impl<T0, T1, T2, T3, T4, T5> DataTypeTuple
  for (T0, T1, T2, T3, T4, T5)
  where T0: DataType, T1: DataType, T2: DataType, T3: DataType,
        T4: DataType, T5: DataType,
{
  fn info_vec() -> Vec<DataTypeInfo> {
    Vec::from(&[
      data_type_info_for::<T0>(), data_type_info_for::<T1>(),
      data_type_info_for::<T2>(), data_type_info_for::<T3>(),
      data_type_info_for::<T4>(), data_type_info_for::<T5>(),
    ] as &[_])
  }
}

impl<T0, T1, T2, T3, T4, T5, T6> DataTypeTuple
  for (T0, T1, T2, T3, T4, T5, T6)
  where T0: DataType, T1: DataType, T2: DataType, T3: DataType,
        T4: DataType, T5: DataType, T6: DataType,
{
  fn info_vec() -> Vec<DataTypeInfo> {
    Vec::from(&[
      data_type_info_for::<T0>(), data_type_info_for::<T1>(),
      data_type_info_for::<T2>(), data_type_info_for::<T3>(),
      data_type_info_for::<T4>(), data_type_info_for::<T5>(),
      data_type_info_for::<T6>(),
    ] as &[_])
  }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> DataTypeTuple
  for (T0, T1, T2, T3, T4, T5, T6, T7)
  where T0: DataType, T1: DataType, T2: DataType, T3: DataType,
        T4: DataType, T5: DataType, T6: DataType, T7: DataType,
{
  fn info_vec() -> Vec<DataTypeInfo> {
    Vec::from(&[
      data_type_info_for::<T0>(), data_type_info_for::<T1>(),
      data_type_info_for::<T2>(), data_type_info_for::<T3>(),
      data_type_info_for::<T4>(), data_type_info_for::<T5>(),
      data_type_info_for::<T6>(), data_type_info_for::<T7>(),
    ] as &[_])
  }
}
