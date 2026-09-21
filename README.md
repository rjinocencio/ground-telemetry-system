# Ground Telemetry Processing System

A Rust-based ground software portfolio project built as a hands-on way to learn Rust while exploring spacecraft telemetry, operator command handling, networking, persistence, and software reliability.

The project is being developed incrementally through a 14-day sprint. Each ticket introduces a new Rust or software-engineering concept while advancing the system toward a ground-station CLI communicating with a simulated spacecraft over TCP and persisting telemetry and command history in PostgreSQL.

## Current Status

Completed through **Day 08 — Split Simulator and Ground Station into Separate Binaries**.

Current functionality includes:

- Spacecraft telemetry modeled as structured Rust data
- Typed spacecraft operating modes
- Interactive operator CLI
- Persistent command loop
- Typed command parsing
- Graceful handling of blank and unsupported commands
- Modularized command and spacecraft domain logic
- Automated unit tests for deterministic behavior
- Separate `spacecraft-sim` and `ground-station` binaries
- Shared library modules exposed through `lib.rs`
- Spacecraft state owned only by the simulator process

The two binaries intentionally do **not** communicate yet. Telemetry serialization, TCP communication, and PostgreSQL persistence are introduced in later sprint tickets.

---

## Current Commands

The ground station currently recognizes the following commands. Commands that require spacecraft communication are parsed correctly but report that remote communication is not implemented yet.

| Command        | Description                                                            |
| -------------- | ---------------------------------------------------------------------- |
| `status`       | Request spacecraft status; remote communication is not implemented yet |
| `help`, `?`    | Display the command help menu                                          |
| `nominal`      | Request Nominal mode; remote communication is not implemented yet      |
| `safe`         | Request Safe mode; remote communication is not implemented yet         |
| `standby`      | Request Standby mode; remote communication is not implemented yet      |
| `exit`, `quit` | Exit the ground-station application                                    |

---

## Current Project Structure

```text
src/
├── lib.rs
├── command.rs
├── spacecraft.rs
└── bin/
    ├── ground-station.rs
    └── spacecraft-sim.rs
```

### `lib.rs`

Defines the shared library modules used by both binaries:

- `command`
- `spacecraft`

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

### `bin/ground-station.rs`

Owns operator-facing CLI behavior:

- Reads operator input from stdin
- Calls `Command::parser`
- Displays help information
- Handles blank and unsupported input
- Handles clean application exit
- Recognizes spacecraft-related commands without directly owning or mutating spacecraft state

### `bin/spacecraft-sim.rs`

Owns the simulated spacecraft state:

- Creates the `Spacecraft` instance
- Initializes telemetry values
- Starts independently from the ground station
- Does not yet communicate over TCP

---

## Running the Project

Run the spacecraft simulator:

```bash
cargo run --bin spacecraft-sim
```

Current simulator output is similar to:

```text
== SPACECRAFT SIMULATOR ==
SAT-001 initialized in Nominal mode
```

Run the ground station in a separate terminal:

```bash
cargo run --bin ground-station
```

Example ground-station session:

```text
== GROUND STATION ==
> help
=== AVAILABLE COMMANDS ===
help, ?      Display this help menu
status       Check system status
nominal      Sets mode to nominal
safe         Sets mode to safe
standby      Sets mode to standby
exit, quit   Exit the application
==========================

> status
Spacecraft communication not yet implemented!

> safe
Spacecraft communication not yet implemented!

> exit
Exiting application...
```

At this stage, running both programs at the same time does not connect them. Establishing that communication boundary is part of the upcoming networking tickets.

---

## Verification

Format the project:

```bash
cargo fmt
```

Check all binary targets:

```bash
cargo check --bins
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

The Day 7 layout was:

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

## Day 08 — Split Simulator and Ground Station into Separate Binaries

**Jira:** `GTPS-9`

### What the ticket was about

Split the previous single-process application into two independently runnable Rust binaries:

- `spacecraft-sim`
- `ground-station`

The spacecraft simulator now owns the simulated `Spacecraft` state, while the ground station owns operator-facing CLI behavior and command parsing.

Shared types remain in library modules exposed through `lib.rs`.

The two programs intentionally do not communicate yet. This ticket establishes the process boundary that later tickets will connect using serialized telemetry and TCP.

### Concepts learned

- Multiple binary targets in one Cargo package
- Cargo's `src/bin/` project layout
- Using `src/lib.rs` as the root of shared library code
- Sharing modules between independent binary crates
- Importing shared library modules from binaries
- Separating domain ownership from operator-interface logic
- Process boundaries
- Avoiding duplicated shared code
- Preserving command parsing without direct spacecraft access
- Designing architecture before adding transport/networking

### Official Rust references

- [Cargo Guide — Project Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [Cargo Reference — Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)
- [Rust Book — Chapter 7: Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

## Planned Sprint Direction

The project now has two separate Rust processes with an intentional communication boundary:

```text
┌─────────────────────┐
│ Spacecraft Simulator│
│                     │
│ Owns Spacecraft     │
│ State + Telemetry   │
└──────────┬──────────┘
           │
           │ future TCP
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

- Shared telemetry transport types
- JSON serialization and deserialization with Serde
- TCP networking over localhost
- Telemetry streaming
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
