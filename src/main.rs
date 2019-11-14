use std::collections::VecDeque;

enum Ast {
    Add { lvar:Box<Ast>, rvar:Box<Ast> },
    Sub { lvar:Box<Ast>, rvar:Box<Ast> },
    Mul { lvar:Box<Ast>, rvar:Box<Ast> },
    Div { lvar:Box<Ast>, rvar:Box<Ast> },
    IntValue(i32),
}
fn main() {
    let input = String::from("+ 1 * + 1 2 5");
    let mut tokens = tokenize(&input);
    let ast = parse(& mut tokens);
    let ans = execute(ast);
    println!("{:?}", ans);
}

fn tokenize(input:&String) -> VecDeque<&str> {
    input.split(' ').collect()
}

fn parse(input:& mut VecDeque<&str>) -> Ast {
    match input.pop_front() {
        Some("+") => Ast::Add { lvar:Box::new(parse(input)), rvar:Box::new(parse(input)) },
        Some("-") => Ast::Sub { lvar:Box::new(parse(input)), rvar:Box::new(parse(input)) },
        Some("*") => Ast::Mul { lvar:Box::new(parse(input)), rvar:Box::new(parse(input)) },
        Some("/") => Ast::Div { lvar:Box::new(parse(input)), rvar:Box::new(parse(input)) },
        Some(x)   => Ast::IntValue(x.parse().unwrap()),
        None      => panic!("error")
    }
}

fn execute(input:Ast) -> i32 {
    match input {
        Ast::Add { lvar, rvar } => execute(*lvar) + execute(*rvar),
        Ast::Sub { lvar, rvar } => execute(*lvar) - execute(*rvar),
        Ast::Mul { lvar, rvar } => execute(*lvar) * execute(*rvar),
        Ast::Div { lvar, rvar } => execute(*lvar) / execute(*rvar),
        Ast::IntValue(value)    => value
    }
}