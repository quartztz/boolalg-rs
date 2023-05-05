#[allow(unused)]
#[allow(dead_code)]

use crate::alg::*; 

// For this ""language"", we define the following ""grammar"": 
//     expr   -> expr bin_op expr | term
//     term   -> not term | factor
//     factor -> lit | var | ( expr )

enum GrammarItem {
  And, 
  Or, 
  Not, 
  Literal(bool), 
  Paren,
}

#[derive(Debug)]
enum Token {
  BinOp(char),   // either And or Or 
  UnOp,          // Not
  Literal(bool), // T or F
  Var(char),     // char name. arbitrary. could change. no want. 
  Paren(char),   // ( or )
}

/// implements a simple lexical analysis algorithm to transform text input 
/// into an AST representable by our boolean algebra module. For example, 
/// "a ^ b" gets turned into "Expr::And(Expr::Var('a'), Expr::Var('b'))", 
/// and "T" gets turned into "Expr::Lit(true)".
/// 
/// As stated, the tokens we use are not found within the parser itself, 
/// but rather come from the representation we had in `alg.rs`, which is, 
/// admittedly, Not Good Practice, but also, it's 10pm and i'm on vacation. 
/// Good enough for now.  
pub struct Parser {
  // could potentially take in a configuration some time.
  // maybe to change the format of the tokens!
  // '&' instead of '^' for those more used to the other syntax. 
}

impl Parser {
  fn token_stream(&self, input: &str) -> Result<Vec<Token>, String> {
    let mut res = vec![]; 

    let mut iter = input.chars().peekable();

    while let Some(&n) = iter.peek() {
      match n {
        'T' | 'F' => {
          iter.next();
          let val = n == 'T';
          res.push(Token::Literal(val)); 
        }, 
        '^' | 'v' => {
          iter.next(); 
          res.push(Token::BinOp(n));
        },
        '-' => {
          iter.next(); 
          res.push(Token::UnOp);
        }
        '(' | ')' => {
          iter.next(); 
          res.push(Token::Paren(n))
        },
        ' ' => {
          iter.next();
        }
        _ => {
          if 'a' <= n && n <= 'z' { // again, super arbitrary, just made sense imho
            iter.next(); 
            res.push(Token::Var(n))
          } else {
            return Err(format!("i have no clue what {} means, sorry :(", n))
          }
        }
      }
    }
    Ok(res)
  }

  fn parse_expr(&self, tokens: &Vec<Token>, idx: usize) -> Result<(Expr, usize), String> {
    let (lhs, symb_idx) = self.parse_term(tokens, idx)?; 
    let symb = tokens.get(symb_idx);
    match symb {
      Some(&Token::BinOp(c)) => {
        let (rhs, next_idx) = self.parse_expr(tokens, symb_idx + 1)?;
        match c {
          '^' => Ok((Expr::BinOp(Op::And, bbox!(lhs), bbox!(rhs)), next_idx)),
          'v' => Ok((Expr::BinOp(Op::Or, bbox!(lhs), bbox!(rhs)), next_idx)),
          _ => Err("undefined".to_owned()),
        }
      },
      _ => Ok((lhs, symb_idx))
    }
  }

  fn parse_term(&self, tokens: &Vec<Token>, idx: usize) -> Result<(Expr, usize), String> {
    let symb = tokens.get(idx).unwrap(); 
    match symb {
      &Token::UnOp => {
        let (fact, next) = self.parse_factor(tokens, idx + 1)?;
        Ok((Expr::Not(bbox!(fact)), next))
      },
      _ => {
        let (fact, next) = self.parse_factor(tokens, idx)?; 
        Ok((fact, next))
      }
    }
  }

  fn parse_factor(&self, tokens: &Vec<Token>, idx: usize) -> Result<(Expr, usize), String> {
    let symb = tokens.get(idx).unwrap();
    match symb {
      &Token::Literal(b) => {
        Ok((Expr::Lit(b), idx + 1))
      },
      &Token::Var(n) => {
        Ok((Expr::Var(n), idx + 1))
      },
      &Token::Paren(p) => {
        if p != '(' {
          return Err(format!("unexpected weird character {}", p))
        }
        self.parse_expr(tokens, idx + 1).and_then(|(node, next_idx)| {
          if let Some(&Token::Paren(c2)) = tokens.get(next_idx) {
            if c2 == ')' {
              Ok((node, next_idx + 1))
            } else {
              Err("mismatched".to_owned())
            } 
          } else {
            Err("unexpected".to_owned())
          }
        })
      }
      _ => Err(format!("error while processing symbol {:?}", symb))
    }
  }

  pub fn parse(&self, s: &str) -> Result<Expr, String> {
    let tokens = self.token_stream(s)?; 
    self.parse_expr(&tokens, 0).and_then(|(n, i)| {
      if i == tokens.len() {
        Ok(n)
      } else {
        Err("something's off".to_owned())
      }
    })
  } 
}

#[cfg(test)]
mod tests {
  use super::*; 

  #[test]
  fn parse_literal() {
    let p: Parser = Parser {};
    let expr: Expr = p.parse("T").unwrap();
    
    println!("{}", expr.render());

    assert_eq!(expr, Expr::Lit(true));
  }

  #[test]
  fn parse_variable() {
    let p: Parser = Parser {}; 
    let expr: Expr = p.parse("a").unwrap(); 

    println!("{}", expr.render());

    assert_eq!(expr, Expr::Var('a'));
  }

  #[test]
  fn parse_binop() {
    let p = Parser{}; 
    let expr: Expr = p.parse("a ^ T").unwrap();

    println!("{}", expr.render()); 

    assert_eq!(expr, Expr::BinOp(Op::And, bbox!(Expr::Var('a')), bbox!(Expr::Lit(true))));
  }

  #[test]
  fn parse_unop() {
    let p = Parser {}; 
    let expr = p.parse("-a").unwrap();
    
    println!("{}", expr.render()); 
    
    assert_eq!(expr, Expr::Not(bbox!(Expr::Var('a'))));
  }

  #[test]
  fn parse_parentheses_1() {
    let p = Parser {}; 
    let expr = p.parse("(a v b) ^ c").unwrap();
    
    println!("{}", expr.render()); 
    
    assert_eq!(expr, Expr::BinOp(Op::And, bbox!(Expr::BinOp(Op::Or, bbox!(Expr::Var('a')), bbox!(Expr::Var('b')))), bbox!(Expr::Var('c'))));
  }
  #[test]
  fn parse_parentheses_2() {
    let p = Parser {}; 
    let expr = p.parse("a v (b ^ c)").unwrap();
    
    println!("{}", expr.render()); 
    
    assert_eq!(expr, Expr::BinOp(Op::Or, bbox!(Expr::Var('a')), bbox!(Expr::BinOp(Op::And, bbox!(Expr::Var('b')), bbox!(Expr::Var('c'))))));
  }
}
