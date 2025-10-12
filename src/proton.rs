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

    // Crystallization system (for H phase transitions)
    is_crystallized: bool,
    crystal_bonds: Vec<usize>, // Indices of bonded protons
    vibration_phase: f32, // For vibration animation
    red_wave_hits: u8, // Count of dark red wave hits (for melting)
    freeze_cooldown: f32, // Time before can crystallize again after melting
    last_red_wave_hit_time: f32, // Tracks time of last hit to prevent double-counting

    // Oxygen-16 bonding system (C12 + He4 molecular bond)
    is_oxygen16_bonded: bool,
    oxygen_bond_partner: Option<usize>, // Index of bonded partner particle
    oxygen_bond_rest_length: f32, // Rest length of O16 bond

    // Water molecule flag
    is_h2o: bool,

    // Neon-20 flag
    is_neon20: bool,

    // Magnesium-24 flag
    is_magnesium24: bool,

    // Silicon-28 flag
    is_silicon28: bool,

    // Sulfur-32 flag
    is_sulfur32: bool,

    // Hydrogen compound molecule flags
    is_h2s: bool,      // Hydrogen Sulfide (S32 + 2H)
    is_mgh2: bool,     // Magnesium Hydride (Mg24 + 2H)
    is_ch4: bool,      // Methane (C12 + 4H)
    is_sih4: bool,     // Silane (Si28 + 4H)
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
            is_crystallized: false,
            crystal_bonds: Vec::new(),
            vibration_phase: 0.0,
            red_wave_hits: 0,
            freeze_cooldown: 0.0,
            last_red_wave_hit_time: -999.0,
            is_oxygen16_bonded: false,
            oxygen_bond_partner: None,
            oxygen_bond_rest_length: 0.0,
            is_h2o: false,
            is_neon20: false,
            is_magnesium24: false,
            is_silicon28: false,
            is_sulfur32: false,
            is_h2s: false,
            is_mgh2: false,
            is_ch4: false,
            is_sih4: false,
        }
    }

    pub fn update(&mut self, delta_time: f32, window_size: (f32, f32)) {
        if !self.is_alive {
            return;
        }

        // Always update visual pulse
        self.pulse_timer += delta_time;

        // Update vibration phase for crystallized particles
        if self.is_crystallized {
            self.vibration_phase += delta_time * 5.0; // 5 rad/s
        }

        // Update freeze cooldown
        if self.freeze_cooldown > 0.0 {
            self.freeze_cooldown -= delta_time;
            if self.freeze_cooldown < 0.0 {
                self.freeze_cooldown = 0.0;
            }
        }

        // SLEEPING OPTIMIZATION
        if self.is_stable_hydrogen || self.is_stable_helium4() || self.is_stable_carbon12() {
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

        // Clamp velocity to max speed
        let speed = self.velocity.length();
        if speed > pc::MAX_SPEED {
            self.velocity = (self.velocity / speed) * pc::MAX_SPEED;
        }

        // Straight-line movement
        self.position += self.velocity * delta_time;

        // Boundary collisions
        self.handle_boundary_collision(window_size);

        // Off-screen culling
        const CULL_MARGIN: f32 = 200.0;
        if self.position.x < -CULL_MARGIN || self.position.x > window_size.0 + CULL_MARGIN ||
           self.position.y < -CULL_MARGIN || self.position.y > window_size.1 + CULL_MARGIN {
            if !self.is_stable_hydrogen && !self.is_stable_helium4() && !self.is_stable_carbon12() {
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
        // Check molecular flags first (take precedence)
        // Hydrogen compounds first
        if self.is_sih4 {
            "SiH4".to_string()
        } else if self.is_ch4 {
            "CH4".to_string()
        } else if self.is_h2s {
            "H2S".to_string()
        } else if self.is_mgh2 {
            "MgH2".to_string()
        } else if self.is_h2o {
            "H2O".to_string()
        }
        // Then alpha ladder elements
        else if self.is_sulfur32 {
            "S32".to_string()
        } else if self.is_silicon28 {
            "Si28".to_string()
        } else if self.is_magnesium24 {
            "Mg24".to_string()
        } else if self.is_neon20 {
            "Ne20".to_string()
        } else if self.is_oxygen16_bonded {
            "O16".to_string()
        } else if self.charge == 6 && self.neutron_count == 6 {
            "C12".to_string()
        } else if self.charge == 2 && self.neutron_count == 2 {
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

        // Hydrogen compound molecules - check first (higher priority)
        if self.is_sih4 {
            render_color = Color::from_rgba(220, 100, 50, 255);
            render_radius *= pc::SIH4_RADIUS_MULTIPLIER;
        }
        else if self.is_ch4 {
            render_color = Color::from_rgba(120, 200, 150, 255);
            render_radius *= pc::CH4_RADIUS_MULTIPLIER;
        }
        else if self.is_h2s {
            render_color = Color::from_rgba(200, 220, 80, 255);
            render_radius *= pc::H2S_RADIUS_MULTIPLIER;
        }
        else if self.is_mgh2 {
            render_color = Color::from_rgba(180, 180, 190, 255);
            render_radius *= pc::MGH2_RADIUS_MULTIPLIER;
        }
        else if self.is_h2o {
            render_color = Color::from_rgba(40, 100, 180, 255);
            render_radius *= pc::WATER_RADIUS_MULTIPLIER;
        }
        // Alpha ladder elements
        else if self.is_sulfur32 {
            render_color = Color::from_rgba(220, 220, 80, 255);
            render_radius *= pc::SULFUR32_RADIUS_MULTIPLIER;
        }
        else if self.is_silicon28 {
            render_color = Color::from_rgba(160, 130, 90, 255);
            render_radius *= pc::SILICON28_RADIUS_MULTIPLIER;
        }
        else if self.is_magnesium24 {
            render_color = Color::from_rgba(200, 200, 220, 255);
            render_radius *= pc::MAGNESIUM24_RADIUS_MULTIPLIER;
        }
        else if self.is_neon20 {
            render_color = Color::from_rgba(255, 100, 150, 255);
            render_radius *= pc::NEON20_RADIUS_MULTIPLIER;
        }
        // Oxygen-16 bonded pair - check third as it overrides base element colors
        else if self.is_oxygen16_bonded {
            render_color = Color::from_rgba(100, 180, 255, 255);
            // Keep original radius for bonded particles
        }
        // Carbon-12
        else if self.charge == 6 && self.neutron_count == 6 {
            render_color = Color::from_rgba(100, 100, 100, 255);
            render_radius *= pc::CARBON12_RADIUS_MULTIPLIER;
        }
        // Helium-3
        else if self.charge == 1 && self.neutron_count == 2 {
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
    pub fn set_stable_hydrogen(&mut self, stable: bool) { self.is_stable_hydrogen = stable; }
    pub fn is_stable_helium4(&self) -> bool { self.charge == 2 && self.neutron_count == 2 }
    pub fn is_stable_carbon12(&self) -> bool { self.charge == 6 && self.neutron_count == 6 }
    pub fn is_sleeping(&self) -> bool { self.is_sleeping }
    pub fn is_crystallized(&self) -> bool { self.is_crystallized }
    pub fn crystal_bonds(&self) -> &Vec<usize> { &self.crystal_bonds }
    pub fn vibration_phase(&self) -> f32 { self.vibration_phase }

    // Setters
    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
        self.is_sleeping = false;
    }
    pub fn add_velocity(&mut self, delta_velocity: Vec2) {
        self.velocity += delta_velocity;
        self.is_sleeping = false;
    }
    pub fn mark_for_deletion(&mut self) { self.marked_for_deletion = true; }
    pub fn set_neutron_count(&mut self, count: i32) { self.neutron_count = count; }
    pub fn set_max_lifetime(&mut self, lifetime: f32) { self.max_lifetime = lifetime; }
    pub fn wake(&mut self) { self.is_sleeping = false; }
    pub fn set_crystallized(&mut self, crystallized: bool) { self.is_crystallized = crystallized; }
    pub fn set_crystal_bonds(&mut self, bonds: Vec<usize>) { self.crystal_bonds = bonds; }
    pub fn clear_crystal_bonds(&mut self) { self.crystal_bonds.clear(); }
    pub fn add_crystal_bond(&mut self, index: usize) {
        if !self.crystal_bonds.contains(&index) {
            self.crystal_bonds.push(index);
        }
    }
    pub fn red_wave_hits(&self) -> u8 { self.red_wave_hits }
    pub fn increment_red_wave_hits(&mut self) {
        if self.red_wave_hits < 255 {
            self.red_wave_hits += 1;
        }
    }
    pub fn reset_red_wave_hits(&mut self) { self.red_wave_hits = 0; }
    pub fn freeze_cooldown(&self) -> f32 { self.freeze_cooldown }
    pub fn set_freeze_cooldown(&mut self, cooldown: f32) { self.freeze_cooldown = cooldown; }
    pub fn last_red_wave_hit_time(&self) -> f32 { self.last_red_wave_hit_time }
    pub fn set_last_red_wave_hit_time(&mut self, time: f32) { self.last_red_wave_hit_time = time; }

    // Oxygen-16 bonding getters/setters
    pub fn is_oxygen16_bonded(&self) -> bool { self.is_oxygen16_bonded }
    pub fn set_oxygen16_bonded(&mut self, bonded: bool) { self.is_oxygen16_bonded = bonded; }
    pub fn oxygen_bond_partner(&self) -> Option<usize> { self.oxygen_bond_partner }
    pub fn set_oxygen_bond_partner(&mut self, partner: Option<usize>) { self.oxygen_bond_partner = partner; }
    pub fn oxygen_bond_rest_length(&self) -> f32 { self.oxygen_bond_rest_length }
    pub fn set_oxygen_bond_rest_length(&mut self, length: f32) { self.oxygen_bond_rest_length = length; }
    pub fn clear_oxygen_bond(&mut self) {
        self.is_oxygen16_bonded = false;
        self.oxygen_bond_partner = None;
        self.oxygen_bond_rest_length = 0.0;
    }

    // Water molecule getters/setters
    pub fn is_h2o(&self) -> bool { self.is_h2o }
    pub fn set_h2o(&mut self, is_water: bool) { self.is_h2o = is_water; }

    // Neon-20 getters/setters
    pub fn is_neon20(&self) -> bool { self.is_neon20 }
    pub fn set_neon20(&mut self, is_neon: bool) { self.is_neon20 = is_neon; }

    // Magnesium-24 getters/setters
    pub fn is_magnesium24(&self) -> bool { self.is_magnesium24 }
    pub fn set_magnesium24(&mut self, is_mg: bool) { self.is_magnesium24 = is_mg; }

    // Silicon-28 getters/setters
    pub fn is_silicon28(&self) -> bool { self.is_silicon28 }
    pub fn set_silicon28(&mut self, is_si: bool) { self.is_silicon28 = is_si; }

    // Sulfur-32 getters/setters
    pub fn is_sulfur32(&self) -> bool { self.is_sulfur32 }
    pub fn set_sulfur32(&mut self, is_s: bool) { self.is_sulfur32 = is_s; }

    // Hydrogen compound molecule getters/setters
    pub fn is_h2s(&self) -> bool { self.is_h2s }
    pub fn set_h2s(&mut self, is_h2s: bool) { self.is_h2s = is_h2s; }

    pub fn is_mgh2(&self) -> bool { self.is_mgh2 }
    pub fn set_mgh2(&mut self, is_mgh2: bool) { self.is_mgh2 = is_mgh2; }

    pub fn is_ch4(&self) -> bool { self.is_ch4 }
    pub fn set_ch4(&mut self, is_ch4: bool) { self.is_ch4 = is_ch4; }

    pub fn is_sih4(&self) -> bool { self.is_sih4 }
    pub fn set_sih4(&mut self, is_sih4: bool) { self.is_sih4 = is_sih4; }
}
