use std::marker::PhantomData;

use crate::{
  Pewter,
  pewter::PewterConstruct,
  PewterError,
};

/**
 * An API to perform a sequence of operations on a pewter
 * instance. 
 */
pub struct ConstructSession<'a, T: 'static> {
  pewter: &'a Pewter,
  _dummy: PhantomData<T>,
}
impl<'a, T: 'static> ConstructSession<'a, T> {
  pub(crate) fn new(pewter: &'a Pewter) -> Self {
    ConstructSession { pewter, _dummy: PhantomData }
  }

  pub(crate) fn make_construct(&self)
    -> Result<PewterConstruct<'a, T>, PewterError>
  {
    panic!("Implement.");
  }
}