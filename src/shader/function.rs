use crate::{
  types::DataTypeInfo,
};

/**
 * A pure function in a shader language.
 */
pub struct ShaderFunction {
  arguments: Vec<DataTypeInfo>,
  result: DataTypeInfo,
}