# Ground Telemetry Processing System — Sprint Learning Log

This document tracks the implementation and learning progress of the 14-day Ground Telemetry Processing System sprint.

Each ticket records:

- What the ticket implemented
- Rust and software-engineering concepts learned
- Relevant official Rust documentation

---

<a id="day-01"></a>

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

- Creating and running a Cargo
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

<a id="day-02"></a>

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

<a id="day-03"></a>

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

<a id="day-04"></a>

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

<a id="day-05"></a>

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

<a id="day-06"></a>

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
- Modeling different input states explicitly

### Official Rust references

- [3.5 — Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [6.2 — `match`](https://doc.rust-lang.org/book/ch06-02-match.html)
- [Chapter 9 — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [`String::clear`](https://doc.rust-lang.org/std/string/struct.String.html#method.clear)

---

<a id="day-07"></a>

## Day 07 — Refactor into Modules and Add Unit Tests

**Jira:** `GTPS-8`

### What the ticket was about

Refactored the working CLI before introducing networking.

The application was separated into command parsing, spacecraft domain logic, and application orchestration.

Automated unit tests were added for command parsing, spacecraft mode mutation, and deterministic behavior.

### Concepts learned

- Rust modules
- Separating code across files
- Module paths
- `use`
- Visibility with `pub`
- Separation of concerns
- Unit tests
- `#[cfg(test)]`
- `#[test]`
- `assert_eq!`
- `assert!`
- `PartialEq`
- `Debug`
- Designing code to be easier to test

### Official Rust references

- [Chapter 7 — Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [11.1 — How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [11.3 — Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

---

<a id="day-08"></a>

## Day 08 — Split Simulator and Ground Station into Separate Binaries

**Jira:** `GTPS-9`

### What the ticket was about

Restructured the package so the spacecraft simulator and ground station run as separate executable programs.

Shared application types were moved behind `lib.rs`, while process-specific orchestration moved into:

```text
src/bin/spacecraft-sim.rs
src/bin/ground-station.rs
```

This established the process boundary needed for later TCP communication.

### Concepts learned

- Cargo packages with multiple binaries
- `src/bin`
- Library crates and binary crates
- Sharing modules through `lib.rs`
- Process boundaries
- Separating domain logic from executable orchestration
- Reusing shared Rust types across binaries

### Official Rust references

- [Chapter 7 — Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)
- [Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)

---

<a id="day-09"></a>

## Day 09 — Define and Serialize Shared Telemetry as JSON

**Jira:** `GTPS-10`

### What the ticket was about

Introduced a shared `TelemetrySnapshot` type representing the telemetry transmitted from a spacecraft simulator to the ground station.

Added Serde serialization and deserialization so telemetry can be encoded as JSON and reconstructed as strongly typed Rust data.

Telemetry now includes:

- Spacecraft identifier
- Operating mode
- Battery voltage
- Temperature
- Uptime
- Unix timestamp
- Sequence number field added later during Day 11

### Concepts learned

- Serialization
- Deserialization
- Serde derives
- JSON
- Shared wire-format types
- Converting domain state into transport data
- Unix timestamps
- `SystemTime`
- `UNIX_EPOCH`
- `Duration`
- Round-trip serialization testing

### Official Rust references

- [`SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
- [`UNIX_EPOCH`](https://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html)
- [`Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html)
- [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)

### Serialization references

- [Serde](https://serde.rs/)
- [`serde_json`](https://docs.rs/serde_json/latest/serde_json/)

---

<a id="day-10"></a>

## Day 10 — Add Configurable TCP Connections

**Jira:** `GTPS-11`

### What the ticket was about

Introduced TCP connection establishment between the spacecraft simulator and ground station.

The spacecraft simulator became a TCP server using `TcpListener`.

The ground station gained:

- `connect [ip:port]`
- Default connection address
- `connections`
- A registry of active spacecraft streams

The simulator accepts ground-station connections while the ground station can retain multiple spacecraft TCP connections.

Localhost integration tests were added to verify real socket behavior without requiring external hardware.

### Concepts learned

- TCP client/server architecture
- `TcpListener`
- `TcpStream`
- Socket addresses
- Binding
- Connecting
- Accepting connections
- Localhost networking
- Ephemeral client ports
- Dynamic test ports with port `0`
- Returning `io::Result`
- Using `?` for I/O error propagation
- Managing multiple retained connections
- `HashMap`

### Official Rust references

- [`TcpListener`](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
- [`TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
- [`SocketAddr`](https://doc.rust-lang.org/std/net/enum.SocketAddr.html)
- [`std::io::Result`](https://doc.rust-lang.org/std/io/type.Result.html)
- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [The `?` Operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

---

<a id="day-11"></a>

## Day 11 — Stream Telemetry from Spacecraft Simulator to Ground Station over TCP

**Jira:** `GTPS-12`

### What the ticket was about

Completed the first end-to-end live telemetry path between separate spacecraft simulator and ground-station processes.

The spacecraft simulator now:

- Generates a fresh `TelemetrySnapshot` once per second
- Serializes snapshots to JSON
- Appends a newline delimiter to each telemetry message
- Sends telemetry over a connected `TcpStream`
- Assigns an independent sequence number to each telemetry connection
- Spawns a sender thread for each connected ground station
- Logs connection and disconnect events

The ground station now:

- Connects to one or more spacecraft simulators
- Clones each socket for a dedicated receiver
- Spawns one telemetry receiver thread per spacecraft connection
- Uses `BufReader::read_line` for newline framing
- Deserializes incoming JSON into `TelemetrySnapshot`
- Displays live telemetry
- Detects unexpected sequence numbers
- Shares its connection registry between threads using `Arc<Mutex<_>>`
- Removes dead connections after EOF or socket errors
- Allows a disconnected spacecraft to reconnect later

Manual verification was performed with two spacecraft simulators:

```text
SAT-001 -> 127.0.0.1:7878
SAT-002 -> localhost:7879
```

Both telemetry streams were received concurrently by the same ground-station process.

### Concepts learned

- TCP is a byte stream rather than a message protocol
- Application-level message framing
- Newline-delimited JSON
- `Write`
- `writeln!`
- `BufReader`
- `BufRead::read_line`
- EOF represented by `Ok(0)`
- `thread::spawn`
- `move` closures
- Ownership across threads
- `TcpStream::try_clone`
- `Arc<T>`
- `Mutex<T>`
- `Arc<Mutex<T>>`
- Sharing mutable state safely across threads
- `String` ownership versus borrowed `&str`
- Creating owned thread data with `to_string`
- Connection lifecycle management
- Per-connection sequence state
- `Option<u64>`
- Sequence-gap detection
- Unix millisecond timestamps stored as `u64`
- Checked integer conversion with `TryFrom`
- TCP integration testing
- Synchronizing tests instead of relying on thread timing
- Multiple independent spacecraft streams

### Official Rust references

- [`std::net`](https://doc.rust-lang.org/std/net/)
- [`TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
- [`TcpStream::try_clone`](https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.try_clone)
- [`BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html)
- [`BufRead::read_line`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line)
- [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html)
- [`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html)
- [Using `move` Closures with Threads](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
- [`str`](https://doc.rust-lang.org/std/primitive.str.html)
- [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
- [`SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
- [`Duration::as_millis`](https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_millis)

---
