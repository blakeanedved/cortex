use crate::types::*;

peg::parser! {
  pub grammar grammar() for str {
    pub rule expression() -> LocExpr = precedence!{
        start:position!() e:@ end:position!() { Locatable::new(e, (start, end)) }
        --
        a:(@) _? "+" _? b:@ { Expression::Add { lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _? "-" _? b:@ { Expression::Sub { lhs: Box::new(a), rhs: Box::new(b) } }
        --
        a:(@) _? "*" _? b:@ { Expression::Mul { lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _? "/" _? b:@ { Expression::Div { lhs: Box::new(a), rhs: Box::new(b) } }
        --
        ident:ident() "(" _? args:expression() ** comma() _? ")" { Expression::Call { ident, args } }
        "(" _? e:expression() _? ")" { e.data }
        i:int() { i }
        f:float() { f }
        l:list() { l }
        id:ident() { Expression::Ident( id ) }
    }

    pub rule function_definition() -> Locatable<Statement>
        = start:position!() ident:ident() "(" _? args:ident() ** comma() _? ")" _? "=" _? e:expression() end:position!() {
            Locatable::new(Statement::FunctionDefinition {
                ident,
                args,
                body: e
            }, (start, end))
        }

    pub rule assignment() -> Locatable<Statement>
        = start:position!() ident:ident() _? "=" _? e:expression() end:position!() {
            Locatable::new(Statement::Assignment {
                ident,
                expr: e
            }, (start, end))
        }

    rule alpha() -> &'input str
        = a:$(['a'..='z' | 'A'..='Z']) { a }

    rule digit() -> &'input str
        = d:$(['0'..='9']) { d }

    rule ident() -> String
        = id:$((alpha() / "_")+) { String::from(id) }

    rule int() -> Expression
        = i:$(("-" / "+")? digit()+) { Expression::Int(i.parse::<i32>().unwrap()) }

    rule list() -> Expression
        = "[" _? values:expression() ** comma() _? "]" { Expression::List(values) }

    rule iterator() -> LocExpr
        = start:position!() ident:ident() _? "::" _? e:expression() end:position!() { Locatable::new(Expression::Iterator { ident, content: Box::new(e) }, (start, end)) }

    rule condition() -> Locatable<Condition>
        = start:position!() a:expression() _? cmp:$(">" / "<" / "==" / "<=" / ">=" / "!=") _? b:expression() end:position!() {
            Locatable::new(Condition {
                lhs: Box::new(a),
                rhs: Box::new(b),
                cond_type: match cmp {
                    ">" => ConditionType::Greater,
                    "<" => ConditionType::Lesser,
                    ">=" => ConditionType::GreaterEqual,
                    "<=" => ConditionType::LesserEqual,
                    "==" => ConditionType::Equal,
                    "!=" => ConditionType::NotEqual,
                    _ => unreachable!(),
                } 
            }, (start, end)) 
        }

    pub rule list_generator() -> Expression
        = "{" it:iterator() ** comma() _? "|" _? c:condition() ** comma() _? "}" { Expression::ListGenerator { iterators: it, qualifications: c } }

    rule float() -> Expression
        = f:$(("-" / "+")? ((digit()+ "."? digit()*) / (digit()* "."? digit()+))) { Expression::Float(f.parse::<f32>().unwrap()) }

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
