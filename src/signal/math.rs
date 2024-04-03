use crate::Signal;

#[derive(Debug, Clone)]
pub struct Product<S1: Signal, S2: Signal> {
  child_1: S1,
  child_2: S2,
}

impl<S1: Signal, S2: Signal> Product<S1, S2> {
  pub fn new(child_1: S1, child_2: S2) -> Self {
    Product::<S1, S2> { child_1, child_2 }
  }
}

impl<S1: Signal, S2: Signal> Signal for Product<S1, S2> {
  fn sample(&mut self, t: f64) -> f64 {
    self.child_1.sample(t) * self.child_2.sample(t)
  }
}

#[derive(Debug, Clone)]
pub struct MultiProduct<S: Signal> {
  children: Vec<S>,
}

impl<S: Signal> MultiProduct<S> {
  pub fn new(children: Vec<S>) -> Self {
    MultiProduct::<S> { children }
  }
}

impl<S: Signal> Signal for MultiProduct<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self
      .children
      .iter_mut()
      .map(|child| child.sample(t))
      .product()
  }
}

#[derive(Debug, Clone)]
pub struct Sum<S1: Signal, S2: Signal> {
  child_1: S1,
  child_2: S2,
}

impl<S1: Signal, S2: Signal> Sum<S1, S2> {
  pub fn new(child_1: S1, child_2: S2) -> Self {
    Sum::<S1, S2> { child_1, child_2 }
  }
}

impl<S1: Signal, S2: Signal> Signal for Sum<S1, S2> {
  fn sample(&mut self, t: f64) -> f64 {
    self.child_1.sample(t) + self.child_2.sample(t)
  }
}

#[derive(Debug, Clone)]
pub struct MultiSum<S: Signal> {
  children: Vec<S>,
}

impl<S: Signal> MultiSum<S> {
  pub fn new(children: Vec<S>) -> Self {
    MultiSum::<S> { children }
  }
}

impl<S: Signal> Signal for MultiSum<S> {
  fn sample(&mut self, t: f64) -> f64 {
    self.children.iter_mut().map(|child| child.sample(t)).sum()
  }
}
