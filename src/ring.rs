// Ring module - Energy wave propagation with bouncing and frequency-based speed
// Rust port of Ring.h/cpp

use macroquad::prelude::*;
use crate::constants::*;

#[derive(Debug, Clone)]
struct BounceData {
    has_bounced_left: bool,
    has_bounced_right: bool,
    has_bounced_top: bool,
    has_bounced_bottom: bool,
    max_radius: f32,
}

impl Default for BounceData {
    fn default() -> Self {
        Self {
            has_bounced_left: false,
            has_bounced_right: false,
            has_bounced_top: false,
            has_bounced_bottom: false,
            max_radius: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct BounceShape {
    center: Vec2,
    color: Color,
}

#[derive(Debug)]
pub struct Ring {
    center: Vec2,
    original_center: Vec2,
    current_radius: f32,
    growth_speed: f32,
    color: Color,
    is_alive: bool,
    thickness: f32,
    bounce_data: BounceData,
    bounce_shapes: Vec<BounceShape>,
}

impl Ring {
    /// Calculate growth speed based on light frequency
    /// Blue dominant = fastest, red dominant = slowest
    pub fn calculate_frequency_based_speed(color: Color) -> f32 {
        // macroquad colors are already normalized 0.0-1.0, don't divide by 255
        let speed_factor = color.r * COLOR_WEIGHT_RED
            + color.g * COLOR_WEIGHT_GREEN
            + color.b * COLOR_WEIGHT_BLUE;

        // Map to speed range
        MIN_RING_SPEED + (speed_factor * (MAX_RING_SPEED - MIN_RING_SPEED))
    }

    /// Create a new ring at the given position with the specified color
    pub fn new(center: Vec2, color: Color, thickness: f32) -> Self {
        let growth_speed = Self::calculate_frequency_based_speed(color);

        Self {
            center,
            original_center: center,
            current_radius: INITIAL_RING_RADIUS,
            growth_speed,
            color,
            is_alive: true,
            thickness,
            bounce_data: BounceData::default(),
            bounce_shapes: Vec::new(),
        }
    }

    /// Update the ring (growth and bouncing)
    pub fn update(&mut self, delta_time: f32, window_size: (f32, f32)) {
        if !self.is_alive {
            return;
        }

        // Grow the ring
        self.current_radius += self.growth_speed * delta_time;

        // Update bounce shapes and reflections
        self.update_bounce_shapes(window_size);

        // Kill ring when it gets too large
        if self.current_radius > MAX_RADIUS_THRESHOLD {
            self.is_alive = false;
            return;
        }

        // Kill ring early if center is far off-screen
        let (window_width, window_height) = window_size;
        if self.center.x < -OFF_SCREEN_MARGIN
            || self.center.x > window_width + OFF_SCREEN_MARGIN
            || self.center.y < -OFF_SCREEN_MARGIN
            || self.center.y > window_height + OFF_SCREEN_MARGIN
        {
            self.is_alive = false;
            return;
        }

        // Fade out as ring gets bigger
        let alpha = (COLOR_MAX
            * (MINIMUM_ALPHA.max(1.0 - self.current_radius / ALPHA_CALCULATION_DIVISOR)))
            as u8;
        self.color.a = alpha as f32 / 255.0;
    }

    /// Update bounce shapes for wall reflections
    fn update_bounce_shapes(&mut self, window_size: (f32, f32)) {
        self.bounce_shapes.clear();

        let (window_width, window_height) = window_size;

        let left_edge = self.original_center.x - self.current_radius;
        let right_edge = self.original_center.x + self.current_radius;
        let top_edge = self.original_center.y - self.current_radius;
        let bottom_edge = self.original_center.y + self.current_radius;

        // Track maximum radius for fading effect
        self.bounce_data.max_radius = self.bounce_data.max_radius.max(self.current_radius);

        // Calculate bounce color with reduced opacity
        let bounce_color = Color::new(
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a * BOUNCE_REFLECTION_OPACITY,
        );

        // Culling margin
        let cull_margin = self.current_radius + CULL_MARGIN;

        // Helper closure to check if a bounce shape center would be near the screen
        let is_near_screen = |x: f32, y: f32| -> bool {
            x + self.current_radius >= -cull_margin
                && x - self.current_radius <= window_width + cull_margin
                && y + self.current_radius >= -cull_margin
                && y - self.current_radius <= window_height + cull_margin
        };

        // Left wall bounce
        if left_edge <= 0.0 && !self.bounce_data.has_bounced_left {
            self.bounce_data.has_bounced_left = true;
        }
        if self.bounce_data.has_bounced_left {
            let reflected_x = -self.original_center.x;
            if is_near_screen(reflected_x, self.original_center.y) {
                self.bounce_shapes.push(BounceShape {
                    center: vec2(reflected_x, self.original_center.y),
                    color: bounce_color,
                });
            }
        }

        // Right wall bounce
        if right_edge >= window_width && !self.bounce_data.has_bounced_right {
            self.bounce_data.has_bounced_right = true;
        }
        if self.bounce_data.has_bounced_right {
            let reflected_x = WINDOW_WIDTH_MULTIPLIER * window_width - self.original_center.x;
            if is_near_screen(reflected_x, self.original_center.y) {
                self.bounce_shapes.push(BounceShape {
                    center: vec2(reflected_x, self.original_center.y),
                    color: bounce_color,
                });
            }
        }

        // Top wall bounce
        if top_edge <= 0.0 && !self.bounce_data.has_bounced_top {
            self.bounce_data.has_bounced_top = true;
        }
        if self.bounce_data.has_bounced_top {
            let reflected_y = -self.original_center.y;
            if is_near_screen(self.original_center.x, reflected_y) {
                self.bounce_shapes.push(BounceShape {
                    center: vec2(self.original_center.x, reflected_y),
                    color: bounce_color,
                });
            }
        }

        // Bottom wall bounce
        if bottom_edge >= window_height && !self.bounce_data.has_bounced_bottom {
            self.bounce_data.has_bounced_bottom = true;
        }
        if self.bounce_data.has_bounced_bottom {
            let reflected_y = WINDOW_HEIGHT_MULTIPLIER * window_height - self.original_center.y;
            if is_near_screen(self.original_center.x, reflected_y) {
                self.bounce_shapes.push(BounceShape {
                    center: vec2(self.original_center.x, reflected_y),
                    color: bounce_color,
                });
            }
        }
    }

    /// Draw the ring and all bounce reflections
    pub fn render(&self, segments: u8) {
        if !self.is_alive {
            return;
        }

        // Draw main ring
        draw_circle_lines(
            self.center.x,
            self.center.y,
            self.current_radius,
            self.thickness,
            self.color,
        );

        // Alternative: draw as hollow circle with segments for better quality
        if segments > 0 {
            draw_poly_lines(
                self.center.x,
                self.center.y,
                segments,
                self.current_radius,
                0.0,
                self.thickness,
                self.color,
            );
        }

        // Draw all bounce reflections
        for bounce_shape in &self.bounce_shapes {
            draw_poly_lines(
                bounce_shape.center.x,
                bounce_shape.center.y,
                segments,
                self.current_radius,
                0.0,
                self.thickness,
                bounce_shape.color,
            );
        }
    }

    // Getters
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn get_radius(&self) -> f32 {
        self.current_radius
    }

    pub fn get_center(&self) -> Vec2 {
        self.center
    }

    pub fn get_growth_speed(&self) -> f32 {
        self.growth_speed
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Set new color and recalculate speed
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.growth_speed = Self::calculate_frequency_based_speed(color);
    }

    /// Reset ring to new position
    pub fn reset(&mut self, new_center: Vec2) {
        self.center = new_center;
        self.original_center = new_center;
        self.current_radius = RESET_RING_RADIUS;
        self.is_alive = true;
        self.bounce_data = BounceData::default();
        self.bounce_shapes.clear();
        self.growth_speed = Self::calculate_frequency_based_speed(self.color);
    }

    /// Get bounce shape center for intersection detection
    pub fn get_bounce_shape_center(&self, index: i32) -> Vec2 {
        if index == -1 {
            return self.center;
        }

        if index >= 0 && (index as usize) < self.bounce_shapes.len() {
            return self.bounce_shapes[index as usize].center;
        }

        self.center // Fallback
    }

    pub fn get_bounce_shape_count(&self) -> usize {
        self.bounce_shapes.len()
    }
}

/// RingManager - Manages lifecycle of all rings
pub struct RingManager {
    rings: Vec<Ring>,
    colors: Vec<Color>,
    current_color: Color,
    current_color_index: usize,
}

impl RingManager {
    pub fn new() -> Self {
        let colors = RING_COLORS.to_vec();
        let current_color = colors[0];

        Self {
            rings: Vec::new(),
            colors,
            current_color,
            current_color_index: 0,
        }
    }

    /// Add a new ring at the given position
    pub fn add_ring(&mut self, position: Vec2) {
        self.rings
            .push(Ring::new(position, self.current_color, DEFAULT_RING_THICKNESS));
    }

    /// Add an energy-based colored ring (red=low energy, white=high energy)
    pub fn add_energy_ring(&mut self, position: Vec2, energy: f32) {
        // Map energy to color: 0-100 = red to white
        let normalized = (energy / 100.0).clamp(0.0, 1.0);

        // Red (low) to white (high)
        let color = Color::new(1.0, normalized, normalized, 1.0);

        self.rings
            .push(Ring::new(position, color, DEFAULT_RING_THICKNESS));
    }

    /// Add a ring with a custom color
    pub fn add_ring_with_color(&mut self, position: Vec2, color: Color) {
        self.rings
            .push(Ring::new(position, color, DEFAULT_RING_THICKNESS));
    }

    /// Update all rings
    pub fn update(&mut self, delta_time: f32, window_size: (f32, f32)) {
        // Update all rings
        for ring in &mut self.rings {
            ring.update(delta_time, window_size);
        }

        // Remove dead rings
        self.rings.retain(|ring| ring.is_alive());
    }

    /// Draw all rings
    pub fn draw(&self, segments: u8) {
        for ring in &self.rings {
            ring.render(segments);
        }
    }

    /// Clear all rings
    pub fn clear(&mut self) {
        self.rings.clear();
    }

    /// Get ring count
    pub fn get_ring_count(&self) -> usize {
        self.rings.len()
    }

    /// Get all rings (for intersection detection)
    pub fn get_all_rings(&self) -> &[Ring] {
        &self.rings
    }

    /// Cycle to next color in the palette
    pub fn cycle_to_next_color(&mut self) {
        self.current_color_index = (self.current_color_index + 1) % self.colors.len();
        self.current_color = self.colors[self.current_color_index];
    }

    /// Cycle to previous color in the palette
    pub fn cycle_to_previous_color(&mut self) {
        if self.current_color_index == 0 {
            self.current_color_index = self.colors.len() - 1;
        } else {
            self.current_color_index -= 1;
        }
        self.current_color = self.colors[self.current_color_index];
    }

    /// Get current color
    pub fn get_current_color(&self) -> Color {
        self.current_color
    }

    /// Get current color index
    pub fn get_current_color_index(&self) -> usize {
        self.current_color_index
    }

    /// Set color by index
    pub fn set_color_by_index(&mut self, index: usize) {
        if index < self.colors.len() {
            self.current_color_index = index;
            self.current_color = self.colors[index];
        }
    }

    /// Get current color as a string
    pub fn get_current_color_string(&self) -> String {
        format!(
            "RGB({}, {}, {})",
            (self.current_color.r * 255.0) as u8,
            (self.current_color.g * 255.0) as u8,
            (self.current_color.b * 255.0) as u8
        )
    }

    /// Get frequency info for current color
    pub fn get_current_frequency_info(&self) -> String {
        let speed = Ring::calculate_frequency_based_speed(self.current_color);
        let color_str = self.get_current_color_string();

        let freq_desc = if speed < LOW_FREQUENCY_THRESHOLD {
            "Low frequency"
        } else if speed < MEDIUM_FREQUENCY_THRESHOLD {
            "Medium frequency"
        } else {
            "High frequency"
        };

        format!("{} - Speed: {:.1} px/s ({})", color_str, speed, freq_desc)
    }
}
