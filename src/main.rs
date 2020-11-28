mod ast;
mod lexer;
mod parser;
mod repl;
mod token;
mod wasm;

fn main() {
    repl::start(&mut std::io::stdin(), &mut std::io::stdout())
}
