use std::{
  marker::PhantomData,
};
use crate::{
  types::{
    DataType,
    data_type_info_for,
    SharedName, DataTypeInfo, intern_name, PrimType
  },
  declare::{
    DeclareShaderFunction,
    DeclareCodeExpr,
    DeclareCodeStatement
  },
  session::SessionCodeExpr,
};


pub struct BaseShaderFunctionSession<'a> {
  declare: DeclareShaderFunction,
  _dummy: PhantomData<&'a ()>,
}
impl<'a> BaseShaderFunctionSession<'a> {
  pub(crate) fn finish(self) -> DeclareShaderFunction {
    self.declare
  }
}

pub struct ShaderFunctionSession<'a, Ret: DataType> {
  base: BaseShaderFunctionSession<'a>,
  _dummy: PhantomData<&'a Ret>,
}
impl<'a, Ret: DataType> std::ops::Deref
  for ShaderFunctionSession<'a, Ret>
{
  type Target = BaseShaderFunctionSession<'a>;
  fn deref(&self) -> &Self::Target {
    &self.base
  }
}
impl<'a, Ret: DataType> std::ops::DerefMut
  for ShaderFunctionSession<'a, Ret>
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.base
  }
}
impl<'a, Ret: DataType> ShaderFunctionSession<'a, Ret> {
  pub(crate) fn new(name: SharedName) -> Self {
    let ret_dt = data_type_info_for::<Ret>();
    let declare = DeclareShaderFunction::new(name, false, Some(ret_dt));
    let base = BaseShaderFunctionSession { declare, _dummy: PhantomData };
    ShaderFunctionSession { base, _dummy: PhantomData }
  }

  pub fn use_arg<DT>(&mut self, name: &str) -> SessionCodeExpr<'a, DT>
   where DT: DataType
  {
    let name = intern_name(name);
    let data_type = data_type_info_for::<DT>();
    let arg_idx =
      self.base.declare.push_argument(&name, data_type.clone());
    let arg = DeclareCodeExpr::new_argument(name);
    SessionCodeExpr::new(data_type, arg)
  }

  pub fn return_stmt(&mut self, expr: SessionCodeExpr<'a, Ret>) {
    let stmt = DeclareCodeStatement::new_return(expr.declaration());
    self.base.declare.body().add_statement(stmt);
  }

  pub(crate) fn finish(self) -> DeclareShaderFunction {
    self.base.declare
  }
}

pub struct ShaderEntryFunctionSession<'a, Coord: PrimType> {
  base: BaseShaderFunctionSession<'a>,
  _dummy: PhantomData<&'a Coord>,
}
impl<'a, Coord> std::ops::Deref
  for ShaderEntryFunctionSession<'a, Coord>
  where Coord: PrimType,
{
  type Target = BaseShaderFunctionSession<'a>;
  fn deref(&self) -> &Self::Target {
    &self.base
  }
}
impl<'a, Coord> std::ops::DerefMut
  for ShaderEntryFunctionSession<'a, Coord>
  where Coord: PrimType,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.base
  }
}
impl<'a, Coord> ShaderEntryFunctionSession<'a, Coord>
  where Coord: PrimType,
{
  pub(crate) fn new(name: SharedName) -> Self {
    let data_type = data_type_info_for::<Coord>();
    let coord = intern_name("global_id");
    let mut declare = DeclareShaderFunction::new(name, false, None);
    declare.push_argument(&coord, data_type);
    let base = BaseShaderFunctionSession {
      declare, _dummy: PhantomData
    };
    ShaderEntryFunctionSession { base, _dummy: PhantomData }
  }

  pub fn get_arg(&mut self) -> SessionCodeExpr<'a, u32> {
    let data_type = self.declare.argument_type(0).clone();
    let name = self.declare.argument_name(0).clone();
    debug_assert!(data_type == data_type_info_for::<u32>());

    let arg = DeclareCodeExpr::new_argument(name);
    SessionCodeExpr::new(data_type, arg)
  }

  pub(crate) fn finish(self) -> DeclareShaderFunction {
    self.base.declare
  }
}