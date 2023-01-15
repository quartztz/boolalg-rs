use std::collections::HashMap;

type Context = HashMap<char, bool>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    And, 
    Or
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Lit(bool),                       // literal, True or False 
    Var(char),                       // name of variable value, single char for now
    Not(Box<Expr>),                  // ¬
    BinOp(Op, Box<Expr>, Box<Expr>), // ^, v
}

fn evaluate(e: Expr) -> Expr { // add a context later?
    
    use Expr::*; // worst practice
    use Op::*;

    match e {
        Lit(_) => e,
        Var(_) => e,
        Not(ex) => {
            let val = evaluate(*ex);
            match val {
                Lit(x) => Lit(!x),
                _ => Not(Box::new(val)), 
            }
        },
        BinOp(op, e1, e2) => {
            let val1 = evaluate(*e1);
            let val2 = evaluate(*e2);
            match op {
                And => { // e1 && e2
                    match (val1, val2) {
                        (Lit(false), _) | 
                        (_, Lit(false)) => Lit(false),
                        
                        (Lit(true), th) | 
                        (th, Lit(true)) => th,

                        (v1, v2) => BinOp(op, Box::new(v1), Box::new(v2)),
                    }
                } 
                Or => {
                    match (val1, val2) {
                        (Lit(true), _) |
                        (_, Lit(true)) => Lit(true), 

                        (Lit(false), th) |
                        (th, Lit(false)) => th,

                        (v1, v2) => BinOp(op, Box::new(v1), Box::new(v2)),
                    }
                } 
                _ => todo!(),
            }
        },
        _ => todo!(),
    }
}

fn substitute(e: Expr, c: &Context) -> Expr {
    use Expr::*; 
    use Op::*; 

    match e {
        Lit(_) => e,
        Var(n) => {
            let val = c.get(&n);
            match val {
                Some(b) => Lit(*b), 
                None => e
            }
        },
        Not(e) => Not(Box::new(substitute(*e, c))),
        BinOp(op, e1, e2) => BinOp(op, Box::new(substitute(*e1, c)), Box::new(substitute(*e2, c))),
    }
}

fn show(e: Expr) -> String {
    use Expr::*;
    use Op::*;

    match e {
        Lit(b) => if b { "T".to_string() } else { "F".to_string() },
        Var(n) => format!("{}", n),
        Not(e) => format!("¬({})", show(*e)),
        BinOp(op, e1, e2) => match op { // easily refactored in aux function
            And => format!("({} ^ {})", show(*e1), show(*e2)),
            Or => format!("({} v {})", show(*e1), show(*e2)), 
            _ => format!("unimplemented!"),
        },
    }
}

fn main() {
    let BT = Box::new(Expr::Lit(true)); 
    let BF = Box::new(Expr::Lit(false)); 
    
    let render: String = show(Expr::BinOp(Op::And, Box::new(Expr::Not(BF)), Box::new(Expr::Var('x'))));

    println!("{}", render);
}


#[cfg(test)]

mod tests {
    use super::*; 
    use Expr::*; 
    use Op::*;
    
    #[test]
    fn test_or_axiom_1() {
        // x v T = T
        
        let BT = Box::new(Lit(true));
        let var = Box::new(Var('x')); 
        
        assert_eq!(evaluate(BinOp(Or, BT, var)), Lit(true));
    }

    #[test]
    fn test_or_axiom_2() {
        // x v F = x
        
        let BF = Box::new(Lit(false));
        let var = Box::new(Var('x')); 
        
        assert_eq!(evaluate(BinOp(Or, BF, var)), Var('x'));
    }

    #[test]
    fn test_and_axiom_1() {
        // x ^ F = F
         
        let BF = Box::new(Lit(false));
        let var = Box::new(Var('x'));

        assert_eq!(evaluate(BinOp(And, BF, var)), Lit(false));
    }

    #[test]
    fn test_and_axiom_2() {
        // x ^ T = x
        
        let BT = Box::new(Lit(true)); 
        let var = Box::new(Var('x'));

        assert_eq!(evaluate(BinOp(And, BT, var)), Var('x'));
    }
}
