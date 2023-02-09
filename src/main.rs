#[allow(unused_imports)]

use std::io::{self, stdin, stdout, Write};
use std::collections::HashMap; 

use crate::alg::*; 
use crate::parse::*; 

#[macro_use]
mod alg; 
mod parse; 

fn main() -> Result<(), String> {
    let stdin = stdin();
    let mut done: bool = false; 

    let mut definitions: HashMap<char, Expr> = HashMap::new();
    let mut eval: Evaluator = Evaluator::new();
    let parser: Parser = Parser {}; 
    
    while !done {
        print!("> "); 
        stdout().flush().unwrap();
        
        let mut command: String = String::new();
        stdin.read_line(&mut command).unwrap();
        
        let args: Vec<&str> = command.split_whitespace().collect();
        
        // ugly
        if args.len() == 0 {
            // catch-all case
        } else if args.len() == 1 { 
            // single word arguments
            if args[0] == "close" {
                println!("closing"); 
                done = true;
            } else if args[0] == "help" {
                println!("TODO");
            } else if args[0] == "status" {
                println!("{:?}", eval.context);
            } else {
                println!("malformed input: unknown command");
            }
        } else {
            match args[0] {
                "$:" => {
                    // input of the form: 
                    // "$: char = { expr }"
                    let name: char = args[1].chars().next().unwrap();
                    let expr: &str = &args[3..].concat();
                    let res = parser.parse(expr)?;
                    println!("added => {} = {}", name, res.render());
                    definitions.insert(name, res);
                },
                "?:" => {
                    let expr: &str = &args[1..].concat();
                    if expr.chars().nth(0) == Some('$') {
                        // input of the form
                        // "?: $char"
                        if expr.len() == 2 {
                            let name = expr.chars().nth(1).unwrap();
                            let fetched = definitions.get(&name).unwrap();
                            println!("using prev def => {}", fetched.render());
                            let evaled = eval.evaluate(fetched.clone()); 
                            println!("evaluated => {}", evaled.render());
                        }
                    } else {
                        let parsed = parser.parse(expr)?;
                        println!("parsed => {}", parsed.render());
                        let evaled = eval.evaluate(parsed);
                        println!("evaluated => {}", evaled.render());
                    }
                },
                "!:" => {
                    let input = &args[1..];
                    if input.len() != 3 {
                        println!("malformed input");
                        break;
                    }

                    let var = input[0].chars().next().unwrap(); // ugly as fuck 
                    let val = input[input.len() - 1];

                    println!("=> setting {} to {}", var, val); 

                    eval.update_var(var, val);
                },
                _ => println!("malformed input: unrecognized directive")
            }
        }
    }

    Ok(())
}
