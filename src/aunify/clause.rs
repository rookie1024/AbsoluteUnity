use super::{prelude::*, tracer::prelude::*};

#[derive(Clone, Debug)]
pub enum Clause {
  Top,
  Bot,
  App(App),
  Not(Box<Clause>),
  And(Box<Clause>, Box<Clause>),
  Or(Box<Clause>, Box<Clause>),
}

impl Clause {
  fn disp_prec(&self, prec: u32, fmt: &mut fmt::Formatter) -> fmt::Result {
    use self::Clause::*;

    let my_prec = match self {
      Top => 3,
      Bot => 3,
      App(_) => 3,
      Not(_) => 2,
      And(_, _) => 1,
      Or(_, _) => 0,
    };

    if my_prec < prec {
      fmt.write_str("(")?;
    }

    match self {
      Top => fmt.write_str("⊤")?,
      Bot => fmt.write_str("⊥")?,
      App(a) => Display::fmt(a, fmt)?,
      Not(c) => {
        fmt.write_str("~")?;
        c.disp_prec(my_prec, fmt)?;
      },
      And(a, b) => {
        a.disp_prec(my_prec, fmt)?;
        fmt.write_str(", ")?;
        b.disp_prec(my_prec, fmt)?;
      },
      Or(a, b) => {
        a.disp_prec(my_prec, fmt)?;
        fmt.write_str(", ")?;
        b.disp_prec(my_prec, fmt)?;
      },
    }

    if my_prec < prec {
      fmt.write_str(")")?;
    }

    Ok(())
  }
}

impl Thing for Clause {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    match self {
      Clause::Top => {},
      Clause::Bot => {},
      Clause::App(a) => a.collect_free_vars(set),
      Clause::Not(c) => c.collect_free_vars(set),
      Clause::And(a, b) => {
        a.collect_free_vars(set);
        b.collect_free_vars(set);
      },
      Clause::Or(a, b) => {
        a.collect_free_vars(set);
        b.collect_free_vars(set);
      },
    }
  }

  fn sub_impl<T: ThingTracer>(
    self,
    sub: &Sub,
    tracer: T::SubHandle,
  ) -> Result<Self> {
    use self::Clause::*;

    Ok(match self {
      Top => Top,
      Bot => Bot,
      App(a) => App(a.sub(sub, tracer)?),
      Not(c) => Not(Box::new(c.sub(sub, tracer)?)),
      And(a, b) => And(
        Box::new(a.sub(sub, tracer.clone())?),
        Box::new(b.sub(sub, tracer)?),
      ),
      Or(a, b) => Or(
        Box::new(a.sub(sub, tracer.clone())?),
        Box::new(b.sub(sub, tracer)?),
      ),
    })
  }

  fn can_sub(&self, sub: &Sub) -> bool {
    use self::Clause::*;

    match self {
      Top => true,
      Bot => true,
      App(a) => a.can_sub(sub),
      Not(c) => c.can_sub(sub),
      And(a, b) => a.can_sub(sub) && b.can_sub(sub),
      Or(a, b) => a.can_sub(sub) && b.can_sub(sub),
    }
  }
}

impl Display for Clause {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    self.disp_prec(0, fmt)
  }
}
