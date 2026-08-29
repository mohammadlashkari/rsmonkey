use std::io::{stdin, stdout};

fn main() -> std::io::Result<()> {
    println!("rsmonkey programming language");
    rsmonkey::repl::start(stdin().lock(), stdout())
}
