pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;
#[macro_use]
extern crate nom;

fn main() {
    let mut repl = repl::Repl::new();
    repl.run();
}
