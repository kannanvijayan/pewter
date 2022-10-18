
use crate::{
  Pewter,
  types::{intern_name, SharedName},
};

use super::ShaderFileSession;

/**
 * An API to perform a sequence of operations on a pewter
 * instance. 
 */
pub struct DeclareSession<'a> {
  pewter: &'a Pewter,
}
impl<'a> DeclareSession<'a> {
  pub(crate) fn new(pewter: &'a Pewter) -> Self {
    DeclareSession { pewter }
  }

  pub fn shader_file<'b, Func>(&'b mut self, name: &str, func: Func)
    -> SharedName
    where Func: FnOnce (&mut ShaderFileSession<'b>)
  {
    let name = intern_name(name);
    let decl_file = {
      let mut func_sess = ShaderFileSession::new(name.clone());
      func(&mut func_sess);
      func_sess.finish()
    };
    self.pewter.register_shader_file(decl_file);
    name
  }

  pub fn shader_file_text(&self, name: &SharedName) -> String {
    self.pewter.shader_file_text(name)
      .expect(&format!(
        "Failed to generate shader file text for {:?}",
        name
      ))
  }
}