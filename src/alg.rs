#[allow(unused)]
#[allow(dead_code)]

use std::collections::HashMap;

type Context = HashMap<char, bool>;

macro_rules! bbox {
    ($a:expr) => { 
        Box::new($a)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Op { // public for debug purposes 
    And, 
    Or
}

impl Op {
    fn sym(&self) -> String {
        match self {
            Op::And => {
                "^".to_owned()
            },
            Op::Or => {
                "v".to_owned()
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Lit(bool),                       // literal, True or False 
    Var(char),                       // name of variable value, single char for now
    Not(Box<Expr>),                   // ¬
    BinOp(Op, Box<Expr>, Box<Expr>),   // ^, v
}

impl Expr {
    pub fn render(&self) -> String {
        use Expr::*; // would want to avoid this ideally. 
        
        match self {
            Lit(b) => if *b { "T".to_string() } else { "F".to_string() },
            Var(n) => format!("{}", n),
            Not(e) => format!("¬({})", e.render()),
            BinOp(op, e1, e2) => {
                let s1: String = e1.render(); 
                let s2: String = e2.render(); 
                format!("({} {} {})", s1, op.sym(), s2)
            },
        }
    }
}

impl From<bool> for Expr {
    fn from(other: bool) -> Expr {
        Expr::Lit(other)
    }
}

impl From<char> for Expr {
    fn from(other: char) -> Expr {
        Expr::Var(other)
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
                Or => { // e1 || e2
                    match (val1, val2) {
                        (Lit(true), _) |
                        (_, Lit(true)) => Lit(true), 

                        (Lit(false), th) |
                        (th, Lit(false)) => th,

                        (v1, v2) => BinOp(op, Box::new(v1), Box::new(v2)),
                    }
                } 
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    }
}

fn substitute(e: Expr, c: &Context) -> Expr {
    use Expr::*; 

    match e {
        Lit(_) => e,
        Var(n) => {
            match c.get(&n) {
                Some(b) => Expr::from(*b),
                None => e
            }
        },
        Not(e) => Not(Box::new(substitute(*e, c))),
        BinOp(op, e1, e2) => BinOp(op, Box::new(substitute(*e1, c)), Box::new(substitute(*e2, c))),
    }
}

pub struct Evaluator { 
    pub context: Context 
}

impl Evaluator {

    /// Constructs a new evaluator with an empty context
    pub fn new() -> Evaluator {
        Evaluator {
            context: Context::new()
        }
    }

    /// Given a variable name and a new value, updates its value within the
    /// Evaluator's context.  
    /// 
    /// ### Arguments
    ///  * `var`: name of the variable
    ///  * `val`: value of the variable
    /// 
    /// ### Returns
    /// true if `var` was already defined, false otherwise
    
    pub fn update_var(&mut self, var: char, val: bool) -> bool {
        match self.context.insert(var, val) {
            Some(v) => true,
            None => false, 
        }
    }

    /// Counts how many variables have been set. 
    /// Potentially useless. 
    /// 
    /// ### Returns
    /// the amount of

    pub fn count_vars(&self) -> usize {
        self.context.len()
    }

    pub fn get_vars(&self) -> Vec<&char> {
        self.context.keys().collect::<Vec<&char>>()
    }

    /// Evaluates the given expression within the context of the current hashmap
    /// and returns the result. 
    /// 
    /// ### Arguments
    ///  * `e`: the expression to evaluate
    /// 
    /// ### Returns
    /// the result of the evaluation. 
    
    pub fn evaluate(&self, e: Expr) -> Expr {
        let subst = substitute(e, &self.context); 
        evaluate(subst)
    }
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
