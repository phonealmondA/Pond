use macroquad::prelude::*;

// =============================================================================
// VISUAL/RENDERING CONSTANTS
// =============================================================================

// Color constants for rendering
pub const LIPID_TAIL_COLOR: Color = Color::new(0.8, 0.6, 0.2, 1.0);  // Hydrophobic tail (orange/yellow)
pub const LIPID_HEAD_COLOR: Color = Color::new(0.3, 0.6, 1.0, 1.0);  // Hydrophilic head (blue)
pub const HEAD_ZONE_COLOR: Color = Color::new(1.0, 0.0, 0.0, 0.1);  // Head zone fill (transparent red)
pub const HEAD_ZONE_BORDER_COLOR: Color = Color::new(1.0, 0.0, 0.0, 0.3);  // Head zone border
pub const EXPANSION_ZONE_COLOR: Color = Color::new(0.0, 0.5, 1.0, 0.05);  // Expansion zone fill (transparent blue)
pub const EXPANSION_ZONE_BORDER_COLOR: Color = Color::new(0.0, 0.5, 1.0, 0.2);  // Expansion zone border

// Lipid component visuals
pub const LIPID_CIRCLE_RADIUS: f32 = 3.5;  // Radius of the lipid head (hydrophilic)
pub const LIPID_BAR_LENGTH: f32 = 4.4;  // Length of the lipid tail (hydrophobic)
pub const LIPID_BAR_WIDTH: f32 = 3.0;  // Width of the lipid tail

// Visual element sizes
pub const CENTER_MARKER_RADIUS: f32 = 3.0;  // Radius of center point markers
pub const LIPID_HEAD_OUTLINE_WIDTH: f32 = 0.1;  // Line width for lipid head circle outline
pub const EXPANSION_ZONE_BORDER_WIDTH: f32 = 1.5;  // Line width for expansion zone border
pub const HEAD_ZONE_BORDER_WIDTH: f32 = 1.0;  // Line width for head zone border

// =============================================================================
// SCREEN SETTINGS
// =============================================================================

pub const SCREEN_WIDTH: f32 = 1200.0;
pub const SCREEN_HEIGHT: f32 = 800.0;

// =============================================================================
// MEMBRANE STRUCTURE
// =============================================================================

pub const INNER_MEMBRANE_RADIUS: f32 = 100.0;
pub const OUTER_MEMBRANE_RADIUS: f32 = 110.0;  // 5x closer to inner membrane (was 125.0)
pub const NUM_MEMBRANE_COMPONENTS: usize = 60;  // Number of lipid components in each membrane layer

// Ideal spacing between neighbors - calculated based on circumference / component count
pub const INNER_DESIRED_NEIGHBOR_DISTANCE: f32 = 6.28;
pub const OUTER_DESIRED_NEIGHBOR_DISTANCE: f32 = 6.41;

// =============================================================================
// HEAD/CORE PHYSICS
// =============================================================================

pub const HEAD_RADIUS: f32 = 40.0;  // Radius of the head zone around center (80.0 / 5)
pub const HEAD_PUSH_FORCE: f32 = 1500.0;  // How strongly the head pushes membrane components outward
pub const HEAD_ACCELERATION: f32 = 200.0;    // Head moves faster than center to lead
pub const HEAD_DAMPING: f32 = 0.96;          // Head damping
pub const HEAD_MAX_SPEED: f32 = 100.0;       // Maximum head velocity
pub const HEAD_STATIONARY_THRESHOLD: f32 = 20.0;  // Velocity below which head is considered stationary

// =============================================================================
// CENTER PHYSICS
// =============================================================================

pub const CENTER_FOLLOW_STRENGTH: f32 = 5.0; // How strongly center is pulled toward head
pub const CENTER_DAMPING: f32 = 0.95;        // Center velocity damping (more damping for biological feel)
pub const CENTER_MAX_SPEED: f32 = 150.0;     // Maximum center velocity (reduced for slower, cell-like motion)

// =============================================================================
// MEMBRANE PHYSICS
// =============================================================================

pub const DAMPING: f32 = 0.85;             // Velocity damping (0.0 = no damping, 1.0 = instant stop)
pub const NEIGHBOR_FORCE_STRENGTH: f32 = 2000.0;  // How strongly neighbors push/pull each other (high value makes membrane act like a continuous string)
pub const MEMBRANE_SEPARATION_FORCE: f32 = 1000.0;  // How strongly the inner and outer membranes repel each other to maintain minimum distance
pub const MEMBRANE_ALIGNMENT_FORCE: f32 = 2500.0;  // How strongly paired inner/outer components stay angularly aligned

// =============================================================================
// MEMBRANE FLOW
// =============================================================================

pub const MEMBRANE_FORWARD_FLOW_STRENGTH: f32 = 280.0;  // How strongly rear components flow forward during movement
pub const MEMBRANE_FLOW_SPEED: f32 = 0.5;  // Flow speed multiplier for membrane surface flow
pub const FLOW_DISTANCE_NORMALIZER: f32 = 1.0;  // Distance used to normalize flow factor (0-1 range)
pub const MAX_FLOW_FACTOR: f32 = 1.0;  // Maximum flow factor for membrane migration
pub const MOVEMENT_DIRECTION_THRESHOLD: f32 = 0.1;  // Minimum velocity to calculate movement direction

// =============================================================================
// EXPANSION/REFORMATION
// =============================================================================

pub const EXPANSION_GROWTH_RATE: f32 = 30.0;  // How fast the expansion radius grows (units per second)
pub const EXPANSION_PUSH_FORCE: f32 = 800.0;   // How strongly the expansion zone pushes membrane outward
pub const EXPANSION_INITIAL_RADIUS: f32 = 40.0;  // Starting radius when expansion begins
pub const EXPANSION_PERSIST_TIME: f32 = 1.5;  // How long expansion zone stays active after movement starts (seconds)
pub const STATIONARY_DELAY: f32 = 0.001;     // Seconds head must be stationary before reforming to circle
