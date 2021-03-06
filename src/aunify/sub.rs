use super::{prelude::*, tracer::prelude::*};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Sub(HashMap<Var, Value>);

impl Sub {
  #[inline]
  pub fn top() -> Self { Sub(HashMap::new()) }

  pub fn with(mut self, var: Var, is: Value) -> Result<Self> {
    if self.0.insert(var, is).is_some() {
      Err(ErrorKind::DuplicateSub.into())
    } else {
      Ok(self)
    }
  }

  pub fn relevant_to<T: Thing>(mut self, t: &T) -> Self {
    let vars = t.free_vars();

    self.0.retain(|k, _| vars.contains(k));

    self
  }

  pub fn without_autos(mut self) -> Self {
    self.0.retain(|k, _| match k {
      Var::Formal(_) => true,
      Var::Auto(_) => false,
    });

    self
  }

  pub fn into_map(self) -> HashMap<Var, Value> { self.0 }

  pub fn get(&self, var: &Var) -> Option<&Value> { self.0.get(var) }

  pub fn is_top(&self) -> bool { self.0.is_empty() }
}

impl Thing for Sub {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    for (var, is) in &self.0 {
      set.insert(var.clone());
      is.collect_free_vars(set);
    }
  }

  fn sub_impl<T: ThingTracer>(
    mut self,
    sub: &Sub,
    tracer: T::SubHandle,
  ) -> Result<Self> {
    use self::HashEntry::*;

    let mut sub = sub.clone();

    sub.0.retain(|v, _| !self.0.contains_key(v));

    for (var, is) in &mut self.0 {
      if !sub.0.contains_key(var) {
        is.sub_self(&sub, tracer.clone())?;
      }
    }

    for (var, is) in &sub.0 {
      match self.0.entry(var.clone()) {
        Vacant(v) => {
          v.insert(is.clone());
        },
        Occupied(o) => o.into_mut().sub_self(&sub, tracer.clone())?,
      }
    }

    Ok(self)
  }

  fn can_sub(&self, sub: &Sub) -> bool {
    self.0.values().all(|v| v.can_sub(sub))
  }
}

impl Display for Sub {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    if self.0.is_empty() {
      // An empty Sub implies top
      fmt.write_str("⊤")
    } else {
      let mut first = true;

      for (var, is) in &self.0 {
        if first {
          first = false;
        } else {
          fmt.write_str(", ")?;
        }

        Display::fmt(var, fmt)?;

        fmt.write_str(" <- ")?;

        Display::fmt(is, fmt)?;
      }

      Ok(())
    }
  }
}
