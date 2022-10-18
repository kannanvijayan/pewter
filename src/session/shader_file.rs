
use std::marker::PhantomData;

use crate::{
  types::{
    DataType,
    data_type_info_for,
    intern_name, SharedName
  },
  declare::{DeclareShaderFile, DeclareBuffer},
  session::{
    SessionBuffer,
    DeclareSession,
    ShaderFunctionSession,
    ShaderEntryFunctionSession,
  },
};


pub struct ShaderFileSession<'a> {
  declare: DeclareShaderFile,
  _dummy: PhantomData<&'a ()>
}
impl<'a> ShaderFileSession<'a> {
  pub(crate) fn new(name: SharedName) -> Self {
    let declare = DeclareShaderFile::new(name);
    ShaderFileSession { declare, _dummy: PhantomData }
  }
  pub fn use_buf<DT>(&mut self, name: &str) -> SessionBuffer<'a, DT>
   where DT: DataType
  {
    let name = intern_name(name);
    let data_type = data_type_info_for::<DT>();
    let decl_buffer = DeclareBuffer::new(name, data_type.clone());
    let idx = self.declare.push_buffer(decl_buffer);
    SessionBuffer::new(data_type, idx)
  }

  pub fn function<Ret, Func>(&mut self, name: &str, func: Func)
    where Ret: DataType,
          Func: for <'x> FnOnce (&mut ShaderFunctionSession<'x, Ret>)
  {
    let name = intern_name(name);
    let decl_func = {
      let mut func_sess = ShaderFunctionSession::new(name);
      func(&mut func_sess);
      func_sess.finish()
    };
    self.declare.push_function(decl_func, /* is_entry = */ false);
  }

  pub fn entry_function_1d<Func>(&mut self,
    name: &str,
    func: Func
  ) where Func: for <'x> FnOnce (
            &mut ShaderEntryFunctionSession<'x, u32>
          )
  {
    let name = intern_name(name);
    let decl_func = {
      let mut sess = ShaderEntryFunctionSession::new(name);
      func(&mut sess);
      sess.finish()
    };
    self.declare.push_function(decl_func, /* is_entry = */ false);
  }

  pub fn entry_function_2d<Ret, Func>(&mut self,
    name: &str,
    func: Func
  ) where Func: for <'x> FnOnce (
            &mut ShaderEntryFunctionSession<'x, [u32; 2]>
          )
  {
    let name = intern_name(name);
    let decl_func = {
      let mut sess = ShaderEntryFunctionSession::new(name);
      func(&mut sess);
      sess.finish()
    };
    self.declare.push_function(decl_func, /* is_entry = */ false);
  }

  pub(crate) fn finish(self) -> DeclareShaderFile { self.declare }
}