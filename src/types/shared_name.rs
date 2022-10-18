
use std::{
  sync::{Arc, Mutex},
  borrow::Borrow,
  collections::HashSet,
};

/**
 * Interned string/names.
 */
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SharedName(Arc<String>);
impl Borrow<str> for SharedName {
  fn borrow(&self) -> &str { self.0.as_str() }
}
impl AsRef<str> for SharedName {
  fn as_ref(&self) -> &str { self.0.as_str() }
}
impl std::ops::Deref for SharedName {
  type Target = str;
  fn deref(&self) -> &str { self.0.as_str() }
}

lazy_static::lazy_static! {
  static ref SHARED_NAMES: Arc<Mutex<HashSet<SharedName>>>
    = Arc::new(Mutex::new(HashSet::new()));
}

pub(crate) fn intern_name(s: &str) -> SharedName {
  let mut locked = SHARED_NAMES.lock()
    .expect("Failed to lock SHARED_NAMES");
  let found = locked.get(s);
  if let Some(result) = found {
    return result.clone();
  }
  let name = SharedName(Arc::new(s.to_owned()));
  locked.insert(name.clone());
  name
}