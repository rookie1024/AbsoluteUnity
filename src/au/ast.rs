pub use aunify::{
  list::Tail, App, Clause, List, MaybeScheme, Pred, RcPred, Scheme, Statement,
  Tuple, Value, Var,
};
use std::collections::HashMap;

pub struct ParserTag {
  // Yeah, I'm aware this stores redundant values.
  pred_src: HashMap<(String, usize), RcPred>,
}

impl ParserTag {
  pub fn new() -> Self {
    Self {
      pred_src: HashMap::new(),
    }
  }

  pub fn make_app(&mut self, name: String, vals: Vec<Value>) -> App {
    let pred = self
      .pred_src
      .entry((name.clone(), vals.len()))
      .or_insert_with(|| Pred::new_rc(name, vals.len()));

    App::new(pred.clone(), Tuple(vals))
  }
}

#[derive(Debug)]
pub enum Expr {
  Assert(Vec<MaybeScheme<Statement>>),
  Query(Clause),
  UnifyVal(MaybeScheme<Value>, MaybeScheme<Value>),
  UnifyApp(MaybeScheme<App>, MaybeScheme<App>),
  PrintVal(MaybeScheme<Value>),
  PrintStmt(MaybeScheme<Statement>),
  PrintEnv,
  Reset,
}

#[derive(Debug)]
pub struct Input(pub Vec<MaybeScheme<Statement>>);
