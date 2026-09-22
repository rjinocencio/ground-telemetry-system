# Ground Telemetry Processing System

A Rust-based ground software portfolio project built as a hands-on way to learn Rust while exploring spacecraft telemetry, operator command handling, networking, persistence, and software reliability.

The project is being developed incrementally through a 14-day sprint. Each ticket introduces a new Rust or software-engineering concept while advancing the system toward a ground-station CLI communicating with a simulated spacecraft over TCP and persisting telemetry and command history in PostgreSQL.

## Current Status

Completed through **Day 09 — Define and Serialize Shared Telemetry as JSON**.

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
- Shared `TelemetrySnapshot` transport model
- JSON serialization and deserialization with Serde
- Telemetry snapshots created from borrowed spacecraft state
- Round-trip serialization test coverage
- Real-time spacecraft uptime measured with `Instant`
- Unix-millisecond timestamps generated for each telemetry snapshot
- Repeated local telemetry snapshots for observing time progression

The two binaries intentionally do **not** communicate yet. TCP communication, telemetry streaming between processes, command transmission, and PostgreSQL persistence are introduced in later sprint tickets.

---

## Current Commands

The ground station currently recognizes the following commands.

Commands that require spacecraft communication are parsed correctly but report that remote communication is not implemented yet.

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
├── telemetry.rs
└── bin/
    ├── ground-station.rs
    └── spacecraft-sim.rs
```

### `lib.rs`

Defines the shared library modules used by the binaries:

- `command`
- `spacecraft`
- `telemetry`

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
- `Spacecraft::uptime`
- Simulator start time stored with `Instant`
- Spacecraft state unit tests

### `telemetry.rs`

Contains the transport representation for spacecraft telemetry:

- `TelemetrySnapshot`
- Conversion from borrowed `&Spacecraft`
- Serde serialization and deserialization support
- Real-time uptime measurement
- Unix-millisecond telemetry timestamps
- Round-trip JSON serialization tests

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
- Initializes spacecraft telemetry values
- Records simulator start time
- Creates `TelemetrySnapshot` values from current spacecraft state
- Serializes telemetry to JSON
- Generates repeated telemetry snapshots with a delay between samples
- Prints serialized telemetry locally
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

{"spacecraft_id":"SAT-001","mode":"Nominal","battery_voltage":28.5,"temperature":68.0,"uptime_seconds":0.0001,"timestamp_ms":1780000000123}

{"spacecraft_id":"SAT-001","mode":"Nominal","battery_voltage":28.5,"temperature":68.0,"uptime_seconds":1.0003,"timestamp_ms":1780000001124}

{"spacecraft_id":"SAT-001","mode":"Nominal","battery_voltage":28.5,"temperature":68.0,"uptime_seconds":2.0005,"timestamp_ms":1780000002125}
```

The simulator currently generates a limited number of local telemetry snapshots separated by a one-second delay so real-time uptime and timestamp behavior can be observed before networking is introduced.

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

At this stage, running both programs at the same time does not connect them.

The spacecraft simulator can generate serialized telemetry messages, but transmitting those messages to the ground station is part of the upcoming networking tickets.

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

Run the spacecraft simulator:

```bash
cargo run --bin spacecraft-sim
```

Run the ground station:

```bash
cargo run --bin ground-station
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

## Day 09 — Define and Serialize Shared Telemetry as JSON

**Jira:** `GTPS-10`

### What the ticket was about

Introduced a dedicated `TelemetrySnapshot` type to represent spacecraft telemetry intended to cross the process boundary.

The spacecraft simulator continues to own the internal `Spacecraft` state. A telemetry snapshot is created from a borrowed `&Spacecraft`, allowing telemetry to be exported without consuming or transferring ownership of the simulator state.

The snapshot includes:

- Spacecraft identifier
- Operating mode
- Battery voltage
- Temperature
- Real-time uptime
- Unix-millisecond timestamp

Serde is used to serialize `TelemetrySnapshot` into compact JSON and deserialize JSON back into the typed Rust representation.

Telemetry snapshots also include real timing information:

- `uptime_seconds` is calculated from a monotonic `Instant` recorded when the spacecraft simulator starts.
- `timestamp_ms` records the wall-clock time when each telemetry snapshot is created as Unix milliseconds.

This keeps simulator runtime measurement separate from the timestamp used to identify when telemetry was generated.

A round-trip unit test verifies that telemetry can be:

```text
TelemetrySnapshot
        ↓
      JSON
        ↓
TelemetrySnapshot
```

without changing the telemetry values.

The spacecraft simulator also generates multiple snapshots with a short delay between each one so changes in uptime and timestamp can be observed locally before TCP transport is introduced.

### Concepts learned

- Separating internal application state from transport data
- Designing a transport/data-transfer type
- Serialization and deserialization
- Serde `Serialize` and `Deserialize`
- JSON serialization with `serde_json`
- Creating telemetry from a borrowed `&Spacecraft`
- Preserving ownership while exporting data
- Cloning owned `String` data when necessary
- `Clone` versus `Copy`
- Making enum values serializable
- Round-trip serialization testing
- Using typed data instead of formatted display strings for transport
- Measuring elapsed runtime with `Instant`
- Representing wall-clock time with `SystemTime`
- Unix epoch timestamps
- Unix timestamps in milliseconds
- Difference between monotonic time and wall-clock time
- Keeping non-serializable runtime state separate from serializable telemetry
- Introducing delays with `thread::sleep`
- Representing delays with `Duration`
- Creating fresh telemetry snapshots over time

### Official references

- [Serde — Using derive](https://serde.rs/derive.html)
- [Serde — Attributes](https://serde.rs/attributes.html)
- [`serde_json` documentation](https://docs.rs/serde_json/)
- [Rust Book — 4.1 What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust Book — 4.2 References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust Book — Chapter 5: Using Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust Book — Chapter 11: Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html)
- [`SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
- [`UNIX_EPOCH`](https://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html)
- [`Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html)
- [`thread::sleep`](https://doc.rust-lang.org/std/thread/fn.sleep.html)

---

## Planned Sprint Direction

The project now has two separate Rust processes with a shared telemetry representation ready to cross the process boundary:

```text
┌─────────────────────┐
│ Spacecraft Simulator│
│                     │
│ Spacecraft State    │
│        ↓            │
│ TelemetrySnapshot   │
│        ↓            │
│       JSON          │
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

- TCP networking over localhost
- Continuous telemetry streaming between processes
- Command transmission
- Command acknowledgements
- PostgreSQL telemetry persistence
- Command history persistence
- Reliability and failure handling
- Final portfolio packaging

Potential post-sprint extensions include:

- Async Rust
- WebSocket telemetry broadcasting
- Browser-based operator interface
- Multiple simulated spacecraft
- Multiple ground-station clients
- Fault injection and simulated spacecraft anomalies
- Remote deployment for interactive portfolio demonstrations

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
