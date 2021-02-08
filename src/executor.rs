pub use crate::types::{ASTNode, ASTType::*, Operator, Operator::*};
pub use crate::math;

use std::collections::HashMap;
use std::cell::RefCell;

thread_local! {
    static REFENV: RefCell<HashMap<String, ASTNode>> = RefCell::new(HashMap::new());
}

pub fn execute_op<'a>(root: &'a ASTNode, op: &'a Operator) -> ASTNode {
  match op {
    Add => &execute(&root.children[0]) + &execute(&root.children[1]),
    Subtract => &execute(&root.children[0]) - &execute(&root.children[1]),
    Multiply => &execute(&root.children[0]) * &execute(&root.children[1]),
    Divide => &execute(&root.children[0]) / &execute(&root.children[1]),
    Modulus => &execute(&root.children[0]) % &execute(&root.children[1]),
    Equals => ASTNode::new_boolean(execute(&root.children[0]) == execute(&root.children[1])),
    Not => math::not(&execute(&root.children[0])),
    Factorial => math::factorial(&execute(&root.children[0])),
    NotEquals => ASTNode::new_boolean(execute(&root.children[0]) != execute(&root.children[1])),
    LessThan => ASTNode::new_boolean(execute(&root.children[0]) < execute(&root.children[1])),
    GreaterThan => ASTNode::new_boolean(execute(&root.children[0]) > execute(&root.children[1])),
    LessThanEquals => ASTNode::new_boolean(execute(&root.children[0]) <= execute(&root.children[1])),
    GreaterThanEquals => ASTNode::new_boolean(execute(&root.children[0]) >= execute(&root.children[1])),
    Power => math::pow(&execute(&root.children[0]), &execute(&root.children[1])),
    Abs => math::abs(&execute(&root.children[0])),
    Or => math::or(&execute(&root.children[0]), &execute(&root.children[1])),
    And => math::and(&execute(&root.children[0]), &execute(&root.children[1])),
    SetAdd => math::setadd(&execute(&root.children[0]), &execute(&root.children[1])),
    SetSubtract => math::setsubtract(&execute(&root.children[0]), &execute(&root.children[1])),
  }
}

pub fn insert_var(s: String, v: ASTNode) {
  REFENV.with(|re| {
    re.borrow_mut().insert(s, v)
  });
}

pub fn execute(root: &ASTNode) -> ASTNode {
  match &root.kind {
    Number(n) => ASTNode::new_number(*n),
    Boolean(b) => ASTNode::new_boolean(*b),
    Op(op) => execute_op(root, &op),
    Ident(i) => REFENV.with(|re| {
      let vars = re.borrow();
      if vars.contains_key(i) { vars[i].clone() } else { panic!() }
    }),
    List(l) => {
      let mut list = vec![];
      for l in l {
        list.push(execute(&l));
      }
      ASTNode::new_list(list)
    },
    _ => panic!(),
  }
}