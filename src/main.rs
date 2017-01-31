extern crate regex;

use regex::Regex;
use std::fmt;
use std::ops::Add;

use std::collections::HashMap;

use Expr::Atom;
// use Expr::Nil;
use Expr::List;

#[derive(PartialEq, Clone, Debug)]
enum Expr<T> {
    // Nil,
    Atom(T),
    List(Vec<Expr<T>>)
}

type E = Expr<String>;

struct Env {
    bindings: HashMap<String, E>
}

impl Env {
    fn new() -> Env {
        Env { bindings: HashMap::new() }
    }

    fn find(&self, key: &str) -> Option<E> {
        self.bindings.get(key).cloned()
    }
}

type EvalResult = Result<E, &'static str>;

fn eval(env: &mut Env, expr: E) -> EvalResult {
    match expr {
        // Nil => Ok(Nil),
        Atom(v) => {
            match env.find(&v) {
                None => Err("Symbol not found."),
                Some(e) => Ok(e)
            }
        },
        List(vec) => {
            if vec.len() == 0 {
                return Err("No procedure to call.")
            }
            if is_symbol("quote", &vec[0]) {
                if vec.len() != 2 {
                    Err("`quote` expects exactly one argument.")
                } else {
                    Ok(vec[1].clone())
                }
            } else if is_symbol("atom", &vec[0]) {
                eval_atom(env, vec)
            } else if is_symbol("eq", &vec[0]) {
                eval_eq(env, vec)
            } else if is_symbol("car", &vec[0]) {
                eval_car(env, vec)
            } else if is_symbol("cdr", &vec[0]) {
                eval_cdr(env, vec)
            } else {
                Ok(Expr::empty_list())
            }
        }
    }
}

fn eval_atom(env: &mut Env, vec: Vec<E>) -> EvalResult {
    if vec.len() != 2 {
        Err("`atom` expects exactly one argument.")
    } else {
        let val = try!(eval(env, vec[1].clone()));
        if val.is_atom() || val.is_empty_list() {
            Ok(Atom("t".to_string()))
        } else {
            Ok(Expr::empty_list())
        }
    }
}

fn eval_eq(env: &mut Env, vec: Vec<E>) -> EvalResult {
    if vec.len() != 3 {
        Err("`eq` expects exactly two arguments.")
    } else {
        let val1 = try!(eval(env, vec[1].clone()));
        let val2 = try!(eval(env, vec[2].clone()));
        if (val1.is_empty_list() && val2.is_empty_list())
            || (val1.is_atom() && val2.is_atom() && val1.eq(&val2)) {
            Ok(Atom("t".to_string()))
        } else {
            Ok(Expr::empty_list())
        }
    }
}

fn eval_car(env: &mut Env, vec: Vec<E>) -> EvalResult {
    if vec.len() != 2 {
        Err("`car` expects exactly one argument.")
    } else {
        let val = try!(eval(env, vec[1].clone()));
        if val.is_list() && !val.is_empty_list() {
            let list = val.unwrap_list();
            Ok(list[0].clone())
        } else {
            Ok(Expr::empty_list())
        }
    }
}

fn eval_cdr(env: &mut Env, vec: Vec<E>) -> EvalResult {
    if vec.len() != 2 {
        Err("`cdr` expects exactly one argument.")
    } else {
        let val = try!(eval(env, vec[1].clone()));
        if val.is_list() && !val.is_empty_list() {
            let mut list = val.unwrap_list();
            list.remove(0);
            Ok(List(list.clone()))
        } else {
            Ok(Expr::empty_list())
        }
    }
}

fn is_symbol(op: &str, expr: &E) -> bool {
    if expr.is_atom() {
        let expr_op = expr.get_ref_atom();
        op == expr_op
    } else {
        false
    }
}

impl<T: fmt::Display> Expr<T> {

    fn p(&self) {
        self.print();
        println!("");
    }

    fn print(&self) {
        match *self {
            Atom(ref val) => print!("{}", *val),
            // Nil => print!("Nil"),
            List(ref vec) => {
                print!("(");
                if vec.len() > 0 {
                    let mut vec_iter = vec.iter();
                    let first = vec_iter.next();
                    first.unwrap().print();
                    for expr in vec_iter {
                        print!(" ");
                        expr.print();
                    }
                }
                print!(")");
            }
        }
    }
}

impl<T: Eq> Expr<T> {
    fn is_empty_list(&self) -> bool {
        self.eq(&Expr::empty_list())
    }
}

impl<T> Expr<T> {
    fn empty_list() -> Expr<T> {
        List(vec!())
    }

    // fn is_nil(&self) -> bool {
    //     match *self {
    //         Nil => true,
    //         _ => false
    //     }
    // }

    fn is_list(&self) -> bool {
        !self.is_atom()
    }

    fn is_atom(&self) -> bool {
        match *self {
            Atom(_) => true,
            _ => false
        }
    }

    fn get_ref_atom(&self) -> &T {
        match *self {
            Atom(ref v) => v,
            _ => panic!("called Expression::get_ref_atom() on non-Atom")
        }
    }

    fn unwrap_atom(self) -> T {
        match self {
            Atom(val) => val,
            _ => panic!("called Expression::unwrap_atom() on non-Atom")
        }
    }

    fn unwrap_list(self) -> Vec<Expr<T>> {
        match self {
            List(val) => val,
            _ => panic!("called Expression::unwrap_list() on non-List")
        }
    }
}


fn main() {

    let mut env = Env::new();

    let a1 = Atom("cdr".to_string());
    let a2 = Atom("quote".to_string());
    let a3 = Atom("a".to_string());
    let a4 = Atom("b".to_string());
    let a5 = Atom("c".to_string());
    // let list2 = List(vec![Atom("a".to_string()), Atom("b".to_string()), Atom("c".to_string())]);
    // let list1 = List(vec![a1, list2]);

    let arg = List(vec![a1, List(vec![a2, List(vec![a3, a4, a5])])]);

    arg.p();

    let ret = eval(&mut env, arg);

    println!("{:?}", ret);

    ret.unwrap().p();
}

#[test]
fn test_eval() {
    assert!(true);
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
