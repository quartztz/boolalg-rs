use crate::alg::*; 
#[macro_use]
mod alg; 

fn int_to_bool(val: usize) -> bool {
    if val % 2 == 0 { false } else { true }
}

fn main() {
    let mut eval: Evaluator = Evaluator::new();

    let f = Expr::BinOp(Op::And, bbox!(Expr::Var('a')), bbox!(Expr::Var('b'))); 

    eval.update_var('a', false);
    eval.update_var('b', false);

    let n = eval.count_vars();  
    let vars = eval.context.keys().clone(); 

    for i in 0..usize::pow(2, n as u32) {
        let mut j = 0;

        // TODO this doesn't work. 
        for k in vars.into_iter() {
            eval.update_var(*k, int_to_bool(i >> j)); 
            j += 1; 
        }
    }
}
