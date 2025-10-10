// Proton - Direct port from Proton.h/cpp
// Rare, persistent physics particle with nuclear fusion capabilities

use macroquad::prelude::*;
use crate::constants::*;
use crate::constants::proton as pc;

#[derive(Clone)]
pub struct Proton {
    position: Vec2,
    velocity: Vec2,
    color: Color,
    energy: f32,
    radius: f32,
    mass: f32,
    is_alive: bool,
    marked_for_deletion: bool,
    lifetime: f32,
    max_lifetime: f32,

    // Visual effects
    pulse_timer: f32,
    fade_start_time: f32,

    // Charge state system
    charge: i32,
    neutron_count: i32,
    is_stable_hydrogen: bool,
    wave_field_timer: f32,

    // Sleeping system for optimization
    is_sleeping: bool,
}

impl Proton {
    pub fn new(position: Vec2, velocity: Vec2, color: Color, energy: f32, charge: i32) -> Self {
        let radius = Self::calculate_radius(energy);
        let mass = Self::calculate_mass(energy);
        let max_lifetime = pc::DEFAULT_LIFETIME;
        let fade_start_time = max_lifetime * pc::FADE_START_RATIO;

        Self {
            position,
            velocity,
            color,
            energy,
            radius,
            mass,
            is_alive: true,
            marked_for_deletion: false,
            lifetime: 0.0,
            max_lifetime,
            pulse_timer: 0.0,
            fade_start_time,
            charge,
            neutron_count: 0,
            is_stable_hydrogen: false,
            wave_field_timer: 0.0,
            is_sleeping: false,
        }
    }

    pub fn update(&mut self, delta_time: f32, window_size: (f32, f32)) {
        if !self.is_alive {
            return;
        }

        // Always update visual pulse
        self.pulse_timer += delta_time;

        // SLEEPING OPTIMIZATION
        if self.is_stable_hydrogen || self.is_stable_helium4() {
            let velocity_magnitude = self.velocity.length();

            if velocity_magnitude < 1.0 {
                self.is_sleeping = true;
                return; // Skip all physics
            } else {
                self.is_sleeping = false;
            }
        }

        // Update lifetime
        self.lifetime += delta_time;

        // Check death from age
        if self.max_lifetime >= 0.0 && self.lifetime >= self.max_lifetime {
            self.is_alive = false;
            return;
        }

        // Straight-line movement
        self.position += self.velocity * delta_time;

        // Boundary collisions
        self.handle_boundary_collision(window_size);

        // Off-screen culling
        const CULL_MARGIN: f32 = 200.0;
        if self.position.x < -CULL_MARGIN || self.position.x > window_size.0 + CULL_MARGIN ||
           self.position.y < -CULL_MARGIN || self.position.y > window_size.1 + CULL_MARGIN {
            if !self.is_stable_hydrogen && self.charge != 2 {
                self.is_alive = false;
            }
        }
    }

    fn handle_boundary_collision(&mut self, window_size: (f32, f32)) {
        let mut collided = false;

        // Left/right
        if self.position.x - self.radius < 0.0 {
            self.position.x = self.radius;
            self.velocity.x = -self.velocity.x * pc::BOUNCE_DAMPENING;
            collided = true;
        } else if self.position.x + self.radius > window_size.0 {
            self.position.x = window_size.0 - self.radius;
            self.velocity.x = -self.velocity.x * pc::BOUNCE_DAMPENING;
            collided = true;
        }

        // Top/bottom
        if self.position.y - self.radius < 0.0 {
            self.position.y = self.radius;
            self.velocity.y = -self.velocity.y * pc::BOUNCE_DAMPENING;
            collided = true;
        } else if self.position.y + self.radius > window_size.1 {
            self.position.y = window_size.1 - self.radius;
            self.velocity.y = -self.velocity.y * pc::BOUNCE_DAMPENING;
            collided = true;
        }

        if collided {
            self.is_sleeping = false;
        }
    }

    pub fn try_neutron_formation(&mut self, delta_time: f32, near_atom: bool) {
        if self.charge != 1 {
            return;
        }

        if !near_atom {
            self.wave_field_timer = 0.0;
            return;
        }

        self.wave_field_timer += delta_time;

        if self.wave_field_timer >= pc::NEUTRON_FORMATION_TIME {
            self.neutron_count = 1;
            self.charge = 0;
            self.radius *= pc::NEUTRON_RADIUS_MULTIPLIER;
            self.wave_field_timer = 0.0;
        }
    }

    pub fn try_capture_electron(&mut self, electron_pos: Vec2) -> bool {
        if self.charge != 0 || self.neutron_count != 1 || self.is_stable_hydrogen {
            return false;
        }

        let distance = self.position.distance(electron_pos);

        if distance < pc::ELECTRON_CAPTURE_DISTANCE {
            self.is_stable_hydrogen = true;
            self.max_lifetime = pc::INFINITE_LIFETIME;
            return true;
        }

        false
    }

    pub fn get_element_label(&self) -> String {
        if self.charge == 2 && self.neutron_count == 2 {
            "He4".to_string()
        } else if self.charge == 1 && self.neutron_count == 2 {
            "He3".to_string()
        } else if self.charge == -1 {
            "H-".to_string()
        } else if self.charge == 0 && self.neutron_count == 1 {
            "H".to_string()
        } else if self.charge == 1 && self.neutron_count == 0 {
            "H+".to_string()
        } else if self.is_stable_hydrogen {
            "H1".to_string()
        } else {
            "?".to_string()
        }
    }

    pub fn render(&self, segments: i32) {
        if !self.is_alive {
            return;
        }

        let mut render_color = self.color;
        let mut render_radius = self.radius;

        // Apply charge state visuals
        if self.is_stable_hydrogen {
            render_color = Color::from_rgba(255, 255, 255, 255);
            render_radius *= pc::STABLE_HYDROGEN_RADIUS_MULTIPLIER;
        } else if self.charge == 0 {
            render_color = Color::from_rgba(200, 200, 200, 255);
        } else if self.charge == 1 {
            let r = (render_color.r * pc::BARE_PROTON_RED_TINT).min(1.0);
            render_color.r = r;
        } else if self.charge == -1 {
            let b = (render_color.b * 1.3).min(1.0);
            render_color.b = b;
        }

        // Helium-3
        if self.charge == 1 && self.neutron_count == 2 {
            render_color = Color::from_rgba(255, 200, 100, 255);
            render_radius *= pc::HELIUM3_RADIUS_MULTIPLIER;
        }
        // Helium-4
        else if self.charge == 2 && self.neutron_count == 2 {
            render_color = Color::from_rgba(255, 255, 100, 255);
            render_radius *= pc::HELIUM4_RADIUS_MULTIPLIER;
        }

        // Pulsing effect
        let pulse_frequency = pc::PULSE_FREQUENCY_BASE + (self.energy * pc::PULSE_FREQUENCY_ENERGY_FACTOR);
        let pulse = (self.pulse_timer * pulse_frequency).sin() * pc::PULSE_INTENSITY + pc::PULSE_BASE;
        render_radius *= pulse;

        // Fade out near end of lifetime
        if self.max_lifetime >= 0.0 && self.lifetime > self.fade_start_time {
            let fade_ratio = (self.lifetime - self.fade_start_time) / (self.max_lifetime - self.fade_start_time);
            let fade_amount = 1.0 - fade_ratio;
            render_color.a = fade_amount;
        }

        // Draw core
        draw_poly(self.position.x, self.position.y, segments as u8, render_radius, 0.0, render_color);

        // Glow layer 1
        let mut glow1 = render_color;
        glow1.a *= pc::GLOW_LAYER1_ALPHA;
        draw_poly(self.position.x, self.position.y, segments as u8, render_radius * pc::GLOW_LAYER1_RADIUS, 0.0, glow1);

        // Glow layer 2
        let mut glow2 = render_color;
        glow2.a *= pc::GLOW_LAYER2_ALPHA;
        draw_poly(self.position.x, self.position.y, segments as u8, render_radius * pc::GLOW_LAYER2_RADIUS, 0.0, glow2);
    }

    fn calculate_radius(energy: f32) -> f32 {
        let radius = pc::MIN_RADIUS + (energy * pc::ENERGY_TO_RADIUS_FACTOR);
        radius.clamp(pc::MIN_RADIUS, pc::MAX_RADIUS)
    }

    fn calculate_mass(energy: f32) -> f32 {
        energy * pc::ENERGY_TO_MASS_FACTOR
    }

    // Getters
    pub fn is_alive(&self) -> bool { self.is_alive && !self.marked_for_deletion }
    pub fn is_marked_for_deletion(&self) -> bool { self.marked_for_deletion }
    pub fn position(&self) -> Vec2 { self.position }
    pub fn velocity(&self) -> Vec2 { self.velocity }
    pub fn radius(&self) -> f32 { self.radius }
    pub fn energy(&self) -> f32 { self.energy }
    pub fn mass(&self) -> f32 { self.mass }
    pub fn color(&self) -> Color { self.color }
    pub fn charge(&self) -> i32 { self.charge }
    pub fn neutron_count(&self) -> i32 { self.neutron_count }
    pub fn is_stable_hydrogen(&self) -> bool { self.is_stable_hydrogen }
    pub fn is_stable_helium4(&self) -> bool { self.charge == 2 && self.neutron_count == 2 }
    pub fn is_sleeping(&self) -> bool { self.is_sleeping }

    // Setters
    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
        self.is_sleeping = false;
    }
    pub fn mark_for_deletion(&mut self) { self.marked_for_deletion = true; }
    pub fn set_neutron_count(&mut self, count: i32) { self.neutron_count = count; }
    pub fn set_max_lifetime(&mut self, lifetime: f32) { self.max_lifetime = lifetime; }
    pub fn wake(&mut self) { self.is_sleeping = false; }
}
