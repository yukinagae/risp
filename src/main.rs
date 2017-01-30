extern crate regex;

use regex::Regex;
use std::fmt;
use std::ops::Add;

use SType::Nil;
use SType::Str;
use SType::Symbol;
use SType::Int;


enum SType {
    Nil,
    Bool(bool),
    Str(String),
    Symbol(String),
    Int(isize)
}

impl fmt::Display for Nil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "nil")
    }
}

// impl SType {
//     fn to_string(&self) -> String {
//         match *self {
//             Nil => "nil".to_string(),
//             _ => "hoge".to_string()
//         }
//     }
// }

fn main() {

    println!("{:?}", Nil);
    // println!("{:?}", Str(String::from("foo")));
    // println!("{:?}", Symbol(String::from("bar")));
    // println!("{:?}", Int(32));
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
