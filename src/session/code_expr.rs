use std::marker::PhantomData;
use crate::{
  types::{
    DataType,
    DataTypeInfo,
    PrimType, VectorType, ScalarType,
  },
  session::DeclareSession,
  declare::DeclareCodeExpr,
};


/**
 * Representation of an expression built during a session.
 */
pub struct SessionCodeExpr<'a, DT: DataType> {
  data_type: DataTypeInfo,
  declaration: DeclareCodeExpr,
  _dummy: PhantomData<&'a DT>,
}
impl<'a, DT: DataType> SessionCodeExpr<'a, DT> {
  pub(crate) fn new(
    data_type: DataTypeInfo,
    declaration: DeclareCodeExpr
  ) -> Self {
    SessionCodeExpr {
      data_type,
      declaration,
      _dummy: PhantomData
    }
  }

  pub(crate) fn declaration(self) -> DeclareCodeExpr {
    self.declaration
  }
}

pub trait AddableTo<R: DataType>: DataType {
  type Result: DataType;
}
impl AddableTo<u32> for u32 {
  type Result = u32;
}
impl AddableTo<i32> for i32 {
  type Result = i32;
}
impl AddableTo<f32> for f32 {
  type Result = f32;
}
impl AddableTo<[u32; 2]> for [u32; 2] {
  type Result = [u32; 2];
}
impl AddableTo<[u32; 3]> for [u32; 3] {
  type Result = [u32; 3];
}
impl AddableTo<[u32; 4]> for [u32; 4] {
  type Result = [u32; 4];
}
impl AddableTo<[i32; 2]> for [i32; 2] {
  type Result = [i32; 2];
}
impl AddableTo<[i32; 3]> for [i32; 3] {
  type Result = [i32; 3];
}
impl AddableTo<[i32; 4]> for [i32; 4] {
  type Result = [i32; 4];
}
impl AddableTo<[f32; 2]> for [f32; 2] {
  type Result = [f32; 2];
}
impl AddableTo<[f32; 3]> for [f32; 3] {
  type Result = [f32; 3];
}
impl AddableTo<[f32; 4]> for [f32; 4] {
  type Result = [f32; 4];
}

impl<'a, L, R> std::ops::Add<SessionCodeExpr<'a, R>>
  for SessionCodeExpr<'a, L>
  where L: AddableTo<R>,
        R: DataType,
{
  type Output = SessionCodeExpr<'a, <L as AddableTo<R>>::Result>;
  fn add(self, rhs: SessionCodeExpr<'a, R>) -> Self::Output {
    debug_assert!(self.data_type == u32::PRIM_INFO.into_data_type());
    debug_assert!(self.data_type == rhs.data_type);
    let add_expr =
      DeclareCodeExpr::new_add(
        Box::new(self.declaration),
        Box::new(rhs.declaration)
      );
    SessionCodeExpr::new(self.data_type, add_expr)
  }
}