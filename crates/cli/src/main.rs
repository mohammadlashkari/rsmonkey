mod repl;

use std::io::{stdin, stdout};

fn main() -> std::io::Result<()> {
    println!("rsmonkey programming language");
    repl::start(stdin().lock(), stdout())
}
