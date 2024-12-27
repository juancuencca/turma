use std::{env, process};
use turma::{Args, Machine};

fn main() {
    let args = Args::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing args: {e}");
        eprintln!("Usage: \n\tcargo run -- <input.card> <input.tape>");
        process::exit(1);
    });

    let mut machine = Machine::build(&args).unwrap_or_else(|e| {
        eprintln!("Problem building machine: {e}");
        process::exit(1);
    });

    machine.summary();
    while machine.next().is_some() {
        machine.summary();
    } 
}
