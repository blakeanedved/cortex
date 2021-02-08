pub use crate::types::{ASTNode, ASTType, Operator};

peg::parser! {
  pub grammar language() for str {
    rule expr() -> ASTNode = precedence!{
      a:(@) _? "||" _? b:@ { ASTNode::new_op(Operator::Or, vec![a,b]) }
      --
      a:(@) _? "&&" _? b:@ { ASTNode::new_op(Operator::And, vec![a,b]) }
      --
      a:(@) _? "==" _? b:@ { ASTNode::new_op(Operator::Equals, vec![a,b]) }
      a:(@) _? "!=" _? b:@ { ASTNode::new_op(Operator::NotEquals, vec![a,b]) }
      --
      a:(@) _? "<" _? b:@ { ASTNode::new_op(Operator::LessThan, vec![a,b]) }
      a:(@) _? ">" _? b:@ { ASTNode::new_op(Operator::GreaterThan, vec![a,b]) }
      a:(@) _? "<=" _? b:@ { ASTNode::new_op(Operator::LessThanEquals, vec![a,b]) }
      a:(@) _? ">=" _? b:@ { ASTNode::new_op(Operator::GreaterThanEquals, vec![a,b]) }
      --
      a:(@) _? "+" _? b:@ { ASTNode::new_op(Operator::Add, vec![a,b]) }
      a:(@) _? "-" _? b:@ { ASTNode::new_op(Operator::Subtract, vec![a,b]) }
      --
      a:(@) _? "*" _? b:@ { ASTNode::new_op(Operator::Multiply, vec![a,b]) }
      a:(@) _? "/" _? b:@ { ASTNode::new_op(Operator::Divide, vec![a,b]) }
      a:(@) _? "%" _? b:@ { ASTNode::new_op(Operator::Modulus, vec![a,b]) }
      "-" _? a:(@) { ASTNode::new_op(Operator::Multiply, vec![ASTNode::new_number(-1.0), a]) }
      a:(@) _? b:@ { ASTNode::new_op(Operator::Multiply, vec![a,b]) }
      --
      a:@ _? "^" _? b:(@) { ASTNode::new_op(Operator::Power, vec![a, b]) }
      --
      "!" _? a:(@) { ASTNode::new_op(Operator::Not, vec![a]) }
      a:(@) _? "!" { ASTNode::new_op(Operator::Factorial, vec![a]) }
      --
      a:(@) _? "<>" _? b:@ { ASTNode::new_op(Operator::SetAdd, vec![a, b]) }
      a:(@) _? "><" _? b:@ { ASTNode::new_op(Operator::SetSubtract, vec![a, b]) }
      --
      n:t() { n }
      i:ident() { i }
      "(" _? e:expr() _? ")" { e }
      "|" _? e:expr() _? "|" { ASTNode::new_op(Operator::Abs, vec![e]) }
    }

    rule _() = quiet!{[' ' | '\n' | '\t']+}

    rule comma() = _? "," _?

    rule t() -> ASTNode = ty:(number() / list() / boolean()) { ty }

    rule ident() -> ASTNode
        = s:$(quiet!{['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*}) { ASTNode::new_ident(s) }
        / expected!("identifier") 

    rule number() -> ASTNode
      = n:$("-"? (['0'..='9']+ "."? ['0'..='9']* / ['0'..='9']* "."? ['0'..='9']+)) { ASTNode::new_number(n.parse::<f64>().unwrap()) }

    rule boolean() -> ASTNode
      = s:$("true" / "false") { ASTNode::new_boolean(s == "true") }

    rule list() -> ASTNode
      = "[" _? l:t() ** comma() _? "]" { ASTNode::new_list(l) }

    pub rule program() -> ASTNode
      = expr()
  }
}