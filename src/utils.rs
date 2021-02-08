pub use crate::types::{ASTNode, ASTType::*, Operator, ASTType};

pub fn print_astnode(node: &ASTNode) {
    match &node.kind {
        Number(s) => print!(" {} ", s),
        Boolean(b) => print!(" {} ", b),
        Ident(s) => print!(" {} ", s),
        List(l) => {
          print!("[ ");

          for child in l {
              print_astnode(&child);
              print!(",");
          }

          print!(" ]");
        }
        Op(op) => {
            print!("{:?}( ", op);

            for child in &node.children {
                print_astnode(&child);
            }

            print!(" ) ");
        }
        Fun {
            name,
            args,
            body: _,
        } => {
            print!("fn {}({})", name, args.join(", "));
        }
    }
}

impl ASTNode {
  pub fn new_op(op: Operator, children: Vec<ASTNode>) -> ASTNode {
    ASTNode { kind: ASTType::Op(op), children}
  }

  pub fn new_ident<'a>(s: &'a str) -> ASTNode {
    ASTNode { kind: Ident(String::from(s)), children: Vec::new()}
  }

  pub fn new_list(list: Vec<ASTNode>) -> ASTNode {
    ASTNode { kind: List(list), children: Vec::new() }
  }

  pub fn new_number(number: f64) -> ASTNode {
    ASTNode { kind: Number(number), children: Vec::new() }
  }

  pub fn new_boolean(b: bool) -> ASTNode {
    ASTNode { kind: Boolean(b), children: Vec::new() }
  }
}