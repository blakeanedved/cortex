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
        ident:ident() "(" _? args:expression() ** comma() _? ")" { Expression::FunctionCall { ident, args } }
        "(" _? e:expression() _? ")" { e.data }
        i:int() { i }
        f:float() { f }
        l:list() { l }
        t:tuple() { t }
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
        = start:position!() a:assignable() _? "=" _? e:expression() end:position!() {
            Locatable::new(Statement::Assignment {
                assignable: a,
                expr: e
            }, (start, end))
        }

    rule assignable() -> Locatable<Assignable>
        = start:position!() ident:ident() end:position!() { Locatable::new(Assignable::Single(ident), (start, end)) }
        / start:position!() "(" _? a:assignable() ** comma() _? ")" end:position!() { Locatable::new(Assignable::Multi(a), (start, end)) }

    rule alpha() -> &'input str
        = a:$(['a'..='z' | 'A'..='Z']) { a }

    rule digit() -> &'input str
        = d:$(['0'..='9']) { d }

    rule ident() -> String
        = id:$((alpha() / "_")+) { String::from(id) }

    rule int() -> Expression
        = i:$(("-" / "+")? digit()+) { Expression::Int(i.parse::<i32>().unwrap()) }

    rule tuple() -> Expression
        = "(" _? values:expression() ** comma() _? ")" { Expression::Tuple(values) }

    rule list() -> Expression
        = "[" _? values:expression() ** comma() _? "]" { Expression::List(values) }

    rule ident_list() -> Locatable<Vec<String>>
        = start:position!() "(" _? idents:ident() ** comma() _? ")" end:position!() { Locatable::new(idents, (start, end)) }

    rule iterator() -> LocExpr
        = start:position!() a:assignable() _? "::" _? e:expression() end:position!() { Locatable::new(Expression::Iterator { assignable: a, value: Box::new(e) }, (start, end)) }

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
