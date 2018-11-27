use crate::ast::Expr;
use aunify::{App, Env, Sub, Thing, Value, VarSource};

pub struct Evaluator {
  env: Env,
  var_src: VarSource,
}

pub enum EvalResult<'a> {
  Unit,
  Query(Box<Iterator<Item = Sub> + 'a>),
  UnifyVal(aunify::Result<(Value, Value, Sub, Value, Value)>),
  UnifyApp(aunify::Result<(App, App, Sub, App, App)>),
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
        for stmt in v {
          self.env.state(stmt);
        }
        
        EvalResult::Unit
      },
      Expr::Query(c) => {
        EvalResult::Query(Box::new(self.env.solve_clause(c, &mut self.var_src)))
      },
      Expr::UnifyVal(a, b) => EvalResult::UnifyVal(
        a.inst_and_unify(b, &mut self.var_src).map(|(a, b, sub)| {
          let a1 = a.clone();
          let b1 = b.clone();
          let a2 = a.sub(&sub);
          let b2 = b.sub(&sub);

          (a1, b1, sub, a2, b2)
        }),
      ),
      Expr::UnifyApp(a, b) => EvalResult::UnifyApp(
        a.inst_and_unify(b, &mut self.var_src).map(|(a, b, sub)| {
          let a1 = a.clone();
          let b1 = b.clone();
          let a2 = a.sub(&sub);
          let b2 = b.sub(&sub);

          (a1, b1, sub, a2, b2)
        }),
      ),
    }
  }
}