use std::fmt;
use super::DeclareCodeExpr;


/**
 * A statement is a top-level entity within a code-body.
 * Statements can be bare expressions, or variable definitions,
 * or control flow groups containing their own code bodies.
 */
#[derive(Clone)]
pub(crate) enum DeclareCodeStatement {
  ReturnValue(DeclareCodeExpr),
}
impl DeclareCodeStatement {
  pub(crate) fn new_return(expr: DeclareCodeExpr) -> Self {
    DeclareCodeStatement::ReturnValue(expr)
  }

  pub(crate) fn to_text<W>(&self, out: &mut W) -> fmt::Result
    where W: fmt::Write
  {
    match self {
      &DeclareCodeStatement::ReturnValue(ref expr) => {
        write!(out, "return ") ?;
        expr.to_text(out) ?;
        writeln!(out, ";")
      }
    }
  }
}