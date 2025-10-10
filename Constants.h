#pragma once
#include <SFML/Graphics.hpp>

// Centralized constants for the Pond physics simulation
// All magic numbers and configuration values are defined here for easy tuning

namespace Constants
{
    // ===== SYSTEM LIMITS =====
    namespace System
    {
        constexpr size_t MAX_PROTONS = 75;
        constexpr size_t MAX_ATOMS = 35;
        constexpr int CIRCLE_SEGMENTS = 24;  // Number of segments for circle approximation
        constexpr int COLOR_PALETTE_SIZE = 35;  // Number of predefined ring colors
        constexpr int COLOR_CYCLE_SIZE = 6;  // Number of colors in right-click cycle
    }

    // ===== MATHEMATICAL CONSTANTS =====
    namespace Math
    {
        constexpr float PI = 3.14159265f;
        constexpr float EPSILON = 0.001f;  // For floating-point comparisons
        constexpr float COLOR_MAX = 255.0f;  // Maximum RGB color value
    }

    // ===== PROTON PHYSICS =====
    namespace Proton
    {
        // Movement
        constexpr float FRICTION = 1.0f;  // No friction - simulates vacuum
        constexpr float BOUNCE_DAMPENING = 0.7f;  // Energy retention on wall bounce

        // Size
        constexpr float MIN_RADIUS = 3.0f;
        constexpr float MAX_RADIUS = 8.0f;
        constexpr float ENERGY_TO_RADIUS_FACTOR = 0.01f;

        // Mass and Energy
        constexpr float ENERGY_TO_MASS_FACTOR = 0.1f;  // E=mc² approximation

        // Lifetime
        constexpr float DEFAULT_LIFETIME = 20.0f;  // Seconds
        constexpr float FADE_START_RATIO = 0.8f;  // Start fading at 80% of lifetime
        constexpr float INFINITE_LIFETIME = -1.0f;  // For stable hydrogen

        // Visual Effects
        constexpr float PULSE_FREQUENCY_BASE = 2.0f;
        constexpr float PULSE_FREQUENCY_ENERGY_FACTOR = 0.01f;
        constexpr float PULSE_INTENSITY = 0.2f;
        constexpr float PULSE_BASE = 1.0f;
        constexpr float STABLE_HYDROGEN_RADIUS_MULTIPLIER = 1.3f;
        constexpr float BARE_PROTON_RED_TINT = 1.2f;
        constexpr float GLOW_LAYER1_RADIUS = 1.5f;
        constexpr float GLOW_LAYER1_ALPHA = 0.5f;
        constexpr float GLOW_LAYER2_RADIUS = 2.0f;
        constexpr float GLOW_LAYER2_ALPHA = 0.25f;

        // Colors
        constexpr int STABLE_HYDROGEN_R = 255;
        constexpr int STABLE_HYDROGEN_G = 255;
        constexpr int STABLE_HYDROGEN_B = 255;
        constexpr int NEUTRAL_PROTON_R = 200;
        constexpr int NEUTRAL_PROTON_G = 200;
        constexpr int NEUTRAL_PROTON_B = 200;

        // Neutron Formation
        constexpr float NEUTRON_FORMATION_TIME = 0.50f;  // Seconds near atom
        constexpr float NEUTRON_RADIUS_MULTIPLIER = 1.2f;

        // Electron Capture
        constexpr float ELECTRON_CAPTURE_DISTANCE = 15.0f;

        // Negative Proton Decay
        constexpr float NEGATIVE_DECAY_TIME = 5.0f;

        // Fusion thresholds
        constexpr float DEUTERIUM_FUSION_VELOCITY_THRESHOLD = 30.0f;  // Speed needed for D + H fusion (reduced from 50)
        constexpr float HELIUM3_FUSION_VELOCITY_THRESHOLD = 60.0f;    // Speed needed for He3 + He3 fusion (reduced from 100)
        constexpr float FUSION_ENERGY_RELEASE = 80.0f;                 // Energy released as wave

        // Helium colors
        constexpr int HELIUM3_COLOR_R = 255;
        constexpr int HELIUM3_COLOR_G = 200;
        constexpr int HELIUM3_COLOR_B = 100;
        constexpr int HELIUM4_COLOR_R = 255;
        constexpr int HELIUM4_COLOR_G = 255;
        constexpr int HELIUM4_COLOR_B = 100;

        constexpr float HELIUM3_RADIUS_MULTIPLIER = 1.5f;
        constexpr float HELIUM4_RADIUS_MULTIPLIER = 1.8f;
    }

    // ===== PROTON MANAGER PHYSICS =====
    namespace ProtonManager
    {
        // Proton-Proton Interactions
        constexpr float REPULSION_RANGE = 80.0f;
        constexpr float REPULSION_STRENGTH = 8000.0f;
        constexpr float REPULSION_SAFETY_FACTOR = 1.0f;  // Avoid division by zero

        // Atom Interactions
        constexpr float ATOM_ATTRACTION_RANGE = 220.0f;
        constexpr float ATOM_ATTRACTION_STRENGTH = 15000.0f;
        constexpr float ATOM_REPULSION_STRENGTH = 8000.0f;
        constexpr float NEUTRON_FORMATION_DISTANCE = 225.0f;

        // Spawning from Atom Collisions
        constexpr float MIN_ATOM_ENERGY_THRESHOLD = 150.0f;
        constexpr float MIN_COMBINED_ENERGY = 100.0f;
        constexpr float COLLISION_THRESHOLD = 15.0f;
        constexpr float COOLDOWN_DISTANCE = 20.0f;
        constexpr float SPAWN_COOLDOWN_TIME = 0.5f;  // Seconds
        constexpr float MAX_SPAWN_SPEED = 200.0f;
        constexpr float VELOCITY_ENERGY_FACTOR = 0.5f;
        constexpr float NEGATIVE_PROTON_ENERGY_THRESHOLD = 400.0f;
    }

    // ===== ATOM PHYSICS =====
    namespace Atom
    {
        // Size
        constexpr float RADIUS_BASE = 1.8f;
        constexpr float RADIUS_ENERGY_FACTOR = 0.025f;

        // Lifetime
        constexpr float LIFETIME_BASE = 5.0f;  // Seconds
        constexpr float LIFETIME_ENERGY_FACTOR = 0.02f;
        constexpr float FADE_START_RATIO = 0.7f;

        // Visual Effects
        constexpr float PULSE_FREQUENCY_BASE = 1.8f;
        constexpr float PULSE_FREQUENCY_ENERGY_FACTOR = 0.06f;
        constexpr float PULSE_INTENSITY_BASE = 0.3f;
        constexpr float PULSE_INTENSITY_ENERGY_FACTOR = 0.01f;
        constexpr float SIZE_PULSE_FACTOR = 0.2f;
        constexpr float SIZE_PULSE_ENERGY_FACTOR = 0.01f;

        // Interference
        constexpr float ENERGY_DIFFERENCE_AMPLIFICATION = 0.4f;
        constexpr int COLOR_TOLERANCE = 8;

        // Update Rate
        constexpr float DELTA_TIME_COMPENSATION = 2.0f;  // Half update rate compensation

        // Intersection Detection
        constexpr float INTERSECTION_MARGIN = 50.0f;
        constexpr int CLEANUP_INTERVAL = 600;  // Frames (~10 seconds at 60 FPS)
    }

    // ===== RING PHYSICS =====
    namespace Ring
    {
        // Speed Calculation
        constexpr float COLOR_WEIGHT_RED = 0.1f;
        constexpr float COLOR_WEIGHT_GREEN = 0.3f;
        constexpr float COLOR_WEIGHT_BLUE = 0.6f;
        constexpr float COLOR_DIVISOR = 255.0f;
        constexpr float MIN_SPEED = 20.0f;  // Pixels per second
        constexpr float MAX_SPEED = 120.0f;

        // Size
        constexpr float INITIAL_RADIUS = 5.0f;
        constexpr float RESET_RADIUS = 5.0f;
        constexpr float MAX_RADIUS_THRESHOLD = 2000.0f;
        constexpr float DEFAULT_THICKNESS = 3.0f;

        // Rendering
        constexpr float BOUNCE_REFLECTION_OPACITY = 0.7f;
        constexpr float ALPHA_CALCULATION_DIVISOR = 800.0f;
        constexpr float MINIMUM_ALPHA = 0.1f;

        // Culling
        constexpr float CULL_MARGIN = 100.0f;
        constexpr float OFF_SCREEN_MARGIN = 500.0f;
        constexpr float WINDOW_WIDTH_MULTIPLIER = 2.0f;  // For bounce reflection
        constexpr float WINDOW_HEIGHT_MULTIPLIER = 2.0f;

        // Frequency Classification
        constexpr float LOW_FREQUENCY_THRESHOLD = 40.0f;
        constexpr float MEDIUM_FREQUENCY_THRESHOLD = 80.0f;
    }

    // ===== SPATIAL GRID OPTIMIZATION =====
    namespace SpatialGrid
    {
        constexpr float DEFAULT_CELL_SIZE = 200.0f;
        constexpr float VIEWPORT_MARGIN = 200.0f;
        constexpr float NEAR_VIEWPORT_MARGIN = 200.0f;
        constexpr int GRID_MARGIN_CELLS = 4;
        constexpr size_t POTENTIAL_INTERSECTIONS_RESERVE = 32;
    }

    // ===== RENDERING =====
    namespace Rendering
    {
        constexpr size_t VERTEX_RESERVE_SIZE = 10000;
    }

    // ===== EVENTS =====
    namespace Events
    {
        constexpr float NEW_SHAPE_RADIUS = 10.0f;  // Radius on click reset
    }

    // ===== RING COLOR PALETTE =====
    // 35 predefined RGB colors ordered from lowest to highest frequency/speed
    namespace RingColors
    {
        inline const sf::Color COLORS[System::COLOR_PALETTE_SIZE] = {
            sf::Color(44, 0, 0),      // Darkest red (slowest)
            sf::Color(80, 0, 0),
            sf::Color(120, 0, 0),
            sf::Color(160, 0, 0),
            sf::Color(200, 0, 0),
            sf::Color(255, 0, 0),     // Pure red
            sf::Color(255, 50, 0),
            sf::Color(255, 100, 0),
            sf::Color(255, 150, 0),
            sf::Color(255, 200, 0),
            sf::Color(255, 255, 0),   // Yellow
            sf::Color(200, 255, 0),
            sf::Color(150, 255, 0),
            sf::Color(100, 255, 0),
            sf::Color(50, 255, 0),
            sf::Color(0, 255, 0),     // Pure green
            sf::Color(0, 255, 50),
            sf::Color(0, 255, 100),
            sf::Color(0, 255, 150),
            sf::Color(0, 255, 200),
            sf::Color(0, 255, 255),   // Cyan
            sf::Color(0, 200, 255),
            sf::Color(0, 150, 255),
            sf::Color(0, 100, 255),
            sf::Color(0, 50, 255),
            sf::Color(0, 0, 255),     // Pure blue
            sf::Color(50, 0, 255),
            sf::Color(100, 0, 255),
            sf::Color(150, 0, 255),
            sf::Color(200, 0, 255),
            sf::Color(255, 0, 255),   // Magenta
            sf::Color(255, 100, 255),
            sf::Color(255, 150, 255),
            sf::Color(255, 200, 255),
            sf::Color(255, 255, 255)  // White (fastest)
        };
    }
}
