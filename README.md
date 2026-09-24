# Ground Telemetry Processing System

A Rust-based ground software portfolio project built as a hands-on way to learn Rust while exploring spacecraft telemetry, operator command handling, networking, persistence, and software reliability.

The project is being developed incrementally through a 14-day sprint. Each ticket introduces a new Rust or software-engineering concept while advancing the system toward a ground-station CLI communicating with simulated spacecraft over TCP and persisting telemetry and command history in PostgreSQL.

## Project Goals

This project is intentionally built incrementally rather than starting with the final architecture.

The goals are to:

1. Learn Rust fundamentals through practical implementation.
2. Understand each system layer before introducing additional complexity.
3. Build software relevant to spacecraft ground systems and mission operations.
4. Practice networking, concurrency, persistence, testing, and failure handling.
5. Produce a portfolio project that demonstrates both Rust learning and production software-engineering experience.

## Current Status

Completed through **Day 11 — Stream Telemetry from Spacecraft Simulator to Ground Station over TCP**.

The project now consists of separate spacecraft simulator and ground-station binaries communicating over TCP.

Current functionality includes:

- Structured spacecraft state
- Typed spacecraft operating modes
- Separate spacecraft simulator and ground-station processes
- Shared telemetry data model
- JSON telemetry serialization with Serde
- Configurable spacecraft TCP addresses
- Configurable spacecraft identifiers
- Ground-station connection management
- Multiple simultaneous spacecraft connections
- Concurrent telemetry receiver threads
- Continuous newline-delimited JSON telemetry streaming
- Per-connection telemetry sequence numbers
- Sequence-gap detection
- Spacecraft uptime tracking
- Unix millisecond telemetry timestamps
- Connection and disconnect logging
- Automatic removal of disconnected spacecraft from the connection registry
- Automated unit and localhost TCP integration tests

PostgreSQL persistence and command transmission will be introduced in the remaining sprint tickets.

---

## Architecture

```text
┌──────────────────────────┐
│   Spacecraft Simulator   │
│                          │
│ Spacecraft State         │
│ TelemetrySnapshot        │
│ Sequence Number          │
└────────────┬─────────────┘
             │
             │ serde_json::to_string
             │ JSON + '\n'
             │
             │ TCP
             ▼
┌──────────────────────────┐
│      Ground Station      │
│                          │
│ TcpStream                │
│ BufReader::read_line     │
│ serde_json::from_str     │
│ Telemetry Display        │
│ Connection Registry      │
└──────────────────────────┘
```

The simulator and ground station run as separate Rust processes.

Each spacecraft simulator listens on its own TCP address and continuously sends newline-delimited JSON telemetry to connected ground stations.

The ground station can maintain multiple spacecraft connections simultaneously:

```text
                     ┌─────────────────────┐
                     │ SAT-001             │
                     │ 127.0.0.1:7878      │
                     └──────────┬──────────┘
                                │
                                │ TCP telemetry
                                │
┌─────────────────────┐         │
│                     │◄────────┘
│   Ground Station    │
│                     │
│ Connection Registry │◄────────┐
│ Receiver Threads    │         │
└─────────────────────┘         │
                                │ TCP telemetry
                                │
                     ┌──────────┴──────────┐
                     │ SAT-002             │
                     │ localhost:7879      │
                     └─────────────────────┘
```

---

## Current Commands

| Command             | Description                                    |
| ------------------- | ---------------------------------------------- |
| `connect [ip:port]` | Connect to a spacecraft simulator              |
| `connections`       | List currently retained spacecraft connections |
| `help`, `?`         | Display the command help menu                  |
| `status`            | Reserved for spacecraft communication          |
| `nominal`           | Reserved for spacecraft mode commands          |
| `safe`              | Reserved for spacecraft mode commands          |
| `standby`           | Reserved for spacecraft mode commands          |
| `exit`, `quit`      | Exit the ground station                        |

If no address is supplied to `connect`, the ground station uses:

```text
127.0.0.1:7878
```

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

Exposes shared project modules:

- `command`
- `spacecraft`
- `telemetry`

### `command.rs`

Contains the ground-station operator command model and parser:

- `Command` enum
- `Command::parser`
- Optional connection address parsing
- Invalid argument handling
- Command parser unit tests

### `spacecraft.rs`

Contains spacecraft domain state and behavior:

- `Spacecraft`
- `SpacecraftMode`
- `Spacecraft::set_mode`
- Spacecraft uptime calculation
- Spacecraft state unit tests

### `telemetry.rs`

Defines the shared network telemetry representation:

- `TelemetrySnapshot`
- Spacecraft identifier
- Operating mode
- Battery voltage
- Temperature
- Uptime
- Unix timestamp in milliseconds
- Sequence number
- Conversion from `Spacecraft` state
- Serde serialization and deserialization
- JSON round-trip test

### `bin/spacecraft-sim.rs`

Runs the simulated spacecraft TCP server.

Responsibilities include:

- Creating spacecraft state
- Binding a configurable TCP listener
- Accepting ground-station connections
- Spawning one telemetry sender thread per connection
- Generating telemetry snapshots
- Assigning per-connection sequence numbers
- Serializing telemetry as JSON
- Framing messages with newline delimiters
- Streaming telemetry once per second
- Logging ground-station connection and disconnect events

### `bin/ground-station.rs`

Runs the operator-facing ground software.

Responsibilities include:

- Reading operator commands
- Connecting to spacecraft simulators
- Maintaining a shared connection registry
- Preventing duplicate retained connections
- Spawning telemetry receiver threads
- Reading newline-delimited TCP telemetry
- Deserializing JSON into `TelemetrySnapshot`
- Displaying telemetry
- Checking telemetry sequence continuity
- Removing disconnected spacecraft from the connection registry

---

## Running the Project

### Run the first spacecraft simulator

```bash
cargo run --bin spacecraft-sim -- 127.0.0.1:7878 SAT-001
```

Example output:

```text
== SPACECRAFT SIMULATOR ==
SAT-001 initialized in Nominal mode
Listening on 127.0.0.1:7878
```

### Run a second spacecraft simulator

```bash
cargo run --bin spacecraft-sim -- 127.0.0.1:7879 SAT-002
```

This allows the ground station to receive telemetry from multiple simulated spacecraft concurrently.

### Run the ground station

```bash
cargo run --bin ground-station
```

Connect to the default spacecraft:

```text
== GROUND STATION ==
> connect
Connecting to 127.0.0.1:7878...
Connected to spacecraft at 127.0.0.1:7878!
```

Connect to another spacecraft:

```text
> connect localhost:7879
Connecting to localhost:7879...
Connected to spacecraft at localhost:7879!
```

Example telemetry:

```text
[SAT-002] seq=0 | Nominal | 28.5 V | 68.0 F | 8.1s | 1790209243919
[SAT-002] seq=1 | Nominal | 28.5 V | 68.0 F | 9.1s | 1790209244919
[SAT-001] seq=0 | Nominal | 28.5 V | 68.0 F | 19.2s | 1790209246980
[SAT-002] seq=2 | Nominal | 28.5 V | 68.0 F | 10.1s | 1790209245919
[SAT-001] seq=1 | Nominal | 28.5 V | 68.0 F | 20.2s | 1790209247981
```

List active connections:

```text
> connections
== ACTIVE CONNECTIONS ==
127.0.0.1:7878
localhost:7879
```

---

## Telemetry Format

Telemetry is represented by a shared Rust type:

```rust
pub struct TelemetrySnapshot {
    pub spacecraft_id: String,
    pub mode: SpacecraftMode,
    pub battery_voltage: f64,
    pub temperature: f64,
    pub uptime_seconds: f64,
    pub timestamp_ms: u64,
    pub sequence_number: u64,
}
```

The simulator serializes each snapshot using Serde JSON.

Example:

```json
{
  "spacecraft_id": "SAT-001",
  "mode": "Nominal",
  "battery_voltage": 28.5,
  "temperature": 68.0,
  "uptime_seconds": 19.2,
  "timestamp_ms": 1790209246980,
  "sequence_number": 0
}
```

Each serialized snapshot is followed by a newline:

```text
JSON\n
```

This creates an application-level message boundary on top of TCP.

TCP itself provides an ordered byte stream but does not preserve individual application messages. The ground station therefore uses `BufReader::read_line` to receive one newline-delimited telemetry message at a time.

---

## Connection Model

The ground station stores active connections in:

```rust
Arc<Mutex<HashMap<String, TcpStream>>>
```

Each part serves a different role:

- `HashMap` associates a TCP address with its retained stream.
- `Mutex` provides synchronized mutable access to the registry.
- `Arc` allows the registry to be shared between the operator thread and telemetry receiver threads.

When a spacecraft connection is created:

```text
TcpStream
    │
    ├── original handle
    │      ↓
    │   connection registry
    │
    └── try_clone()
           ↓
       receiver thread
```

`TcpStream::try_clone` creates another handle to the same underlying socket connection rather than creating a second TCP connection.

When the receiver detects EOF or a socket error, it removes the spacecraft address from the shared registry so the operator can reconnect later.

---

## Sequence Numbers

Each simulator telemetry thread maintains an independent sequence number:

```text
0
1
2
3
...
```

The sequence counter belongs to the telemetry connection rather than being global across all spacecraft.

The ground station remembers the previously received sequence number for each receiver thread.

If it receives:

```text
10
11
12
```

there is no warning.

If it receives:

```text
10
12
```

the ground station can report:

```text
Telemetry sequence warning: expected 11, received 12
```

TCP already provides reliable and ordered delivery. The sequence number is therefore primarily useful for detecting application-level telemetry loss, parsing failures, future transport changes, or software defects.

---

## Known CLI Limitation

Telemetry receiver threads print asynchronously while the main thread reads operator input.

As a result, telemetry output can visually interrupt text currently being typed:

```text
> connect[SAT-002] seq=3 | Nominal | ...
```

This is currently treated as a terminal presentation limitation rather than a networking defect.

A future terminal UI could coordinate input and asynchronous output more cleanly.

---

## Verification

Format the project:

```bash
cargo fmt
```

Check all binaries:

```bash
cargo check --bins
```

Run automated tests:

```bash
cargo test
```

The test suite currently covers areas including:

- Command parsing
- Spacecraft mode mutation
- Telemetry JSON serialization/deserialization
- TCP listener binding
- TCP connection acceptance
- Newline-delimited telemetry transport
- Telemetry sequence preservation
- Connection registry behavior
- Removal of disconnected spacecraft
- Sequence continuity detection

Manual Day 11 verification also includes running two spacecraft simulators simultaneously and confirming that both telemetry streams continue independently in one ground-station process.

## Remaining Sprint Direction

The remaining sprint builds on the working TCP telemetry pipeline:

```text
Spacecraft Simulator
        │
        │ TCP
        ▼
Ground Station
        │
        ├── Telemetry display
        │
        ├── PostgreSQL telemetry storage
        │
        └── Operator commands
                 │
                 ▼
           Command / ACK flow
```

Upcoming work introduces:

- PostgreSQL telemetry persistence
- Command transmission
- Command acknowledgements
- Command history persistence
- Additional reliability handling
- Final project packaging

---

## Development Progress

This project is being built through a 14-day implementation and learning sprint.

Detailed ticket-by-ticket notes, concepts learned, and official Rust documentation references are maintained in:

**[Sprint Learning Log](docs/SPRINT_LOG.md)**

| Day | Jira                                 | Milestone                                                 |
| --- | ------------------------------------ | --------------------------------------------------------- |
| 01  | [GTPS-2](docs/SPRINT_LOG.md#day-01)  | Bootstrap project and display telemetry                   |
| 02  | [GTPS-3](docs/SPRINT_LOG.md#day-02)  | Model spacecraft state with a struct                      |
| 03  | [GTPS-4](docs/SPRINT_LOG.md#day-03)  | Add typed spacecraft operating modes                      |
| 04  | [GTPS-5](docs/SPRINT_LOG.md#day-04)  | Accept and parse operator commands                        |
| 05  | [GTPS-6](docs/SPRINT_LOG.md#day-05)  | Implement mode-changing commands                          |
| 06  | [GTPS-7](docs/SPRINT_LOG.md#day-06)  | Add persistent command loop and error handling            |
| 07  | [GTPS-8](docs/SPRINT_LOG.md#day-07)  | Refactor into modules and add unit tests                  |
| 08  | [GTPS-9](docs/SPRINT_LOG.md#day-08)  | Split simulator and ground station into separate binaries |
| 09  | [GTPS-10](docs/SPRINT_LOG.md#day-09) | Define and serialize shared telemetry as JSON             |
| 10  | [GTPS-11](docs/SPRINT_LOG.md#day-10) | Add configurable TCP connections                          |
| 11  | [GTPS-12](docs/SPRINT_LOG.md#day-11) | Stream telemetry over TCP                                 |
| 12  | [GTPS-13](docs/SPRINT_LOG.md#day-12) | PostgreSQL telemetry persistence                          |
| 13  | [GTPS-14](docs/SPRINT_LOG.md#day-13) | Command and acknowledgement persistence                   |
| 14  | [GTPS-15](docs/SPRINT_LOG.md#day-14) | Reliability improvements and final packaging              |
