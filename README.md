# RustPond - Nuclear Physics Simulation

## Overview

RustPond is an interactive nuclear physics simulation built in Rust with `macroquad`.
 It simulates particle physics, fusion reactions, molecular bonding, and phase transitions
 (like water freezing into ice crystals and element crystallization).

Spawn energy waves of different frequencies (colors) to create protons, which then interact 
to form elements from hydrogen all the way to sulfur, as well as molecules like water (H₂O),
 methane (CH₄), and other compounds. Watch as your creations undergo fusion, crystallization, 
 and phase changes based on energy levels and environmental conditions.

Think of it as your own personal particle accelerator and chemistry lab where you can watch nuclear physics unfold in real-time!

## Features

### Nuclear Physics & Fusion Reactions

#### Proton Creation
- **Energy Wave Interaction**: When energy waves (rings) pass through space, they create protons with energy proportional to the wave's frequency (color)
- **Wave Frequency Spectrum**: 35 colors from dark red (lowest frequency, ~15 px/s) to white (highest frequency, 200 px/s)
- **Energy Transfer**: Higher frequency waves create higher-energy protons

#### Neutron Formation
- **Electron Capture**: Positive protons (H⁺) near low-energy waves can capture electrons to become neutral deuterium (H with 1 neutron)
- **Proximity-Based**: Occurs when protons are close to atom particles from wave interactions
- **Time-Gated**: Takes 0.1 seconds to complete the transformation

#### Complete Fusion Chain
The simulation implements realistic fusion reactions similar to stellar nucleosynthesis:

1. **Proton → Deuterium**: H⁺ captures electron → H (neutral, with neutron)
2. **Deuterium-Proton Fusion**: H (neutral) + H⁺ → He³ (Helium-3)
   - Requires relative velocity > 0.5 (moderate collision speed)
   - Forms He³ with 2 protons, 2 neutrons, charge +1
3. **Helium-3 Fusion**: He³ + He³ → He⁴ (Helium-4) + energy release
   - Requires relative velocity > 0.6 (higher collision speed)
   - Forms stable He⁴ with 2 protons, 2 neutrons, charge +2
   - Releases 30 units of fusion energy as new rings
4. **Triple-Alpha Process**: 3 He⁴ → C¹² (Carbon-12)
   - Requires 3 He⁴ nuclei in close proximity
   - Minimum combined energy threshold: 60 units
   - Average relative velocity > 0.7 (high-energy collisions)
   - Forms C¹² with 6 protons, 6 neutrons
5. **Alpha Capture Chain**: Progressive fusion with He⁴
   - **C¹² + He⁴ → O¹⁶** (Oxygen-16) - velocity threshold 0.6
   - **O¹⁶ + He⁴ → Ne²⁰** (Neon-20) - velocity threshold 0.65
   - **Ne²⁰ + He⁴ → Mg²⁴** (Magnesium-24) - velocity threshold 0.68
   - **Mg²⁴ + He⁴ → Si²⁸** (Silicon-28) - velocity threshold 0.70
   - **Si²⁸ + He⁴ → S³²** (Sulfur-32) - velocity threshold 0.72

Each fusion step requires progressively higher collision velocities, simulating the increasing Coulomb barrier in nuclear physics!

### Molecular Chemistry

#### Hydrogen Compounds
- **Water (H₂O)**: Oxygen-16 captures 2 hydrogen atoms
  - Capture range: 45 pixels
  - Forms stable blue molecules
  - Can form hydrogen bonds with other water molecules
- **Hydrogen Sulfide (H₂S)**: Sulfur-32 + 2 hydrogen
  - Yellow-green color
  - Capture range: 45 pixels
- **Methane (CH₄)**: Carbon-12 + 4 hydrogen atoms
  - Pale blue-green color
  - Capture range: 50 pixels
  - All 4 hydrogens must be captured
- **Silane (SiH₄)**: Silicon-28 + 4 hydrogen atoms
  - Orange-red color
  - Capture range: 50 pixels
- **Magnesium Hydride (MgH₂)**: Magnesium-24 + 2 hydrogen
  - Gray-metallic appearance
  - Capture range: 45 pixels

#### Molecular Bonding Mechanics
- Elements actively seek out hydrogen atoms within their capture range
- Molecules are stable and won't break apart unless destroyed manually
- Each molecule type has unique visual appearance with appropriate colors

### Phase Transitions & Crystallization

RustPond simulates realistic crystallization for each element based on their real-world bonding properties!

#### Water Ice Formation (Hexagonal Crystals)
- **Hydrogen Bonding**: H₂O molecules form directional hydrogen bonds with up to 5 neighbors
- **Geometric Structure**: Forms hexagonal ice patterns (like real ice!)
  - Maximum 5 bonds per molecule
  - Ideal bond length: 68 pixels
  - Angular tolerance: ~20-30 degrees for proper hexagonal geometry
- **Freezing Mechanics**:
  - Requires 2+ frozen neighbors to trigger rapid freezing (seed crystal growth)
  - Slow-moving water molecules gradually align and bond
  - Once frozen, becomes highly stable
- **Melting**:
  - Normal water evaporates at speed > 40 px/s
  - Frozen ice requires speed > 120 px/s to melt
  - Red waves can break bonds with repeated hits (5 hits from dark red waves)
- **Visual Bonds**: Blue-white lines connecting bonded water molecules

#### Hydrogen (H₁) Crystallization
- **Molecular Solid**: H atoms form weak molecular crystals
- **Bonding**:
  - Minimum 3 neighbors required (within 80 pixels)
  - Bond strength: 35 (moderate)
  - Rest length: 45 pixels
- **Phase Transition**:
  - Evaporation speed: 60 px/s (normal)
  - Frozen evaporation speed: 150 px/s (much higher for solid)
  - Freeze cooldown: 9 seconds after melting
- **Vibration**: Crystallized H can vibrate when bonds are compressed

#### Noble Gases (Helium & Neon)

**Helium-3 (He³) - Ultra-Weak Van der Waals**
- **Bonding**: Barely bonds at all! (strength: 3)
- Requires 6-8 close-packed neighbors
- Bond length: 48 pixels
- Evaporates at slightest movement (10 px/s)
- Frozen evaporation: 30 px/s
- Very flexible angular geometry (~80° tolerance)
- Orange-yellow tint

**Helium-4 (He⁴) - Ultra-Weak Van der Waals**
- **Bonding**: Slightly less reluctant than He³ (strength: 5)
- Hexagonal close-packing (6-8 neighbors)
- Bond length: 50 pixels
- Evaporates at 12 px/s
- Frozen evaporation: 35 px/s
- Extremely flexible, no strict angular preference
- Bright yellow color

**Neon-20 (Ne²⁰) - Weak Van der Waals**
- **Crystal Structure**: Face-centered cubic packing
- Bond strength: 8 (weak, but stronger than helium)
- Requires 6-8 neighbors
- Bond length: 52 pixels
- Evaporation: 15 px/s normal, 40 px/s frozen
- 60° hexagonal close-pack angles with ~80° tolerance
- Pink/magenta bonds and color

#### Carbon-12 (C¹²) - DUAL MODE: Graphite vs Diamond

Carbon has TWO different crystal structures depending on pressure!

**Graphite Mode (Normal Pressure)**
- **Structure**: Flat sheets with 120° angles
- Requires exactly 3 neighbors (trigonal planar)
- Bond strength: 65 (strong covalent)
- 120° angle spacing with ±23° tolerance
- Forms layered sheet-like structures
- Gray bonds

**Diamond Mode (High Pressure)**
- **Activation**: When 8+ carbon atoms nearby (pressure detection radius: 120 pixels)
- **Structure**: Tetrahedral 3D network (approximated as 90° in 2D)
- Requires exactly 4 neighbors
- Bond strength: 120 (ultra-strong covalent!)
- 90° angle spacing with ±17° tolerance (very rigid)
- Hardest material in the simulation
- Gray bonds, more tightly packed

**Shared Properties**
- Bond rest length: 60 pixels
- Evaporation: 100 px/s normal, 250 px/s frozen
- Freeze cooldown: 12 seconds
- Dark gray color

#### Magnesium-24 (Mg²⁴) - Metallic Flexibility

- **Crystal Structure**: Hexagonal close-packed (HCP) metal
- **Metallic Bonding**: Bonds bend and deform rather than break!
- Flexible coordination: 4-8 neighbors acceptable
- Bond strength: 40 (moderate)
- Bond length: 65 pixels (can stretch to 110 pixels!)
- 60° hexagonal angles with ±45° tolerance (very flexible)
- Evaporation: 110 px/s normal, 220 px/s frozen
- Silvery light blue-gray color and bonds
- **Unique Property**: Simulates metallic flow - atoms slide past each other

#### Silicon-28 (Si²⁸) - Semiconductor Diamond Cubic

- **Crystal Structure**: Diamond cubic (like diamond, but silicon!)
- **Tetrahedral Bonding**: Always exactly 4 neighbors
- Bond strength: 70 (strong covalent)
- Bond length: 62 pixels
- 90° tetrahedral angles (2D approximation) with ±23° tolerance
- Evaporation: 90 px/s normal, 220 px/s frozen
- Freeze cooldown: 11 seconds
- Brown/tan color and bonds
- Rigid semiconductor structure

#### Sulfur-32 (S³²) - Crown Ring Formation

Sulfur is COMPLETELY DIFFERENT - it forms S₈ ring molecules!

- **Unique Bonding**: Each sulfur wants EXACTLY 2 bonds (not 4!)
- **S₈ Rings**: Forms crown-shaped 8-atom rings
- Bond strength: 50 (moderate covalent within rings)
- Bond length: 55 pixels
- Ring angle: ~105° (crown shape) with ±40° tolerance (flexible rings)
- Ring detection: Searches up to 10 atoms deep to find closed loops
- Evaporation: 65 px/s normal, 150 px/s frozen
- Bright yellow color and bonds
- **Realistic Chemistry**: Mimics how real sulfur forms S₈ molecular crystals!

### Energy Wave Physics

#### The 35-Color Frequency Spectrum

RustPond uses color to represent electromagnetic wave frequency, just like real light!

**Spectrum Breakdown:**
- **Indices 0-4: Dark Red** → Lowest frequency (15-45 px/s)
  - Slowest waves, lowest energy
  - Create low-energy protons
  - Can create negative protons (H⁻)
  - Dark red waves repel H⁻ (electron-electron repulsion)
- **Indices 5-9: Red-Orange** → Low-medium frequency (45-80 px/s)
- **Indices 10-14: Yellow-Green** → Medium frequency (80-115 px/s)
- **Indices 15-19: Green-Cyan** → Medium-high frequency (115-150 px/s)
- **Indices 20-24: Blue** → High frequency (150-175 px/s)
- **Indices 25-29: Violet** → Very high frequency (175-190 px/s)
- **Indices 30-34: Magenta-White** → Highest frequency (190-200 px/s)
  - Fastest waves, highest energy
  - Create high-energy protons
  - Best for fusion reactions

#### Wave Behavior
- **Expansion**: Rings grow outward from spawn point at frequency-dependent speed
- **Wall Bouncing**: Waves reflect off screen edges, creating reflected wave patterns
- **Opacity Fading**: Waves fade as they expand (alpha = max(0.1, 1 - radius/800))
- **Particle Creation**: Create atoms/protons at wave intersections
- **Energy Transfer**: Waves accelerate particles on contact based on wave speed
- **Visual Feedback**: Color slider shows all 35 colors with indicator at current selection

#### Red Wave Special Mechanics
- **Negative Proton Creation**: Dark red waves (indices 0-4, speed < 30 px/s) can create H⁻
- **Repulsion Force**: Red waves repel H⁻ protons (simulating electron-electron repulsion)
  - Repulsion strength: 5000
  - Active when wave speed > 100 px/s
  - Interaction width: 15 pixels around wave
- **Ice Melting**: Red waves can melt frozen water ice
  - Requires 5 hits from dark red waves
  - Hit cooldown: 0.3 seconds between hits
  - Progressive melting mechanic

### Force & Interaction Systems

#### Charge-Based Forces
- **Like Charges Repel**: H⁺ repels H⁺, H⁻ repels H⁻
  - Repulsion strength: 1000
  - Range: 150 pixels
- **Opposite Charges Attract**: H⁺ attracts H⁻
  - Attraction strength: 800
  - Range: 150 pixels
- **Neutral Hydrogen Clustering**: Neutral H atoms (deuterium) attract each other
  - Attraction range: 1100 pixels
  - Attraction strength: 600
  - Enables deuterium "clouds"

#### Helium-4 Clustering
- **Alpha Particle Attraction**: He⁴ nuclei attract each other for fusion
  - Range: 1420 pixels
  - Strength: 500
  - Essential for triple-alpha process

#### Solid Collisions
- **Close-Range Bouncing**: At very close distances (< 1.5 pixels), particles bounce like solid balls
- **Elastic Collisions**: Velocity exchange with dampening factor
- **Prevents Overlap**: Keeps particles physically separated
- **Applies to**: All charged and neutral particles

#### Oxygen-16 Bonding
- **Strong Bonds**: O¹⁶ pairs bond together with spring forces
- Bond strength: 200
- Bond length: Can extend up to 380 pixels before breaking
- Enables complex oxygen molecule behavior

### Interactive UI System

#### Main Buttons
- **Elements Button** (top-left, 120×40px)
  - Opens element discovery menu
  - Shows all discovered elements with counts
  - Click elements to select for spawning
  - Two-column layout for easy viewing
- **Controls Button** (top-right, 120×40px)
  - Opens controls & statistics menu
  - Real-time FPS counter
  - Particle counts (rings, atoms, protons)
  - Current wave frequency info
  - Complete control reference

#### Color Slider (Bottom Center)
- **Visual Spectrum**: Shows all 35 colors in a gradient bar
- **Width**: 600 pixels
- **Interactive**:
  - Click to instantly jump to a color
  - Drag to smoothly scan through colors
  - Mouse wheel to cycle (up = next, down = previous)
- **Indicator**: Shows current color with a circle marker
- **Always Visible**: Persists during gameplay for quick access

#### Selected Element Display
- **Top Center**: Shows currently selected element name and color
- Semi-transparent black background
- Large, readable text
- Updates when you select from Elements menu

#### Menu System
- **Element Discovery Menu**:
  - Semi-transparent overlay
  - Displays element symbols with colors
  - Shows count of each element type
  - Two-column layout (9 per column)
  - Click element to select, click outside to close
- **Controls & Stats Menu**:
  - Statistics section (FPS, counts, current wave info)
  - Controls reference section
  - Scrollable list of all keyboard/mouse controls
  - Click outside to close

#### Pause Indicator
- Large red "PAUSED" text appears in center when simulation is paused
- Black outline for visibility
- 60pt font size

### Element Discovery System

- **Automatic Tracking**: Elements are discovered when first created
- **Persistent Counts**: Track total count of each element type currently in existence
- **13 Discoverable Elements/Molecules**:
  1. H1 (Hydrogen)
  2. He3 (Helium-3)
  3. He4 (Helium-4)
  4. C12 (Carbon-12)
  5. Ne20 (Neon-20)
  6. Mg24 (Magnesium-24)
  7. Si28 (Silicon-28)
  8. S32 (Sulfur-32)
  9. H2O (Water)
  10. H2S (Hydrogen Sulfide)
  11. MgH2 (Magnesium Hydride)
  12. CH4 (Methane)
  13. SiH4 (Silane)

- **Visual Feedback**: Each element shown with its unique color
- **Count Display**: Shows current population of each element type

## Quick Start

```bash
# Build and run in release mode (recommended for performance)
cargo run --release

# Development mode (faster compile, slower runtime)
cargo run
```

**System Requirements:**
- Rust toolchain (1.70+)
- Works on Windows, macOS, Linux
- OpenGL-compatible graphics

## Controls

### Wave & Element Spawning
- **Left Click**: Spawn energy ring at cursor (uses current selected color)
- **Right Click & Drag**: Spawn selected element with velocity
  - Click and hold at starting point
  - Drag to set direction and speed
  - Release to spawn with velocity vector (2× drag distance)
- **Mouse Wheel Up**: Cycle to next wave color (higher frequency/energy)
- **Mouse Wheel Down**: Cycle to previous wave color (lower frequency/energy)
- **Color Slider** (bottom): Click anywhere or drag to select wave color

### Menus & UI
- **Elements Button** (top-left): Open discovered elements menu
  - Click an element to select it for spawning
  - View element counts
  - Click outside menu to close
- **Controls Button** (top-right): View controls and real-time statistics

### Clearing & Management
- **R**: Clear all non-stable particles (rings, atoms, unstable protons)
- **Space**: Same as R - clear all non-stable particles
- **H**: Delete all stable hydrogen atoms (H1)
- **Z**: Clear ALL protons, including immortal elements (complete reset)

### Simulation Control
- **P**: Pause/unpause simulation (freezes all physics)
- **Esc**: Exit application

## File Structure

```
pond/
├── src/
│   ├── main.rs           - Game loop, UI, input handling, rendering coordination
│   ├── constants.rs      - All physics constants, element properties, wave colors
│   ├── proton.rs         - Individual proton physics and properties
│   ├── proton_manager.rs - Proton lifecycle, fusion logic, element formation,
│   │                       crystallization systems, force calculations
│   ├── atom.rs           - Atom physics, wave-following behavior
│   ├── ring.rs           - Energy wave rings, RingManager, color palette
├── Cargo.toml            - Dependencies: macroquad for graphics
└── README.md             - This file
```

### Module Deep-Dive

#### `constants.rs` - Physics Parameter Database
Centralized configuration for all physics constants:
- **Proton Physics**: Friction (1.0), max speed (200 px/s), default lifetime (20s)
- **Element Properties**: Colors (RGB tuples), radii multipliers, capture ranges
- **Fusion Thresholds**: Velocity requirements for each fusion type
- **Crystallization Parameters**: Bond strengths, rest lengths, evaporation speeds, geometric angles
- **Wave Spectrum**: 35 hardcoded colors, speed calculation weights
- **Force Strengths**: Repulsion (2000), attraction (800-1000), bond forces

**Key Constant Groups:**
- `proton::*` - Individual proton behavior
- `proton_manager::*` - Inter-proton forces and crystallization
- `atom::*` - Atom particle properties
- `ring::*` - Wave behavior and color-to-speed mapping
- `RING_COLORS` - The 35-color frequency spectrum array

#### `proton.rs` - Single Particle Physics
Each proton is a self-contained entity with:
- **Position & Velocity**: Vec2 coordinates, movement vector
- **Energy & Mass**: Energy level, calculated mass (energy × 0.1)
- **Charge**: -1 (H⁻), 0 (neutral), +1 (H⁺), +2 (He⁴), etc.
- **Neutron Count**: 0 (bare proton), 1 (deuterium), 2+ (heavier elements)
- **Lifetime**: Countdown timer, fades out before death
- **Visual Effects**: Pulsing based on energy, glowing layers, radius scaling
- **State Flags**: Stable hydrogen, stable helium, part of molecule, bonded, frozen, etc.

**Methods:**
- `update()` - Apply velocity, friction, check lifetime
- `try_neutron_formation()` - Electron capture logic
- `try_capture_electron()` - Convert to negative charge
- `calculate_visual_radius()` - Size based on energy and element type

#### `proton_manager.rs` - The Physics Engine
This is the heart of the simulation! Manages all protons and their interactions:

**Update Pipeline (runs every frame):**
1. **Simple Physics** - Apply velocity, friction, wall bouncing
2. **Charge Forces** - Repulsion/attraction based on charge
3. **Red Wave Repulsion** - Repel H⁻ from red waves
4. **Crystallization Updates** - All 8 element crystallization systems
5. **Oxygen Bonds** - Spring forces for O¹⁶ pairs
6. **Water H-Bonds** - Hydrogen bonding for ice formation
7. **Neutron Formation** - Electron capture from atoms
8. **Electron Capture** - Neutral → negative charge conversion
9. **Nuclear Fusion** - All fusion reactions
10. **Solid Collisions** - Close-range elastic bouncing
11. **Atom Spawning** - Create protons from atom collisions
12. **Cleanup** - Remove dead non-stable particles

**Fusion Handler:**
- Checks all proton pairs every frame
- Calculates collision conditions (distance, velocity, energy)
- Applies fusion rules with velocity thresholds
- Creates fusion products with correct properties
- Releases energy as new wave rings

**Crystallization Systems (8 separate update functions):**
- `update_h_crystallization()` - Hydrogen molecular crystals
- `update_he3_crystallization()` - Helium-3 noble gas
- `update_he4_crystallization()` - Helium-4 noble gas
- `update_ne20_crystallization()` - Neon noble gas
- `update_c12_crystallization()` - Carbon graphite/diamond
- `update_mg24_crystallization()` - Magnesium metal
- `update_si28_crystallization()` - Silicon semiconductor
- `update_s32_crystallization()` - Sulfur ring formation

**Element Spawning:**
- `spawn_element()` - Create any element at position with velocity
- Sets up correct charge, neutron count, mass, color
- Used for manual element spawning via right-click drag

**Statistics:**
- `get_element_counts()` - Count each element type
- `get_proton_count()` - Total active protons
- Used by UI for real-time stats

#### `atom.rs` & `AtomManager` - Wave Followers
Atoms are invisible particles created at wave intersections:
- **Energy-Based**: Lifetime proportional to energy
- **Wave Following**: Move along wave edges, tracking rings
- **Collision Detection**: Can collide to create protons
- **Visual Effects**: Pulsing size/color (but not rendered in current build)
- **Backend Role**: Primarily used for physics calculations

#### `ring.rs` & `RingManager` - Wave System
**Individual Ring:**
- Expands outward from spawn point
- Speed calculated from color (frequency)
- Bounces off walls creating reflected copies
- Fades with distance (opacity drops)
- Max radius: 2000 pixels before death

**RingManager:**
- Maintains array of all active rings
- Color palette (35 colors)
- Current color selection & index
- `cycle_to_next_color()` / `cycle_to_previous_color()` - Mouse wheel
- `set_color_by_index()` - Color slider interaction
- `add_ring()` - Spawn new wave at position
- `get_current_frequency_info()` - String for UI display

#### `main.rs` - Application Core
**Game Loop:**
- FPS tracking (updates every 1 second)
- Delta time calculation
- Update managers (rings → atoms → protons)
- Render everything
- Handle input events

**UI State:**
- Menu state machine (None, Elements, Controls)
- Element discovery tracking (HashSet)
- Selected element (Option<ElementType>)
- Right-click drag state for element spawning
- Color slider drag state

**Input Handling:**
- Mouse button events (left, right, pressed/down/released)
- Mouse wheel delta detection
- Keyboard events (R, Space, H, Z, P, Esc)
- Color slider interaction
- Menu interaction (buttons, element selection)

**Rendering Order:**
1. Clear background (black)
2. Draw rings (18 segments)
3. Draw protons (24 segments) with bonds
4. Draw element labels
5. Draw UI (buttons, slider, menus, pause indicator)

**Element Type Enum:**
- 13 element types with name() and color() methods
- Used for discovery system and spawning
- Mapped to proton configurations

## Rust Advantages

1. **Memory Safety**: No segfaults, use-after-free, or dangling pointers
   - Vec<Option<Proton>> prevents access to deleted particles
   - Borrow checker ensures no simultaneous mutable access
2. **Performance**: Zero-cost abstractions with LLVM optimization
   - Release builds are highly optimized
   - Inlining and loop unrolling automatic
3. **Cargo Ecosystem**: Simple dependency management
   - `macroquad` handles all graphics, input, windowing
   - One-command build and run
4. **Pattern Matching**: Clean element type handling
   - `match` statements for fusion logic
   - Exhaustive checking prevents bugs
5. **Type Safety**: Compile-time guarantees
   - Strong typing prevents unit errors
   - Enums for menu states prevent invalid states

## Physics Highlights

### Fusion Chain (Stellar Nucleosynthesis)

The simulation mirrors how stars build heavy elements:

1. **Proton Formation**: Energy from waves creates free protons
2. **Neutron Formation**: Low-energy electron capture converts H⁺ → H
3. **Deuterium-Proton Fusion**: H + H⁺ → He³ (requires velocity)
4. **Helium-3 Fusion**: 2 He³ → He⁴ + 2H⁺ + energy (releases heat)
5. **Triple-Alpha**: 3 He⁴ → C¹² (carbon formation, like in red giants)
6. **Alpha Capture Ladder**: Progressive fusion up the periodic table
   - C¹² → O¹⁶ → Ne²⁰ → Mg²⁴ → Si²⁸ → S³²
   - Each step harder than the last (higher velocity thresholds)
   - Simulates Coulomb barrier increasing with atomic number

**Realism:**
- Velocity thresholds simulate quantum tunneling probability
- Energy releases create new waves (fusion releases energy)
- Charge increases with atomic number (correct proton counts)
- Stable isotopes chosen (He⁴, C¹², O¹⁶, etc.)

### Crystallization Mechanics (Solid-State Physics)

Each element crystallizes according to its real bonding type:

**Van der Waals (Noble Gases - He, Ne):**
- Extremely weak, temporary bonds
- High angular flexibility (atoms barely care about positions)
- Evaporate at slightest movement
- Close-packed structures (maximize contacts)

**Metallic (Mg):**
- Flexible, deformable bonds
- Electron sea allows atoms to slide past each other
- Bonds bend rather than break
- Hexagonal close-packed structure

**Covalent Network (C, Si):**
- Strong, directional bonds
- Specific angles required (120° for graphite, 90° for diamond/Si)
- Rigid structures
- High evaporation thresholds

**Molecular (S₈, H₂):**
- Atoms bond into discrete molecules
- Weak intermolecular forces between molecules
- S forms crown-shaped 8-atom rings
- H forms pairs (H₂)

**Hydrogen Bonding (H₂O):**
- Directional electrostatic attraction
- Highly geometric (hexagonal ice)
- Stronger than Van der Waals, weaker than covalent
- Temperature-dependent (melts with energy)

### Red Wave Mechanics (Photon-Electron Interaction)

**Physical Basis:**
- Red light = low energy photons
- Can knock electrons off atoms → negative ions (H⁻)
- Electron-electron repulsion (like charges repel)
- Multiple photon absorption for bond breaking

**Implementation:**
- Dark red waves create H⁻ via electron transfer
- Fast red waves repel H⁻ (radiation pressure on electrons)
- 5 wave hits needed to melt ice (multi-photon process)
- Hit cooldown simulates quantum state transitions

## Color Spectrum (Electromagnetic Frequency)

The 35 colors represent the electromagnetic spectrum from infrared to ultraviolet:

**Spectrum Physics:**
- **Red** = Low frequency = Low energy = Long wavelength
- **Blue** = High frequency = High energy = Short wavelength
- **White** = Full spectrum = Highest energy in simulation

**Wave Speed = Frequency:**
- Lower frequency (red) → slower propagation (15 px/s)
- Higher frequency (blue/white) → faster propagation (200 px/s)
- Linear mapping: speed = 15 + (color_factor × 185)

**Energy Transfer:**
- Wave frequency determines energy given to particles
- High-frequency waves create high-energy protons
- High-energy protons more likely to fuse (overcome Coulomb barrier)

**Color-to-Speed Formula:**
```
color_factor = (R × 0.1) + (G × 0.3) + (B × 0.6)
speed = MIN_SPEED + color_factor × (MAX_SPEED - MIN_SPEED)
      = 15 + color_factor × 185
```

**Spectrum Distribution:**
- 6 reds, 5 oranges, 5 yellows, 5 greens, 5 cyans
- 5 blues, 5 violets, 5 magentas/whites
- Uniform distribution across color wheel

## Tips & Tricks

### Creating Elements
1. **Start with white waves** for high energy
2. **Click rapidly** in one spot to create many protons
3. **Wait for fusion** - protons need to collide with correct velocity
4. **Use He⁴ farming** - get 3 He⁴ together for carbon

### Building Molecules
1. **Create water**: Make oxygen (O¹⁶), then spawn H atoms nearby
2. **Methane**: Carbon + 4 hydrogen (tricky to get all 4!)
3. **Sulfur rings**: Create lots of S³² in one area - they'll link into S₈ rings

### Crystallization Experiments
1. **Carbon pressure test**: Pack lots of C¹² together, watch graphite → diamond
2. **Water freezing**: Slow-moving H₂O will freeze into hexagonal patterns
3. **Noble gas liquefaction**: Slow down He/Ne to see weak bonding

### Fusion Optimization
1. **High-speed collisions**: Spawn elements with velocity (right-click drag)
2. **Confined space**: Clear often to keep particles together
3. **Energy management**: Use white waves to energize sluggish particles

## License

This project is a physics simulation for educational and entertainment purposes.

