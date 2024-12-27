use std::{fs, error, path::Path};

type Error = Box<dyn error::Error>;

#[derive(Debug, Clone)]
enum Step {
    Left,
    Right,
}

type Symbol = String;
type State = String;

pub struct Args {
    cards_file_path: String,
    tape_file_path: String,
}

impl Args {
    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Args, Error> {
        args.next();

        let cards_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not provide cards path".into()),
        };

        let tape_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not provide tape path".into()),
        };

        Ok(Args {
            cards_file_path,
            tape_file_path,
        })
    }
}

#[derive(Debug)]
pub struct Machine {
    tape: Vec<Symbol>,
    head: usize,
    cards: Vec<Card>,
    state: State,
}

impl Machine {
    pub fn build(args: &Args) -> Result<Machine, Error> {
        let tape = read_tape(&Path::new(&args.tape_file_path))?;
        let head = tape.len() / 2;
        let cards = read_cards(&Path::new(&args.cards_file_path))?;
        let state = match cards.iter().nth(0) {
            Some(card) => card.current.clone(),
            None => return Err("Could not retrieve initial state".into()),
        };

        Ok(Machine {
            tape,
            head,
            cards,
            state,
        })
    }

    pub fn summary(&self) {
        for item in &self.tape {
            print!("{item}  ");
        }
        println!();
        for _ in 0..self.head {
            print!("   ");
        }
        println!("^");
    }

    fn get_card(&mut self) -> Option<Card> {
        for card in &self.cards {
            if card.current == self.state && card.read == self.tape[self.head] {
                return Some(card.clone());
            }
        }

        None
    }

    pub fn next(&mut self) -> Option<usize> {
        if self.state == "HALT" {
            return None;
        }
        
        if self.head == 0 {
            self.tape.insert(0, "0".to_string());
            self.head += 1;
        }

        if self.head == self.tape.len() - 1 {
            self.tape.push("0".to_string());
        }

        let card = match self.get_card() {
            Some(card) => card,
            None => return None,
        };
        
        self.tape[self.head] = card.write.clone();
        self.state = card.next.clone();
        self.head = match card.step {
            Step::Left => self.head - 1,
            Step::Right => self.head + 1,
        };
                
        Some(self.head)
    }
}

fn read_tape<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Error> {
    let contents = fs::read_to_string(path)?;
    
    let result = contents
        .trim()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    Ok(result)
}

fn read_cards<P: AsRef<Path>>(path: P) -> Result<Vec<Card>, Error> {
    let contents = fs::read_to_string(path)?;
    
    let result = contents
        .lines()
        .map(|line| line
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>())
        .filter(|item| item.len() > 0)
        .map(|item| Card::build(&item))
        .collect::<Result<Vec<_>, _>>();

    Ok(result?)
}

#[derive(Debug, Clone)]
struct Card {
    current: State,
    read: String,
    write: String,
    step: Step,
    next: State,  
}

impl Card {
    fn build(tokens: &[&str]) -> Result<Card, Error> {
        let mut tokens_iter = tokens.iter();

        let current = match tokens_iter.next() {
            Some(&token) => token,
            None => return Err("did not provide current state token".into()),
        };

        let read = match tokens_iter.next() {
            Some(&token) => token,
            None => return Err("did not provide read token".into()),
        };

        let write = match tokens_iter.next() {
            Some(&token) => token,
            None => return Err("did not provide write token".into()),
        };

        let step = match tokens_iter.next() {
            Some(&token) => token,
            None => return Err("did not provide step token".into()),
        };

        let next = match tokens_iter.next() {
            Some(&token) => token,
            None => return Err("did not provide next state token".into()),
        };

        let step = match step {
            "L" => Step::Left,
            "R" => Step::Right,
            _ => return Err("Step is expected to be L or R".into()),
        };

        Ok(Card {
            current: current.to_string(),
            read: read.to_string(),
            write: write.to_string(),
            step,
            next: next.to_string(),
        })
    }
}