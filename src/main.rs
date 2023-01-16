use std::collections::HashMap;

type Context = HashMap<char, bool>;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    And, 
    Or
}

fn op_symbol(o: Op) -> String{
    use Op::*;

    match o {
        And => "^".to_string(),
        Or => "v".to_string(),
        _ => todo!("what"),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Lit(bool),                       // literal, True or False 
    Var(char),                       // name of variable value, single char for now
    Not(Box<Expr>),                  // ¬
    BinOp(Op, Box<Expr>, Box<Expr>), // ^, v
}

impl Expr {
    fn from_bool(b: bool) -> Expr {
        Expr::Lit(b)
    }
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

fn show_bin_op(e: Expr) -> String {
    use Expr::*;
    match e {
        BinOp(op, e1, e2) => { 
            let s1: String = show(*e1); 
            let s2: String = show(*e2); 
            format!("{} {} {}", s1, op_symbol(op), s2)
        },
        _ => todo!("what"),
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
    
    let mut C: Context = HashMap::new(); 

    C.insert('x', false);

    let e1: Expr = Expr::BinOp(Op::Or, Box::new(Expr::from_bool(false)), Box::new(Expr::Var('x')));
    let e2: Expr = Expr::BinOp(Op::Or, Box::new(Expr::from_bool(false)), Box::new(Expr::Var('x')));
    println!("{}", show(evaluate(e1)));
    let e3: Expr = substitute(e2, &C);
    println!("{}", show(evaluate(e3)));

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
