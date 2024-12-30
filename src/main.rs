use std::{fs, process, env};
use turma::{Machine, Instruction};

fn main() { 
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        eprintln!("Usage: cargo run -- <states_file_path> <tape_file_path>");
        process::exit(1);
    });

    let states = read_states(&config.states_file_path).unwrap_or_else(|e| {
        eprintln!("Could not read {}: {}", &config.states_file_path, e);
        process::exit(1);
    });

    let tape = read_tape(&config.tape_file_path).unwrap_or_else(|e| {
        eprintln!("Could not read {}: {}", &config.tape_file_path, e);
        process::exit(1);
    });

    let mut machine = Machine::default();
    let head = tape.len() / 2;
    machine.with_state_table(states);
    machine.with_tape(tape);
    machine.with_head(head);

    loop {
        machine.summary();

        if machine.next().is_none() {
            break;
        }
    }
}

struct Config {
    states_file_path: String,
    tape_file_path: String,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn std::error::Error>> {
        args.next();

        let states_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("requires a state table file".into()),
        }; 

        let tape_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("requires a tape file".into()),
        };  
    
        Ok(Config{ states_file_path, tape_file_path})
    }
}

fn read_tape(file_path: &str) -> Result<Vec<char>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    
    let result = contents
        .trim()
        .split_whitespace()
        .map(|x| x.chars().nth(0).unwrap())
        .collect::<Vec<_>>();

    Ok(result)
}

fn read_states(file_path: &str) -> Result<Vec<Instruction>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    
    let result = contents
        .lines()
        .filter(|line| line.len() > 0)
        .map(|s| Instruction::from_str(&s))
        .collect::<Result<Vec<_>, _>>();

    Ok(result?)
}


