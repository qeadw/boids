# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A boids flocking simulation with a Rust/WebAssembly core for simulation physics and JavaScript/React frontend for rendering and UI. Features emergent flocking behavior, predator-prey dynamics, day/night cycle, and genetic mutations.

## Build & Run

### Prerequisites
- Rust toolchain (`rustup`)
- wasm-pack (`cargo install wasm-pack`)

### Build Commands
```bash
# Build Wasm module (from project root)
wasm-pack build --target web --release

# Output goes to pkg/ directory
```

### Running
1. Build the Wasm module first
2. Serve `index.html` via a local HTTP server (required for ES modules)
   ```bash
   python -m http.server 8000
   # or
   npx serve .
   ```
3. Open http://localhost:8000 in browser

## Security

All secrets and API keys must be stored in `.env` (git-ignored). Never commit credentials, tokens, or keys.

## Architecture

```
index.html          # React UI + Canvas rendering (ES module)
pkg/                # wasm-pack output (generated, git-ignored)
src/
  lib.rs            # Wasm exports (WasmWorld)
  vector.rs         # Vec2 math
  spatial.rs        # SpatialHash for O(1) neighbor queries
  boid.rs           # Boid struct with flocking + mutations
  predator.rs       # Predator AI hunting logic
  bug.rs            # Bug entities
  world.rs          # World container, main tick() loop
Cargo.toml          # Rust dependencies
```

### Rust Modules (src/)

- **vector.rs**: `Vec2` struct with mutable/immutable math operations
- **spatial.rs**: `SpatialHash` grid-based spatial partitioning (50px cells)
- **boid.rs**: `Boid` struct with position, velocity, energy, mutations bitfield, flocking forces
- **predator.rs**: `Predator` hunting AI with target priority
- **bug.rs**: Simple wandering bug entities
- **world.rs**: `World` containing all entities, `tick()` advances simulation

### Wasm API (lib.rs exports)

```rust
WasmWorld::new(width, height, start_boids)
WasmWorld::tick(cursor_x, cursor_y, cursor_mode, cursor_strength)
WasmWorld::get_stats() -> Vec<f32>  // [boid_count, predator_count, bug_count, day_phase]
WasmWorld::get_boid_data() -> Vec<f32>  // 11 floats per boid
WasmWorld::get_predator_data() -> Vec<f32>  // 7 floats per predator
WasmWorld::get_bug_data() -> Vec<f32>  // 4 floats per bug
WasmWorld::add_boid(x, y, species)
WasmWorld::add_boids(x, y, count)
WasmWorld::add_predator(x, y)
WasmWorld::add_obstacle(x, y)
WasmWorld::add_food(x, y)
WasmWorld::reset(width, height, start_boids)
```

### JavaScript (index.html)

- **React component `App`**: UI controls, canvas rendering
- **Visual effects classes**: Ripple, DangerZone, Tree, Pond, Shelter, Nest (rendering only)
- **Animation loop**: Calls `world.tick()`, renders entities from flat arrays

### Data Flow

1. JS calls `world.tick()` with cursor position/mode
2. Rust advances simulation (flocking, predator hunting, spawning, death)
3. JS calls `get_*_data()` to get flat Float32Arrays
4. JS renders entities to canvas using array data

### Key Constants

- `MAX_SPEED = 4.0`, `MAX_FORCE = 0.15`
- Boid perception radius: 50px
- Predator hunt radius: 150px
- Day/night cycle: ~2000 ticks

### Mutation System

27 mutation types stored as bitfield in `Mutations` struct. Affects size, speed, energy, behavior. Mutations inherited probabilistically during reproduction.
