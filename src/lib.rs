pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
enum Step {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Instruction {
    current: String,
    read: char,
    write: char,
    step: Step,
    next: String,  
}

impl Instruction {
    pub fn from_str(s: &str) -> Result<Instruction> {
        let mut s_iter = s.split_whitespace();
        
        let current = match s_iter.next() {
            Some(val) => val.to_string(),
            None => return Err("did not provide current state".into()),
        };

        let read = match s_iter.next() {
            Some(val) => match val.chars().nth(0) {
                Some(ch) => ch,
                None => unreachable!(),
            },
            None => return Err("did not provide read char".into()),
        };

        let write = match s_iter.next() {
            Some(val) => match val.chars().nth(0) {
                Some(ch) => ch,
                None => unreachable!(),
            },
            None => return Err("did not provide write char".into()),
        };

        let step = match s_iter.next() {
            Some(val) => match val {
                "L" => Step::Left,
                "R" => Step::Right,
                _ => return Err("step is expected to be L or R".into()),
            },
            None => return Err("did not provide direction".into()),
        };

        let next = match s_iter.next() {
            Some(val) => val.to_string(),
            None => return Err("did not provide next state".into()),
        };

        Ok(Instruction {
            current,
            read,
            write,
            step,
            next,
        })
    }
}

#[derive(Debug)]
pub struct Machine {
    tape: Vec<char>,
    head: usize,
    state_table: Vec<Instruction>,
    state: String,
}

impl Default for Machine {
    fn default() -> Self {
        Machine {
            tape: vec!['0'; 20],
            head: 0,
            state_table: vec![],
            state: String::from("A"),
        }
    } 
}

impl Machine {
    pub fn with_tape(&mut self, tape: Vec<char>) {
        self.tape = tape;
    }

    pub fn with_head(&mut self, head: usize) {
        self.head = head;
    }

    pub fn with_state_table(&mut self, state_table: Vec<Instruction>) {
        self.state_table = state_table;
    }

    pub fn with_state(&mut self, state: String) {
        self.state = state;
    }
}

impl Machine {
    pub fn next(&mut self) -> Option<()> {
        if self.state == "HALT" {
            return None;
        }
        
        let instruction = get_instruction(&self.state_table, &self.state, self.tape[self.head])?;
        
        if instruction.step == Step::Left && self.head == 0 {
            self.tape.insert(0, '0');
            self.head += 1;
        }

        if instruction.step == Step::Right && self.head == self.tape.len() - 1 {
            self.tape.push('0');
        }
        
        self.tape[self.head] = instruction.write.clone();
        self.state = instruction.next.clone();
        self.head = match instruction.step {
            Step::Left => self.head - 1,
            Step::Right => self.head + 1,
        };
                
        Some(())
    }

    pub fn summary(&self) {
        for sym in &self.tape {
            print!("{}  ", sym);
        }
        println!();
        for _ in 0..self.head {
            print!("   ");
        }
        println!("^");
    }
}

fn get_instruction<'a>(state_table: &'a Vec<Instruction>, state: &str, read: char) -> Option<&'a Instruction> {
    for ins in state_table {
        if ins.current == state && ins.read == read {
            return Some(ins);
        }
    }

    None
}