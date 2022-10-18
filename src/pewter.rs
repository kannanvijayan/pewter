use std::{
  sync::Mutex,
  collections::HashMap,
  marker::PhantomData,
};
use crate::{
  types::SharedName,
  buf::Buffer,
  session::{ConstructSession, DeclareSession},
  declare::DeclareShaderFile,
};


pub struct Pewter {
  config: PewterConfig,

  // The persistent buffers loaded by this instance.
  buffers: Mutex<HashMap<SharedName, Buffer>>,

  // The declared shaders.
  shaders: Mutex<HashMap<SharedName, DeclareShaderFile>>,
}
impl Pewter {
  pub async fn new(config: PewterConfig)
    -> Result<Pewter, PewterError>
  {
    let buffers = Mutex::new(HashMap::new());
    let shaders = Mutex::new(HashMap::new());
    Ok(Pewter { config, buffers, shaders })
  }

  pub fn declare<F>(&self, func: F) -> Result<(), PewterError>
    where F: FnOnce (&mut DeclareSession) -> Result<(), PewterError>
  {
    let mut session = DeclareSession::new(self);
    func(&mut session) ?;
    Ok(())
  }

  pub fn construct<R: 'static, F>(&self, func: F)
    -> Result<PewterConstruct<R>, PewterError>
    where F: FnOnce (&mut ConstructSession<R>)
  {
    let mut session = ConstructSession::new(self);
    func(&mut session);
    let construct = session.make_construct() ?;
    Ok(construct)
  }

  pub(crate) fn register_shader_file(&self,
    shader_file: DeclareShaderFile
  ) {
    let mut locked = self.shaders.lock()
      .expect("Failed to lock shaders registry");
    assert!(locked.get(shader_file.name()).is_none(),
      "Shader file name collision {:?}", shader_file.name());
    locked.insert(shader_file.name().clone(), shader_file);
  }

  pub(crate) fn shader_file_text(&self, name: &SharedName)
    -> Option<String>
  {
    let locked = self.shaders.lock()
      .expect("Failed to lock shaders registry");
    let decl_file = locked.get(name) ?;
    let mut output = String::with_capacity(1024);
    decl_file.to_text(&mut output);
    Some(output)
  }
}

pub struct PewterConstruct<'a, T> {
  _dummy: PhantomData<&'a T>,
}
impl<'a, T> PewterConstruct<'a, T> {
  pub(crate) async fn perform(&self) -> Result<T, PewterError> {
    panic!("Implement!");
  }
}

pub struct PewterConfig {
}
impl Default for PewterConfig {
  fn default() -> PewterConfig {
    PewterConfig {  }
  }
}

#[derive(Debug)]
pub enum PewterError {
}