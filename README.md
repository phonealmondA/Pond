# RustPond - Nuclear Physics Simulation

## Overview
RustPond is an interactive nuclear physics simulation built in Rust with `macroquad`. It simulates particle physics, fusion reactions, molecular bonding, and phase transitions (like water freezing into ice crystals and element crystallization).

Spawn energy waves of different frequencies (colors) to create protons, which then interact to form elements from hydrogen all the way to sulfur, as well as molecules like water (H₂O), methane (CH₄), and other compounds. Watch as your creations undergo fusion, crystallization, and phase changes based on energy levels and environmental conditions.

## Features

### Nuclear Physics
- **Proton Creation**: Energy waves create protons with energy based on wave frequency
- **Fusion Reactions**: Protons fuse to form increasingly complex elements
  - H → He³ → He⁴ → C¹² → Ne²⁰ → Mg²⁴ → Si²⁸ → S³²
- **Neutron Formation**: Protons capture electrons from low-energy waves to become neutrons
- **Element Discovery System**: Track which elements you've successfully created

### Molecular Chemistry
- **Water (H₂O)**: Oxygen captures hydrogen atoms to form water molecules
- **Hydrogen Sulfide (H₂S)**: Sulfur combines with hydrogen
- **Magnesium Hydride (MgH₂)**: Magnesium-hydrogen compounds
- **Methane (CH₄)**: Carbon captures four hydrogen atoms
- **Silane (SiH₄)**: Silicon-hydrogen compounds

### Phase Transitions & Crystallization
- **Water Ice Formation**: H₂O molecules form hydrogen bonds and freeze into hexagonal crystal patterns
- **Element Crystallization**: Each element has unique crystal structures
  - **Hydrogen**: Molecular crystals with moderate bonds
  - **Helium (He³/He⁴)**: Noble gas - ultra-weak Van der Waals bonds, barely crystallizes
  - **Carbon (C¹²)**: DUAL MODE - forms graphite (3-fold, sheets) at normal pressure or diamond (4-fold, tetrahedral) under high pressure
  - **Neon (Ne²⁰)**: Noble gas - weak bonds, face-centered cubic packing
  - **Magnesium (Mg²⁴)**: Metallic - flexible hexagonal close-packed structure, bonds bend not break
  - **Silicon (Si²⁸)**: Semiconductor - diamond cubic with rigid tetrahedral bonding
  - **Sulfur (S³²)**: Forms S₈ crown-shaped ring molecules (each atom bonds to exactly 2 neighbors)
- **Evaporation & Melting**: High-speed waves can break bonds and melt crystalline structures

### Energy Wave Physics
- **35-Color Spectrum**: From dark red (slowest) to white (fastest)
  - Red waves (low frequency): Slow, low energy, can create negative protons
  - White waves (high frequency): Fast, high energy, powerful fusion catalyst
- **Wave-Particle Interaction**: Waves transfer energy to particles on contact
- **Mouse Wheel Color Cycling**: Easily cycle through the color spectrum
- **Interactive Color Slider**: Visual feedback with clickable/draggable color selection

### Interactive UI
- **Elements Menu**: View all discovered elements with counts and colors
- **Controls Menu**: Complete control reference and real-time statistics
- **Element Spawning**: Right-click and drag to spawn selected elements with custom velocity
- **Real-time Stats**: Monitor FPS, particle counts, and current wave frequency

## Quick Start

```bash
# Build and run in release mode (recommended)
cargo run --release

# Development mode (faster compile, slower runtime)
cargo run
```

## 🎮 Controls

### Wave & Element Spawning
- **Left Click**: Spawn energy ring at cursor position
- **Right Click & Drag**: Spawn selected element with velocity (drag direction = velocity)
- **Mouse Wheel Up**: Cycle to next wave color (higher frequency)
- **Mouse Wheel Down**: Cycle to previous wave color (lower frequency)
- **Color Slider** (bottom of screen): Click or drag to select wave color

### Menus
- **Elements Button** (top left): Open discovered elements menu and select which element to spawn
- **Controls Button** (top right): View controls and statistics

### Clearing & Control
- **R** or **Space**: Clear all non-stable particles (rings and unstable atoms)
- **H**: Delete all stable hydrogen atoms
- **Z**: Clear all protons (including immortal elements)
- **P**: Pause/unpause simulation
- **Esc**: Exit application

## File Structure

```
pond/
├── src/
│   ├── main.rs           - Game loop, UI, input handling, and rendering
│   ├── constants.rs      - All physics constants and configuration values
│   ├── proton.rs         - Proton particle physics and properties
│   ├── proton_manager.rs - Proton lifecycle, fusion reactions, and element formation
│   ├── atom.rs           - Atom physics and wave-following behavior
│   ├── ring.rs           - Energy wave rings with RingManager
│   └── (ring_manager integrated in ring.rs)
├── Cargo.toml            - Rust dependencies and project configuration
└── README.md             - This file
```

### Module Descriptions

#### `constants.rs`
Defines all physics parameters:
- Proton behavior (friction, speed, lifetime, fusion thresholds)
- Element properties (colors, radii, capture ranges)
- Crystallization parameters (bond strengths, evaporation speeds, geometric patterns)
- Ring colors (35-color frequency spectrum)
- Wave physics (speed ranges, bouncing behavior)

#### `proton.rs` & `proton_manager.rs`
- Individual proton physics (movement, energy, lifetime)
- Element formation and fusion logic
- Molecular bonding (H₂O, CH₄, etc.)
- Crystallization system for all elements
- Force calculations (repulsion, attraction, charge interactions)
- Phase transition mechanics

#### `atom.rs`
- Wave-following particle behavior
- Energy-based lifetime and visual effects
- Used internally for physics calculations (not visually rendered)

#### `ring.rs` (includes RingManager)
- Expanding energy wave rings
- Frequency-based speed calculation (color → speed)
- Wall bouncing and reflections
- Color palette management
- Mouse wheel and slider integration

#### `main.rs`
- Main game loop
- UI system (menus, buttons, color slider)
- Input handling (mouse, keyboard, wheel)
- Element discovery tracking
- Rendering coordination

## Rust Advantages

1. **Memory Safety**: No segfaults, use-after-free, or dangling pointers
2. **Performance**: Zero-cost abstractions with excellent compiler optimizations
3. **Cargo Ecosystem**: Simple dependency management (`macroquad` for graphics)
4. **Pattern Matching**: Clean element type handling and state machines
5. **Type Safety**: Compile-time guarantees prevent many runtime errors

## Physics Highlights

### Fusion Chain
The simulation implements a realistic fusion chain similar to stellar nucleosynthesis:
1. Energy waves create protons
2. Protons fuse to form deuterium (with neutron)
3. Deuterium + proton → Helium-3
4. Two Helium-3 → Helium-4 + energy release
5. Triple-alpha process: 3 He⁴ → Carbon-12
6. Alpha capture continues: C¹² → O¹⁶ → Ne²⁰ → Mg²⁴ → Si²⁸ → S³²

### Crystallization Mechanics
Each element has scientifically-inspired bonding:
- **Noble gases** (He, Ne): Extremely weak Van der Waals forces
- **Metals** (Mg): Flexible metallic bonds that deform rather than break
- **Covalent networks** (C, Si): Strong directional bonds with specific angles
- **Molecular solids** (S₈ rings, H₂ pairs): Discrete molecular units
- **Hydrogen bonding** (H₂O ice): Directional bonding with hexagonal geometry

### Red Wave Mechanics
- Dark red waves create negative protons (H⁻)
- Red waves repel negative protons (simulating electron-electron repulsion)
- Sufficient red wave hits can melt ice crystals and break bonds
- Multiple hit counting prevents instant melting

## Color Spectrum

The simulation uses a 35-color frequency spectrum:
- **Dark Red (0-4)**: Lowest frequency, slowest waves (~15-80 px/s)
- **Red-Orange (5-9)**: Low-medium frequency
- **Yellow-Green (10-14)**: Medium frequency
- **Green-Cyan (15-19)**: Medium-high frequency
- **Blue-Violet (20-29)**: High frequency
- **Magenta-White (30-34)**: Highest frequency, fastest waves (up to 200 px/s)

Color affects both wave speed AND energy transfer to particles!

## License

This project is a physics simulation for educational and entertainment purposes.
