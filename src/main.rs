#[allow(unused_imports)]

use crate::alg::*; 
use crate::parse::*; 
#[macro_use]
mod alg; 
mod parse; 

fn int_to_bool(val: usize) -> bool {
    if val % 2 == 0 { false } else { true }
}

fn main() -> Result<(), String> {
    let stdin = std::io::stdin();

    let mut eval: Evaluator = Evaluator::new();
    let parser: Parser = Parser {}; 
    let mut inpt: String = String::new(); 
    let mut done: bool = false; 

    while !done {
        inpt = String::new(); 
        stdin.read_line(&mut inpt).unwrap();
        
        inpt = (&inpt[..inpt.len() - 1]).to_owned();
        
        // ugly
        if inpt == "close" {
            done = true;
        } else if inpt == "help" {
            println!("TODO");
        } else if inpt.starts_with("?: ") { // query
            let parsed = parser.parse(&inpt[3..])?;
            println!("parsed => {}", parsed.render()); 
            let evaled = eval.evaluate(parsed); 
            println!("evaluated => {}", evaled.render());    
        } else if inpt.starts_with("!: ") { // directive
            // input must be of shape char -> {T | F}
            let dir: Vec<char> = (&inpt[3..]).chars().collect();
            let var = dir.get(0).unwrap(); 
            let val = dir.last().unwrap() == &'T';

            println!("=> setting {} to {}", var, val); 

            eval.update_var(*var, val);
        } else {
            println!("bad input.");
        }
    }

    Ok(())
}
