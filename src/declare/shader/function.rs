use std::fmt;
use crate::{
  types::{DataTypeInfo, SharedName, data_type_info_for},
  declare::DeclareCodeBody
};

/**
 * A type that encapsulates the declaration of a shader function
 */
pub(crate) struct DeclareShaderFunction {
  // The name of the function.
  name: SharedName,

  // The buffer binding info for the function.
  buffers: Vec<DataTypeInfo>,

  // The arguments info for the function.
  arguments: Vec<(SharedName, DataTypeInfo)>,

  // If this function is an entry-point.
  is_entry: bool,

  // The return type of the function.
  ret: Option<DataTypeInfo>,

  // The body of the function.
  body: DeclareCodeBody,
}
impl DeclareShaderFunction {
  pub(crate) fn new(
    name: SharedName,
    is_entry: bool,
    ret: Option<DataTypeInfo>
  ) -> Self {
    let buffers = Vec::new();
    let arguments = Vec::new();
    let body = DeclareCodeBody::new();
    DeclareShaderFunction {
      name, buffers, arguments, is_entry, ret, body
    }
  }

  pub(crate) fn name(&self) -> &SharedName { &self.name }

  pub(crate) fn get_bound_buffer(&self, idx: usize) -> &DataTypeInfo {
    &self.buffers[idx]
  }

  pub(crate) fn push_bound_buffer(&mut self, data_type: DataTypeInfo)
    -> usize
  {
    let idx = self.buffers.len();
    self.buffers.push(data_type);
    idx
  }

  pub(crate) fn argument_type(&self, idx: usize) -> &DataTypeInfo {
    &self.arguments[idx].1
  }
  pub(crate) fn argument_name(&self, idx: usize) -> &SharedName {
    &self.arguments[idx].0
  }

  pub(crate) fn push_argument(&mut self,
    name: &SharedName,
    data_type: DataTypeInfo
  ) {
    assert!(self.arguments.iter().find(|(n, _)| n == name).is_none(),
      "Argument name collision: {:?}", name);
    self.arguments.push((name.clone(), data_type));
  }

  pub(crate) fn body(&mut self) -> &mut DeclareCodeBody {
    &mut self.body
  }

  pub(crate) fn to_text<W>(&self, out: &mut W) -> fmt::Result
    where W: fmt::Write
  {
    write!(out, "fn {}(", self.name.as_ref()) ?;
    for (i, &(ref nm, ref dt)) in self.arguments.iter().enumerate() {
      if i > 0 { write!(out, ", ") ?; }

      if self.is_entry {
        debug_assert!(i == 0, "Entry function should take 1 arg.");
        write!(out, "\n  @builtin(global_invocation_id) x_global_id: vec3<u32>") ?;
      } else {
        write!(out, "{}: {}", nm.as_ref(), dt.name()) ?;
      }
    }
    writeln!(out, ") {{") ?;
    if self.is_entry {
      let dt = &self.arguments[0].1;
      if dt == &data_type_info_for::<u32>() {
        writeln!(out, "let {}: u32 = x_global_id.x;",
          self.arguments[0].0.as_ref()) ?;
      } else if dt == &data_type_info_for::<[u32; 2]>() {
        writeln!(out, "let {}: vec2<u32> = x_global_id.xy;",
          self.arguments[0].0.as_ref()) ?;
      } else {
        panic!("Unknown entry-function type {}", dt.name());
      }
    }
    for stmt in self.body.statements() {
      write!(out, "  ") ?;
      stmt.to_text(out) ?;
    }
    writeln!(out, "}}") ?;
    Ok(())
  }
}