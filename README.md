# Ground Telemetry Processing System

A Rust-based ground software portfolio project built as a hands-on way to learn Rust while exploring spacecraft telemetry, operator command handling, networking, persistence, and software reliability.

The project is being developed incrementally through a 14-day sprint. Each ticket introduces a new Rust or software-engineering concept while advancing the system toward a ground-station CLI communicating with a simulated spacecraft over TCP and persisting telemetry and command history in PostgreSQL.

## Current Status

Completed through **Day 07 — Refactor into Modules and Add Unit Tests**.

Current functionality includes:

- Spacecraft telemetry modeled as structured Rust data
- Typed spacecraft operating modes
- Interactive operator CLI
- Persistent command loop
- Spacecraft mode transitions
- Command parsing
- Graceful handling of blank and unsupported commands
- Modularized command and spacecraft logic
- Automated unit tests for deterministic behavior

Networking and PostgreSQL persistence will be introduced in later sprint tickets.

---

## Current Commands

| Command        | Description                              |
| -------------- | ---------------------------------------- |
| `status`       | Display the current spacecraft telemetry |
| `help`, `?`    | Display the command help menu            |
| `nominal`      | Change spacecraft mode to Nominal        |
| `safe`         | Change spacecraft mode to Safe           |
| `standby`      | Change spacecraft mode to Standby        |
| `exit`, `quit` | Exit the application                     |

---

## Current Project Structure

```text
src/
├── main.rs
├── command.rs
└── spacecraft.rs
```

### `main.rs`

Responsible for application orchestration and terminal I/O:

- Reads operator input
- Dispatches parsed commands
- Displays spacecraft status
- Displays help information
- Coordinates spacecraft mode transitions

### `command.rs`

Contains the operator command model and parser:

- `Command` enum
- `Command::parser`
- Command parser unit tests

### `spacecraft.rs`

Contains spacecraft domain state and behavior:

- `Spacecraft`
- `SpacecraftMode`
- `Spacecraft::set_mode`
- Spacecraft state unit tests

---

## Running the Project

```bash
cargo run
```

Example session:

```text
== GROUND TELEMETRY PROCESSING SYSTEM ==
> status
Spacecraft Identifier: SAT-001
Mode: Nominal
Battery Level: 28.5 V
Temperature: 68.0 F
Uptime: 2.0 hrs

> safe
== Previous Status ==
...
=== New Status ===
Mode: Safe
...

> exit
Exiting application...
```

---

## Verification

Format the project:

```bash
cargo fmt
```

Check compilation:

```bash
cargo check
```

Run automated tests:

```bash
cargo test
```

---

# Sprint Learning Log

## Day 01 — Bootstrap Project and Display Telemetry

**Jira:** `GTPS-2`

### What the ticket was about

Created the initial Cargo project and the first working spacecraft telemetry display.

The simulator started with fixed telemetry values for:

- Spacecraft identifier
- Operating mode
- Battery voltage
- Temperature

Telemetry values were stored as variables rather than embedded directly into output strings. Floating-point telemetry was formatted to one decimal place.

### Concepts learned

- Creating and running a Cargo project
- Variables and bindings
- Primitive data types
- Floating-point values
- `println!`
- Format specifiers such as `{:.1}`
- Basic functions and program structure
- Compiling with `cargo check`
- Running with `cargo run`

### Official Rust references

- [1.3 — Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [3.1 — Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [3.2 — Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [3.3 — Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

---

## Day 02 — Model Spacecraft State with a Struct

**Jira:** `GTPS-3`

### What the ticket was about

Replaced independent telemetry variables with a `Spacecraft` struct so related spacecraft state could be represented as one object.

A status function received a reference to the spacecraft rather than taking ownership of it.

As Chapter 5 concepts were introduced, spacecraft behavior was also explored through an `impl` block.

### Concepts learned

- Defining structs
- Instantiating structs
- Accessing struct fields
- Grouping related state
- Ownership
- Immutable references
- Borrowing with `&T`
- Passing structs to functions without consuming them
- `impl` blocks
- Methods and `&self`

### Official Rust references

- [4.2 — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [5.1 — Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [5.2 — An Example Program Using Structs](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
- [5.3 — Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

---

## Day 03 — Add Typed Spacecraft Operating Modes

**Jira:** `GTPS-4`

### What the ticket was about

Replaced the spacecraft mode `String` with a typed `SpacecraftMode` enum.

Supported operating modes:

- `Nominal`
- `Safe`
- `Standby`

The status display was also renamed to `print_status` to better describe its purpose.

### Concepts learned

- Defining enums
- Modeling a fixed set of valid states
- Replacing free-form strings with typed values
- Pattern matching
- Exhaustive `match`
- Enum variants
- `Debug` representation
- Improving naming through refactoring

### Official Rust references

- [6.1 — Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [6.2 — The `match` Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)

---

## Day 04 — Accept and Parse Operator Commands

**Jira:** `GTPS-5`

### What the ticket was about

Changed the simulator from static output into an interactive command-line application.

The program reads operator input from stdin and dispatches behavior based on the entered command.

Initial commands included:

- `status`
- `help`
- `?`

### Concepts learned

- Reading from standard input
- Mutable `String` buffers
- `read_line`
- Trimming newline characters and surrounding whitespace
- Standard output
- Explicit stdout flushing
- Matching string slices
- CLI command dispatch
- Handling unsupported input

### Official Rust references

- [Chapter 2 — Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- [3.5 — Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [6.2 — `match`](https://doc.rust-lang.org/book/ch06-02-match.html)
- [8.2 — Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [`std::io`](https://doc.rust-lang.org/std/io/)

---

## Day 05 — Implement Mode-Changing Commands

**Jira:** `GTPS-6`

### What the ticket was about

Added operator commands that modify the state of the existing spacecraft:

- `nominal`
- `safe`
- `standby`

Mode changes mutate the existing `Spacecraft` rather than constructing a new one. Other telemetry values remain unchanged during a mode transition.

### Concepts learned

- Mutable bindings
- Mutable references
- `&mut self`
- Mutating struct fields
- Methods that change state
- Comparing enum values
- `PartialEq`
- Separating read-only behavior from state-changing behavior
- Preserving unrelated state during mutations

### Official Rust references

- [3.1 — Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [4.2 — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [5.3 — Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

---

## Day 06 — Add Command Loop and Graceful Error Handling

**Jira:** `GTPS-7`

### What the ticket was about

Converted the CLI into a persistent operator session so multiple commands can be executed without restarting the application.

Added:

- Persistent command loop
- `exit` and `quit`
- Blank input handling
- Unsupported command handling
- Dedicated command parsing
- Separate `Empty` and `Invalid` command states

Spacecraft state remains alive outside the command loop so mode changes persist between commands.

### Concepts learned

- `loop`
- `break`
- Persistent mutable state
- Clearing and reusing a `String`
- Converting text into typed commands
- Command enums
- Associated functions
- Returning values from `match`
- Separating parsing from command execution
- Graceful error handling
- Modeling different failure/input states explicitly

### Official Rust references

- [3.5 — Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [6.2 — `match`](https://doc.rust-lang.org/book/ch06-02-match.html)
- [Chapter 9 — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [`String::clear`](https://doc.rust-lang.org/std/string/struct.String.html#method.clear)

---

## Day 07 — Refactor into Modules and Add Unit Tests

**Jira:** `GTPS-8`

### What the ticket was about

Refactored the working CLI before introducing networking.

The application was separated into command parsing, spacecraft domain logic, and application orchestration.

Current layout:

```text
src/
├── main.rs
├── command.rs
└── spacecraft.rs
```

Automated unit tests were added for command parsing, spacecraft mode mutation, and mode-change orchestration.

### Concepts learned

- Rust modules
- Separating code across files
- Module paths
- `use`
- Visibility with `pub`
- Separation of concerns
- Keeping `main` focused on orchestration
- Deterministic functions
- Unit tests
- `#[cfg(test)]`
- `#[test]`
- `assert_eq!`
- `assert!`
- `PartialEq`
- `Debug`
- Testing state changes without depending on stdin
- Designing code to be easier to test

### Official Rust references

- [Chapter 7 — Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [11.1 — How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [11.3 — Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
- [Chapter 12 — An I/O Project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

---

## Planned Sprint Direction

The remaining sprint builds toward two separate Rust processes:

```text
┌─────────────────────┐
│ Spacecraft Simulator│
│                     │
│ State + Telemetry   │
└──────────┬──────────┘
           │
           │ TCP
           │
┌──────────▼──────────┐
│   Ground Station    │
│                     │
│ CLI + Commands      │
│ Telemetry Receiver  │
│ PostgreSQL Storage  │
└─────────────────────┘
```

Upcoming work introduces:

- Separate spacecraft and ground-station processes
- TCP networking over localhost
- Telemetry serialization
- Command transmission
- Command acknowledgements
- PostgreSQL telemetry persistence
- Command history persistence
- Reliability and failure handling
- Final portfolio packaging

---

## Project Goals

This project is intentionally built incrementally rather than starting with the final architecture.

The goals are to:

1. Learn Rust fundamentals through practical implementation.
2. Understand each layer before introducing additional complexity.
3. Build software relevant to ground systems and spacecraft operations.
4. Maintain readable Git history and Jira traceability.
5. Develop automated testing habits.
6. Produce a portfolio project that can be explained technically rather than merely demonstrated.
