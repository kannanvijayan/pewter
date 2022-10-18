use std::fmt;
use crate::types::SharedName;


/**
 * Represents an expression in the code.
 */
#[derive(Clone)]
pub(crate) enum DeclareCodeExpr {
  Argument { name: SharedName },
  Add { lhs: Box<DeclareCodeExpr>, rhs: Box<DeclareCodeExpr> }
}
impl DeclareCodeExpr {
  pub(crate) fn new_argument(name: SharedName) -> Self {
    DeclareCodeExpr::Argument { name }
  }

  pub(crate) fn new_add(
    lhs: Box<DeclareCodeExpr>,
    rhs: Box<DeclareCodeExpr>,
  ) -> Self {
    DeclareCodeExpr::Add { lhs, rhs }
  }

  pub(crate) fn to_text<W>(&self, out: &mut W) -> fmt::Result
    where W: fmt::Write
  {
    match self {
      &DeclareCodeExpr::Argument{ ref name } => {
        write!(out, "{}", name.as_ref() )
      },
      &DeclareCodeExpr::Add { ref lhs, ref rhs } => {
        write!(out, "(") ?;
        lhs.to_text(out) ?;
        write!(out, " + ") ?;
        rhs.to_text(out) ?;
        write!(out, ")")
      }
    }
  }
}