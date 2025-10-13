// Constants module - Direct port from Constants.h
// All physics constants and configuration values

use macroquad::prelude::*;

// ===== SYSTEM LIMITS =====
pub const MAX_PROTONS: usize = 750;
pub const MAX_ATOMS: usize = 250;
pub const CIRCLE_SEGMENTS: i32 = 24;
pub const COLOR_PALETTE_SIZE: usize = 35;
pub const COLOR_CYCLE_SIZE: usize = 6;

// ===== MATHEMATICAL CONSTANTS =====
pub const PI: f32 = std::f32::consts::PI;
pub const EPSILON: f32 = 0.001;
pub const COLOR_MAX: f32 = 255.0;

// ===== PROTON PHYSICS =====
pub mod proton {
    // Movement
    pub const FRICTION: f32 = 1.0;
    pub const BOUNCE_DAMPENING: f32 = 0.7;
    pub const MAX_SPEED: f32 = 200.0;  // Same as white ring speed

    // Size
    pub const MIN_RADIUS: f32 = 3.0;
    pub const MAX_RADIUS: f32 = 8.0;
    pub const ENERGY_TO_RADIUS_FACTOR: f32 = 0.01;

    // Mass and Energy
    pub const ENERGY_TO_MASS_FACTOR: f32 = 0.1;

    // Lifetime
    pub const DEFAULT_LIFETIME: f32 = 20.0;
    pub const FADE_START_RATIO: f32 = 0.8;
    pub const INFINITE_LIFETIME: f32 = -1.0;

    // Visual Effects
    pub const PULSE_FREQUENCY_BASE: f32 = 2.0;
    pub const PULSE_FREQUENCY_ENERGY_FACTOR: f32 = 0.01;
    pub const PULSE_INTENSITY: f32 = 0.2;
    pub const PULSE_BASE: f32 = 1.0;
    pub const STABLE_HYDROGEN_RADIUS_MULTIPLIER: f32 = 1.3;
    pub const BARE_PROTON_RED_TINT: f32 = 1.2;
    pub const GLOW_LAYER1_RADIUS: f32 = 1.5;
    pub const GLOW_LAYER1_ALPHA: f32 = 0.5;
    pub const GLOW_LAYER2_RADIUS: f32 = 2.0;
    pub const GLOW_LAYER2_ALPHA: f32 = 0.25;

    // Colors
    pub const STABLE_HYDROGEN_COLOR: (u8, u8, u8) = (255, 255, 255);
    pub const NEUTRAL_PROTON_COLOR: (u8, u8, u8) = (200, 200, 200);

    // Neutron Formation
    pub const NEUTRON_FORMATION_TIME: f32 = 0.1;
    pub const NEUTRON_RADIUS_MULTIPLIER: f32 = 1.2;

    // Electron Capture
    pub const ELECTRON_CAPTURE_DISTANCE: f32 = 15.0;

    // Negative Proton Decay
    pub const NEGATIVE_DECAY_TIME: f32 = 5.0;

    // Fusion thresholds
    pub const DEUTERIUM_FUSION_VELOCITY_THRESHOLD: f32 = 0.5;
    pub const HELIUM3_FUSION_VELOCITY_THRESHOLD: f32 = 0.6;
    pub const FUSION_ENERGY_RELEASE: f32 = 30.0;

    // Helium colors
    pub const HELIUM3_COLOR: (u8, u8, u8) = (255, 200, 100);
    pub const HELIUM4_COLOR: (u8, u8, u8) = (255, 255, 100);

    pub const HELIUM3_RADIUS_MULTIPLIER: f32 = 1.5;
    pub const HELIUM4_RADIUS_MULTIPLIER: f32 = 1.8;

    // Carbon-12 (triple-alpha process)
    pub const CARBON12_COLOR: (u8, u8, u8) = (100, 100, 100);
    pub const CARBON12_RADIUS_MULTIPLIER: f32 = 2.5;
    pub const TRIPLE_ALPHA_VELOCITY_THRESHOLD: f32 = 0.7;
    pub const TRIPLE_ALPHA_ENERGY_THRESHOLD: f32 = 60.0;

    // Oxygen-16 (alpha capture on carbon)
    pub const OXYGEN16_COLOR: (u8, u8, u8) = (100, 180, 255);
    pub const OXYGEN16_CAPTURE_VELOCITY_THRESHOLD: f32 = 0.6;
    pub const OXYGEN16_BOND_STRENGTH: f32 = 200.0;  // Reduced from 800.0 to allow particles to be further apart
    pub const OXYGEN16_BREAKING_DISTANCE: f32 = 380.0;

    // Water (H2O molecule)
    pub const WATER_COLOR: (u8, u8, u8) = (40, 100, 180);
    pub const WATER_RADIUS_MULTIPLIER: f32 = 3.0;
    pub const WATER_CAPTURE_RANGE: f32 = 45.0;

    // Water hydrogen bonding (polarity-based weak bonds)
    pub const WATER_H_BOND_STRENGTH: f32 = 350.0;  // Increased force to pull molecules together
    pub const WATER_H_BOND_BREAKING_DISTANCE: f32 = 150.0;  // Distance at which bonds break
    pub const WATER_H_BOND_RANGE: f32 = 120.0;  // Shorter detection range for bonding
    pub const WATER_H_BOND_REST_LENGTH: f32 = 75.0;  // Hold molecules ~75 pixels apart
    pub const WATER_EVAPORATION_SPEED: f32 = 80.0;  // Speed at which H2O breaks bonds (evaporates)
    pub const WATER_2BOND_BREAKING_DISTANCE: f32 = 95.0;  // 2-bonded H2O breaks easier (closer distance)

    // Water ice formation (H2O freezing when compressed)
    pub const WATER_ICE_COMPRESSION_DISTANCE: f32 = 85.0;  // Distance at which H2O bonds freeze
    pub const WATER_ICE_MIN_NEIGHBORS: usize = 3;  // Minimum H2O bonds to freeze (3+ bonds = frozen)
    pub const WATER_ICE_FROZEN_BOND_STRENGTH: f32 = 1200.0;  // Very strong rigid bonds when frozen
    pub const WATER_ICE_FROZEN_REST_LENGTH: f32 = 68.0;  // Compressed rest length for ice (tighter)
    pub const WATER_ICE_MELT_SPEED: f32 = 60.0;  // Speed at which ice melts (breaks frozen bonds)
    pub const WATER_ICE_MAX_BONDS: usize = 5;  // All H2O can form up to 5 bonds (3 neg + 2 pos)
    pub const WATER_ICE_REPULSION_STRENGTH: f32 = 4500.0;  // Strong repulsion from 3-5 bonded frozen H2O
    pub const WATER_ICE_REPULSION_RANGE: f32 = 110.0;  // Range of repulsion effect

    // Neon-20 (alpha capture on oxygen)
    pub const NEON20_COLOR: (u8, u8, u8) = (255, 100, 150);
    pub const NEON20_RADIUS_MULTIPLIER: f32 = 2.8;
    pub const NEON20_CAPTURE_VELOCITY_THRESHOLD: f32 = 0.65;

    // Magnesium-24 (alpha capture on neon)
    pub const MAGNESIUM24_COLOR: (u8, u8, u8) = (200, 200, 220);
    pub const MAGNESIUM24_RADIUS_MULTIPLIER: f32 = 3.0;
    pub const MAGNESIUM24_CAPTURE_VELOCITY_THRESHOLD: f32 = 0.68;

    // Silicon-28 (alpha capture on magnesium)
    pub const SILICON28_COLOR: (u8, u8, u8) = (160, 130, 90);
    pub const SILICON28_RADIUS_MULTIPLIER: f32 = 3.2;
    pub const SILICON28_CAPTURE_VELOCITY_THRESHOLD: f32 = 0.70;

    // Sulfur-32 (alpha capture on silicon)
    pub const SULFUR32_COLOR: (u8, u8, u8) = (220, 220, 80);
    pub const SULFUR32_RADIUS_MULTIPLIER: f32 = 3.4;
    pub const SULFUR32_CAPTURE_VELOCITY_THRESHOLD: f32 = 0.72;

    // Hydrogen Sulfide (H2S) - S32 + 2H
    pub const H2S_COLOR: (u8, u8, u8) = (200, 220, 80);  // Yellow-green
    pub const H2S_RADIUS_MULTIPLIER: f32 = 3.2;
    pub const H2S_CAPTURE_RANGE: f32 = 45.0;

    // Magnesium Hydride (MgH2) - Mg24 + 2H
    pub const MGH2_COLOR: (u8, u8, u8) = (180, 180, 190);  // Gray-metallic
    pub const MGH2_RADIUS_MULTIPLIER: f32 = 2.8;
    pub const MGH2_CAPTURE_RANGE: f32 = 45.0;

    // Methane (CH4) - C12 + 4H
    pub const CH4_COLOR: (u8, u8, u8) = (120, 200, 150);  // Pale blue-green
    pub const CH4_RADIUS_MULTIPLIER: f32 = 2.9;
    pub const CH4_CAPTURE_RANGE: f32 = 50.0;

    // Silane (SiH4) - Si28 + 4H
    pub const SIH4_COLOR: (u8, u8, u8) = (220, 100, 50);  // Orange-red
    pub const SIH4_RADIUS_MULTIPLIER: f32 = 3.1;
    pub const SIH4_CAPTURE_RANGE: f32 = 50.0;
}

// ===== PROTON MANAGER PHYSICS =====
pub mod proton_manager {
    pub const REPULSION_RANGE: f32 = 180.0;
    pub const REPULSION_STRENGTH: f32 = 2000.0;
    pub const REPULSION_SAFETY_FACTOR: f32 = 1.0;

    // Charge-based forces
    pub const CHARGE_INTERACTION_RANGE: f32 = 150.0;
    pub const CHARGE_REPULSION_STRENGTH: f32 = 1000.0;
    pub const CHARGE_ATTRACTION_STRENGTH: f32 = 800.0;

    // H (neutral deuterium) clustering forces
    pub const H_ATTRACTION_RANGE: f32 = 1100.0;
    pub const H_ATTRACTION_STRENGTH: f32 = 600.0;

    // He4 clustering forces
    pub const HE4_ATTRACTION_RANGE: f32 = 1420.0;
    pub const HE4_ATTRACTION_STRENGTH: f32 = 500.0;

    // Solid collision parameters
    pub const COLLISION_ELASTICITY: f32 = 0.8;

    pub const ATOM_ATTRACTION_RANGE: f32 = 220.0;
    pub const ATOM_ATTRACTION_STRENGTH: f32 = 15000.0;
    pub const ATOM_REPULSION_STRENGTH: f32 = 8000.0;
    pub const NEUTRON_FORMATION_DISTANCE: f32 = 225.0;

    pub const MIN_ATOM_ENERGY_THRESHOLD: f32 = 30.0;
    pub const MIN_COMBINED_ENERGY: f32 = 10.0;
    pub const COLLISION_THRESHOLD: f32 = 70.0;
    pub const COOLDOWN_DISTANCE: f32 = 10.0;
    pub const SPAWN_COOLDOWN_TIME: f32 = 0.1;
    pub const MAX_SPAWN_SPEED: f32 = 400.0;
    pub const VELOCITY_ENERGY_FACTOR: f32 = 0.5;
    pub const NEGATIVE_PROTON_ENERGY_THRESHOLD: f32 = 600.0;

    pub const FUSION_UPDATE_INTERVAL: i32 = 12;

    // Red wave repulsion for H- protons
    pub const RED_WAVE_REPULSION_STRENGTH: f32 = 5000.0;
    pub const RED_WAVE_INTERACTION_THRESHOLD: f32 = 100.0; // Speed threshold to be "red"
    pub const RED_WAVE_REPULSION_WIDTH: f32 = 15.0; // Thickness of interaction zone

    // Red wave melting for H ice
    pub const DARK_RED_WAVE_SPEED_THRESHOLD: f32 = 30.0; // Only lowest 5 red colors
    pub const RED_WAVE_HITS_TO_MELT: u8 = 5; // Number of hits needed to melt ice
    pub const RED_WAVE_HIT_COOLDOWN: f32 = 0.3; // Cooldown between hits to prevent double-counting

    // H crystallization (phase transitions)
    pub const H_CRYSTAL_MIN_NEIGHBORS: usize = 3; // Minimum H's to crystallize (1 center + 6 sides)
    pub const H_CRYSTAL_NEIGHBOR_DISTANCE: f32 = 80.0; // Max distance to be neighbors
    pub const H_CRYSTAL_MIN_SPACING: f32 = 40.0; // Minimum distance - too close particles cannot bond
    pub const H_CRYSTAL_BOND_STRENGTH: f32 = 70.0; // Spring force between bonded H's (very strong)
    pub const H_CRYSTAL_BOND_REST_LENGTH: f32 = 45.0; // Ideal distance from center to sides
    pub const H_CRYSTAL_VIBRATION_THRESHOLD: f32 = 50.0; // Space needed to start vibrating
    pub const H_CRYSTAL_BREAKOFF_DISTANCE: f32 = 70.0; // Distance at which bonds break
    pub const H_CRYSTAL_FREEZE_COOLDOWN: f32 = 9.0; // Cooldown time before can refreeze after melting
}

// ===== ATOM PHYSICS =====
pub mod atom {
    pub const RADIUS_BASE: f32 = 8.0;
    pub const RADIUS_ENERGY_FACTOR: f32 = 0.1;

    pub const LIFETIME_BASE: f32 = 5.0;
    pub const LIFETIME_ENERGY_FACTOR: f32 = 0.02;
    pub const FADE_START_RATIO: f32 = 0.7;

    pub const PULSE_FREQUENCY_BASE: f32 = 1.8;
    pub const PULSE_FREQUENCY_ENERGY_FACTOR: f32 = 0.06;
    pub const PULSE_INTENSITY_BASE: f32 = 0.3;
    pub const PULSE_INTENSITY_ENERGY_FACTOR: f32 = 0.01;
    pub const SIZE_PULSE_FACTOR: f32 = 0.2;
    pub const SIZE_PULSE_ENERGY_FACTOR: f32 = 0.01;

    pub const ENERGY_DIFFERENCE_AMPLIFICATION: f32 = 0.4;
    pub const COLOR_TOLERANCE: i32 = 8;

    pub const DELTA_TIME_COMPENSATION: f32 = 2.0;

    pub const INTERSECTION_MARGIN: f32 = 50.0;
    pub const CLEANUP_INTERVAL: i32 = 600;
}

// ===== RING PHYSICS =====
pub mod ring {
    pub const COLOR_WEIGHT_RED: f32 = 0.1;
    pub const COLOR_WEIGHT_GREEN: f32 = 0.3;
    pub const COLOR_WEIGHT_BLUE: f32 = 0.6;
    pub const COLOR_DIVISOR: f32 = 255.0;
    pub const MIN_SPEED: f32 = 15.0;
    pub const MAX_SPEED: f32 = 200.0;

    pub const INITIAL_RADIUS: f32 = 5.0;
    pub const RESET_RADIUS: f32 = 5.0;
    pub const MAX_RADIUS_THRESHOLD: f32 = 2000.0;
    pub const DEFAULT_THICKNESS: f32 = 6.0;

    pub const BOUNCE_REFLECTION_OPACITY: f32 = 0.7;
    pub const ALPHA_CALCULATION_DIVISOR: f32 = 800.0;
    pub const MINIMUM_ALPHA: f32 = 0.1;

    pub const CULL_MARGIN: f32 = 100.0;
    pub const OFF_SCREEN_MARGIN: f32 = 500.0;
    pub const WINDOW_WIDTH_MULTIPLIER: f32 = 2.0;
    pub const WINDOW_HEIGHT_MULTIPLIER: f32 = 2.0;

    pub const LOW_FREQUENCY_THRESHOLD: f32 = 100.0;
    pub const MEDIUM_FREQUENCY_THRESHOLD: f32 = 250.0;
}

// ===== SPATIAL GRID OPTIMIZATION =====
pub mod spatial_grid {
    pub const DEFAULT_CELL_SIZE: f32 = 200.0;
    pub const VIEWPORT_MARGIN: f32 = 200.0;
    pub const NEAR_VIEWPORT_MARGIN: f32 = 200.0;
    pub const GRID_MARGIN_CELLS: i32 = 4;
    pub const POTENTIAL_INTERSECTIONS_RESERVE: usize = 32;
}

// ===== RENDERING =====
pub mod rendering {
    pub const VERTEX_RESERVE_SIZE: usize = 10000;
}

// ===== EVENTS =====
pub mod events {
    pub const NEW_SHAPE_RADIUS: f32 = 10.0;
}

// ===== RING CONSTANTS (Top-level exports for convenience) =====
pub const COLOR_WEIGHT_RED: f32 = ring::COLOR_WEIGHT_RED;
pub const COLOR_WEIGHT_GREEN: f32 = ring::COLOR_WEIGHT_GREEN;
pub const COLOR_WEIGHT_BLUE: f32 = ring::COLOR_WEIGHT_BLUE;
pub const MIN_RING_SPEED: f32 = ring::MIN_SPEED;
pub const MAX_RING_SPEED: f32 = ring::MAX_SPEED;
pub const INITIAL_RING_RADIUS: f32 = ring::INITIAL_RADIUS;
pub const RESET_RING_RADIUS: f32 = ring::RESET_RADIUS;
pub const MAX_RADIUS_THRESHOLD: f32 = ring::MAX_RADIUS_THRESHOLD;
pub const DEFAULT_RING_THICKNESS: f32 = ring::DEFAULT_THICKNESS;
pub const BOUNCE_REFLECTION_OPACITY: f32 = ring::BOUNCE_REFLECTION_OPACITY;
pub const ALPHA_CALCULATION_DIVISOR: f32 = ring::ALPHA_CALCULATION_DIVISOR;
pub const MINIMUM_ALPHA: f32 = ring::MINIMUM_ALPHA;
pub const CULL_MARGIN: f32 = ring::CULL_MARGIN;
pub const OFF_SCREEN_MARGIN: f32 = ring::OFF_SCREEN_MARGIN;
pub const WINDOW_WIDTH_MULTIPLIER: f32 = ring::WINDOW_WIDTH_MULTIPLIER;
pub const WINDOW_HEIGHT_MULTIPLIER: f32 = ring::WINDOW_HEIGHT_MULTIPLIER;
pub const LOW_FREQUENCY_THRESHOLD: f32 = ring::LOW_FREQUENCY_THRESHOLD;
pub const MEDIUM_FREQUENCY_THRESHOLD: f32 = ring::MEDIUM_FREQUENCY_THRESHOLD;

// ===== RING COLOR PALETTE =====
pub const RING_COLORS: [Color; 35] = [
    Color::new(0.17, 0.00, 0.00, 1.0),  // Darkest red
    Color::new(0.31, 0.00, 0.00, 1.0),
    Color::new(0.47, 0.00, 0.00, 1.0),
    Color::new(0.63, 0.00, 0.00, 1.0),
    Color::new(0.78, 0.00, 0.00, 1.0),
    Color::new(1.00, 0.00, 0.00, 1.0),  // Pure red
    Color::new(1.00, 0.20, 0.00, 1.0),
    Color::new(1.00, 0.39, 0.00, 1.0),
    Color::new(1.00, 0.59, 0.00, 1.0),
    Color::new(1.00, 0.78, 0.00, 1.0),
    Color::new(1.00, 1.00, 0.00, 1.0),  // Yellow
    Color::new(0.78, 1.00, 0.00, 1.0),
    Color::new(0.59, 1.00, 0.00, 1.0),
    Color::new(0.39, 1.00, 0.00, 1.0),
    Color::new(0.20, 1.00, 0.00, 1.0),
    Color::new(0.00, 1.00, 0.00, 1.0),  // Pure green
    Color::new(0.00, 1.00, 0.20, 1.0),
    Color::new(0.00, 1.00, 0.39, 1.0),
    Color::new(0.00, 1.00, 0.59, 1.0),
    Color::new(0.00, 1.00, 0.78, 1.0),
    Color::new(0.00, 1.00, 1.00, 1.0),  // Cyan
    Color::new(0.00, 0.78, 1.00, 1.0),
    Color::new(0.00, 0.59, 1.00, 1.0),
    Color::new(0.00, 0.39, 1.00, 1.0),
    Color::new(0.00, 0.20, 1.00, 1.0),
    Color::new(0.00, 0.00, 1.00, 1.0),  // Pure blue
    Color::new(0.20, 0.00, 1.00, 1.0),
    Color::new(0.39, 0.00, 1.00, 1.0),
    Color::new(0.59, 0.00, 1.00, 1.0),
    Color::new(0.78, 0.00, 1.00, 1.0),
    Color::new(1.00, 0.00, 1.00, 1.0),  // Magenta
    Color::new(1.00, 0.39, 1.00, 1.0),
    Color::new(1.00, 0.59, 1.00, 1.0),
    Color::new(1.00, 0.78, 1.00, 1.0),
    Color::new(1.00, 1.00, 1.00, 1.0),  // White (fastest)
];
