extern crate regex;

use regex::Regex;
use std::fmt;
use Sexp::Nil;
use Sexp::Integer;
use Sexp::Float;
use Sexp::Symbol;
use std::ops::Add;

#[derive(Debug)]
enum Sexp {
    Nil,
    Integer { value: i32 },
    Float { value: f32 },
    Symbol { name: String, value: String }
}

// enum Atom {
//     Integer,
//     Float,
//     Symbol
// }

// trait Number {
//     fn to_string(&self) -> String;
// }

// impl Add for Integer {
//     type Output = Integer;
//     fn add(self, other: Integer) -> Integer {
//         Integer {
//             value: self.value + other.value
//         }
//     }
// }

// impl Add for Float {
//     type Output = Float;
//     fn add(self, other: Float) -> Float {
//         Float {
//             value: self.value + other.value
//         }
//     }
// }

// impl PartialEq for Integer {
//     fn eq(&self, other: &Integer) -> bool {
//         self.value == other.value
//     }
// }

// impl PartialEq for Float {
//     fn eq(&self, other: &Float) -> bool {
//         self.value == other.value
//     }
// }

// impl fmt::Debug for Number {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

#[derive(Debug)]
struct Cell { car: Sexp, cdr: Sexp }

impl Cell {
    fn new() -> Cell {
        let cell = Cell { car: Nil, cdr: Integer { value: 1 } };
        return cell;
    }
}

// #[derive(Debug)]
// struct Integer { value: i32 }

// impl Number for Integer {
//     fn to_string(&self) -> String { self.value.to_string() }
// }

// #[derive(Debug)]
// struct Float { value: f32 }

// impl Number for Float {
//     fn to_string(&self) -> String {
//         return self.value.to_string();
//     }
// }

// struct Symbol { name: String, value: String }

fn main() {
    let cell = Cell::new();
    println!("{:?}", cell);
    // let stdin = "(begin (define r 10) (* pi (* r r)))";
    // let tokens = tokenize(&stdin);

    // println!("{:?}", tokens);
}

#[derive(Debug)]
struct Token {
    name: &'static str,
    value: String
}

/// Convert a string of characters into a list of tokens.
fn tokenize(input: &str) ->Vec<Token> {
    let replaced_input: String = input.replace("(", " ( ").replace(")", " ) ");
    let words: Vec<String> = replaced_input.split_whitespace().map(String::from).collect();
    let mut tokens = Vec::new();

    let whitespace = Regex::new(r"\s").unwrap();
    let numeral = Regex::new(r"[0-9]").unwrap();
    let letters = Regex::new(r"[a-zA-Z]").unwrap();
    let punctuations = Regex::new(r"\*").unwrap();

    for word in words {
        match word.as_ref() {
            "(" => tokens.push(Token { name: "paren", value: String::from("(") }),
            ")" => tokens.push(Token { name: "paren", value: String::from(")") }),
            s if whitespace.is_match(s) => continue,
            s if numeral.is_match(s) => tokens.push(Token { name: "number", value: String::from(s) }),
            s if letters.is_match(s) => tokens.push(Token { name: "name", value: String::from(s) }),
            s if punctuations.is_match(s) => tokens.push(Token { name: "punctuation", value: String::from(s) }),
            _ => panic!("invalid word!"),
        }
    }
    return tokens;
}

fn parse(program: &str) -> Vec<String> {
    // TODO
    return Vec::new();
}

// fn read_from_tokens(tokens: Vec<Token>) -> Vec<Token> {
//     if tokens.len() == 0 {
//         panic!("unexpected EOF while reading");
//     }
//     let mut index = 0;
//     let token = tokens[index];
//     if token.value == "(" {

//     } else if token.value == ")" {
//         panic!("unexpected )");
//     }
//     // TODO
//     return tokens;
// }
