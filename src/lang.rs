use crate::types::*;

peg::parser! {
  pub grammar grammar() for str {
    pub rule expression() -> Expression = precedence!{
        a:(@) _? "+" _? b:@ { Expression::Add { lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _? "-" _? b:@ { Expression::Sub { lhs: Box::new(a), rhs: Box::new(b) } }
        --
        a:(@) _? "*" _? b:@ { Expression::Mul { lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _? "/" _? b:@ { Expression::Div { lhs: Box::new(a), rhs: Box::new(b) } }
        --
        i:int() { i }
    }

    rule int() -> Expression
        = i:$(['0'..='9']+) { Expression::Int(i.parse::<i32>().unwrap()) }

    rule _() = quiet!{[' ' | '\n' | '\t']+}

    rule comma() = _? "," _?
  }
}

    //rule t() -> ASTNode = ty:(number() / list() / boolean()) { ty }

    //rule ident() -> ASTNode
        // = s:$(quiet!{['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*}) { ASTNode::new_ident(s) }

    //rule number() -> ASTNode
      //= n:$("-"? (['0'..='9']+ "."? ['0'..='9']* / ['0'..='9']* "."? ['0'..='9']+)) { ASTNode::new_number(n.parse::<f64>().unwrap()) }

    //rule boolean() -> ASTNode
      //= s:$("true" / "false") { ASTNode::new_boolean(s == "true") }

    //rule list() -> ASTNode
      //= "[" _? l:t() ** comma() _? "]" { ASTNode::new_list(l) }
