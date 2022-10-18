
/** Declaration of a code body (sequence of statements). */
mod body;

/** Declaration of a code statement. */
mod statement;

/** Declaration of a code expression. */
mod expr;

pub(crate) use self::{
  body::DeclareCodeBody,
  statement::DeclareCodeStatement,
  expr::DeclareCodeExpr,
};