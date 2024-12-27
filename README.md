# Turing Machine Interpreter

A Rust-based Turing Machine interpreter inspired by the [Wikipedia article on Turing Machines](https://en.wikipedia.org/wiki/Turing_machine).

## Quick Start

Ensure you have [Rust installed](https://www.rust-lang.org/tools/install), then execute the program with the following command:

```bash
$ cargo run -- ./cards/beaver.card ./tapes/input-01.tape
```

## Cards File Format

The instructions for the Turing Machine are stored in **cards files** (examples can be found in the [./cards/](./cards/) directory). These files define the machine's behavior as follows:

- Each instruction occupies a single line.
- The format of an instruction is:  
  `<current-state> <read-symbol> <write-symbol> <step> <next-state>`  
  where:
  - **`<current-state>`**: The active state of the Turing Machine for this instruction.
  - **`<read-symbol>`**: The symbol under the machine's head that triggers this instruction.
  - **`<write-symbol>`**: The symbol to write to the current tape cell.
  - **`<step>`**: Direction for the head's movement after execution (`L` for left, `R` for right).
  - **`<next-state>`**: The next state the machine transitions to after executing the instruction.

### Additional Notes:
- Leading and trailing whitespace is ignored.
- Empty lines are ignored.
- Lines starting with `#` are treated as comments and ignored.

## Tape File Format

The initial state of the Turing Machine's tape is specified in **tape files** (examples available in the [./tapes/](./tapes/) directory).

- A tape file consists of symbols separated by whitespace.
- Each symbol represents a cell's initial value on the tape.

## Execution Process

1. **Initialization**:
   - Load the specified tape file to initialize the tape.
   - Place the machine's head at the center of the tape.
   - Set the initial state to the first state in the provided cards file.

2. **Execution**:
   - On each step, the machine:
     - Matches its current state and the symbol under the head to an instruction in the cards file.
     - Executes the instruction by updating the tape, moving the head, and transitioning to the specified next state.
   - The machine halts if no matching instruction is found.

## Important Details

- The tape automatically expands as needed:
  - Inserts a `0` at the front if the head moves before the start of the tape.
  - Appends a `0` at the end if the head moves beyond the tape's current length.
- Infinite loops may occur if no halt state is defined or reachable.

This interpreter provides a robust platform to simulate Turing Machine behavior, making it a great tool for understanding computational theory.
