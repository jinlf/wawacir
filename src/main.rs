mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    repl::start(&mut std::io::stdin(), &mut std::io::stdout())
}
