
/**
 * This module defines all the types, interfaces, and logic used
 * to declare runtime constructs - e.g. shader functions,
 * shader files, buffers, and constructs.
 */

/** Declaration of buffers. */
mod buffer;

/** Declaration of shader-related entities. */
mod shader;

/** Declaration of code-related entities. */
mod code;

pub(crate) use self::{
  buffer::DeclareBuffer,
  shader::{
    DeclareShaderFunction,
    DeclareShaderFile,
  },
  code::{
    DeclareCodeBody,
    DeclareCodeStatement,
    DeclareCodeExpr,
  },
};