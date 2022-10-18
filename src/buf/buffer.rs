
use wgpu;
use std::{
  sync::Arc,
};

/**
 * An internal buffer.  This type is not exposed publically,
 * but whenever a gpu buffer exists associated with a Peweter
 * instance, it is fronted by this proxy type.
 * 
 * Carries a DataTypeInfo and a dimensionalized size.
 */
#[derive(Clone)]
pub(crate) struct Buffer {
  buffer: Arc<wgpu::Buffer>,
}