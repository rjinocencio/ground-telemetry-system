# Ground Telemetry Processing System

A Rust-based ground software portfolio project built as a hands-on way to learn Rust while exploring spacecraft telemetry, operator command handling, networking, persistence, and software reliability.

The project is being developed incrementally through a 14-day sprint. Each ticket introduces a new Rust or software engineering concept while advancing the system toward a ground-station CLI communicating with a simulated spacecraft over TCP and persisting telemetry and command history in PostgreSQL.

## Current Status

Completed through **Day 10 — Establish Configurable TCP Connections Between Processes**.

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
- Unix-millisecond timestamps generated for telemetry snapshots
- Configurable TCP listener for the spacecraft simulator
- Runtime `connect [IP:PORT]` command in the ground station
- Multiple spacecraft connections retained by socket address
- Multiple ground-station connections accepted by the simulator
- Graceful TCP connection failure handling
- Localhost TCP integration tests

The spacecraft simulator and ground station can now establish real TCP connections.

Telemetry and operator commands are **not yet transmitted over those connections**. The current TCP layer establishes and retains the communication channels that later sprint tickets will use.

---

## Current Commands

The ground station currently recognizes the following commands.

Commands that require spacecraft communication are parsed correctly, but telemetry requests and mode-change commands are not yet transmitted over TCP.

| Command             | Description                                                         |
| ------------------- | ------------------------------------------------------------------- |
| `connect [IP:PORT]` | Connect to a spacecraft; uses the default address if omitted        |
| `connections`       | List currently retained spacecraft connections                      |
| `status`            | Request spacecraft status; network transmission not implemented yet |
| `help`, `?`         | Display the command help menu                                       |
| `nominal`           | Request Nominal mode; network transmission not implemented yet      |
| `safe`              | Request Safe mode; network transmission not implemented yet         |
| `standby`           | Request Standby mode; network transmission not implemented yet      |
| `exit`, `quit`      | Exit the ground-station application                                 |

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

Defines the shared library modules used by both binaries:

- `command`
- `spacecraft`
- `telemetry`

### `command.rs`

Contains the operator command model and parser:

- `Command` enum
- `Command::parser`
- Mode-change commands represented with typed spacecraft modes
- `Connect(Option<String>)` for optional connection addresses
- `Connections` command
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

Owns operator-facing CLI and spacecraft connection management:

- Reads operator input from stdin
- Calls `Command::parser`
- Displays help information
- Handles blank and unsupported input
- Handles clean application exit
- Accepts `connect [IP:PORT]`
- Uses a default spacecraft address when no address is supplied
- Establishes TCP connections using `TcpStream`
- Stores spacecraft connections in a `HashMap`
- Uses socket addresses as connection keys
- Prevents duplicate connections to the same address
- Lists retained spacecraft connections
- Handles failed connection attempts without panicking
- Does not directly own or mutate spacecraft state

### `bin/spacecraft-sim.rs`

Owns simulated spacecraft state and acts as a TCP server:

- Creates the `Spacecraft` instance
- Initializes spacecraft telemetry values
- Records simulator start time
- Accepts a configurable listening address
- Uses a default localhost address when none is supplied
- Creates a `TcpListener`
- Accepts incoming ground-station TCP connections
- Reports connected peer addresses
- Retains accepted `TcpStream` values
- Supports multiple connected ground stations
- Does not yet transmit telemetry over TCP

---

## Running the Project

### Start a Spacecraft Simulator

Use the default address:

```bash
cargo run --bin spacecraft-sim
```

Default:

```text
127.0.0.1:7878
```

Example output:

```text
== SPACECRAFT SIMULATOR ==
SAT-001 initialized in Nominal mode
Listening on 127.0.0.1:7878
```

The simulator then waits for incoming TCP connections.

A custom listening address can also be supplied:

```bash
cargo run --bin spacecraft-sim -- 127.0.0.1:9000
```

Example:

```text
== SPACECRAFT SIMULATOR ==
SAT-001 initialized in Nominal mode
Listening on 127.0.0.1:9000
```

---

### Start the Ground Station

In another terminal:

```bash
cargo run --bin ground-station
```

Example session:

```text
== GROUND STATION ==
> connect
Connecting to 127.0.0.1:7878...
Connected to spacecraft at 127.0.0.1:7878

> connections
== ACTIVE CONNECTIONS ==
127.0.0.1:7878

> status
Spacecraft communication not yet implemented!

> exit
Exiting ground station...
```

When the ground station connects, the spacecraft simulator reports the peer connection:

```text
Ground station connected from 127.0.0.1:54321
```

The peer port is typically an ephemeral port selected by the operating system for the TCP client.

---

## Connecting to Multiple Spacecraft

Multiple simulator processes can be started on different ports.

Terminal 1:

```bash
cargo run --bin spacecraft-sim -- 127.0.0.1:7878
```

Terminal 2:

```bash
cargo run --bin spacecraft-sim -- 127.0.0.1:7879
```

Terminal 3:

```bash
cargo run --bin ground-station
```

Then connect to both:

```text
> connect 127.0.0.1:7878
Connected to spacecraft at 127.0.0.1:7878

> connect 127.0.0.1:7879
Connected to spacecraft at 127.0.0.1:7879

> connections
== ACTIVE CONNECTIONS ==
127.0.0.1:7878
127.0.0.1:7879
```

At this stage, socket addresses identify the connections.

Later telemetry will provide spacecraft identifiers such as `SAT-001`, allowing connection management to evolve from raw network addresses toward spacecraft identities.

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

Spacecraft state remained alive outside the command loop so mode changes persisted between commands.

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

The two programs intentionally did not communicate yet. This ticket established the process boundary that later tickets could connect using serialized telemetry and TCP.

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

The spacecraft simulator continues to own the internal `Spacecraft` state. A telemetry snapshot is created from a borrowed `&Spacecraft`, allowing telemetry to be exported without consuming or transferring ownership of simulator state.

The snapshot includes:

- Spacecraft identifier
- Operating mode
- Battery voltage
- Temperature
- Real-time uptime
- Unix-millisecond timestamp

Serde is used to serialize `TelemetrySnapshot` into compact JSON and deserialize JSON back into the typed Rust representation.

Telemetry snapshots include real timing information:

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

During development, the spacecraft simulator also generated multiple local snapshots with a delay between samples so real-time uptime and timestamp behavior could be observed before networking was introduced.

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

## Day 10 — Establish Configurable TCP Connections Between Processes

**Jira:** `GTPS-11`

### What the ticket was about

Established real TCP connectivity between the independently running spacecraft simulator and ground station.

The spacecraft simulator now acts as a TCP server. It binds to a configurable socket address, listens for incoming ground-station connections, accepts them, and retains the resulting `TcpStream` values.

The ground station acts as a TCP client and can establish spacecraft connections at runtime using:

```text
connect [IP:PORT]
```

The address argument is optional. If no address is supplied, the ground station uses its default spacecraft address.

Multiple spacecraft connections can be retained by the ground station and accessed using their socket address.

The spacecraft simulator can also accept and retain multiple ground-station connections.

No telemetry or commands are transmitted through the TCP streams yet. This ticket establishes and verifies the transport layer that future tickets will use.

### Ground Station Connection Model

The ground station stores spacecraft connections in a:

```text
HashMap<String, TcpStream>
```

Conceptually:

```text
"127.0.0.1:7878" → TcpStream
"127.0.0.1:7879" → TcpStream
```

Using a `HashMap` allows a specific spacecraft connection to be retrieved later using its configured socket address.

Successful connections remain owned by the ground-station process so they are not dropped after the `connect` command completes.

### Spacecraft Simulator Connection Model

The spacecraft simulator owns a `TcpListener` and retains accepted ground-station streams in a:

```text
Vec<TcpStream>
```

At this stage, the simulator only needs to retain connected clients rather than retrieve a specific ground station by identity.

Conceptually:

```text
TcpListener
    │
    ├── TcpStream → Ground Station 1
    ├── TcpStream → Ground Station 2
    └── TcpStream → Ground Station 3
```

The simulator uses `TcpListener::accept` to accept one connection at a time.

The earlier `TcpListener::incoming` approach was also explored. `incoming()` provides an iterator that repeatedly performs the connection-accepting behavior, while `accept()` exposes one connection operation directly and is easier to isolate for testing.

### Error Handling

TCP operations return `Result` because connection failures are expected runtime conditions rather than necessarily programming errors.

Examples include:

- Invalid socket addresses
- Connection refused
- Port already in use
- Failed connection acceptance

Network helpers return errors to their caller instead of panicking.

The `?` operator is used where appropriate to propagate an `io::Error` to the caller.

For example:

```rust
let stream = TcpStream::connect(address)?;
```

is conceptually equivalent to:

```rust
let stream = match TcpStream::connect(address) {
    Ok(stream) => stream,
    Err(error) => return Err(error),
};
```

### Testing

TCP functionality is tested using real localhost sockets rather than mocked networking.

Tests bind to:

```text
127.0.0.1:0
```

Port `0` asks the operating system to select an available ephemeral port for the test, avoiding dependence on a hard-coded test port.

Networking tests cover:

- Successful TCP listener binding
- Invalid bind-address errors
- Successful spacecraft connection establishment
- Connection retention
- Duplicate ground-station connection prevention
- Invalid connection-address errors
- Accepting a ground-station connection
- Accepting multiple ground-station connections

Threads are used in connection tests because `TcpListener::accept()` blocks until a client connects.

### Concepts learned

- TCP client/server architecture
- `TcpListener`
- `TcpStream`
- Binding a TCP server to an address
- Accepting incoming TCP connections
- Connecting to a remote socket
- Configurable IP addresses and ports
- Reading command-line arguments with `std::env::args`
- Default configuration values
- Optional command arguments with `Option<String>`
- Enum variants that contain data
- Parsing CLI commands with `split_whitespace`
- Tuple pattern matching
- `HashMap<String, TcpStream>` for addressable spacecraft connections
- `Vec<TcpStream>` for retaining ground-station connections
- Keeping network connections alive through ownership
- Moving `TcpStream` values into collections
- Borrowing an address with `&str`
- Creating owned `String` keys only when needed
- `Result`
- `io::Error`
- Propagating errors with the `?` operator
- Manually handling `Result` with `match`
- Distinguishing command parsing errors from runtime network errors
- `TcpListener::accept`
- `TcpListener::incoming`
- Retrieving peer socket addresses
- Blocking network operations
- Ephemeral client ports
- Using port `0` for isolated network tests
- Localhost TCP integration testing
- Using threads to coordinate client/server tests

### Official Rust references

- [`TcpListener`](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
- [`TcpListener::bind`](https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.bind)
- [`TcpListener::accept`](https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.accept)
- [`TcpListener::incoming`](https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.incoming)
- [`TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
- [`TcpStream::connect`](https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect)
- [`TcpStream::peer_addr`](https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.peer_addr)
- [`SocketAddr`](https://doc.rust-lang.org/std/net/enum.SocketAddr.html)
- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html)
- [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
- [Rust Book — Recoverable Errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Rust Book — Patterns and Matching](https://doc.rust-lang.org/book/ch19-00-patterns.html)
- [`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html)
- [Rust Book — Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

## Planned Sprint Direction

The project now has a real TCP communication boundary between the spacecraft simulator and ground station:

```text
┌────────────────────────┐
│  Spacecraft Simulator  │
│                        │
│  Spacecraft State      │
│        ↓               │
│  TelemetrySnapshot     │
│        ↓               │
│       JSON             │
│                        │
│     TcpListener        │
└───────────┬────────────┘
            │
            │ TCP connection established
            │
┌───────────▼────────────┐
│     Ground Station     │
│                        │
│ CLI + Command Parser   │
│                        │
│ HashMap of TcpStreams  │
│                        │
│ Telemetry Receiver     │
│ PostgreSQL Storage     │
│     (upcoming)         │
└────────────────────────┘
```

The transport connection now exists.

The next step is to begin using the TCP streams to carry application data.

Upcoming work introduces:

- Continuous telemetry streaming over TCP
- Message framing for serialized telemetry
- Telemetry deserialization in the ground station
- Command transmission
- Command acknowledgements
- PostgreSQL telemetry persistence
- Command history persistence
- Reliability and failure handling
- Final portfolio packaging

Potential post-sprint extensions include:

- Async Rust
- Tokio
- WebSocket telemetry broadcasting
- Browser-based operator interface
- Multiple simulated spacecraft
- Spacecraft identity-based connection management
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
