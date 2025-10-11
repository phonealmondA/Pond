// ProtonManager - Manages all protons with physics interactions and spawning
// Rust port of ProtonManager.h/cpp

use macroquad::prelude::*;
use crate::constants::*;
use crate::constants::proton_manager as pm;
use crate::proton::Proton;
use crate::atom::AtomManager;
use crate::ring::RingManager;

pub struct ProtonManager {
    protons: Vec<Option<Proton>>,
    next_slot: usize,
    max_protons: usize,
    spawn_cooldowns: Vec<(Vec2, f32)>,
}

impl ProtonManager {
    pub fn new(max_protons: usize) -> Self {
        let mut protons = Vec::with_capacity(max_protons);
        for _ in 0..max_protons {
            protons.push(None);
        }

        Self {
            protons,
            next_slot: 0,
            max_protons,
            spawn_cooldowns: Vec::new(),
        }
    }

    /// Main update - physics, interactions, and spawning from atoms
    pub fn update(
        &mut self,
        delta_time: f32,
        window_size: (f32, f32),
        atom_manager: &mut AtomManager,
        ring_manager: &mut RingManager,
    ) {
        // Update cooldowns
        self.update_cooldowns(delta_time);

        // STEP 1: Simple straight-line physics
        self.update_proton_physics(delta_time, window_size);

        // STEP 2: Charge-based forces (H+/H- interactions and H clustering)
        self.apply_charge_forces(delta_time);

        // STEP 2.5: Red wave repulsion (only affects H-)
        self.apply_red_wave_repulsion(delta_time, ring_manager);

        // STEP 3: Solid collisions (H and He4)
        self.handle_solid_collisions();

        // STEP 4: Neutron formation (proximity to atoms)
        for i in 0..self.protons.len() {
            // First, collect info about the proton
            let (should_check, proton_pos) = {
                if let Some(proton) = &self.protons[i] {
                    if proton.is_alive() && proton.charge() == 1 {
                        (true, proton.position())
                    } else {
                        (false, Vec2::ZERO)
                    }
                } else {
                    (false, Vec2::ZERO)
                }
            };

            if should_check {
                let near_atom = self.is_near_atom(proton_pos, atom_manager);
                if let Some(proton) = &mut self.protons[i] {
                    proton.try_neutron_formation(delta_time, near_atom);
                }
            }
        }

        // STEP 5: Electron capture (for neutral protons)
        for i in 0..self.protons.len() {
            // First, collect info about the proton
            let (should_check, proton_pos) = {
                if let Some(proton) = &self.protons[i] {
                    if proton.is_alive() && proton.charge() == 0 && proton.neutron_count() == 1 {
                        (true, proton.position())
                    } else {
                        (false, Vec2::ZERO)
                    }
                } else {
                    (false, Vec2::ZERO)
                }
            };

            if should_check {
                if let Some(atom_pos) = self.find_nearby_atom(proton_pos, atom_manager) {
                    let captured = if let Some(proton) = &mut self.protons[i] {
                        proton.try_capture_electron(atom_pos)
                    } else {
                        false
                    };

                    if captured {
                        self.mark_atom_at_position(atom_pos, atom_manager);
                    }
                }
            }
        }

        // STEP 6: Nuclear fusion
        self.handle_nuclear_fusion(ring_manager);

        // STEP 7: Spawn from atom collisions
        self.detect_and_spawn_from_atom_collisions(atom_manager);

        // STEP 8: Cleanup dead protons
        for proton_opt in &mut self.protons {
            if let Some(proton) = proton_opt {
                if !proton.is_alive() || proton.is_marked_for_deletion() {
                    // Never remove stable hydrogen or stable Helium-4
                    if !proton.is_stable_hydrogen() && !proton.is_stable_helium4() {
                        *proton_opt = None;
                    }
                }
            }
        }
    }

    /// Draw all protons
    pub fn draw(&self, segments: i32) {
        for proton_opt in &self.protons {
            if let Some(proton) = proton_opt {
                if proton.is_alive() {
                    proton.render(segments);
                }
            }
        }
    }

    /// Draw labels centered on protons
    pub fn draw_labels(&self) {
        for proton_opt in &self.protons {
            if let Some(proton) = proton_opt {
                if proton.is_alive() {
                    let label = proton.get_element_label();
                    let pos = proton.position();

                    // Measure text dimensions for centering
                    let font_size = 12.0;
                    let text_dims = measure_text(&label, None, font_size as u16, 1.0);

                    // Center text on proton (both horizontally and vertically)
                    let text_x = pos.x - text_dims.width / 2.0;
                    let text_y = pos.y + text_dims.height / 3.0; // Adjust for baseline

                    // Draw text with black outline for visibility
                    draw_text(&label, text_x + 1.0, text_y + 1.0, font_size, BLACK);
                    draw_text(&label, text_x - 1.0, text_y - 1.0, font_size, BLACK);
                    draw_text(&label, text_x + 1.0, text_y - 1.0, font_size, BLACK);
                    draw_text(&label, text_x - 1.0, text_y + 1.0, font_size, BLACK);
                    draw_text(&label, text_x, text_y, font_size, WHITE);
                }
            }
        }
    }

    /// Clear all protons (except stable ones)
    pub fn clear(&mut self) {
        for proton_opt in &mut self.protons {
            if let Some(proton) = proton_opt {
                // Preserve stable H1 and He4 only
                if !proton.is_stable_hydrogen() && !proton.is_stable_helium4() {
                    *proton_opt = None;
                }
            }
        }
        self.next_slot = 0;
        self.spawn_cooldowns.clear();
    }

    /// Get proton count (excluding stable hydrogen and He4)
    pub fn get_proton_count(&self) -> usize {
        self.protons
            .iter()
            .filter(|p| {
                if let Some(proton) = p {
                    proton.is_alive() && !proton.is_stable_hydrogen() && !proton.is_stable_helium4()
                } else {
                    false
                }
            })
            .count()
    }

    /// Update physics for all protons
    fn update_proton_physics(&mut self, delta_time: f32, window_size: (f32, f32)) {
        for proton_opt in &mut self.protons {
            if let Some(proton) = proton_opt {
                if proton.is_alive() {
                    proton.update(delta_time, window_size);
                }
            }
        }
    }

    /// Apply charge-based forces between protons
    fn apply_charge_forces(&mut self, delta_time: f32) {
        // Collect all charged proton data (H+ and H-)
        let mut charged_protons: Vec<(usize, Vec2, i32, f32)> = Vec::new();
        // Collect neutral H (deuterium) data
        let mut neutral_h: Vec<(usize, Vec2, f32)> = Vec::new();
        // Collect He4 data
        let mut he4_protons: Vec<(usize, Vec2, f32)> = Vec::new();

        for (i, proton_opt) in self.protons.iter().enumerate() {
            if let Some(proton) = proton_opt {
                if proton.is_alive() {
                    let charge = proton.charge();
                    let neutron_count = proton.neutron_count();

                    // H+ (charge=1) and H- (charge=-1) participate in charge forces
                    if charge == 1 || charge == -1 {
                        charged_protons.push((i, proton.position(), charge, proton.mass()));
                    }
                    // H (charge=0, neutron=1) participates in clustering
                    else if charge == 0 && neutron_count == 1 {
                        neutral_h.push((i, proton.position(), proton.mass()));
                    }
                    // He4 (charge=2, neutron=2) participates in clustering
                    else if charge == 2 && neutron_count == 2 {
                        he4_protons.push((i, proton.position(), proton.mass()));
                    }
                }
            }
        }

        // Calculate forces for all pairs
        let mut forces: Vec<Vec2> = vec![Vec2::ZERO; self.protons.len()];

        for i in 0..charged_protons.len() {
            for j in (i + 1)..charged_protons.len() {
                let (idx1, pos1, charge1, mass1) = charged_protons[i];
                let (idx2, pos2, charge2, mass2) = charged_protons[j];

                let delta = pos2 - pos1;
                let dist_squared = delta.length_squared();
                let dist = dist_squared.sqrt();

                // Skip if too far apart
                if dist > pm::CHARGE_INTERACTION_RANGE {
                    continue;
                }

                // Avoid division by zero
                if dist < 1.0 {
                    continue;
                }

                let dir = delta / dist;

                // Same charge = repulsion, opposite charge = attraction
                let force_magnitude = if charge1 == charge2 {
                    // Repulsion (H+ repels H+, H- repels H-)
                    -pm::CHARGE_REPULSION_STRENGTH / (dist_squared + 1.0)
                } else {
                    // Attraction (H+ attracts H-)
                    pm::CHARGE_ATTRACTION_STRENGTH / (dist_squared + 1.0)
                };

                let force = dir * force_magnitude;

                // Apply equal and opposite forces
                forces[idx1] += force;
                forces[idx2] -= force;
            }
        }

        // Calculate H attraction forces (neutral deuterium clustering)
        for i in 0..neutral_h.len() {
            for j in (i + 1)..neutral_h.len() {
                let (idx1, pos1, _mass1) = neutral_h[i];
                let (idx2, pos2, _mass2) = neutral_h[j];

                let delta = pos2 - pos1;
                let dist_squared = delta.length_squared();
                let dist = dist_squared.sqrt();

                // Skip if too far apart
                if dist > pm::H_ATTRACTION_RANGE {
                    continue;
                }

                // Avoid division by zero
                if dist < 1.0 {
                    continue;
                }

                let dir = delta / dist;

                // Attraction force for H clustering
                let force_magnitude = pm::H_ATTRACTION_STRENGTH / (dist_squared + 1.0);
                let force = dir * force_magnitude;

                // Apply equal and opposite forces
                forces[idx1] += force;
                forces[idx2] -= force;
            }
        }

        // Calculate He4 attraction forces (helium clustering)
        for i in 0..he4_protons.len() {
            for j in (i + 1)..he4_protons.len() {
                let (idx1, pos1, _mass1) = he4_protons[i];
                let (idx2, pos2, _mass2) = he4_protons[j];

                let delta = pos2 - pos1;
                let dist_squared = delta.length_squared();
                let dist = dist_squared.sqrt();

                // Skip if too far apart
                if dist > pm::HE4_ATTRACTION_RANGE {
                    continue;
                }

                // Avoid division by zero
                if dist < 1.0 {
                    continue;
                }

                let dir = delta / dist;

                // Attraction force for He4 clustering
                let force_magnitude = pm::HE4_ATTRACTION_STRENGTH / (dist_squared + 1.0);
                let force = dir * force_magnitude;

                // Apply equal and opposite forces
                forces[idx1] += force;
                forces[idx2] -= force;
            }
        }

        // Apply accumulated forces to velocities
        for (i, force) in forces.iter().enumerate() {
            if force.length_squared() > 0.0001 {
                if let Some(proton) = &mut self.protons[i] {
                    if proton.is_alive() {
                        let acceleration = *force / proton.mass();
                        proton.add_velocity(acceleration * delta_time);
                    }
                }
            }
        }
    }

    /// Apply repulsion force from red (low-frequency) waves to H-, He3, He4, and H protons
    fn apply_red_wave_repulsion(&mut self, delta_time: f32, ring_manager: &RingManager) {
        // Get all rings
        let rings = ring_manager.get_all_rings();

        // Collect protons affected by red waves: H-, He3, He4, and H (neutral deuterium)
        let mut affected_protons: Vec<(usize, Vec2, f32)> = Vec::new();
        for (i, proton_opt) in self.protons.iter().enumerate() {
            if let Some(proton) = proton_opt {
                if proton.is_alive() {
                    let charge = proton.charge();
                    let neutron_count = proton.neutron_count();

                    // Check if this proton type is affected by red waves
                    let is_affected = charge == -1  // H-
                        || (charge == 1 && neutron_count == 2)  // He3
                        || (charge == 2 && neutron_count == 2)  // He4
                        || (charge == 0 && neutron_count == 1); // H (neutral deuterium)

                    if is_affected {
                        affected_protons.push((i, proton.position(), proton.mass()));
                    }
                }
            }
        }

        // Calculate repulsion forces from red waves
        let mut forces: Vec<Vec2> = vec![Vec2::ZERO; self.protons.len()];

        for (idx, proton_pos, _mass) in &affected_protons {
            for ring in rings {
                // Check if ring is red/slow (low frequency)
                if ring.get_growth_speed() > pm::RED_WAVE_INTERACTION_THRESHOLD {
                    continue; // Skip fast/blue rings
                }

                // Get ring center and radius
                let ring_center = ring.get_center();
                let ring_radius = ring.get_radius();

                // Calculate distance from proton to ring center
                let delta = *proton_pos - ring_center;
                let dist_to_center = delta.length();

                // Check if proton is near the ring's circumference
                let dist_to_edge = (dist_to_center - ring_radius).abs();

                if dist_to_edge < pm::RED_WAVE_REPULSION_WIDTH {
                    // Proton is near the ring - apply radial repulsion
                    if dist_to_center > 1.0 {
                        let dir = delta / dist_to_center; // Direction away from center

                        // Stronger force when closer to the edge
                        let proximity_factor = 1.0 - (dist_to_edge / pm::RED_WAVE_REPULSION_WIDTH);
                        let force_magnitude = pm::RED_WAVE_REPULSION_STRENGTH * proximity_factor;

                        forces[*idx] += dir * force_magnitude;
                    }
                }
            }
        }

        // Apply forces to affected protons
        for (i, force) in forces.iter().enumerate() {
            if force.length_squared() > 0.0001 {
                if let Some(proton) = &mut self.protons[i] {
                    if proton.is_alive() {
                        let acceleration = *force / proton.mass();
                        proton.add_velocity(acceleration * delta_time);
                    }
                }
            }
        }
    }

    /// Handle solid collisions between H and He4 protons
    fn handle_solid_collisions(&mut self) {
        // Collect solid proton data (H and He4)
        let mut solid_protons: Vec<(usize, Vec2, Vec2, f32, f32)> = Vec::new();

        for (i, proton_opt) in self.protons.iter().enumerate() {
            if let Some(proton) = proton_opt {
                if proton.is_alive() {
                    let charge = proton.charge();
                    let neutron_count = proton.neutron_count();

                    // H (charge=0, neutron=1) and He4 (charge=2, neutron=2) are solid
                    if (charge == 0 && neutron_count == 1) || (charge == 2 && neutron_count == 2) {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                    }
                }
            }
        }

        // Check all pairs for collisions
        for i in 0..solid_protons.len() {
            for j in (i + 1)..solid_protons.len() {
                let (idx1, pos1, vel1, r1, m1) = solid_protons[i];
                let (idx2, pos2, vel2, r2, m2) = solid_protons[j];

                let delta = pos2 - pos1;
                let dist = delta.length();
                let collision_dist = r1 + r2;

                // Check if colliding
                if dist < collision_dist && dist > 0.1 {
                    let normal = delta / dist;

                    // Calculate relative velocity
                    let rel_vel = vel1 - vel2;
                    let vel_along_normal = rel_vel.dot(normal);

                    // Don't resolve if velocities are separating
                    if vel_along_normal < 0.0 {
                        continue;
                    }

                    // Calculate impulse
                    let elasticity = pm::COLLISION_ELASTICITY;
                    let impulse_magnitude = -(1.0 + elasticity) * vel_along_normal / (1.0 / m1 + 1.0 / m2);
                    let impulse = normal * impulse_magnitude;

                    // Apply impulse to both protons
                    if let Some(p1) = &mut self.protons[idx1] {
                        p1.add_velocity(-impulse / m1);
                    }
                    if let Some(p2) = &mut self.protons[idx2] {
                        p2.add_velocity(impulse / m2);
                    }
                }
            }
        }
    }

    /// Check if proton is near any atom
    fn is_near_atom(&self, proton_pos: Vec2, atom_manager: &AtomManager) -> bool {
        // Simple distance check - 50px proximity threshold
        let atoms = atom_manager.get_atoms();

        for atom_opt in atoms {
            if let Some(atom) = atom_opt {
                if atom.is_alive() {
                    let atom_pos = atom.get_position();
                    let dx = proton_pos.x - atom_pos.x;
                    let dy = proton_pos.y - atom_pos.y;
                    let dist_squared = dx * dx + dy * dy;

                    if dist_squared < 50.0 * 50.0 {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Find nearby atom position for electron capture
    fn find_nearby_atom(&self, proton_pos: Vec2, atom_manager: &AtomManager) -> Option<Vec2> {
        // Find closest alive atom within 15px (ELECTRON_CAPTURE_DISTANCE)
        let atoms = atom_manager.get_atoms();
        let mut closest_atom_pos: Option<Vec2> = None;
        let mut closest_dist_sq = proton::ELECTRON_CAPTURE_DISTANCE * proton::ELECTRON_CAPTURE_DISTANCE;

        for atom_opt in atoms {
            if let Some(atom) = atom_opt {
                if atom.is_alive() {
                    let atom_pos = atom.get_position();
                    let dx = proton_pos.x - atom_pos.x;
                    let dy = proton_pos.y - atom_pos.y;
                    let dist_squared = dx * dx + dy * dy;

                    if dist_squared < closest_dist_sq {
                        closest_dist_sq = dist_squared;
                        closest_atom_pos = Some(atom_pos);
                    }
                }
            }
        }

        closest_atom_pos
    }

    /// Mark atom at position for deletion
    fn mark_atom_at_position(&self, atom_pos: Vec2, atom_manager: &mut AtomManager) {
        atom_manager.mark_atom_at_position(atom_pos);
    }

    /// Handle nuclear fusion between protons
    fn handle_nuclear_fusion(&mut self, ring_manager: &mut RingManager) {
        // Check all proton pairs for fusion conditions
        for i in 0..self.protons.len() {
            if self.protons[i].is_none() {
                continue;
            }

            let (pos1, vel1, charge1, neutron1, radius1, mass1, energy1) = {
                let p = self.protons[i].as_ref().unwrap();
                if !p.is_alive() || p.is_stable_hydrogen() || p.is_stable_helium4() {
                    continue;
                }
                (p.position(), p.velocity(), p.charge(), p.neutron_count(), p.radius(), p.mass(), p.energy())
            };

            for j in (i + 1)..self.protons.len() {
                if self.protons[j].is_none() {
                    continue;
                }

                let (pos2, vel2, charge2, neutron2, radius2, mass2, energy2) = {
                    let p = self.protons[j].as_ref().unwrap();
                    if !p.is_alive() || p.is_stable_hydrogen() || p.is_stable_helium4() {
                        continue;
                    }
                    (p.position(), p.velocity(), p.charge(), p.neutron_count(), p.radius(), p.mass(), p.energy())
                };

                // Calculate distance
                let distance_sq = pos1.distance_squared(pos2);
                let collision_dist = radius1 + radius2;

                // Not colliding - skip
                if distance_sq > collision_dist * collision_dist {
                    continue;
                }

                // Calculate relative velocity
                let rel_vel = vel1 - vel2;
                let rel_speed = rel_vel.length();

                // FUSION CASE 1: Deuterium (0, neutron=1) + Proton (+1, neutron=0) → Helium-3
                if (charge1 == 0 && neutron1 == 1 && charge2 == 1 && neutron2 == 0) ||
                   (charge2 == 0 && neutron2 == 1 && charge1 == 1 && neutron1 == 0)
                {
                    if rel_speed > proton::DEUTERIUM_FUSION_VELOCITY_THRESHOLD {
                        // Calculate center of mass
                        let total_mass = mass1 + mass2;
                        let center_of_mass = (pos1 * mass1 + pos2 * mass2) / total_mass;
                        let combined_vel = (vel1 * mass1 + vel2 * mass2) / total_mass;

                        // Create Helium-3 in first slot
                        let combined_energy = energy1 + energy2;
                        let mut he3 = Proton::new(
                            center_of_mass,
                            combined_vel,
                            Color::from_rgba(255, 200, 100, 255),
                            combined_energy,
                            1,
                        );
                        he3.set_neutron_count(2);
                        self.protons[i] = Some(he3);

                        // Spawn energy wave (D + H+ → He3)
                        ring_manager.add_energy_ring(center_of_mass, combined_energy);

                        // Delete second proton
                        self.protons[j] = None;
                        break;
                    }
                }
                // FUSION CASE 2: Helium-3 + Helium-3 → Helium-4 + 2 protons
                else if charge1 == 1 && neutron1 == 2 && charge2 == 1 && neutron2 == 2 {
                    if rel_speed > proton::HELIUM3_FUSION_VELOCITY_THRESHOLD {
                        // Calculate center of mass
                        let total_mass = mass1 + mass2;
                        let center_of_mass = (pos1 * mass1 + pos2 * mass2) / total_mass;
                        let combined_vel = (vel1 * mass1 + vel2 * mass2) / total_mass;

                        // Create Helium-4 in first slot
                        let combined_energy = energy1 + energy2;
                        let mut he4 = Proton::new(
                            center_of_mass,
                            combined_vel,
                            Color::from_rgba(255, 255, 100, 255),
                            combined_energy * 0.5,
                            2,
                        );
                        he4.set_neutron_count(2);
                        he4.set_max_lifetime(-1.0); // Helium-4 is stable
                        self.protons[i] = Some(he4);

                        // Spawn BIG energy waves with random colors between blue and white
                        // Blue = (0,0,1), White = (1,1,1)
                        // Random interpolation: (t, t, 1) where t ∈ [0,1]
                        use macroquad::rand::gen_range;
                        let t1 = gen_range(0.0, 1.0);
                        let color1 = Color::new(t1, t1, 1.0, 1.0);
                        ring_manager.add_ring_with_color(center_of_mass, color1);

                        let t2 = gen_range(0.0, 1.0);
                        let color2 = Color::new(t2, t2, 1.0, 1.0);
                        ring_manager.add_ring_with_color(center_of_mass, color2);

                        // Spawn 2 high-energy protons
                        let release_speed = 200.0;
                        let perp_vel = vec2(-rel_vel.y, rel_vel.x);
                        let perp_len = perp_vel.length();
                        let perp_dir = if perp_len > 0.001 {
                            perp_vel / perp_len
                        } else {
                            vec2(1.0, 0.0)
                        };

                        self.spawn_proton(
                            center_of_mass + perp_dir * 10.0,
                            perp_dir * release_speed,
                            WHITE,
                            combined_energy * 0.25,
                            1,
                        );
                        self.spawn_proton(
                            center_of_mass - perp_dir * 10.0,
                            -perp_dir * release_speed,
                            WHITE,
                            combined_energy * 0.25,
                            1,
                        );

                        // Delete second He3
                        self.protons[j] = None;
                        break;
                    }
                }
                // FUSION CASE 3: H- (charge=-1) + H+ (charge=1) → He3 + energy
                else if (charge1 == -1 && neutron1 == 0 && charge2 == 1 && neutron2 == 0) ||
                        (charge2 == -1 && neutron2 == 0 && charge1 == 1 && neutron1 == 0)
                {
                    // No velocity threshold - attraction brings them together naturally
                    // Calculate center of mass
                    let total_mass = mass1 + mass2;
                    let center_of_mass = (pos1 * mass1 + pos2 * mass2) / total_mass;
                    let combined_vel = (vel1 * mass1 + vel2 * mass2) / total_mass;

                    // Create Helium-3 in first slot
                    let combined_energy = energy1 + energy2;
                    let mut he3 = Proton::new(
                        center_of_mass,
                        combined_vel,
                        Color::from_rgba(255, 200, 100, 255),
                        combined_energy,
                        1,
                    );
                    he3.set_neutron_count(2);
                    self.protons[i] = Some(he3);

                    // Spawn energy wave (H- + H+ → He3)
                    ring_manager.add_energy_ring(center_of_mass, combined_energy);

                    // Delete second proton
                    self.protons[j] = None;
                    break;
                }
            }
        }
    }

    /// Detect atom collisions and spawn protons
    fn detect_and_spawn_from_atom_collisions(&mut self, atom_manager: &AtomManager) {
        // Struct to hold safe snapshot of atom data (no lifetimes)
        struct AtomSnapshot {
            position: Vec2,
            energy: f32,
        }

        // 1. Create safe snapshots of all high-energy atoms
        let mut high_energy_atoms = Vec::new();
        let atoms = atom_manager.get_atoms();

        for atom_opt in atoms {
            if let Some(atom) = atom_opt {
                if atom.is_alive() && atom.get_energy() >= pm::MIN_ATOM_ENERGY_THRESHOLD {
                    high_energy_atoms.push(AtomSnapshot {
                        position: atom.get_position(),
                        energy: atom.get_energy(),
                    });
                }
            }
        }

        // 2. Check distances between all atom snapshot pairs
        for i in 0..high_energy_atoms.len() {
            for j in (i + 1)..high_energy_atoms.len() {
                let atom1 = &high_energy_atoms[i];
                let atom2 = &high_energy_atoms[j];

                // 3. Calculate distance between atoms
                let dx = atom2.position.x - atom1.position.x;
                let dy = atom2.position.y - atom1.position.y;
                let dist_squared = dx * dx + dy * dy;

                // Collision threshold (atoms are close)
                let collision_threshold_sq = pm::COLLISION_THRESHOLD * pm::COLLISION_THRESHOLD;

                // 4. If atoms collide and have sufficient combined energy, spawn a proton
                if dist_squared < collision_threshold_sq {
                    let combined_energy = atom1.energy + atom2.energy;

                    if combined_energy >= pm::MIN_COMBINED_ENERGY {
                        // Calculate spawn position (midpoint between atoms)
                        let spawn_pos = vec2(
                            (atom1.position.x + atom2.position.x) * 0.5,
                            (atom1.position.y + atom2.position.y) * 0.5,
                        );

                        // Check if this position is on cooldown
                        let mut has_cooldown = false;
                        let cooldown_dist_sq = pm::COOLDOWN_DISTANCE * pm::COOLDOWN_DISTANCE;

                        for cooldown in &self.spawn_cooldowns {
                            let cdx = spawn_pos.x - cooldown.0.x;
                            let cdy = spawn_pos.y - cooldown.0.y;
                            let cd_dist_sq = cdx * cdx + cdy * cdy;

                            if cd_dist_sq < cooldown_dist_sq {
                                has_cooldown = true;
                                break;
                            }
                        }

                        if has_cooldown {
                            continue;
                        }

                        // Calculate velocity (perpendicular to collision line, based on energy)
                        let mut collision_dir = vec2(dx, dy);
                        let dist = dist_squared.sqrt();
                        if dist > EPSILON {
                            collision_dir /= dist;
                        }

                        // Perpendicular direction (rotate 90 degrees)
                        let perp_dir = vec2(-collision_dir.y, collision_dir.x);
                        let speed = (combined_energy * pm::VELOCITY_ENERGY_FACTOR).min(pm::MAX_SPAWN_SPEED);
                        let velocity = perp_dir * speed;

                        // Proton color (white for now)
                        let proton_color = WHITE;

                        // Determine charge based on combined energy
                        let charge = if combined_energy >= pm::NEGATIVE_PROTON_ENERGY_THRESHOLD {
                            -1
                        } else {
                            1
                        };

                        // Spawn the proton
                        self.spawn_proton(spawn_pos, velocity, proton_color, combined_energy, charge);

                        // 5. Add cooldown to prevent duplicate spawns
                        self.spawn_cooldowns.push((spawn_pos, pm::SPAWN_COOLDOWN_TIME));
                    }
                }
            }
        }
    }

    /// Spawn a new proton
    fn spawn_proton(&mut self, position: Vec2, velocity: Vec2, color: Color, energy: f32, charge: i32) {
        // Check if at capacity
        if self.get_proton_count() >= self.max_protons {
            return;
        }

        // Find first empty slot
        for i in 0..self.protons.len() {
            if self.protons[i].is_none() || !self.protons[i].as_ref().unwrap().is_alive() {
                let mut proton = Proton::new(position, velocity, color, energy, charge);

                // Make H+ protons permanent (infinite lifetime)
                // H- decays like He3 (default 20s lifetime)
                if charge == 1 {
                    proton.set_max_lifetime(proton::INFINITE_LIFETIME);
                }

                self.protons[i] = Some(proton);

                break;
            }
        }
    }

    /// Update spawn cooldowns
    fn update_cooldowns(&mut self, delta_time: f32) {
        // Decrease all cooldown timers
        for cooldown in &mut self.spawn_cooldowns {
            cooldown.1 -= delta_time;
        }

        // Remove expired cooldowns
        self.spawn_cooldowns.retain(|cooldown| cooldown.1 > 0.0);
    }
}
