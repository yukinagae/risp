#![allow(unused_imports)]

extern crate regex;

use regex::Regex;
use std::fmt;

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

#[derive(Clone, Debug)]
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
            match vec {
                ref v if v.len() == 0 => Err("No procedure to call."),
                ref v if is_symbol("quote", &v[0]) => eval_quote(v),
                ref v if is_symbol("atom", &v[0]) => eval_atom(env, v),
                ref v if is_symbol("eq", &v[0]) => eval_eq(env, v),
                ref v if is_symbol("car", &v[0]) => eval_car(env, v),
                ref v if is_symbol("cdr", &v[0]) => eval_cdr(env, v),
                ref v if is_symbol("cons", &v[0]) => eval_cons(env, v),
                ref v if is_symbol("cond", &v[0]) => eval_cond(env, v),
                ref v if is_symbol("defun", &v[0]) => eval_defun(env, v),
                ref v => eval_func_call(env, v.clone()),
            }
        }
    }
}

fn eval_quote(vec: &Vec<E>) -> EvalResult {
    if vec.len() != 2 {
       Err("`quote` expects exactly one argument.")
    } else {
        Ok(vec[1].clone())
    }
}

fn eval_atom(env: &mut Env, vec: &Vec<E>) -> EvalResult {
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

fn eval_eq(env: &mut Env, vec: &Vec<E>) -> EvalResult {
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

fn eval_car(env: &mut Env, vec: &Vec<E>) -> EvalResult {
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

fn eval_cdr(env: &mut Env, vec: &Vec<E>) -> EvalResult {
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

fn eval_cons(env: &mut Env, vec: &Vec<E>) -> EvalResult {
    if vec.len() != 3 {
        Err("`cons` expects exactly two argument.")
    } else {
        let val1 = try!(eval(env, vec[1].clone()));
        let val2 = try!(eval(env, vec[2].clone()));
        match val2 {
            List(mut list) => {
                list.insert(0, val1);
                Ok(List(list))
            },
            _ => Err("`cons`'s second argument must be a list")
        }
    }
}

fn eval_cond(env: &mut Env, vec: &Vec<E>) -> EvalResult {
    for expr in vec.into_iter().skip(1) {
        match *expr {
            List(ref list) => {
                if list.len() != 2 {
                    return Err("Invalid argument to `cond`");
                } else {
                    let val = try!(eval(env, list[0].clone()));
                    if val.eq(&Atom("t".to_string())) {
                        return eval(env, list[1].clone());
                    }
                }
            },
            _ => return Err("Invalid argument to `cond`")
        }
    }
    Ok(Expr::empty_list())
}

fn eval_defun(env: &mut Env, vec: &Vec<E>) -> EvalResult {

    if vec.len() != 4 {
        Err("`defun` expects exactly three arguments.")
    } else {

        if !vec[1].is_atom() {
            return Err("First argument to `defun` must be a symbol");
        }

        {
            let params = vec[2].get_ref_list();
            for p in params.iter() {
                if !p.is_atom() {
                    return Err("Second argument to `defun` must be a list of params");
                }
            }
        }

        let func_name = vec[1].clone();
        let params = vec[2].clone();
        let body = vec[3].clone();

        let label_expr = List(vec![
                Atom("label".to_string()),
                func_name,
                List(vec![Atom("lambda".to_string()), params, body])
                ]);
        env.bindings.insert(vec[1].clone().unwrap_atom(), label_expr);
        Ok(Expr::empty_list())
    }
}

fn eval_func_call(env: &mut Env, vec: Vec<E>) -> EvalResult {

    let num_args = vec.len() - 1;

    let mut vec_iter = vec.into_iter();
    let mut op_expr = vec_iter.next().unwrap();

    let func_lit = match parse_func_literal(&op_expr) {
        Some(f) => f,
        None =>  {
            op_expr = try!(eval(env, op_expr));
            match parse_func_literal(&op_expr) {
                Some(f) => f,
                None => return Err("Unrecognized expression."),
            }
        }
    };

    let Func {params, body, sym} = func_lit;
    let mut bindings = HashMap::<String, E>::new();

    match sym {
        Some(s) => {
            bindings.insert(s, op_expr.clone());
        }
        None => {},
    }

    if params.len() != num_args {
        return Err("mismatch between number of procedure args and number of args called with.");
    }

    let mut param_iter = params.into_iter();

    for arg in vec_iter {
        let next_param = param_iter.next().unwrap();
        bindings.insert(next_param, try!(eval(env, arg)));
    }

    let mut new_env = env.clone();

    for (k, v) in bindings.into_iter() {
        new_env.bindings.insert(k, v);
    }

    let val = try!(eval(&mut new_env, body));

    Ok(val)
}

#[derive(Debug)]
struct Func {
    params: Vec<String>,
    body: E,
    sym: Option<String>,
}

fn parse_label_literal(expr: &E) -> Option<Func> {
    if !expr.is_list() {
        return None;
    }
    let vec = expr.get_ref_list();

    if vec.len() != 3 || !vec[1].is_atom() || !is_symbol("label", &vec[0]) {
        return None;
    }

    let lit = parse_lambda_literal(&vec[2]);

    match lit {
        None => None,
        Some(mut func) => {
            func.sym = Some(vec[1].clone().unwrap_atom());
            Some(func)
        }
    }
}

fn parse_func_literal(expr: &E) -> Option<Func> {
    match parse_lambda_literal(expr) {
        None => parse_label_literal(expr),
        lambda@Some(_) => lambda
    }
}

fn parse_lambda_literal(expr: &E) -> Option<Func> {
    if !expr.is_list() {
        None
    } else {
        let vec = expr.get_ref_list();
        if vec.len() != 3 || !vec[1].is_list() || !is_symbol("lambda", &vec[0]) {
            None
        } else {
            let params = vec[1].get_ref_list();
            let mut plist = vec![];

            for p in params.iter() {
                if !p.is_atom() {
                    return None
                } else {
                    plist.push(p.clone().unwrap_atom());
                }
            }
            Some(Func{ params: plist, body: vec[2].clone(), sym: None })
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

    fn get_ref_list(&self) -> &Vec<Expr<T>> {
        match *self {
            List(ref v) => v,
            _ => panic!("called Expression::get_ref_list() on non-List")
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

    let lambda = Atom("lambda".to_string());
    // let params = List(vec![Atom("x".to_string())]);
    let params = List(vec![Atom("x".to_string()), Atom("y".to_string())]);
    // let body = List(vec![Atom("cons".to_string()), Atom("x".to_string()), List(vec![Atom("quote".to_string()), List(vec![Atom("b".to_string())])])]);
    let body = List(vec![Atom("cons".to_string()), Atom("x".to_string()), List(vec![Atom("cdr".to_string()), Atom("y".to_string())])]);

    // let label = Atom("label".to_string());
    // let func_name = Atom("foo".to_string());

    let func = List(vec![lambda, params, body]);

    let a2 = List(vec![Atom("quote".to_string()), List(vec![Atom("a".to_string()), Atom("b".to_string()), Atom("c".to_string())])]);
    let arg = List(vec![func, List(vec![Atom("quote".to_string()), Atom("z".to_string())]), a2]);

    println!("{:?}", arg);
    arg.p();

    let ret = eval(&mut env, arg);

    println!("{:?}", ret);

    ret.unwrap().p();

    println!("{:?}", env);

    println!("{:?}", env.find("f"));

    // env.find("f").unwrap().p();
}

#[test]
fn test_eval() {
    assert!(true);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Token {
    name: &'static str,
    value: String
}

/// Convert a string of characters into a list of tokens.
#[allow(dead_code)]
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

#[allow(dead_code)]
fn parse(program: &str) -> Vec<String> {
    // TODO
    println!("{}", program);
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
