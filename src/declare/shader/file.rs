
use std::fmt;
use crate::{
  types::SharedName,
  declare::{DeclareShaderFunction, DeclareBuffer},
};

/**
 * A type that encapsulates the declaration of a shader function
 */
pub(crate) struct DeclareShaderFile {
  // The name of the shader file,
  name: SharedName,

  // The shader functions defined in this file.
  functions: Vec<DeclareShaderFunction>,

  // The buffer used by this file.
  buffers: Vec<DeclareBuffer>,

  // The names of the entrypoint functions in this file.
  entrypoints: Vec<SharedName>,
}
impl DeclareShaderFile {
  pub(crate) fn new(name: SharedName) -> Self {
    let functions = Vec::new();
    let buffers = Vec::new();
    let entrypoints = Vec::new();
    DeclareShaderFile { name, functions, buffers, entrypoints }
  }

  pub(crate) fn name(&self) -> &SharedName { &self.name }

  pub(crate) fn push_function(&mut self,
    function: DeclareShaderFunction,
    is_entrypoint: bool
  ) -> usize {
    debug_assert!(
      self.functions.iter()
        .find(|f| f.name() == function.name())
        .is_none(),
      "Duplicate function name {:?}",
      function.name()
    );
    let idx = self.functions.len();
    if is_entrypoint {
      self.entrypoints.push(function.name().clone());
    }
    self.functions.push(function);
    idx
  }

  pub(crate) fn push_buffer(&mut self, buffer: DeclareBuffer)
    -> usize
  {
    debug_assert!(
      self.buffers.iter()
        .find(|b| b.name() == buffer.name())
        .is_none(),
      "Duplicate buffer name {:?}",
      buffer.name()
    );
    let idx = self.buffers.len();
    self.buffers.push(buffer);
    idx
  }

  pub(crate) fn to_text<W>(&self, out: &mut W) -> fmt::Result
    where W: fmt::Write
  {
    writeln!(out, "//////") ?;
    writeln!(out, "//////") ?;
    writeln!(out, "/// FILE: {}", self.name.as_ref()) ?;
    writeln!(out, "//////") ?;
    writeln!(out, "//////") ?;
    // Write out each function.
    writeln!(out, "") ?;
    writeln!(out, "//") ?;
    writeln!(out, "// FUNCTIONS") ?;
    writeln!(out, "//") ?;
    for func in &self.functions {
      writeln!(out, "// Function {}", func.name().as_ref()) ?;
      func.to_text(out) ?
    }

    // Write out each buffer.
    writeln!(out, "") ?;
    writeln!(out, "//") ?;
    writeln!(out, "// BUFFERS") ?;
    writeln!(out, "//") ?;
    for buf in &self.buffers {
      writeln!(out, "// buf {}", buf.name().as_ref()) ?;
      // buf.to_text(out) ?
    }
    Ok(())
  }
}