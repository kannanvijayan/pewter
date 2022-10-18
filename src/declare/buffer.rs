use crate::types::{SharedName, DataTypeInfo};

/**
 * A declared buffer, which carries only a name and type.
 */
pub(crate) struct DeclareBuffer {
  name: SharedName,
  data_type: DataTypeInfo,
}
impl DeclareBuffer {
  pub(crate) fn new(name: SharedName, data_type: DataTypeInfo) -> Self {
    DeclareBuffer { name, data_type }
  }

  pub(crate) fn name(&self) -> &SharedName { &self.name }
}