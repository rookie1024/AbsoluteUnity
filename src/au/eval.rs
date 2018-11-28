use crate::ast::{Expr, Input};
use aunify::{App, Env, MaybeScheme, Statement, Sub, Thing, Value, VarSource};

pub struct Evaluator {
  env: Env,
  var_src: VarSource,
}

pub enum EvalResult<'a> {
  Unit,
  Assert(Vec<MaybeScheme<Statement>>),
  Query(Box<Iterator<Item = Sub> + 'a>),
  UnifyVal(aunify::Result<(Value, Value, Sub, Value, Value)>),
  UnifyApp(aunify::Result<(App, App, Sub, App, App)>),
  PrintVal(MaybeScheme<Value>),
  PrintStmt(MaybeScheme<Statement>),
  PrintEnv(&'a Vec<MaybeScheme<Statement>>),
}

impl Evaluator {
  pub fn new() -> Self {
    Self {
      env: Env::new(),
      var_src: VarSource::new(),
    }
  }

  pub fn eval<'a>(&'a mut self, ast: Expr) -> EvalResult {
    match ast {
      Expr::Assert(v) => {
        for stmt in v.clone() {
          self.env.state(stmt);
        }

        EvalResult::Assert(v)
      },
      Expr::Query(c) => {
        EvalResult::Query(Box::new(self.env.solve_clause(c, &mut self.var_src)))
      },
      Expr::UnifyVal(a, b) => EvalResult::UnifyVal(
        a.inst_and_unify(b, &mut self.var_src)
          .and_then(|(a, b, sub)| {
            let a1 = a.clone();
            let b1 = b.clone();
            let a2 = a.sub(&sub)?;
            let b2 = b.sub(&sub)?;

            Ok((a1, b1, sub, a2, b2))
          }),
      ),
      Expr::UnifyApp(a, b) => EvalResult::UnifyApp(
        a.inst_and_unify(b, &mut self.var_src)
          .and_then(|(a, b, sub)| {
            let a1 = a.clone();
            let b1 = b.clone();
            let a2 = a.sub(&sub)?;
            let b2 = b.sub(&sub)?;

            Ok((a1, b1, sub, a2, b2))
          }),
      ),
      Expr::PrintVal(v) => EvalResult::PrintVal(v),
      Expr::PrintStmt(s) => EvalResult::PrintStmt(s),
      Expr::PrintEnv => EvalResult::PrintEnv(self.env.premises()),
      Expr::Reset => {
        *self = Evaluator::new();
        EvalResult::Unit
      },
    }
  }

  pub fn eval_input(&mut self, ast: Input) {
    for stmt in ast.0 {
      self.env.state(stmt);
    }
  }
}
