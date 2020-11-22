use bfmaker::interpreter::*;
use bfmaker::parser::*;
use clap::Clap;
use serde_json;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::result::Result;

#[derive(Clap, Debug)]
#[clap(name = "bfmaker", version = "0.1.0", author = "guricerin")]
struct Opts {
    #[clap(name = "brainfuck-src-code-file")]
    code_path: PathBuf,
    #[clap(short, long, name = "brainfuck-grammar-file")]
    grammar_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let code = fs::read_to_string(opts.code_path)?;
    let mut parser = Parser::new(&code);
    if let Some(grammar_path) = opts.grammar_path {
        let data = fs::read_to_string(grammar_path)?;
        let grammar = serde_json::from_str(&data)?;
        parser.translate(&grammar);
    }
    let tokens = parser.parse();
    let mut interpreter = Interpreter::new(30000);
    interpreter.run(&tokens)?;

    Ok(())
}
