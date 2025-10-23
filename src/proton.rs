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
    h_crystal_group: Option<usize>, // Group ID for connected H crystals (for rigid body movement)

    // Oxygen-16 bonding system (C12 + He4 molecular bond)
    is_oxygen16_bonded: bool,
    oxygen_bond_partner: Option<usize>, // Index of bonded partner particle
    oxygen_bond_rest_length: f32, // Rest length of O16 bond

    // Water molecule flag and hydrogen bonding system
    is_h2o: bool,
    water_polar_angle: f32, // Angle for polar orientation (0-2π)
    water_h_bonds: Vec<usize>, // Indices of hydrogen-bonded water molecules (max 3)
    water_bond_rest_lengths: Vec<f32>, // Rest lengths for each hydrogen bond
    is_water_frozen: bool, // True when H2O is compressed into ice (frozen state)
    ice_crystal_group: Option<usize>, // Group ID for connected ice crystals (for collective movement)

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

    // Universal phase transition system for all elements
    // He3 (charge=1, neutron_count=2) phase transitions
    is_he3_crystallized: bool,
    he3_crystal_bonds: Vec<usize>,
    he3_crystal_group: Option<usize>,
    he3_freeze_cooldown: f32,

    // He4 (charge=2, neutron_count=2) phase transitions
    is_he4_crystallized: bool,
    he4_crystal_bonds: Vec<usize>,
    he4_crystal_group: Option<usize>,
    he4_freeze_cooldown: f32,

    // C12 (charge=6, neutron_count=6) phase transitions
    is_c12_crystallized: bool,
    c12_crystal_bonds: Vec<usize>,
    c12_crystal_group: Option<usize>,
    c12_freeze_cooldown: f32,

    // Ne20 phase transitions
    is_ne20_crystallized: bool,
    ne20_crystal_bonds: Vec<usize>,
    ne20_crystal_group: Option<usize>,
    ne20_freeze_cooldown: f32,

    // Mg24 phase transitions
    is_mg24_crystallized: bool,
    mg24_crystal_bonds: Vec<usize>,
    mg24_crystal_group: Option<usize>,
    mg24_freeze_cooldown: f32,

    // Si28 phase transitions
    is_si28_crystallized: bool,
    si28_crystal_bonds: Vec<usize>,
    si28_crystal_group: Option<usize>,
    si28_freeze_cooldown: f32,

    // S32 phase transitions
    is_s32_crystallized: bool,
    s32_crystal_bonds: Vec<usize>,
    s32_crystal_group: Option<usize>,
    s32_freeze_cooldown: f32,

    // === BIOLOGICAL ELEMENTS ===

    // Nitrogen-14 flag
    is_nitrogen14: bool,

    // Phosphorus-31 flag
    is_phosphorus31: bool,

    // Sodium-23 flag
    is_sodium23: bool,

    // Potassium-39 flag
    is_potassium39: bool,

    // Calcium-40 flag
    is_calcium40: bool,

    // N14 phase transitions
    is_n14_crystallized: bool,
    n14_crystal_bonds: Vec<usize>,
    n14_crystal_group: Option<usize>,
    n14_freeze_cooldown: f32,

    // P31 phase transitions
    is_p31_crystallized: bool,
    p31_crystal_bonds: Vec<usize>,
    p31_crystal_group: Option<usize>,
    p31_freeze_cooldown: f32,

    // Na23 phase transitions
    is_na23_crystallized: bool,
    na23_crystal_bonds: Vec<usize>,
    na23_crystal_group: Option<usize>,
    na23_freeze_cooldown: f32,

    // K39 phase transitions
    is_k39_crystallized: bool,
    k39_crystal_bonds: Vec<usize>,
    k39_crystal_group: Option<usize>,
    k39_freeze_cooldown: f32,

    // Ca40 phase transitions
    is_ca40_crystallized: bool,
    ca40_crystal_bonds: Vec<usize>,
    ca40_crystal_group: Option<usize>,
    ca40_freeze_cooldown: f32,
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
            h_crystal_group: None,
            is_oxygen16_bonded: false,
            oxygen_bond_partner: None,
            oxygen_bond_rest_length: 0.0,
            is_h2o: false,
            water_polar_angle: 0.0,
            water_h_bonds: Vec::new(),
            water_bond_rest_lengths: Vec::new(),
            is_water_frozen: false,
            ice_crystal_group: None,
            is_neon20: false,
            is_magnesium24: false,
            is_silicon28: false,
            is_sulfur32: false,
            is_h2s: false,
            is_mgh2: false,
            is_ch4: false,
            is_sih4: false,
            // Phase transition initializations
            is_he3_crystallized: false,
            he3_crystal_bonds: Vec::new(),
            he3_crystal_group: None,
            he3_freeze_cooldown: 0.0,
            is_he4_crystallized: false,
            he4_crystal_bonds: Vec::new(),
            he4_crystal_group: None,
            he4_freeze_cooldown: 0.0,
            is_c12_crystallized: false,
            c12_crystal_bonds: Vec::new(),
            c12_crystal_group: None,
            c12_freeze_cooldown: 0.0,
            is_ne20_crystallized: false,
            ne20_crystal_bonds: Vec::new(),
            ne20_crystal_group: None,
            ne20_freeze_cooldown: 0.0,
            is_mg24_crystallized: false,
            mg24_crystal_bonds: Vec::new(),
            mg24_crystal_group: None,
            mg24_freeze_cooldown: 0.0,
            is_si28_crystallized: false,
            si28_crystal_bonds: Vec::new(),
            si28_crystal_group: None,
            si28_freeze_cooldown: 0.0,
            is_s32_crystallized: false,
            s32_crystal_bonds: Vec::new(),
            s32_crystal_group: None,
            s32_freeze_cooldown: 0.0,
            // Biological element flags
            is_nitrogen14: false,
            is_phosphorus31: false,
            is_sodium23: false,
            is_potassium39: false,
            is_calcium40: false,
            // Biological element phase transitions
            is_n14_crystallized: false,
            n14_crystal_bonds: Vec::new(),
            n14_crystal_group: None,
            n14_freeze_cooldown: 0.0,
            is_p31_crystallized: false,
            p31_crystal_bonds: Vec::new(),
            p31_crystal_group: None,
            p31_freeze_cooldown: 0.0,
            is_na23_crystallized: false,
            na23_crystal_bonds: Vec::new(),
            na23_crystal_group: None,
            na23_freeze_cooldown: 0.0,
            is_k39_crystallized: false,
            k39_crystal_bonds: Vec::new(),
            k39_crystal_group: None,
            k39_freeze_cooldown: 0.0,
            is_ca40_crystallized: false,
            ca40_crystal_bonds: Vec::new(),
            ca40_crystal_group: None,
            ca40_freeze_cooldown: 0.0,
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

        // Update freeze cooldown for H
        if self.freeze_cooldown > 0.0 {
            self.freeze_cooldown -= delta_time;
            if self.freeze_cooldown < 0.0 {
                self.freeze_cooldown = 0.0;
            }
        }

        // Update freeze cooldowns for all elements
        if self.he3_freeze_cooldown > 0.0 {
            self.he3_freeze_cooldown -= delta_time;
            if self.he3_freeze_cooldown < 0.0 { self.he3_freeze_cooldown = 0.0; }
        }
        if self.he4_freeze_cooldown > 0.0 {
            self.he4_freeze_cooldown -= delta_time;
            if self.he4_freeze_cooldown < 0.0 { self.he4_freeze_cooldown = 0.0; }
        }
        if self.c12_freeze_cooldown > 0.0 {
            self.c12_freeze_cooldown -= delta_time;
            if self.c12_freeze_cooldown < 0.0 { self.c12_freeze_cooldown = 0.0; }
        }
        if self.ne20_freeze_cooldown > 0.0 {
            self.ne20_freeze_cooldown -= delta_time;
            if self.ne20_freeze_cooldown < 0.0 { self.ne20_freeze_cooldown = 0.0; }
        }
        if self.mg24_freeze_cooldown > 0.0 {
            self.mg24_freeze_cooldown -= delta_time;
            if self.mg24_freeze_cooldown < 0.0 { self.mg24_freeze_cooldown = 0.0; }
        }
        if self.si28_freeze_cooldown > 0.0 {
            self.si28_freeze_cooldown -= delta_time;
            if self.si28_freeze_cooldown < 0.0 { self.si28_freeze_cooldown = 0.0; }
        }
        if self.s32_freeze_cooldown > 0.0 {
            self.s32_freeze_cooldown -= delta_time;
            if self.s32_freeze_cooldown < 0.0 { self.s32_freeze_cooldown = 0.0; }
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

        // Apply friction to liquid H2O molecules (helps them settle into ice)
        if self.is_h2o && !self.is_water_frozen {
            self.velocity *= 1.00;  // Per-frame damping for water molecules
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
        }
        // Biological elements
        else if self.is_nitrogen14 || (self.charge == 7 && self.neutron_count == 7) {
            "N14".to_string()
        } else if self.is_phosphorus31 || (self.charge == 15 && self.neutron_count == 16) {
            "P31".to_string()
        } else if self.is_sodium23 || (self.charge == 11 && self.neutron_count == 12) {
            "Na23".to_string()
        } else if self.is_potassium39 || (self.charge == 19 && self.neutron_count == 20) {
            "K39".to_string()
        } else if self.is_calcium40 || (self.charge == 20 && self.neutron_count == 20) {
            "Ca40".to_string()
        }
        // Triple alpha and helium
        else if self.charge == 6 && self.neutron_count == 6 {
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
            // Progressive coloring based on bond count and frozen state
            let bond_count = self.water_h_bonds.len();
            if bond_count >= 5 && self.is_water_frozen {
                // Fully frozen hexagonal ice - white
                render_color = Color::from_rgba(255, 255, 255, 255);
            } else if bond_count == 4 {
                // 4 bonds - lighter blue (approaching freezing)
                render_color = Color::from_rgba(160, 180, 210, 255);
            } else if bond_count == 3 {
                // 3 bonds - light blue (partial bonding)
                render_color = Color::from_rgba(120, 150, 200, 255);
            } else {
                // 0-2 bonds - liquid water (blue)
                render_color = Color::from_rgba(40, 100, 180, 255);
            }
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
        // Biological elements
        else if self.is_nitrogen14 || (self.charge == 7 && self.neutron_count == 7) {
            render_color = Color::from_rgba(50, 150, 200, 255);  // Light blue
            render_radius *= pc::NITROGEN14_RADIUS_MULTIPLIER;
        }
        else if self.is_phosphorus31 || (self.charge == 15 && self.neutron_count == 16) {
            render_color = Color::from_rgba(220, 100, 100, 255);  // Reddish
            render_radius *= pc::PHOSPHORUS31_RADIUS_MULTIPLIER;
        }
        else if self.is_sodium23 || (self.charge == 11 && self.neutron_count == 12) {
            render_color = Color::from_rgba(255, 150, 100, 255);  // Orange
            render_radius *= pc::SODIUM23_RADIUS_MULTIPLIER;
        }
        else if self.is_potassium39 || (self.charge == 19 && self.neutron_count == 20) {
            render_color = Color::from_rgba(100, 200, 150, 255);  // Teal
            render_radius *= pc::POTASSIUM39_RADIUS_MULTIPLIER;
        }
        else if self.is_calcium40 || (self.charge == 20 && self.neutron_count == 20) {
            render_color = Color::from_rgba(200, 220, 180, 255);  // Light gray-green
            render_radius *= pc::CALCIUM40_RADIUS_MULTIPLIER;
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
    pub fn h_crystal_group(&self) -> Option<usize> { self.h_crystal_group }
    pub fn set_h_crystal_group(&mut self, group: Option<usize>) { self.h_crystal_group = group; }

    // He3 phase transition getters/setters
    pub fn is_he3_crystallized(&self) -> bool { self.is_he3_crystallized }
    pub fn set_he3_crystallized(&mut self, crystallized: bool) { self.is_he3_crystallized = crystallized; }
    pub fn he3_crystal_bonds(&self) -> &Vec<usize> { &self.he3_crystal_bonds }
    pub fn set_he3_crystal_bonds(&mut self, bonds: Vec<usize>) { self.he3_crystal_bonds = bonds; }
    pub fn clear_he3_crystal_bonds(&mut self) { self.he3_crystal_bonds.clear(); }
    pub fn he3_crystal_group(&self) -> Option<usize> { self.he3_crystal_group }
    pub fn set_he3_crystal_group(&mut self, group: Option<usize>) { self.he3_crystal_group = group; }
    pub fn he3_freeze_cooldown(&self) -> f32 { self.he3_freeze_cooldown }
    pub fn set_he3_freeze_cooldown(&mut self, cooldown: f32) { self.he3_freeze_cooldown = cooldown; }

    // He4 phase transition getters/setters
    pub fn is_he4_crystallized(&self) -> bool { self.is_he4_crystallized }
    pub fn set_he4_crystallized(&mut self, crystallized: bool) { self.is_he4_crystallized = crystallized; }
    pub fn he4_crystal_bonds(&self) -> &Vec<usize> { &self.he4_crystal_bonds }
    pub fn set_he4_crystal_bonds(&mut self, bonds: Vec<usize>) { self.he4_crystal_bonds = bonds; }
    pub fn clear_he4_crystal_bonds(&mut self) { self.he4_crystal_bonds.clear(); }
    pub fn he4_crystal_group(&self) -> Option<usize> { self.he4_crystal_group }
    pub fn set_he4_crystal_group(&mut self, group: Option<usize>) { self.he4_crystal_group = group; }
    pub fn he4_freeze_cooldown(&self) -> f32 { self.he4_freeze_cooldown }
    pub fn set_he4_freeze_cooldown(&mut self, cooldown: f32) { self.he4_freeze_cooldown = cooldown; }

    // C12 phase transition getters/setters
    pub fn is_c12_crystallized(&self) -> bool { self.is_c12_crystallized }
    pub fn set_c12_crystallized(&mut self, crystallized: bool) { self.is_c12_crystallized = crystallized; }
    pub fn c12_crystal_bonds(&self) -> &Vec<usize> { &self.c12_crystal_bonds }
    pub fn set_c12_crystal_bonds(&mut self, bonds: Vec<usize>) { self.c12_crystal_bonds = bonds; }
    pub fn clear_c12_crystal_bonds(&mut self) { self.c12_crystal_bonds.clear(); }
    pub fn c12_crystal_group(&self) -> Option<usize> { self.c12_crystal_group }
    pub fn set_c12_crystal_group(&mut self, group: Option<usize>) { self.c12_crystal_group = group; }
    pub fn c12_freeze_cooldown(&self) -> f32 { self.c12_freeze_cooldown }
    pub fn set_c12_freeze_cooldown(&mut self, cooldown: f32) { self.c12_freeze_cooldown = cooldown; }

    // Ne20 phase transition getters/setters
    pub fn is_ne20_crystallized(&self) -> bool { self.is_ne20_crystallized }
    pub fn set_ne20_crystallized(&mut self, crystallized: bool) { self.is_ne20_crystallized = crystallized; }
    pub fn ne20_crystal_bonds(&self) -> &Vec<usize> { &self.ne20_crystal_bonds }
    pub fn set_ne20_crystal_bonds(&mut self, bonds: Vec<usize>) { self.ne20_crystal_bonds = bonds; }
    pub fn clear_ne20_crystal_bonds(&mut self) { self.ne20_crystal_bonds.clear(); }
    pub fn ne20_crystal_group(&self) -> Option<usize> { self.ne20_crystal_group }
    pub fn set_ne20_crystal_group(&mut self, group: Option<usize>) { self.ne20_crystal_group = group; }
    pub fn ne20_freeze_cooldown(&self) -> f32 { self.ne20_freeze_cooldown }
    pub fn set_ne20_freeze_cooldown(&mut self, cooldown: f32) { self.ne20_freeze_cooldown = cooldown; }

    // Mg24 phase transition getters/setters
    pub fn is_mg24_crystallized(&self) -> bool { self.is_mg24_crystallized }
    pub fn set_mg24_crystallized(&mut self, crystallized: bool) { self.is_mg24_crystallized = crystallized; }
    pub fn mg24_crystal_bonds(&self) -> &Vec<usize> { &self.mg24_crystal_bonds }
    pub fn set_mg24_crystal_bonds(&mut self, bonds: Vec<usize>) { self.mg24_crystal_bonds = bonds; }
    pub fn clear_mg24_crystal_bonds(&mut self) { self.mg24_crystal_bonds.clear(); }
    pub fn mg24_crystal_group(&self) -> Option<usize> { self.mg24_crystal_group }
    pub fn set_mg24_crystal_group(&mut self, group: Option<usize>) { self.mg24_crystal_group = group; }
    pub fn mg24_freeze_cooldown(&self) -> f32 { self.mg24_freeze_cooldown }
    pub fn set_mg24_freeze_cooldown(&mut self, cooldown: f32) { self.mg24_freeze_cooldown = cooldown; }

    // Si28 phase transition getters/setters
    pub fn is_si28_crystallized(&self) -> bool { self.is_si28_crystallized }
    pub fn set_si28_crystallized(&mut self, crystallized: bool) { self.is_si28_crystallized = crystallized; }
    pub fn si28_crystal_bonds(&self) -> &Vec<usize> { &self.si28_crystal_bonds }
    pub fn set_si28_crystal_bonds(&mut self, bonds: Vec<usize>) { self.si28_crystal_bonds = bonds; }
    pub fn clear_si28_crystal_bonds(&mut self) { self.si28_crystal_bonds.clear(); }
    pub fn si28_crystal_group(&self) -> Option<usize> { self.si28_crystal_group }
    pub fn set_si28_crystal_group(&mut self, group: Option<usize>) { self.si28_crystal_group = group; }
    pub fn si28_freeze_cooldown(&self) -> f32 { self.si28_freeze_cooldown }
    pub fn set_si28_freeze_cooldown(&mut self, cooldown: f32) { self.si28_freeze_cooldown = cooldown; }

    // S32 phase transition getters/setters
    pub fn is_s32_crystallized(&self) -> bool { self.is_s32_crystallized }
    pub fn set_s32_crystallized(&mut self, crystallized: bool) { self.is_s32_crystallized = crystallized; }
    pub fn s32_crystal_bonds(&self) -> &Vec<usize> { &self.s32_crystal_bonds }
    pub fn set_s32_crystal_bonds(&mut self, bonds: Vec<usize>) { self.s32_crystal_bonds = bonds; }
    pub fn clear_s32_crystal_bonds(&mut self) { self.s32_crystal_bonds.clear(); }
    pub fn s32_crystal_group(&self) -> Option<usize> { self.s32_crystal_group }
    pub fn set_s32_crystal_group(&mut self, group: Option<usize>) { self.s32_crystal_group = group; }
    pub fn s32_freeze_cooldown(&self) -> f32 { self.s32_freeze_cooldown }
    pub fn set_s32_freeze_cooldown(&mut self, cooldown: f32) { self.s32_freeze_cooldown = cooldown; }

    // === BIOLOGICAL ELEMENTS GETTERS/SETTERS ===

    // N14 crystallization getters/setters
    pub fn is_n14_crystallized(&self) -> bool { self.is_n14_crystallized }
    pub fn set_n14_crystallized(&mut self, crystallized: bool) { self.is_n14_crystallized = crystallized; }
    pub fn n14_crystal_bonds(&self) -> &Vec<usize> { &self.n14_crystal_bonds }
    pub fn set_n14_crystal_bonds(&mut self, bonds: Vec<usize>) { self.n14_crystal_bonds = bonds; }
    pub fn clear_n14_crystal_bonds(&mut self) { self.n14_crystal_bonds.clear(); }
    pub fn n14_crystal_group(&self) -> Option<usize> { self.n14_crystal_group }
    pub fn set_n14_crystal_group(&mut self, group: Option<usize>) { self.n14_crystal_group = group; }
    pub fn n14_freeze_cooldown(&self) -> f32 { self.n14_freeze_cooldown }
    pub fn set_n14_freeze_cooldown(&mut self, cooldown: f32) { self.n14_freeze_cooldown = cooldown; }

    // P31 crystallization getters/setters
    pub fn is_p31_crystallized(&self) -> bool { self.is_p31_crystallized }
    pub fn set_p31_crystallized(&mut self, crystallized: bool) { self.is_p31_crystallized = crystallized; }
    pub fn p31_crystal_bonds(&self) -> &Vec<usize> { &self.p31_crystal_bonds }
    pub fn set_p31_crystal_bonds(&mut self, bonds: Vec<usize>) { self.p31_crystal_bonds = bonds; }
    pub fn clear_p31_crystal_bonds(&mut self) { self.p31_crystal_bonds.clear(); }
    pub fn p31_crystal_group(&self) -> Option<usize> { self.p31_crystal_group }
    pub fn set_p31_crystal_group(&mut self, group: Option<usize>) { self.p31_crystal_group = group; }
    pub fn p31_freeze_cooldown(&self) -> f32 { self.p31_freeze_cooldown }
    pub fn set_p31_freeze_cooldown(&mut self, cooldown: f32) { self.p31_freeze_cooldown = cooldown; }

    // Na23 crystallization getters/setters
    pub fn is_na23_crystallized(&self) -> bool { self.is_na23_crystallized }
    pub fn set_na23_crystallized(&mut self, crystallized: bool) { self.is_na23_crystallized = crystallized; }
    pub fn na23_crystal_bonds(&self) -> &Vec<usize> { &self.na23_crystal_bonds }
    pub fn set_na23_crystal_bonds(&mut self, bonds: Vec<usize>) { self.na23_crystal_bonds = bonds; }
    pub fn clear_na23_crystal_bonds(&mut self) { self.na23_crystal_bonds.clear(); }
    pub fn na23_crystal_group(&self) -> Option<usize> { self.na23_crystal_group }
    pub fn set_na23_crystal_group(&mut self, group: Option<usize>) { self.na23_crystal_group = group; }
    pub fn na23_freeze_cooldown(&self) -> f32 { self.na23_freeze_cooldown }
    pub fn set_na23_freeze_cooldown(&mut self, cooldown: f32) { self.na23_freeze_cooldown = cooldown; }

    // K39 crystallization getters/setters
    pub fn is_k39_crystallized(&self) -> bool { self.is_k39_crystallized }
    pub fn set_k39_crystallized(&mut self, crystallized: bool) { self.is_k39_crystallized = crystallized; }
    pub fn k39_crystal_bonds(&self) -> &Vec<usize> { &self.k39_crystal_bonds }
    pub fn set_k39_crystal_bonds(&mut self, bonds: Vec<usize>) { self.k39_crystal_bonds = bonds; }
    pub fn clear_k39_crystal_bonds(&mut self) { self.k39_crystal_bonds.clear(); }
    pub fn k39_crystal_group(&self) -> Option<usize> { self.k39_crystal_group }
    pub fn set_k39_crystal_group(&mut self, group: Option<usize>) { self.k39_crystal_group = group; }
    pub fn k39_freeze_cooldown(&self) -> f32 { self.k39_freeze_cooldown }
    pub fn set_k39_freeze_cooldown(&mut self, cooldown: f32) { self.k39_freeze_cooldown = cooldown; }

    // Ca40 crystallization getters/setters
    pub fn is_ca40_crystallized(&self) -> bool { self.is_ca40_crystallized }
    pub fn set_ca40_crystallized(&mut self, crystallized: bool) { self.is_ca40_crystallized = crystallized; }
    pub fn ca40_crystal_bonds(&self) -> &Vec<usize> { &self.ca40_crystal_bonds }
    pub fn set_ca40_crystal_bonds(&mut self, bonds: Vec<usize>) { self.ca40_crystal_bonds = bonds; }
    pub fn clear_ca40_crystal_bonds(&mut self) { self.ca40_crystal_bonds.clear(); }
    pub fn ca40_crystal_group(&self) -> Option<usize> { self.ca40_crystal_group }
    pub fn set_ca40_crystal_group(&mut self, group: Option<usize>) { self.ca40_crystal_group = group; }
    pub fn ca40_freeze_cooldown(&self) -> f32 { self.ca40_freeze_cooldown }
    pub fn set_ca40_freeze_cooldown(&mut self, cooldown: f32) { self.ca40_freeze_cooldown = cooldown; }

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
    pub fn water_polar_angle(&self) -> f32 { self.water_polar_angle }
    pub fn set_water_polar_angle(&mut self, angle: f32) { self.water_polar_angle = angle; }
    pub fn water_h_bonds(&self) -> &Vec<usize> { &self.water_h_bonds }
    pub fn water_h_bonds_mut(&mut self) -> &mut Vec<usize> { &mut self.water_h_bonds }
    pub fn water_bond_rest_lengths(&self) -> &Vec<f32> { &self.water_bond_rest_lengths }
    pub fn add_water_h_bond(&mut self, index: usize, rest_length: f32) {
        // All H2O can form up to 5 bonds (regardless of liquid or frozen state)
        // 0-3 bonds = liquid, 4-5 bonds = frozen
        if !self.water_h_bonds.contains(&index) && self.water_h_bonds.len() < pc::WATER_ICE_MAX_BONDS {
            self.water_h_bonds.push(index);
            self.water_bond_rest_lengths.push(rest_length);
        }
    }
    pub fn clear_water_h_bonds(&mut self) {
        self.water_h_bonds.clear();
        self.water_bond_rest_lengths.clear();
    }
    pub fn is_water_frozen(&self) -> bool { self.is_water_frozen }
    pub fn set_water_frozen(&mut self, frozen: bool) { self.is_water_frozen = frozen; }
    pub fn ice_crystal_group(&self) -> Option<usize> { self.ice_crystal_group }
    pub fn set_ice_crystal_group(&mut self, group: Option<usize>) { self.ice_crystal_group = group; }

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
