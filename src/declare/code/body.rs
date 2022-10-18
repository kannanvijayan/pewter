use crate::{
  declare::DeclareCodeStatement
};


/**
 * The declaration of a code body, consisting of a sequence
 * of statements.
 */
pub(crate) struct DeclareCodeBody {
  statements: Vec<DeclareCodeStatement>,
}
impl DeclareCodeBody {
  pub(crate) fn new() -> DeclareCodeBody {
    DeclareCodeBody { statements: Vec::new() }
  }

  pub(crate) fn add_statement(&mut self, stmt: DeclareCodeStatement) {
    self.statements.push(stmt)
  }

  pub(crate) fn statements(&self) -> &[DeclareCodeStatement] {
    &self.statements
  }
}