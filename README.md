# RustPond - Rust Port of Pond Physics Simulation

## 🎯 Goal
Port the C++/SFML Pond simulation to Rust with `macroquad` for improved:
- **Memory Safety**: No segfaults, use-after-free, or data races
- **Performance**: Rust's zero-cost abstractions and better optimization
- **Maintainability**: Modern tooling, package management, and error handling

## 🚀 Quick Start

```bash
# Build and run
cargo run --release

# Development mode (faster compile, slower runtime)
cargo run
```

## 📊 Performance Comparison

| Metric | C++ (SFML) | Rust (macroquad) |
|--------|------------|------------------|
| Max Protons | 100 | TBD |
| Max Atoms | 100 | TBD |
| FPS (100 particles) | TBD | TBD |
| Compile Time | ~30s | TBD |

## 🔧 Architecture

### Core Modules
- `constants.rs` - All physics constants (from Constants.h)
- `proton.rs` - Proton particle physics
- `atom.rs` - PathFollowingAtom implementation
- `ring.rs` - Energy wave rings
- `spatial_grid.rs` - Spatial partitioning for collision detection
- `batch_renderer.rs` - Efficient batch rendering
- `proton_manager.rs` - Proton lifecycle management
- `atom_manager.rs` - Atom lifecycle management
- `ring_manager.rs` - Ring lifecycle management
- `main.rs` - Game loop and window management

## 🌟 Rust Improvements Over C++

1. **No Manual Memory Management**: No `unique_ptr`, automatic cleanup
2. **Compile-Time Safety**: Impossible to have dangling references
3. **Better Parallelization**: Safe concurrency with `rayon` (future)
4. **Pattern Matching**: Cleaner state machine logic
5. **Cargo Ecosystem**: Easy dependency management
