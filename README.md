# Synchronized Chaos 🦋

A cross-platform chaotic system simulator with real-time network synchronization, built in Rust.

## Overview

This project simulates the **Aizawa Attractor**, a chaotic dynamical system where tiny differences in initial conditions lead to vastly different trajectories. The twist? Multiple machines must stay perfectly synchronized—any desync becomes immediately visible as diverging particle trails.

## Features

- **Chaotic Simulation**: Aizawa attractor with thousands of particles
- **Parallel Processing**: Custom Job System with work-stealing for multi-core utilization
- **Network Synchronization**: Real-time state sync between multiple machines
- **Listen Server Model**: No dedicated server required—first instance becomes host
- **Host Migration**: Seamless handoff when the current host disconnects
- **Cross-Platform**: Runs on macOS and Windows

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      Application                        │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐  │
│  │ Simulation  │  │  Job System │  │    Network      │  │
│  │   Engine    │◄─┤  (Parallel) │  │   (Host/Client) │  │
│  └──────┬──────┘  └─────────────┘  └────────┬────────┘  │
│         │                                    │          │
│         ▼                                    ▼          │
│  ┌─────────────┐                   ┌─────────────────┐  │
│  │  Renderer   │                   │  State Sync     │  │
│  │   (egui)    │                   │  Serialization  │  │
│  └─────────────┘                   └─────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### Network Model

```
Discovery Phase:
  1. Broadcast "HOST_QUERY" via UDP
  2. If no response → Become Host
  3. If response → Connect as Client

Runtime:
  ┌──────────────┐         ┌──────────────┐
  │    Host      │◄───────►│   Client     │
  │ (Authority)  │   TCP   │ (Prediction) │
  └──────────────┘         └──────────────┘
  
  - Host runs authoritative simulation
  - Clients run local prediction
  - Periodic state snapshots for correction

Host Migration:
  1. Detect host disconnect (heartbeat timeout)
  2. Elect new host (lowest peer ID)
  3. Transfer simulation state
  4. Resume synchronization
```

## Tech Stack

| Component | Library |
|-----------|---------|
| Async Runtime | `tokio` |
| Parallelism | `crossbeam` / custom work-stealing |
| Networking | `tokio::net` (TCP/UDP) |
| Serialization | `serde` + `bincode` |
| Rendering | `egui` + `eframe` |
| Math | `glam` or `nalgebra` |

## Building & Running

```bash
# Build
cargo build --release

# Run (first instance becomes host)
cargo run --release

# Run on second machine (auto-discovers host)
cargo run --release
```

## Project Structure

```
synchronized-chaos/
├── src/
│   ├── main.rs
│   ├── simulation/
│   │   ├── mod.rs
│   │   ├── attractor.rs      # Aizawa equations
│   │   └── particle.rs
│   ├── job_system/
│   │   ├── mod.rs
│   │   ├── scheduler.rs
│   │   ├── worker.rs
│   │   └── work_stealing.rs
│   ├── network/
│   │   ├── mod.rs
│   │   ├── discovery.rs      # UDP broadcast
│   │   ├── host.rs
│   │   ├── client.rs
│   │   └── migration.rs
│   ├── sync/
│   │   ├── mod.rs
│   │   ├── snapshot.rs
│   │   └── interpolation.rs
│   └── renderer/
│       ├── mod.rs
│       └── ui.rs
├── Cargo.toml
└── README.md
```

## Aizawa Attractor

The Aizawa system is defined by:

```
dx/dt = (z - b)*x - d*y
dy/dt = d*x + (z - b)*y
dz/dt = c + a*z - z³/3 - (x² + y²)*(1 + e*z) + f*z*x³
```

Default parameters: `a=0.95, b=0.7, c=0.6, d=3.5, e=0.25, f=0.1`

## Roadmap

- [x] Project setup
- [x] **Phase 1**: Basic simulation + visualization
- [ ] **Phase 2**: Job System for parallel particle updates  
- [ ] **Phase 3**: Network sync (fixed host)
- [ ] **Phase 4**: Auto-discovery (dynamic host)
- [ ] **Phase 5**: Host migration
