# Real-Time Sensor–Actuator Control System (Rust)

## Overview

This project implements a **distributed soft real-time control system** in Rust, composed of two independent programs:

- **RTS_A** – Real-time sensor producer (server)
- **RTS_B** – Real-time actuator/controller (client)

The system simulates a 2ms periodic sensor loop, performs signal filtering and anomaly detection, streams data over TCP using Tokio, and applies a PID control loop on the consumer side.

The simulation runs for 10 seconds and logs latency and jitter metrics for performance evaluation.

---

# Architecture

## RTS_A – Sensor Producer

- Generates pseudo-random sensor values (30.0–100.0)
- Applies 5-sample moving average filter
- Flags anomalies (threshold > 80.0)
- Measures latency and jitter
- Logs output to CSV
- Streams JSON over TCP (`127.0.0.1:8888`)

↓

## RTS_B – Actuator / PID Controller

- Connects to RTS_A TCP stream
- Parses incoming JSON sensor data
- Executes PID control loop
- Logs control output
- Measures local processing latency
---

# RTS_A – Sensor Module

### Timing Configuration
- Sensor interval: **2 ms**
- Simulated processing deadline: **2 ms**
- Simulation duration: **10 seconds**

### Signal Processing
- Moving average window: **5 samples**
- Anomaly threshold: **80.0**

### Performance Metrics
- **Latency (ms)** – per-cycle execution time
- **Jitter (ms)** – cycle-to-cycle latency variation

### Output
- `sensor_log.csv`
- Streams JSON-serialized data via TCP

---

# RTS_B – Actuator Module

### Control System
Implements a PID controller targeting a fixed setpoint:

- **Setpoint:** 50.0  
- **Kp:** 1.0  
- **Ki:** 0.01  
- **Kd:** 0.1  

### Processing
For each received sensor packet:
- Deserialize JSON
- Compute error = setpoint − filtered_value
- Apply PID formula
- Log control output and processing latency

### Output
- `RTS_StudentB_Log_<timestamp>.txt`

---

# Technologies Used

- **Rust**
- Tokio (asynchronous runtime)
- TCP networking
- Serde (JSON serialization/deserialization)
- Chrono (timestamping)
- CSV logging
- Mutex + Arc for shared resource management

---

# How to Run

Open two terminals.

### Terminal 1 – Start Sensor (RTS_A)

```bash
cd RTS_A
cargo run
```
Wait for:
[Student A] Waiting for Student B to connect...

### Terminal 2 – Start Actuator (RTS_B)
```bash
cd RTS_B
cargo run
```
The system will run for 10 seconds and terminate automatically.

# Key Real-Time Characteristics

- 2ms periodic loop scheduling
- Soft real-time execution (non-deterministic OS scheduling)
- Jitter measurement
- Cycle latency logging
- Closed-loop control simulation
- Distributed architecture over TCP

# What This Demonstrates

- Periodic task execution
- Soft real-time system modeling
- Asynchronous networking in Rust
- Inter-process communication via TCP
- PID control implementation
- Runtime performance monitoring
- Structured logging for benchmarking

# Notes

This is a soft real-time simulation, not a hard real-time system.
Latency in RTS_A represents full cycle execution time.
Latency in RTS_B represents local processing time only.
RTS_B must be started after RTS_A.
