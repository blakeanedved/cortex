use std::ops::{Add, Sub, Mul, Div, Rem};
use std::cmp::{PartialEq, PartialOrd, Ordering};
use crate::types::{ASTNode, ASTType::*};

pub fn not(a: &ASTNode) -> ASTNode {
  match &a.kind {
    Number(x) => ASTNode::new_number(if x == &0.0 as &f64 { 1.0 } else { 0.0 } ),
    Boolean(b) => ASTNode::new_boolean(!b),
    List(l) => ASTNode::new_list(l.iter().map(|val| not(val)).collect()),
    _ => panic!("invalid types"),
  }
}

#[inline]
fn facf(x: &f64) -> f64 {
    libm::tgamma(*x + 1.0)
}

pub fn factorial(a: &ASTNode) -> ASTNode {
  match &a.kind {
    Number(x) => ASTNode::new_number(facf(x)),
    List(l) => ASTNode::new_list(l.iter().map(|val| factorial(val)).collect()),
    _ => panic!("invalid types"),
  }
}

pub fn pow(a: &ASTNode, b: &ASTNode) -> ASTNode {
  match (&a.kind, &b.kind) {
    (Number(x), Number(y)) => ASTNode::new_number(x.powf(*y)),
    (List(l), Number(x)) => ASTNode::new_list(l.iter().map(|val| pow(val, &ASTNode::new_number(*x))).collect()),
    (Number(x), List(l)) => ASTNode::new_list(l.iter().map(|val| pow(&ASTNode::new_number(*x), val)).collect()),
    _ => panic!("invalid types"),
  }
}

pub fn abs(a: &ASTNode) -> ASTNode {
  match &a.kind {
    Number(x) => ASTNode::new_number(x.abs()),
    List(l) => ASTNode::new_list(l.iter().map(|val| abs(val)).collect()),
    _ => panic!("invalid types"),
  }
}

pub fn or(a: &ASTNode, b: &ASTNode) -> ASTNode {
  match (&a.kind, &b.kind) {
    (Number(x), Number(y)) => ASTNode::new_boolean(x != &0.0 as &f64 || y != &0.0 as &f64),
    (Boolean(x), Boolean(y)) => ASTNode::new_boolean(*x || *y),
    _ => panic!("invalid types"),
  }
}

pub fn and(a: &ASTNode, b: &ASTNode) -> ASTNode {
  match (&a.kind, &b.kind) {
    (Number(x), Number(y)) => ASTNode::new_boolean(x != &0.0 as &f64 && y != &0.0 as &f64),
    (Boolean(x), Boolean(y)) => ASTNode::new_boolean(*x && *y),
    _ => panic!("invalid types"),
  }
}

pub fn setadd(a: &ASTNode, b: &ASTNode) -> ASTNode {
  match (&a.kind, &b.kind) {
    (Number(x), Number(y)) => ASTNode::new_list(vec![ASTNode::new_number(*x), ASTNode::new_number(*y)]),
    (Boolean(x), Boolean(y)) => ASTNode::new_list(vec![ASTNode::new_boolean(*x), ASTNode::new_boolean(*y)]),
    (List(l1), List(l2)) => ASTNode::new_list((*l1).iter().chain((*l2).iter()).map(|x| x.clone()).collect::<Vec<_>>()),
    (List(l), Number(_)) | (List(l), Boolean(_)) => ASTNode::new_list((*l).iter().chain(vec![b.clone()].iter()).map(|x| x.clone()).collect::<Vec<_>>()),
    (Number(_), List(l)) | (Boolean(_), List(l)) => ASTNode::new_list(vec![a.clone()].iter().chain((*l).iter()).map(|x| x.clone()).collect::<Vec<_>>()),
    _ => panic!("invalid types"),
  }
}

pub fn setsubtract(a: &ASTNode, b: &ASTNode) -> ASTNode {
  match (&a.kind, &b.kind) {
    (List(l1), List(l2)) => ASTNode::new_list((*l1).iter().filter(|x| !l2.contains(x)).map(|x| x.clone()).collect::<Vec<_>>()),
    (List(l), Number(_)) | (List(l), Boolean(_)) => ASTNode::new_list((*l).iter().filter(|x| *x != b).map(|x| x.clone()).collect::<Vec<_>>()),
    (Number(_), List(l)) | (Boolean(_), List(l)) => ASTNode::new_list((*l).iter().filter(|x| *x != a).map(|x| x.clone()).collect::<Vec<_>>()),
    _ => panic!("invalid types"),
  }
}

impl Add for &ASTNode {
  type Output = ASTNode;

  fn add(self, other: &ASTNode) -> ASTNode {
    match (&self.kind, &other.kind) {
      (Number(x), Number(y)) => ASTNode::new_number(x + y),
      (List(l), Number(x)) | (Number(x), List(l)) => ASTNode::new_list(l.iter().map(|val| &ASTNode::new_number(*x) + val).collect()),
      _ => panic!("invalid types"),
    }
  }
}

impl Sub for &ASTNode {
  type Output = ASTNode;

  fn sub(self, other: &ASTNode) -> ASTNode {
    match (&self.kind, &other.kind) {
      (Number(x), Number(y)) => ASTNode::new_number(x - y),
      (List(l), Number(x)) | (Number(x), List(l)) => ASTNode::new_list(l.iter().map(|val| &ASTNode::new_number(*x) - val).collect()),
      _ => panic!("invalid types"),
    }
  }
}

impl Mul for &ASTNode {
  type Output = ASTNode;

  fn mul(self, other: &ASTNode) -> ASTNode {
    match (&self.kind, &other.kind) {
      (Number(x), Number(y)) => ASTNode::new_number(x * y),
      (List(l), Number(x)) | (Number(x), List(l)) => ASTNode::new_list(l.iter().map(|val| &ASTNode::new_number(*x) * val).collect()),
      _ => panic!("invalid types"),
    }
  }
}

impl Div for &ASTNode {
  type Output = ASTNode;

  fn div(self, other: &ASTNode) -> ASTNode {
    match (&self.kind, &other.kind) {
      (Number(x), Number(y)) => ASTNode::new_number(x / y),
      (List(l), Number(x)) => ASTNode::new_list(l.iter().map(|val| val / &ASTNode::new_number(*x)).collect()),
      (Number(x), List(l)) => ASTNode::new_list(l.iter().map(|val| &ASTNode::new_number(*x) / val).collect()),
      _ => panic!("invalid types"),
    }
  }
}

impl Rem for &ASTNode {
  type Output = ASTNode;

  fn rem(self, other: &ASTNode) -> ASTNode {
    match (&self.kind, &other.kind) {
      (Number(x), Number(y)) => ASTNode::new_number(x % y),
      (List(l), Number(x)) => ASTNode::new_list(l.iter().map(|val| val % &ASTNode::new_number(*x)).collect()),
      (Number(x), List(l)) => ASTNode::new_list(l.iter().map(|val| &ASTNode::new_number(*x) % val).collect()),
      _ => panic!("invalid types"),
    }
  }
}

impl PartialEq for ASTNode {
  fn eq(&self, other: &ASTNode) -> bool {
    match (&self.kind, &other.kind) {
      (Number(x), Number(y)) => x == y,
      (List(l1), List(l2)) => l1.len() == l2.len() && l1.iter().zip(l2).all(|(x, y)| x == y),
      _ => panic!("invalid types"),
    }
  }
}

impl PartialOrd for ASTNode {
  fn partial_cmp(&self, other: &ASTNode) -> Option<Ordering> {
    match (&self.kind, &other.kind) {
      (Number(x), Number(y)) => if x < y { Some(Ordering::Less) } else if x > y { Some(Ordering::Greater) } else { Some(Ordering::Equal) },
      _ => panic!("invalid types"),
    }
  }
}