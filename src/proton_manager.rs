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
    elapsed_time: f32, // Total elapsed time for tracking wave hits
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
            elapsed_time: 0.0,
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
        // Track elapsed time
        self.elapsed_time += delta_time;

        // Update cooldowns
        self.update_cooldowns(delta_time);

        // STEP 1: Simple straight-line physics
        self.update_proton_physics(delta_time, window_size);

        // STEP 2: Charge-based forces (H+/H- interactions and H clustering)
        self.apply_charge_forces(delta_time);

        // STEP 2.5: Red wave repulsion (only affects H-)
        self.apply_red_wave_repulsion(delta_time, ring_manager);

        // STEP 2.6: H crystallization (phase transitions)
        self.update_h_crystallization(delta_time);

        // STEP 2.7: O16 bond forces and breaking
        self.update_oxygen_bonds(delta_time);

        // STEP 2.8: Water hydrogen bonds (polarity-based bonding)
        self.update_water_hydrogen_bonds(delta_time);

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
                    // Never remove stable particles: H1, He4, C12, O16 bonded, H2O, Ne20, Mg24, Si28, S32, and hydrogen compounds
                    if !proton.is_stable_hydrogen()
                        && !proton.is_stable_helium4()
                        && !proton.is_stable_carbon12()
                        && !proton.is_oxygen16_bonded()
                        && !proton.is_h2o()
                        && !proton.is_neon20()
                        && !proton.is_magnesium24()
                        && !proton.is_silicon28()
                        && !proton.is_sulfur32()
                        && !proton.is_h2s()
                        && !proton.is_mgh2()
                        && !proton.is_ch4()
                        && !proton.is_sih4() {
                        *proton_opt = None;
                    }
                }
            }
        }
    }

    /// Draw all protons
    pub fn draw(&self, segments: i32) {
        // First draw crystal bonds
        self.draw_crystal_bonds();

        // Then draw oxygen bonds
        self.draw_oxygen_bonds();

        // Then draw water hydrogen bonds
        self.draw_water_hydrogen_bonds();

        // Then draw protons on top
        for proton_opt in &self.protons {
            if let Some(proton) = proton_opt {
                if proton.is_alive() {
                    proton.render(segments);
                }
            }
        }
    }

    /// Draw crystal bond lines for hexagonal ice structure
    fn draw_crystal_bonds(&self) {
        for (i, proton_opt) in self.protons.iter().enumerate() {
            if let Some(proton) = proton_opt {
                if proton.is_alive() && proton.is_crystallized() {
                    let pos1 = proton.position();
                    let bonds = proton.crystal_bonds();

                    // Draw bond lines to each bonded neighbor
                    for bond_idx in bonds {
                        // Only draw each bond once (from lower index to higher)
                        if *bond_idx > i {
                            if let Some(other_proton) = &self.protons[*bond_idx] {
                                if other_proton.is_alive() && other_proton.is_crystallized() {
                                    let pos2 = other_proton.position();

                                    // Draw thin white/cyan line for bond
                                    let bond_color = Color::from_rgba(180, 220, 255, 180);
                                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 1.5, bond_color);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Draw oxygen bond lines for O16 bonded pairs (C12 + He4)
    fn draw_oxygen_bonds(&self) {
        for (i, proton_opt) in self.protons.iter().enumerate() {
            if let Some(proton) = proton_opt {
                if proton.is_alive() && proton.is_oxygen16_bonded() {
                    if let Some(partner_idx) = proton.oxygen_bond_partner() {
                        // Only draw each bond once (from lower index to higher)
                        if partner_idx > i {
                            if let Some(partner) = &self.protons[partner_idx] {
                                if partner.is_alive() && partner.is_oxygen16_bonded() {
                                    let pos1 = proton.position();
                                    let pos2 = partner.position();

                                    // Draw light blue line for O16 bond
                                    let bond_color = Color::from_rgba(100, 180, 255, 200);
                                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 2.0, bond_color);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Draw water hydrogen bond lines for H2O polar bonding
    fn draw_water_hydrogen_bonds(&self) {
        for (i, proton_opt) in self.protons.iter().enumerate() {
            if let Some(proton) = proton_opt {
                if proton.is_alive() && proton.is_h2o() {
                    let pos1 = proton.position();
                    let bonds = proton.water_h_bonds();

                    // Draw bond lines to each bonded water molecule
                    for bond_idx in bonds {
                        // Only draw each bond once (from lower index to higher)
                        if *bond_idx > i {
                            if let Some(other_proton) = &self.protons[*bond_idx] {
                                if other_proton.is_alive() && other_proton.is_h2o() {
                                    let pos2 = other_proton.position();

                                    // Check if both molecules are frozen (ice bond)
                                    let both_frozen = proton.is_water_frozen() && other_proton.is_water_frozen();

                                    // Draw line - brighter and thicker for frozen ice bonds
                                    let (bond_color, thickness) = if both_frozen {
                                        (Color::from_rgba(180, 220, 255, 200), 2.5) // Bright cyan for ice
                                    } else {
                                        (Color::from_rgba(100, 150, 200, 120), 1.2) // Faint blue for liquid
                                    };
                                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, thickness, bond_color);
                                }
                            }
                        }
                    }
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
                    let font_size = 18.0;
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
                // Preserve stable H1, He4, C12, O16 bonded, H2O, Ne20, Mg24, Si28, S32, and hydrogen compounds
                if !proton.is_stable_hydrogen()
                    && !proton.is_stable_helium4()
                    && !proton.is_stable_carbon12()
                    && !proton.is_oxygen16_bonded()
                    && !proton.is_h2o()
                    && !proton.is_neon20()
                    && !proton.is_magnesium24()
                    && !proton.is_silicon28()
                    && !proton.is_sulfur32()
                    && !proton.is_h2s()
                    && !proton.is_mgh2()
                    && !proton.is_ch4()
                    && !proton.is_sih4() {
                    *proton_opt = None;
                }
            }
        }
        self.next_slot = 0;
        self.spawn_cooldowns.clear();
    }

    /// Delete all stable H protons
    pub fn delete_stable_hydrogen(&mut self) {
        for proton_opt in &mut self.protons {
            if let Some(proton) = proton_opt {
                if proton.is_stable_hydrogen() {
                    *proton_opt = None;
                }
            }
        }
    }

    /// Clear ALL protons including stable/immortal elements
    pub fn clear_all(&mut self) {
        for proton_opt in &mut self.protons {
            *proton_opt = None;
        }
    }

    /// Get proton count (excluding stable hydrogen, He4, C12, O16 bonded, H2O, Ne20, Mg24, Si28, S32, and hydrogen compounds)
    pub fn get_proton_count(&self) -> usize {
        self.protons
            .iter()
            .filter(|p| {
                if let Some(proton) = p {
                    proton.is_alive()
                        && !proton.is_stable_hydrogen()
                        && !proton.is_stable_helium4()
                        && !proton.is_stable_carbon12()
                        && !proton.is_oxygen16_bonded()
                        && !proton.is_h2o()
                        && !proton.is_neon20()
                        && !proton.is_magnesium24()
                        && !proton.is_silicon28()
                        && !proton.is_sulfur32()
                        && !proton.is_h2s()
                        && !proton.is_mgh2()
                        && !proton.is_ch4()
                        && !proton.is_sih4()
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
    /// Dark red waves (lowest 5 colors) MELT ice bonds after 5 hits
    /// NOTE: C12, O16 bonded pairs, and H2O are intentionally excluded from red wave repulsion
    fn apply_red_wave_repulsion(&mut self, delta_time: f32, ring_manager: &RingManager) {
        // Get all rings
        let rings = ring_manager.get_all_rings();

        // Collect protons affected by red waves: H-, He3, He4, H (neutral deuterium), and H2O
        // C12 and O16 bonded pairs are NOT affected by red waves (stable heavy particles)
        let mut affected_protons: Vec<(usize, Vec2, f32, bool)> = Vec::new();
        for (i, proton_opt) in self.protons.iter().enumerate() {
            if let Some(proton) = proton_opt {
                if proton.is_alive() {
                    let charge = proton.charge();
                    let neutron_count = proton.neutron_count();

                    // Skip O16 bonded particles
                    if proton.is_oxygen16_bonded() {
                        continue;
                    }

                    // Check if this proton type is affected by red waves
                    // C12 (charge=6, neutron_count=6) is intentionally NOT included here
                    let is_affected = charge == -1  // H-
                        || (charge == 1 && neutron_count == 2)  // He3
                        || (charge == 2 && neutron_count == 2)  // He4
                        || (charge == 0 && neutron_count == 1)  // H (neutral deuterium)
                        || proton.is_h2o(); // H2O molecules

                    if is_affected {
                        let is_frozen = proton.is_crystallized();
                        affected_protons.push((i, proton.position(), proton.mass(), is_frozen));
                    }
                }
            }
        }

        // Calculate repulsion forces from red waves and detect melting hits
        let mut forces: Vec<Vec2> = vec![Vec2::ZERO; self.protons.len()];
        let mut hit_by_dark_red: Vec<bool> = vec![false; self.protons.len()];

        for (idx, proton_pos, _mass, is_frozen) in &affected_protons {
            for ring in rings {
                let ring_speed = ring.get_growth_speed();

                // Check if ring is red/slow (low frequency)
                if ring_speed > pm::RED_WAVE_INTERACTION_THRESHOLD {
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
                    // Proton is near the ring
                    if dist_to_center > 1.0 {
                        let dir = delta / dist_to_center; // Direction away from center
                        let proximity_factor = 1.0 - (dist_to_edge / pm::RED_WAVE_REPULSION_WIDTH);

                        // MELTING: Track hits from dark red waves (lowest 5 colors)
                        if *is_frozen && ring_speed <= pm::DARK_RED_WAVE_SPEED_THRESHOLD {
                            hit_by_dark_red[*idx] = true;
                        }

                        // Apply radial repulsion force
                        let force_magnitude = pm::RED_WAVE_REPULSION_STRENGTH * proximity_factor;
                        forces[*idx] += dir * force_magnitude;
                    }
                }
            }
        }

        // Process dark red wave hits and melting
        for (i, was_hit) in hit_by_dark_red.iter().enumerate() {
            if *was_hit {
                if let Some(proton) = &mut self.protons[i] {
                    if proton.is_alive() && proton.is_crystallized() {
                        // Check if enough time has passed since last hit (prevent double-counting same wave)
                        let time_since_last_hit = self.elapsed_time - proton.last_red_wave_hit_time();

                        if time_since_last_hit >= pm::RED_WAVE_HIT_COOLDOWN {
                            // Increment hit counter (unique wave)
                            proton.increment_red_wave_hits();
                            proton.set_last_red_wave_hit_time(self.elapsed_time);

                            // Check if we've reached melting threshold
                            if proton.red_wave_hits() >= pm::RED_WAVE_HITS_TO_MELT {
                                // MELT: Break crystal bonds and decrystallize
                                proton.set_crystallized(false);
                                proton.clear_crystal_bonds();
                                proton.reset_red_wave_hits();
                                proton.set_freeze_cooldown(pm::H_CRYSTAL_FREEZE_COOLDOWN);

                                // Add outward "melting" velocity
                                if forces[i].length() > 0.01 {
                                    let escape_dir = forces[i].normalize();
                                    proton.add_velocity(escape_dir * 30.0);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Apply repulsion forces to non-frozen protons
        for (i, force) in forces.iter().enumerate() {
            if force.length_squared() > 0.0001 {
                if let Some(proton) = &mut self.protons[i] {
                    if proton.is_alive() && !proton.is_crystallized() {
                        let acceleration = *force / proton.mass();
                        proton.add_velocity(acceleration * delta_time);
                    }
                }
            }
        }
    }

    /// Update H crystallization (gas/liquid/solid phase transitions)
    /// Creates simple hexagons: 1 center + 6 sides arranged equidistantly
    fn update_h_crystallization(&mut self, delta_time: f32) {
        // Collect all H (neutral deuterium) protons
        let mut h_protons: Vec<(usize, Vec2)> = Vec::new();
        for (i, proton_opt) in self.protons.iter().enumerate() {
            if let Some(proton) = proton_opt {
                if proton.is_alive() && proton.charge() == 0 && proton.neutron_count() == 1 {
                    h_protons.push((i, proton.position()));
                }
            }
        }

        // Build neighbor lists for each H (with minimum spacing filter)
        let mut neighbor_lists: Vec<Vec<usize>> = vec![Vec::new(); self.protons.len()];
        for i in 0..h_protons.len() {
            for j in (i + 1)..h_protons.len() {
                let (idx1, pos1) = h_protons[i];
                let (idx2, pos2) = h_protons[j];

                let dist = pos1.distance(pos2);

                // Only count as neighbors if within range AND not too close
                if dist >= pm::H_CRYSTAL_MIN_SPACING && dist < pm::H_CRYSTAL_NEIGHBOR_DISTANCE {
                    neighbor_lists[idx1].push(idx2);
                    neighbor_lists[idx2].push(idx1);
                }
            }
        }

        // Find clusters of exactly 7 H particles and assign center + 6 sides
        let mut is_center: Vec<bool> = vec![false; self.protons.len()];
        let mut center_bonds: Vec<Vec<usize>> = vec![Vec::new(); self.protons.len()];

        for (idx, pos) in &h_protons {
            // Check if this proton is on freeze cooldown
            let on_cooldown = if let Some(proton) = &self.protons[*idx] {
                proton.freeze_cooldown() > 0.0
            } else {
                false
            };

            // Skip crystallization if on cooldown
            if on_cooldown {
                if let Some(proton) = &mut self.protons[*idx] {
                    proton.set_crystallized(false);
                    proton.clear_crystal_bonds();
                }
                continue;
            }

            let neighbors = &neighbor_lists[*idx];

            // Need exactly 6 or 7 neighbors to form a hexagon
            if neighbors.len() >= 6 {
                // Find 6 nearest neighbors
                let mut neighbors_with_dist: Vec<(usize, f32)> = neighbors
                    .iter()
                    .filter_map(|&n_idx| {
                        if let Some(n_proton) = &self.protons[n_idx] {
                            let dist = pos.distance(n_proton.position());
                            Some((n_idx, dist))
                        } else {
                            None
                        }
                    })
                    .collect();

                neighbors_with_dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let six_nearest: Vec<usize> = neighbors_with_dist
                    .iter()
                    .take(6)
                    .map(|(idx, _)| *idx)
                    .collect();

                // This particle becomes a center with 6 sides
                is_center[*idx] = true;
                center_bonds[*idx] = six_nearest.clone();

                // Mark all as crystallized
                if let Some(proton) = &mut self.protons[*idx] {
                    proton.set_crystallized(true);
                    proton.set_crystal_bonds(six_nearest);
                }
            } else {
                // Not enough neighbors - decrystallize
                if let Some(proton) = &mut self.protons[*idx] {
                    proton.set_crystallized(false);
                    proton.clear_crystal_bonds();
                    proton.reset_red_wave_hits(); // Reset melt counter when decrystallizing
                }
            }
        }

        // Apply hexagonal arrangement forces
        let mut forces: Vec<Vec2> = vec![Vec2::ZERO; self.protons.len()];

        for (idx, pos) in &h_protons {
            if !is_center[*idx] {
                continue; // Only centers apply forces
            }

            let side_indices = center_bonds[*idx].clone();
            if side_indices.is_empty() {
                continue;
            }

            // Calculate ideal hexagon positions around center
            let ideal_angles: Vec<f32> = (0..6)
                .map(|i| (i as f32) * std::f32::consts::PI / 3.0)
                .collect();

            // Apply forces to arrange sides in perfect hexagon
            for (i, &side_idx) in side_indices.iter().enumerate() {
                if let Some(side_proton) = &self.protons[side_idx] {
                    let side_pos = side_proton.position();
                    let delta = side_pos - *pos;
                    let dist = delta.length();

                    if dist > 0.1 && dist < pm::H_CRYSTAL_BREAKOFF_DISTANCE {
                        // Force 1: Radial - maintain correct distance from center
                        let radial_displacement = dist - pm::H_CRYSTAL_BOND_REST_LENGTH;
                        let radial_force_mag = radial_displacement * pm::H_CRYSTAL_BOND_STRENGTH;
                        let radial_dir = delta / dist;
                        let radial_force = radial_dir * radial_force_mag;

                        // Force 2: Angular - push to ideal angle position
                        let current_angle = delta.y.atan2(delta.x);
                        let ideal_angle = ideal_angles[i % 6];
                        let angle_diff = ideal_angle - current_angle;

                        // Perpendicular direction for angular force
                        let perp_dir = vec2(-radial_dir.y, radial_dir.x);
                        let angular_force = perp_dir * (angle_diff * pm::H_CRYSTAL_BOND_STRENGTH * 0.5);

                        forces[side_idx] += radial_force + angular_force;
                    }
                }
            }
        }

        // Collect non-frozen H positions for breakoff checking
        let non_frozen_h: Vec<Vec2> = h_protons
            .iter()
            .filter_map(|(idx, pos)| {
                if let Some(proton) = &self.protons[*idx] {
                    if !proton.is_crystallized() {
                        Some(*pos)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Check which side particles can break off (ignore frozen H when checking space)
        let mut can_break_off: Vec<bool> = vec![false; self.protons.len()];
        for (idx, pos) in &h_protons {
            if is_center[*idx] {
                continue; // Centers never break off
            }

            if let Some(proton) = &self.protons[*idx] {
                if !proton.is_crystallized() {
                    continue; // Only check crystallized sides
                }

                // Check if there's space around this side particle
                // Only non-frozen H particles block the space
                let mut has_space = false;
                for angle in [0.0, std::f32::consts::PI / 2.0, std::f32::consts::PI, 3.0 * std::f32::consts::PI / 2.0] {
                    let dir = vec2(angle.cos(), angle.sin());
                    let test_pos = *pos + dir * pm::H_CRYSTAL_VIBRATION_THRESHOLD;

                    let mut space_clear = true;
                    for other_pos in &non_frozen_h {
                        if test_pos.distance(*other_pos) < pm::H_CRYSTAL_NEIGHBOR_DISTANCE {
                            space_clear = false;
                            break;
                        }
                    }

                    if space_clear {
                        has_space = true;
                        break;
                    }
                }

                can_break_off[*idx] = has_space;
            }
        }

        // Apply forces and freeze when in position
        for (i, force) in forces.iter().enumerate() {
            if let Some(proton) = &mut self.protons[i] {
                if proton.is_alive() && proton.is_crystallized() {
                    if is_center[i] {
                        // Center: FREEZE completely
                        proton.set_velocity(Vec2::ZERO);
                    } else {
                        // Sides: check if can break off
                        if can_break_off[i] {
                            // Has space to evaporate - decrystallize and release
                            proton.set_crystallized(false);
                            proton.clear_crystal_bonds();
                            proton.reset_red_wave_hits(); // Reset melt counter on sublimation
                            // Add small outward velocity
                            if force.length() > 0.01 {
                                let escape_dir = force.normalize();
                                proton.set_velocity(escape_dir * 20.0);
                            }
                        } else {
                            // No space or still arranging - apply forces or freeze
                            let force_magnitude = force.length();

                            if force_magnitude > 0.0001 {
                                // Still arranging
                                let acceleration = *force / proton.mass();
                                proton.add_velocity(acceleration * delta_time);
                            } else {
                                // Settled - freeze in position
                                proton.set_velocity(Vec2::ZERO);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Update O16 molecular bonds (spring forces and breaking)
    fn update_oxygen_bonds(&mut self, delta_time: f32) {
        // Collect all O16 bonded pairs
        let mut bonded_pairs: Vec<(usize, usize, Vec2, Vec2, f32, f32, f32)> = Vec::new();

        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_oxygen16_bonded() {
                    if let Some(partner_idx) = proton.oxygen_bond_partner() {
                        // Only process each pair once
                        if partner_idx > i {
                            if let Some(partner) = &self.protons[partner_idx] {
                                if partner.is_alive() && partner.is_oxygen16_bonded() {
                                    bonded_pairs.push((
                                        i,
                                        partner_idx,
                                        proton.position(),
                                        partner.position(),
                                        proton.mass(),
                                        partner.mass(),
                                        proton.oxygen_bond_rest_length(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Apply spring forces to maintain bonds and check for breaking
        let mut bonds_to_break: Vec<(usize, usize)> = Vec::new();

        for (idx1, idx2, pos1, pos2, m1, m2, rest_length) in bonded_pairs {
            let delta = pos2 - pos1;
            let dist = delta.length();

            // Check if bond should break
            if dist > proton::OXYGEN16_BREAKING_DISTANCE {
                bonds_to_break.push((idx1, idx2));
                continue;
            }

            // Apply spring force to maintain bond distance
            if dist > 0.1 {
                let displacement = dist - rest_length;
                let force_magnitude = displacement * proton::OXYGEN16_BOND_STRENGTH;
                let dir = delta / dist;
                let force = dir * force_magnitude;

                // Apply forces to both particles
                if let Some(p1) = &mut self.protons[idx1] {
                    let acc1 = force / m1;
                    p1.add_velocity(acc1 * delta_time);
                }
                if let Some(p2) = &mut self.protons[idx2] {
                    let acc2 = -force / m2;
                    p2.add_velocity(acc2 * delta_time);
                }
            }
        }

        // Break bonds that are too stretched
        for (idx1, idx2) in bonds_to_break {
            if let Some(p1) = &mut self.protons[idx1] {
                p1.clear_oxygen_bond();
            }
            if let Some(p2) = &mut self.protons[idx2] {
                p2.clear_oxygen_bond();
            }
        }
    }

    /// Update water hydrogen bonds - polar molecules form weak triangular clusters
    fn update_water_hydrogen_bonds(&mut self, delta_time: f32) {
        use std::f32::consts::PI;

        // PHASE 1: Collect all H2O molecules and initialize polar angles if needed
        let mut water_molecules: Vec<(usize, Vec2, Vec2, f32, f32)> = Vec::new();

        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_h2o() {
                    water_molecules.push((
                        i,
                        proton.position(),
                        proton.velocity(),
                        proton.mass(),
                        proton.water_polar_angle(),
                    ));
                }
            }
        }

        // PHASE 2: Initialize polar angles for new water molecules (random orientation)
        // Also check for evaporation (too much speed breaks bonds)
        for (idx, _, vel, _, angle) in &water_molecules {
            // Check if moving too fast (evaporation)
            let speed = vel.length();
            if speed > proton::WATER_EVAPORATION_SPEED {
                // Moving too fast - break all bonds (evaporation)
                if let Some(proton) = &mut self.protons[*idx] {
                    proton.clear_water_h_bonds();
                }
                continue;
            }

            if *angle == 0.0 {
                // Initialize with velocity direction + some randomness
                let vel_angle = vel.y.atan2(vel.x);
                let random_offset = (rand::gen_range(0.0, 1.0) - 0.5) * PI;
                let new_angle = vel_angle + random_offset;

                if let Some(proton) = &mut self.protons[*idx] {
                    proton.set_water_polar_angle(new_angle);
                }
            }
        }

        // PHASE 3: Clear existing bonds (we'll rebuild them each frame)
        for (idx, _, _, _, _) in &water_molecules {
            if let Some(proton) = &mut self.protons[*idx] {
                proton.clear_water_h_bonds();
            }
        }

        // PHASE 4: Form new bonds based on polarity
        for i in 0..water_molecules.len() {
            let (idx_a, pos_a, _, _, angle_a) = water_molecules[i];

            // Calculate pole positions for molecule A
            // Negative pole at angle, positive pole at angle + PI
            let neg_pole_a = Vec2::new(
                pos_a.x + angle_a.cos() * 30.0,
                pos_a.y + angle_a.sin() * 30.0,
            );
            let pos_pole_a = Vec2::new(
                pos_a.x + (angle_a + PI).cos() * 30.0,
                pos_a.y + (angle_a + PI).sin() * 30.0,
            );

            // Count existing bonds from negative and positive sides
            // All H2O can have up to 5 bonds total (3 neg + 2 pos)
            // 0-2 bonds = liquid, 3-5 bonds = frozen and stationary
            let mut neg_bonds = 0;
            let mut pos_bonds = 0;

            if let Some(proton_a) = &self.protons[idx_a] {
                let total_bonds = proton_a.water_h_bonds().len();
                // Layout: 3 negative bonds first, then 2 positive bonds
                neg_bonds = total_bonds.min(3);
                pos_bonds = if total_bonds > 3 {
                    (total_bonds - 3).min(2)
                } else {
                    0
                };
            }

            // Check nearby water molecules
            for j in (i + 1)..water_molecules.len() {
                let (idx_b, pos_b, _, _, angle_b) = water_molecules[j];

                let dist = pos_a.distance(pos_b);
                if dist > proton::WATER_H_BOND_RANGE {
                    continue;
                }

                // Calculate pole positions for molecule B
                let neg_pole_b = Vec2::new(
                    pos_b.x + angle_b.cos() * 30.0,
                    pos_b.y + angle_b.sin() * 30.0,
                );
                let pos_pole_b = Vec2::new(
                    pos_b.x + (angle_b + PI).cos() * 30.0,
                    pos_b.y + (angle_b + PI).sin() * 30.0,
                );

                // Check bond limits for molecule B
                // All H2O can have up to 3 negative bonds, 2 positive bonds
                let b_neg_bonds = if let Some(p) = &self.protons[idx_b] {
                    let total = p.water_h_bonds().len();
                    total.min(3) // All H2O can have up to 3 negative bonds
                } else {
                    0
                };
                let b_pos_bonds = if let Some(p) = &self.protons[idx_b] {
                    let total = p.water_h_bonds().len();
                    if total > 3 {
                        (total - 3).min(2) // 2 positive bonds after 3 negative
                    } else {
                        0
                    }
                } else {
                    0
                };

                // Try to form bonds based on polarity attraction
                // Negative A to Positive B (if both have capacity)
                // All H2O can have up to 3 negative bonds
                if neg_bonds < 3 && b_pos_bonds < 2 {
                    let bond_dist = neg_pole_a.distance(pos_pole_b);

                    // Check if either molecule has exactly 2 bonds - if so, use shorter breaking distance
                    let total_a = if let Some(p) = &self.protons[idx_a] { p.water_h_bonds().len() } else { 0 };
                    let total_b = if let Some(p) = &self.protons[idx_b] { p.water_h_bonds().len() } else { 0 };
                    let max_bond_dist = if total_a == 2 || total_b == 2 {
                        proton::WATER_2BOND_BREAKING_DISTANCE
                    } else {
                        proton::WATER_H_BOND_RANGE
                    };

                    if bond_dist < max_bond_dist {
                        if let Some(proton_a) = &mut self.protons[idx_a] {
                            proton_a.add_water_h_bond(idx_b, proton::WATER_H_BOND_REST_LENGTH);
                            neg_bonds += 1;
                        }
                        if let Some(proton_b) = &mut self.protons[idx_b] {
                            proton_b.add_water_h_bond(idx_a, proton::WATER_H_BOND_REST_LENGTH);
                        }
                        continue;
                    }
                }

                // Positive A to Negative B (if both have capacity)
                // All H2O can have up to 2 positive bonds and 3 negative bonds
                if pos_bonds < 2 && b_neg_bonds < 3 {
                    let bond_dist = pos_pole_a.distance(neg_pole_b);

                    // Check if either molecule has exactly 2 bonds - if so, use shorter breaking distance
                    let total_a = if let Some(p) = &self.protons[idx_a] { p.water_h_bonds().len() } else { 0 };
                    let total_b = if let Some(p) = &self.protons[idx_b] { p.water_h_bonds().len() } else { 0 };
                    let max_bond_dist = if total_a == 2 || total_b == 2 {
                        proton::WATER_2BOND_BREAKING_DISTANCE
                    } else {
                        proton::WATER_H_BOND_RANGE
                    };

                    if bond_dist < max_bond_dist {
                        if let Some(proton_a) = &mut self.protons[idx_a] {
                            proton_a.add_water_h_bond(idx_b, proton::WATER_H_BOND_REST_LENGTH);
                            pos_bonds += 1;
                        }
                        if let Some(proton_b) = &mut self.protons[idx_b] {
                            proton_b.add_water_h_bond(idx_a, proton::WATER_H_BOND_REST_LENGTH);
                        }
                    }
                }
            }
        }

        // PHASE 4.5: Hexagonal ice crystal optimization
        // For H2O with 4-5 bonds, check if neighbors form a perfect hexagon
        // If yes, rearrange bonds to prioritize hexagonal pattern for beautiful snowflake crystals
        for i in 0..water_molecules.len() {
            let (idx_a, pos_a, _, _, _) = water_molecules[i];

            if let Some(proton_a) = &self.protons[idx_a] {
                let bond_count = proton_a.water_h_bonds().len();

                // Only optimize if we have 4-5 bonds (potential hexagon core)
                if bond_count >= 4 {
                    // Find 6 closest H2O neighbors
                    let mut neighbors: Vec<(usize, f32, f32)> = Vec::new(); // (index, distance, angle)

                    for j in 0..water_molecules.len() {
                        if i == j {
                            continue;
                        }
                        let (idx_b, pos_b, _, _, _) = water_molecules[j];
                        let delta = pos_b - pos_a;
                        let dist = delta.length();

                        if dist < proton::WATER_H_BOND_RANGE * 1.5 {
                            let angle = delta.y.atan2(delta.x);
                            neighbors.push((idx_b, dist, angle));
                        }
                    }

                    // Sort by distance to get closest 6
                    neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                    if neighbors.len() >= 6 {
                        neighbors.truncate(6);
                    }

                    // Check if these form a hexagonal pattern
                    if neighbors.len() == 6 {
                        // Sort by angle
                        neighbors.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

                        // Check if angles are approximately 60 degrees apart
                        let mut is_hexagon = true;
                        for k in 0..6 {
                            let next_k = (k + 1) % 6;
                            let mut angle_diff = neighbors[next_k].2 - neighbors[k].2;

                            // Normalize angle difference to [0, 2π]
                            if angle_diff < 0.0 {
                                angle_diff += 2.0 * std::f32::consts::PI;
                            }

                            // Check if close to 60 degrees (π/3 radians), allow 20 degree tolerance
                            let expected_angle = std::f32::consts::PI / 3.0; // 60 degrees
                            let tolerance = 0.35; // ~20 degrees in radians

                            if (angle_diff - expected_angle).abs() > tolerance {
                                is_hexagon = false;
                                break;
                            }
                        }

                        // If hexagonal, clear bonds and rebond to these 6 neighbors (max 5 bonds)
                        if is_hexagon {
                            if let Some(p) = &mut self.protons[idx_a] {
                                p.clear_water_h_bonds();
                            }

                            // Bond to 5 of the 6 hexagonal neighbors
                            for k in 0..5.min(neighbors.len()) {
                                if let Some(p) = &mut self.protons[idx_a] {
                                    p.add_water_h_bond(neighbors[k].0, proton::WATER_H_BOND_REST_LENGTH);
                                }
                                // Also bond the neighbor back to us
                                if let Some(p) = &mut self.protons[neighbors[k].0] {
                                    if !p.water_h_bonds().contains(&idx_a) {
                                        p.add_water_h_bond(idx_a, proton::WATER_H_BOND_REST_LENGTH);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // PHASE 5: Check for compression and freeze H2O into ice
        // When H2O molecules form 3+ bonds (all compressed), they freeze into ice
        // 0-2 bonds = liquid, 3-5 bonds = frozen and stationary
        for (idx, pos, vel, _, _) in &water_molecules {
            if let Some(proton) = &self.protons[*idx] {
                let speed = vel.length();

                // Check if should melt (moving too fast)
                if proton.is_water_frozen() && speed > proton::WATER_ICE_MELT_SPEED {
                    // Unfreeze (melt)
                    if let Some(p) = &mut self.protons[*idx] {
                        p.set_water_frozen(false);
                    }
                    continue;
                }

                // Check if should freeze
                let bonds = proton.water_h_bonds();
                if bonds.len() >= proton::WATER_ICE_MIN_NEIGHBORS {
                    // Check if all bonded neighbors are within compression distance
                    let mut all_compressed = true;
                    for bond_idx in bonds {
                        if let Some(partner) = &self.protons[*bond_idx] {
                            if partner.is_alive() && partner.is_h2o() {
                                let dist = pos.distance(partner.position());
                                if dist > proton::WATER_ICE_COMPRESSION_DISTANCE {
                                    all_compressed = false;
                                    break;
                                }
                            } else {
                                all_compressed = false;
                                break;
                            }
                        } else {
                            all_compressed = false;
                            break;
                        }
                    }

                    // Freeze if all bonds are compressed
                    if all_compressed && !proton.is_water_frozen() {
                        if let Some(p) = &mut self.protons[*idx] {
                            p.set_water_frozen(true);
                        }
                    } else if !all_compressed && proton.is_water_frozen() {
                        // Unfreeze if bonds are no longer compressed
                        if let Some(p) = &mut self.protons[*idx] {
                            p.set_water_frozen(false);
                        }
                    }
                } else {
                    // Not enough bonds - cannot be frozen
                    if proton.is_water_frozen() {
                        if let Some(p) = &mut self.protons[*idx] {
                            p.set_water_frozen(false);
                        }
                    }
                }
            }
        }

        // PHASE 6: Lock H2O molecules in place if they have 3-5 frozen bonds
        // This makes the ice completely rigid and stationary for more ice formation
        // 3 bonds = locked, 4 bonds = locked, 5 bonds = locked (all frozen)
        // 0-2 bonds = mobile (liquid H2O)
        for (idx, pos, _, _, _) in &water_molecules {
            if let Some(proton) = &self.protons[*idx] {
                if proton.is_water_frozen() {
                    // Count how many bonds are to other frozen H2O molecules
                    let mut frozen_bond_count = 0;
                    for bond_idx in proton.water_h_bonds() {
                        if let Some(partner) = &self.protons[*bond_idx] {
                            if partner.is_alive() && partner.is_h2o() && partner.is_water_frozen() {
                                frozen_bond_count += 1;
                            }
                        }
                    }

                    // Lock completely in place if 3, 4, or 5 frozen bonds (rigid ice)
                    // More ice, less movement!
                    if frozen_bond_count >= 3 {
                        if let Some(p) = &mut self.protons[*idx] {
                            p.set_velocity(Vec2::ZERO);
                        }
                    }
                    // If only 0-2 frozen bonds, allow movement (liquid)
                }
            }
        }

        // PHASE 6.5: Apply repulsion from 3-5 bonded frozen H2O to non-bonded H2O
        // This prevents ice from "folding" and trapping extra molecules inside the lattice
        // 3, 4, and 5 bonded frozen H2O all repel non-bonded H2O to push them to the edges
        let mut ice_repulsion_forces: Vec<Vec2> = vec![Vec2::ZERO; self.protons.len()];

        for i in 0..water_molecules.len() {
            let (idx_a, pos_a, _, _, _) = water_molecules[i];

            if let Some(proton_a) = &self.protons[idx_a] {
                // Check if this is a frozen H2O with 3, 4, or 5 bonds
                let bond_count = proton_a.water_h_bonds().len();
                if proton_a.is_water_frozen() && bond_count >= 3 {
                    let bonded_indices = proton_a.water_h_bonds().clone();

                    // Repel all nearby H2O that are NOT in our bonded partner list
                    // This pushes trapped molecules to the outer edges
                    for j in 0..water_molecules.len() {
                        if i == j {
                            continue;
                        }

                        let (idx_b, pos_b, _, _, _) = water_molecules[j];

                        // Skip if this H2O is in our bonded partner list
                        if bonded_indices.contains(&idx_b) {
                            continue;
                        }

                        let delta = pos_b - pos_a;
                        let dist = delta.length();

                        // Apply strong repulsion if within range
                        if dist < proton::WATER_ICE_REPULSION_RANGE && dist > 0.1 {
                            let dir = delta / dist;
                            let force_magnitude = proton::WATER_ICE_REPULSION_STRENGTH * (1.0 - dist / proton::WATER_ICE_REPULSION_RANGE);
                            ice_repulsion_forces[idx_b] += dir * force_magnitude;
                        }
                    }
                }
            }
        }

        // Apply the repulsion forces
        for (idx, _, _, mass, _) in &water_molecules {
            let force = ice_repulsion_forces[*idx];
            if force.length() > 0.01 {
                if let Some(proton) = &mut self.protons[*idx] {
                    let acc = force / *mass;
                    proton.add_velocity(acc * delta_time);
                }
            }
        }

        // PHASE 7: Apply spring forces to bonded water molecules
        // Use stronger forces and shorter rest length for frozen (ice) bonds
        let mut bonded_pairs: Vec<(usize, usize, Vec2, Vec2, f32, f32, f32, bool)> = Vec::new();

        for (idx, pos, _, mass, _) in &water_molecules {
            if let Some(proton) = &self.protons[*idx] {
                for (bond_idx, partner_idx) in proton.water_h_bonds().iter().enumerate() {
                    // Only process each pair once
                    if *partner_idx > *idx {
                        if let Some(partner) = &self.protons[*partner_idx] {
                            if partner.is_alive() && partner.is_h2o() {
                                // Check if both molecules are frozen (ice bond)
                                let both_frozen = proton.is_water_frozen() && partner.is_water_frozen();
                                let rest_length = proton.water_bond_rest_lengths()[bond_idx];
                                bonded_pairs.push((
                                    *idx,
                                    *partner_idx,
                                    *pos,
                                    partner.position(),
                                    *mass,
                                    partner.mass(),
                                    rest_length,
                                    both_frozen,
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Apply forces and check for breaking
        for (idx1, idx2, pos1, pos2, m1, m2, rest_length, is_frozen) in bonded_pairs {
            let delta = pos2 - pos1;
            let dist = delta.length();

            // Check if bond should break
            if dist > proton::WATER_H_BOND_BREAKING_DISTANCE {
                continue; // Bonds will be rebuilt next frame
            }

            // Apply spring force - use stronger force and different rest length for frozen bonds
            if dist > 0.1 {
                let (bond_strength, ice_rest_length) = if is_frozen {
                    (proton::WATER_ICE_FROZEN_BOND_STRENGTH, proton::WATER_ICE_FROZEN_REST_LENGTH)
                } else {
                    (proton::WATER_H_BOND_STRENGTH, rest_length)
                };

                let displacement = dist - ice_rest_length;
                let force_magnitude = displacement * bond_strength;
                let dir = delta / dist;
                let force = dir * force_magnitude;

                // For frozen bonds, apply forces more strongly to lock molecules in place
                // For liquid bonds, apply normal forces
                if let Some(p1) = &mut self.protons[idx1] {
                    let acc1 = force / m1;
                    p1.add_velocity(acc1 * delta_time);
                }
                if let Some(p2) = &mut self.protons[idx2] {
                    let acc2 = -force / m2;
                    p2.add_velocity(acc2 * delta_time);
                }
            }
        }

        // PHASE 8 (FINAL): Lock 3-5 bonded frozen H2O velocity to ZERO
        // This MUST happen at the end, after ALL forces have been applied
        // This ensures 3-5 bonded H2O absolutely cannot move, no matter what forces push it
        for (idx, _, _, _, _) in &water_molecules {
            if let Some(proton) = &self.protons[*idx] {
                if proton.is_water_frozen() && proton.water_h_bonds().len() >= 3 {
                    // Lock velocity - no movement allowed!
                    if let Some(p) = &mut self.protons[*idx] {
                        p.set_velocity(Vec2::ZERO);
                    }
                }
            }
        }
    }

    /// Handle solid collisions between H, He4, C12, O16 bonded particles, H2O, and hydrogen compound molecules
    fn handle_solid_collisions(&mut self) {
        // Collect solid proton data (H, He4, C12, O16 bonded, H2O, and hydrogen compounds)
        let mut solid_protons: Vec<(usize, Vec2, Vec2, f32, f32)> = Vec::new();

        for (i, proton_opt) in self.protons.iter().enumerate() {
            if let Some(proton) = proton_opt {
                if proton.is_alive() {
                    let charge = proton.charge();
                    let neutron_count = proton.neutron_count();

                    // Hydrogen compound molecules are solid
                    if proton.is_sih4() {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                        continue;
                    }

                    if proton.is_ch4() {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                        continue;
                    }

                    if proton.is_h2s() {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                        continue;
                    }

                    if proton.is_mgh2() {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                        continue;
                    }

                    // S32 particles are solid
                    if proton.is_sulfur32() {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                        continue;
                    }

                    // Si28 particles are solid
                    if proton.is_silicon28() {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                        continue;
                    }

                    // Mg24 particles are solid
                    if proton.is_magnesium24() {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                        continue;
                    }

                    // Ne20 particles are solid
                    if proton.is_neon20() {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                        continue;
                    }

                    // H2O molecules are solid
                    if proton.is_h2o() {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                        continue;
                    }

                    // O16 bonded particles are solid
                    if proton.is_oxygen16_bonded() {
                        solid_protons.push((
                            i,
                            proton.position(),
                            proton.velocity(),
                            proton.radius(),
                            proton.mass(),
                        ));
                        continue;
                    }

                    // H (charge=0, neutron=1), He4 (charge=2, neutron=2), and C12 (charge=6, neutron=6) are solid
                    if (charge == 0 && neutron_count == 1)
                        || (charge == 2 && neutron_count == 2)
                        || (charge == 6 && neutron_count == 6) {
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
                if !p.is_alive() || p.is_stable_hydrogen() || p.is_stable_helium4() || p.is_stable_carbon12() {
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
                    if !p.is_alive() || p.is_stable_hydrogen() || p.is_stable_helium4() || p.is_stable_carbon12() {
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

                        // Spawn energy wave (D + H+ → He3) with dark red to yellow color
                        use macroquad::rand::gen_range;
                        let t: f32 = gen_range(0.0, 1.0);
                        let t = t.powf(3.0);
                        ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

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

                        // Spawn BIG energy waves with random colors between dark red and almost yellow
                        // Dark red = (0.17,0,0), Almost yellow = (1.0,0.8,0)
                        // Use cubic bias to favor dark red: t^3 keeps most values near 0
                        use macroquad::rand::gen_range;
                        let t1: f32 = gen_range(0.0, 1.0);
                        let t1 = t1.powf(3.0);
                        let color1 = Color::new(0.17 + 0.83*t1, 0.8*t1, 0.0, 1.0);
                        ring_manager.add_ring_with_color(center_of_mass, color1);

                        let t2: f32 = gen_range(0.0, 1.0);
                        let t2 = t2.powf(3.0);
                        let color2 = Color::new(0.17 + 0.83*t2, 0.8*t2, 0.0, 1.0);
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

                    // Spawn energy wave (H- + H+ → He3) with dark red to yellow color
                    use macroquad::rand::gen_range;
                    let t: f32 = gen_range(0.0, 1.0);
                    let t = t.powf(3.0);
                    ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                    // Delete second proton
                    self.protons[j] = None;
                    break;
                }
            }
        }

        // FUSION CASE 4: Triple-alpha process - Three He4 → C12
        // Collect all He4 particles
        let mut he4_particles: Vec<(usize, Vec2, Vec2, f32, f32, f32)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_stable_helium4() {
                    he4_particles.push((
                        i,
                        proton.position(),
                        proton.velocity(),
                        proton.radius(),
                        proton.mass(),
                        proton.energy(),
                    ));
                }
            }
        }

        // Check all combinations of three He4 particles
        for i in 0..he4_particles.len() {
            for j in (i + 1)..he4_particles.len() {
                for k in (j + 1)..he4_particles.len() {
                    let (idx1, pos1, vel1, r1, m1, e1) = he4_particles[i];
                    let (idx2, pos2, vel2, r2, m2, e2) = he4_particles[j];
                    let (idx3, pos3, vel3, r3, m3, e3) = he4_particles[k];

                    // Check if all three are within collision range of each other
                    let dist12_sq = pos1.distance_squared(pos2);
                    let dist13_sq = pos1.distance_squared(pos3);
                    let dist23_sq = pos2.distance_squared(pos3);

                    let collision_dist12 = r1 + r2;
                    let collision_dist13 = r1 + r3;
                    let collision_dist23 = r2 + r3;

                    // All three must be colliding with each other
                    if dist12_sq <= collision_dist12 * collision_dist12 &&
                       dist13_sq <= collision_dist13 * collision_dist13 &&
                       dist23_sq <= collision_dist23 * collision_dist23
                    {
                        // Calculate combined energy
                        let combined_energy = e1 + e2 + e3;

                        // Check energy threshold
                        if combined_energy < proton::TRIPLE_ALPHA_ENERGY_THRESHOLD {
                            continue;
                        }

                        // Calculate average relative velocity
                        let rel_vel12 = vel1 - vel2;
                        let rel_vel13 = vel1 - vel3;
                        let rel_vel23 = vel2 - vel3;
                        let avg_rel_speed = (rel_vel12.length() + rel_vel13.length() + rel_vel23.length()) / 3.0;

                        // Check velocity threshold
                        if avg_rel_speed < proton::TRIPLE_ALPHA_VELOCITY_THRESHOLD {
                            continue;
                        }

                        // FUSION OCCURS!
                        // Calculate center of mass
                        let total_mass = m1 + m2 + m3;
                        let center_of_mass = (pos1 * m1 + pos2 * m2 + pos3 * m3) / total_mass;
                        let combined_vel = (vel1 * m1 + vel2 * m2 + vel3 * m3) / total_mass;

                        // Create Carbon-12 in first slot
                        let mut c12 = Proton::new(
                            center_of_mass,
                            combined_vel,
                            Color::from_rgba(100, 100, 100, 255),
                            combined_energy,
                            6,
                        );
                        c12.set_neutron_count(6);
                        c12.set_max_lifetime(-1.0); // Carbon-12 is stable
                        self.protons[idx1] = Some(c12);

                        // Spawn energy wave with dark red to almost yellow (favoring dark red)
                        // Dark red = (0.17,0,0), Almost yellow = (1.0,0.8,0)
                        // Use cubic bias to favor dark red: t^3 keeps most values near 0
                        use macroquad::rand::gen_range;
                        let t: f32 = gen_range(0.0, 1.0);
                        let t = t.powf(3.0);
                        let fusion_color = Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0);
                        ring_manager.add_ring_with_color(center_of_mass, fusion_color);

                        // Delete the other two He4 particles
                        self.protons[idx2] = None;
                        self.protons[idx3] = None;

                                // Only perform one fusion per update cycle
                        return;
                    }
                }
            }
        }

        // BONDING CASE: C12 + He4 → O16 bonded pair (alpha capture on carbon)
        // This MUST happen before Ne20 formation check!
        // Collect all unbonded C12 and He4 particles
        let mut c12_particles: Vec<(usize, Vec2, Vec2, f32)> = Vec::new();
        let mut he4_particles: Vec<(usize, Vec2, Vec2, f32)> = Vec::new();

        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && !proton.is_oxygen16_bonded() {
                    if proton.is_stable_carbon12() {
                        c12_particles.push((i, proton.position(), proton.velocity(), proton.radius()));
                    } else if proton.is_stable_helium4() {
                        he4_particles.push((i, proton.position(), proton.velocity(), proton.radius()));
                    }
                }
            }
        }

        // Check all C12-He4 pairs for bonding
        for (c12_idx, c12_pos, c12_vel, c12_r) in &c12_particles {
            for (he4_idx, he4_pos, he4_vel, he4_r) in &he4_particles {
                let dist_sq = c12_pos.distance_squared(*he4_pos);
                let collision_dist = c12_r + he4_r;

                // Check if colliding
                if dist_sq <= collision_dist * collision_dist {
                    let dist = dist_sq.sqrt();

                    // Calculate relative velocity
                    let rel_vel = *c12_vel - *he4_vel;
                    let rel_speed = rel_vel.length();

                    // Check velocity threshold
                    if rel_speed >= proton::OXYGEN16_CAPTURE_VELOCITY_THRESHOLD {
                        // BONDING OCCURS!
                        // Calculate bond rest length
                        let bond_rest_length = dist.max(1.0);

                        // Calculate midpoint for energy wave
                        let midpoint = (*c12_pos + *he4_pos) / 2.0;

                        // Set bonding on both particles
                        if let Some(c12) = &mut self.protons[*c12_idx] {
                            c12.set_oxygen16_bonded(true);
                            c12.set_oxygen_bond_partner(Some(*he4_idx));
                            c12.set_oxygen_bond_rest_length(bond_rest_length);
                        }
                        if let Some(he4) = &mut self.protons[*he4_idx] {
                            he4.set_oxygen16_bonded(true);
                            he4.set_oxygen_bond_partner(Some(*c12_idx));
                            he4.set_oxygen_bond_rest_length(bond_rest_length);
                        }

                        // Spawn energy wave at bonding site (dark red to yellow, favoring dark red)
                        use macroquad::rand::gen_range;
                        let t: f32 = gen_range(0.0, 1.0);
                        let t = t.powf(3.0);
                        ring_manager.add_ring_with_color(midpoint, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                        // Only one bonding per update cycle
                        return;
                    }
                }
            }
        }

        // FUSION CASE 5: Neon-20 formation - O16 bonded pair + He4 → Ne20
        // Collect all O16 bonded pairs
        let mut o16_pairs: Vec<(usize, usize, Vec2, f32, f32, f32, Vec2, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_oxygen16_bonded() {
                    if let Some(partner_idx) = proton.oxygen_bond_partner() {
                        if partner_idx > i {
                            if let Some(partner) = &self.protons[partner_idx] {
                                if partner.is_alive() && partner.is_oxygen16_bonded() {
                                    // Calculate midpoint of O16 pair
                                    let midpoint = (proton.position() + partner.position()) / 2.0;
                                    let mass1 = proton.mass();
                                    let mass2 = partner.mass();
                                    let energy1 = proton.energy();
                                    let energy2 = partner.energy();
                                    let vel1 = proton.velocity();
                                    let vel2 = partner.velocity();
                                    let radius1 = proton.radius();
                                    let radius2 = partner.radius();
                                    // Use average radius of the pair
                                    let avg_radius = (radius1 + radius2) / 2.0;
                                    o16_pairs.push((i, partner_idx, midpoint, mass1 + mass2, energy1 + energy2, avg_radius, vel1, vel2));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Collect all He4 particles (excluding those already bonded in O16 pairs)
        let mut he4_for_neon: Vec<(usize, Vec2, Vec2, f32, f32, f32)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_stable_helium4() && !proton.is_oxygen16_bonded() {
                    he4_for_neon.push((
                        i,
                        proton.position(),
                        proton.velocity(),
                        proton.radius(),
                        proton.mass(),
                        proton.energy(),
                    ));
                }
            }
        }

        // Check for O16 + He4 collisions to form Ne20
        for (o16_idx1, o16_idx2, o16_midpoint, o16_mass, o16_energy, o16_radius, o16_vel1, o16_vel2) in o16_pairs {
            for (he4_idx, he4_pos, he4_vel, he4_radius, he4_mass, he4_energy) in &he4_for_neon {
                // Calculate distance from He4 to O16 midpoint
                let dist_sq = o16_midpoint.distance_squared(*he4_pos);
                let collision_dist = o16_radius + he4_radius;

                // Check if colliding
                if dist_sq <= collision_dist * collision_dist {
                    // Calculate relative velocity (use average O16 velocity)
                    let o16_avg_vel = (o16_vel1 + o16_vel2) / 2.0;
                    let rel_vel = o16_avg_vel - *he4_vel;
                    let rel_speed = rel_vel.length();

                    // Check velocity threshold
                    if rel_speed >= proton::NEON20_CAPTURE_VELOCITY_THRESHOLD {
                        // NEON-20 FORMATION OCCURS!
                        // Calculate center of mass and combined velocity
                        let total_mass = o16_mass + *he4_mass;
                        let combined_momentum = o16_vel1 * (o16_mass / 2.0) + o16_vel2 * (o16_mass / 2.0) + *he4_vel * *he4_mass;
                        let combined_vel = combined_momentum / total_mass;
                        let combined_energy = o16_energy + *he4_energy;

                        // Calculate center of mass position
                        let (o16_pos1, o16_pos2) = {
                            let p1 = self.protons[o16_idx1].as_ref().unwrap().position();
                            let p2 = self.protons[o16_idx2].as_ref().unwrap().position();
                            (p1, p2)
                        };
                        let center_of_mass = (o16_pos1 * (o16_mass / 2.0) + o16_pos2 * (o16_mass / 2.0) + *he4_pos * *he4_mass) / total_mass;

                        // Create Ne20 in first O16 slot
                        let mut ne20 = Proton::new(
                            center_of_mass,
                            combined_vel,
                            Color::from_rgba(255, 100, 150, 255),
                            combined_energy,
                            10, // Total charge: 6 (C) + 2 (He from O16) + 2 (He4) = 10
                        );
                        ne20.set_neutron_count(10); // Total neutrons: 6 (C) + 2 (He from O16) + 2 (He4) = 10
                        ne20.set_max_lifetime(-1.0); // Ne20 is stable
                        ne20.set_neon20(true);
                        self.protons[o16_idx1] = Some(ne20);

                        // Delete the other particles
                        self.protons[o16_idx2] = None;
                        self.protons[*he4_idx] = None;

                        // Spawn energy wave (dark red to yellow, favoring dark red)
                        use macroquad::rand::gen_range;
                        let t: f32 = gen_range(0.0, 1.0);
                        let t = t.powf(3.0);
                        ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                        // Only one neon formation per update cycle
                        return;
                    }
                }
            }
        }

        // FUSION CASE 6: Magnesium-24 formation - Ne20 + He4 → Mg24
        // Collect all Ne20 particles
        let mut ne20_particles: Vec<(usize, Vec2, Vec2, f32, f32, f32)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_neon20() {
                    ne20_particles.push((
                        i,
                        proton.position(),
                        proton.velocity(),
                        proton.radius(),
                        proton.mass(),
                        proton.energy(),
                    ));
                }
            }
        }

        // Collect He4 particles (excluding those already bonded in O16 pairs)
        let mut he4_for_mg: Vec<(usize, Vec2, Vec2, f32, f32, f32)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_stable_helium4() && !proton.is_oxygen16_bonded() {
                    he4_for_mg.push((
                        i,
                        proton.position(),
                        proton.velocity(),
                        proton.radius(),
                        proton.mass(),
                        proton.energy(),
                    ));
                }
            }
        }

        // Check for Ne20 + He4 collisions to form Mg24
        for (ne20_idx, ne20_pos, ne20_vel, ne20_radius, ne20_mass, ne20_energy) in &ne20_particles {
            for (he4_idx, he4_pos, he4_vel, he4_radius, he4_mass, he4_energy) in &he4_for_mg {
                let dist_sq = ne20_pos.distance_squared(*he4_pos);
                let collision_dist = ne20_radius + he4_radius;

                if dist_sq <= collision_dist * collision_dist {
                    let rel_vel = *ne20_vel - *he4_vel;
                    let rel_speed = rel_vel.length();

                    if rel_speed >= proton::MAGNESIUM24_CAPTURE_VELOCITY_THRESHOLD {
                        // Mg24 formation!
                        let total_mass = ne20_mass + he4_mass;
                        let combined_momentum = *ne20_vel * *ne20_mass + *he4_vel * *he4_mass;
                        let combined_vel = combined_momentum / total_mass;
                        let combined_energy = ne20_energy + he4_energy;
                        let center_of_mass = (*ne20_pos * *ne20_mass + *he4_pos * *he4_mass) / total_mass;

                        let mut mg24 = Proton::new(
                            center_of_mass,
                            combined_vel,
                            Color::from_rgba(200, 200, 220, 255),
                            combined_energy,
                            12,
                        );
                        mg24.set_neutron_count(12);
                        mg24.set_max_lifetime(-1.0);
                        mg24.set_magnesium24(true);
                        self.protons[*ne20_idx] = Some(mg24);

                        self.protons[*he4_idx] = None;

                        use macroquad::rand::gen_range;
                        let t: f32 = gen_range(0.0, 1.0);
                        let t = t.powf(3.0);
                        ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                        return;
                    }
                }
            }
        }

        // FUSION CASE 7: Silicon-28 formation - Mg24 + He4 → Si28
        // Collect all Mg24 particles
        let mut mg24_particles: Vec<(usize, Vec2, Vec2, f32, f32, f32)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_magnesium24() {
                    mg24_particles.push((
                        i,
                        proton.position(),
                        proton.velocity(),
                        proton.radius(),
                        proton.mass(),
                        proton.energy(),
                    ));
                }
            }
        }

        // Collect He4 particles (excluding those already bonded in O16 pairs)
        let mut he4_for_si: Vec<(usize, Vec2, Vec2, f32, f32, f32)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_stable_helium4() && !proton.is_oxygen16_bonded() {
                    he4_for_si.push((
                        i,
                        proton.position(),
                        proton.velocity(),
                        proton.radius(),
                        proton.mass(),
                        proton.energy(),
                    ));
                }
            }
        }

        // Check for Mg24 + He4 collisions to form Si28
        for (mg24_idx, mg24_pos, mg24_vel, mg24_radius, mg24_mass, mg24_energy) in &mg24_particles {
            for (he4_idx, he4_pos, he4_vel, he4_radius, he4_mass, he4_energy) in &he4_for_si {
                let dist_sq = mg24_pos.distance_squared(*he4_pos);
                let collision_dist = mg24_radius + he4_radius;

                if dist_sq <= collision_dist * collision_dist {
                    let rel_vel = *mg24_vel - *he4_vel;
                    let rel_speed = rel_vel.length();

                    if rel_speed >= proton::SILICON28_CAPTURE_VELOCITY_THRESHOLD {
                        // Si28 formation!
                        let total_mass = mg24_mass + he4_mass;
                        let combined_momentum = *mg24_vel * *mg24_mass + *he4_vel * *he4_mass;
                        let combined_vel = combined_momentum / total_mass;
                        let combined_energy = mg24_energy + he4_energy;
                        let center_of_mass = (*mg24_pos * *mg24_mass + *he4_pos * *he4_mass) / total_mass;

                        let mut si28 = Proton::new(
                            center_of_mass,
                            combined_vel,
                            Color::from_rgba(160, 130, 90, 255),
                            combined_energy,
                            14,
                        );
                        si28.set_neutron_count(14);
                        si28.set_max_lifetime(-1.0);
                        si28.set_silicon28(true);
                        self.protons[*mg24_idx] = Some(si28);

                        self.protons[*he4_idx] = None;

                        use macroquad::rand::gen_range;
                        let t: f32 = gen_range(0.0, 1.0);
                        let t = t.powf(3.0);
                        ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                        return;
                    }
                }
            }
        }

        // FUSION CASE 8: Sulfur-32 formation - Si28 + He4 → S32
        // Collect all Si28 particles
        let mut si28_particles: Vec<(usize, Vec2, Vec2, f32, f32, f32)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_silicon28() {
                    si28_particles.push((
                        i,
                        proton.position(),
                        proton.velocity(),
                        proton.radius(),
                        proton.mass(),
                        proton.energy(),
                    ));
                }
            }
        }

        // Collect He4 particles (excluding those already bonded in O16 pairs)
        let mut he4_for_s: Vec<(usize, Vec2, Vec2, f32, f32, f32)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_stable_helium4() && !proton.is_oxygen16_bonded() {
                    he4_for_s.push((
                        i,
                        proton.position(),
                        proton.velocity(),
                        proton.radius(),
                        proton.mass(),
                        proton.energy(),
                    ));
                }
            }
        }

        // Check for Si28 + He4 collisions to form S32
        for (si28_idx, si28_pos, si28_vel, si28_radius, si28_mass, si28_energy) in &si28_particles {
            for (he4_idx, he4_pos, he4_vel, he4_radius, he4_mass, he4_energy) in &he4_for_s {
                let dist_sq = si28_pos.distance_squared(*he4_pos);
                let collision_dist = si28_radius + he4_radius;

                if dist_sq <= collision_dist * collision_dist {
                    let rel_vel = *si28_vel - *he4_vel;
                    let rel_speed = rel_vel.length();

                    if rel_speed >= proton::SULFUR32_CAPTURE_VELOCITY_THRESHOLD {
                        // S32 formation!
                        let total_mass = si28_mass + he4_mass;
                        let combined_momentum = *si28_vel * *si28_mass + *he4_vel * *he4_mass;
                        let combined_vel = combined_momentum / total_mass;
                        let combined_energy = si28_energy + he4_energy;
                        let center_of_mass = (*si28_pos * *si28_mass + *he4_pos * *he4_mass) / total_mass;

                        let mut s32 = Proton::new(
                            center_of_mass,
                            combined_vel,
                            Color::from_rgba(220, 220, 80, 255),
                            combined_energy,
                            16,
                        );
                        s32.set_neutron_count(16);
                        s32.set_max_lifetime(-1.0);
                        s32.set_sulfur32(true);
                        self.protons[*si28_idx] = Some(s32);

                        self.protons[*he4_idx] = None;

                        use macroquad::rand::gen_range;
                        let t: f32 = gen_range(0.0, 1.0);
                        let t = t.powf(3.0);
                        ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                        return;
                    }
                }
            }
        }

        // WATER FORMATION: O16 bonded pair + 2 H atoms → H2O molecule
        // Collect all O16 bonded pairs
        let mut o16_pairs: Vec<(usize, usize, Vec2, f32, f32, f32, Vec2, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_oxygen16_bonded() {
                    if let Some(partner_idx) = proton.oxygen_bond_partner() {
                        if partner_idx > i {
                            if let Some(partner) = &self.protons[partner_idx] {
                                if partner.is_alive() && partner.is_oxygen16_bonded() {
                                    // Calculate midpoint of O16 pair
                                    let midpoint = (proton.position() + partner.position()) / 2.0;
                                    let mass1 = proton.mass();
                                    let mass2 = partner.mass();
                                    let energy1 = proton.energy();
                                    let energy2 = partner.energy();
                                    let vel1 = proton.velocity();
                                    let vel2 = partner.velocity();
                                    o16_pairs.push((i, partner_idx, midpoint, mass1 + mass2, energy1 + energy2, 0.0, vel1, vel2));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Collect all available H atoms (not crystallized)
        let mut h_atoms: Vec<(usize, Vec2, f32, f32, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.charge() == 0 && proton.neutron_count() == 1 && !proton.is_crystallized() {
                    h_atoms.push((i, proton.position(), proton.mass(), proton.energy(), proton.velocity()));
                }
            }
        }

        // Check each O16 pair for nearby H atoms
        for (o16_idx1, o16_idx2, o16_midpoint, o16_mass, o16_energy, _, o16_vel1, o16_vel2) in o16_pairs {
            // Find two H atoms near the O16 midpoint
            let mut nearby_h: Vec<(usize, f32, f32, f32, Vec2)> = Vec::new();
            for (h_idx, h_pos, h_mass, h_energy, h_vel) in &h_atoms {
                let dist = o16_midpoint.distance(*h_pos);
                if dist < proton::WATER_CAPTURE_RANGE {
                    nearby_h.push((*h_idx, *h_mass, *h_energy, dist, *h_vel));
                }
            }

            // Need at least 2 H atoms
            if nearby_h.len() >= 2 {
                // Sort by distance and take the two closest
                nearby_h.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
                let h1_idx = nearby_h[0].0;
                let h1_mass = nearby_h[0].1;
                let h1_energy = nearby_h[0].2;
                let h1_vel = nearby_h[0].4;

                let h2_idx = nearby_h[1].0;
                let h2_mass = nearby_h[1].1;
                let h2_energy = nearby_h[1].2;
                let h2_vel = nearby_h[1].4;

                // WATER FORMATION OCCURS!
                // Calculate center of mass and combined velocity
                let total_mass = o16_mass + h1_mass + h2_mass;
                let o16_com_mass = o16_mass / 2.0;
                let combined_momentum = o16_vel1 * o16_com_mass + o16_vel2 * o16_com_mass + h1_vel * h1_mass + h2_vel * h2_mass;
                let combined_vel = combined_momentum / total_mass;
                let combined_energy = o16_energy + h1_energy + h2_energy;

                // Calculate center of mass position (weighted average)
                // Get O16 positions for accurate COM calculation
                let (o16_pos1, o16_pos2) = {
                    let p1 = self.protons[o16_idx1].as_ref().unwrap().position();
                    let p2 = self.protons[o16_idx2].as_ref().unwrap().position();
                    (p1, p2)
                };
                let (h1_pos, h2_pos) = {
                    let h1p = self.protons[h1_idx].as_ref().unwrap().position();
                    let h2p = self.protons[h2_idx].as_ref().unwrap().position();
                    (h1p, h2p)
                };

                let center_of_mass = (o16_pos1 * o16_com_mass + o16_pos2 * o16_com_mass + h1_pos * h1_mass + h2_pos * h2_mass) / total_mass;

                // Create H2O molecule in first O16 slot
                let mut h2o = Proton::new(
                    center_of_mass,
                    combined_vel,
                    Color::from_rgba(40, 100, 180, 255),
                    combined_energy,
                    10, // Total charge: 6 (C) + 2 (He) + 1 (H) + 1 (H) = 10
                );
                h2o.set_neutron_count(8); // Total neutrons: 6 (C) + 2 (He) = 8
                h2o.set_max_lifetime(-1.0); // Water is stable
                h2o.set_h2o(true);
                self.protons[o16_idx1] = Some(h2o);

                // Delete the other particles
                self.protons[o16_idx2] = None;
                self.protons[h1_idx] = None;
                self.protons[h2_idx] = None;

                // Spawn wave at formation site (dark red to yellow, favoring dark red)
                use macroquad::rand::gen_range;
                let t: f32 = gen_range(0.0, 1.0);
                let t = t.powf(3.0);
                ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                // Only one water formation per update cycle
                return;
            }
        }

        // H2S FORMATION: S32 + 2 H atoms → H2S molecule
        // Collect all S32 particles
        let mut s32_particles: Vec<(usize, Vec2, f32, f32, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_sulfur32() {
                    s32_particles.push((i, proton.position(), proton.mass(), proton.energy(), proton.velocity()));
                }
            }
        }

        // Collect all available H atoms (not crystallized)
        let mut h_atoms: Vec<(usize, Vec2, f32, f32, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.charge() == 0 && proton.neutron_count() == 1 && !proton.is_crystallized() {
                    h_atoms.push((i, proton.position(), proton.mass(), proton.energy(), proton.velocity()));
                }
            }
        }

        // Check each S32 for nearby H atoms
        for (s32_idx, s32_pos, s32_mass, s32_energy, s32_vel) in s32_particles {
            // Find two H atoms near the S32
            let mut nearby_h: Vec<(usize, f32, f32, f32, Vec2)> = Vec::new();
            for (h_idx, h_pos, h_mass, h_energy, h_vel) in &h_atoms {
                let dist = s32_pos.distance(*h_pos);
                if dist < proton::H2S_CAPTURE_RANGE {
                    nearby_h.push((*h_idx, *h_mass, *h_energy, dist, *h_vel));
                }
            }

            // Need at least 2 H atoms
            if nearby_h.len() >= 2 {
                // Sort by distance and take the two closest
                nearby_h.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
                let h1_idx = nearby_h[0].0;
                let h1_mass = nearby_h[0].1;
                let h1_energy = nearby_h[0].2;
                let h1_vel = nearby_h[0].4;

                let h2_idx = nearby_h[1].0;
                let h2_mass = nearby_h[1].1;
                let h2_energy = nearby_h[1].2;
                let h2_vel = nearby_h[1].4;

                // H2S FORMATION OCCURS!
                let total_mass = s32_mass + h1_mass + h2_mass;
                let combined_momentum = s32_vel * s32_mass + h1_vel * h1_mass + h2_vel * h2_mass;
                let combined_vel = combined_momentum / total_mass;
                let combined_energy = s32_energy + h1_energy + h2_energy;
                let center_of_mass = (s32_pos * s32_mass + {
                    let h1p = self.protons[h1_idx].as_ref().unwrap().position();
                    let h2p = self.protons[h2_idx].as_ref().unwrap().position();
                    h1p * h1_mass + h2p * h2_mass
                }) / total_mass;

                // Create H2S molecule
                let mut h2s = Proton::new(
                    center_of_mass,
                    combined_vel,
                    Color::from_rgba(200, 220, 80, 255),
                    combined_energy,
                    18, // S32 has 16 protons + 2 from H = 18
                );
                h2s.set_neutron_count(18); // S32 has 16 neutrons + 2 from H = 18
                h2s.set_max_lifetime(-1.0); // H2S is stable
                h2s.set_h2s(true);
                self.protons[s32_idx] = Some(h2s);

                // Delete the H atoms
                self.protons[h1_idx] = None;
                self.protons[h2_idx] = None;

                // Spawn energy wave
                use macroquad::rand::gen_range;
                let t: f32 = gen_range(0.0, 1.0);
                let t = t.powf(3.0);
                ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                return;
            }
        }

        // MGH2 FORMATION: Mg24 + 2 H atoms → MgH2 molecule
        // Collect all Mg24 particles
        let mut mg24_particles: Vec<(usize, Vec2, f32, f32, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_magnesium24() {
                    mg24_particles.push((i, proton.position(), proton.mass(), proton.energy(), proton.velocity()));
                }
            }
        }

        // Reuse h_atoms from above
        let mut h_atoms: Vec<(usize, Vec2, f32, f32, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.charge() == 0 && proton.neutron_count() == 1 && !proton.is_crystallized() {
                    h_atoms.push((i, proton.position(), proton.mass(), proton.energy(), proton.velocity()));
                }
            }
        }

        // Check each Mg24 for nearby H atoms
        for (mg24_idx, mg24_pos, mg24_mass, mg24_energy, mg24_vel) in mg24_particles {
            let mut nearby_h: Vec<(usize, f32, f32, f32, Vec2)> = Vec::new();
            for (h_idx, h_pos, h_mass, h_energy, h_vel) in &h_atoms {
                let dist = mg24_pos.distance(*h_pos);
                if dist < proton::MGH2_CAPTURE_RANGE {
                    nearby_h.push((*h_idx, *h_mass, *h_energy, dist, *h_vel));
                }
            }

            if nearby_h.len() >= 2 {
                nearby_h.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
                let h1_idx = nearby_h[0].0;
                let h1_mass = nearby_h[0].1;
                let h1_energy = nearby_h[0].2;
                let h1_vel = nearby_h[0].4;

                let h2_idx = nearby_h[1].0;
                let h2_mass = nearby_h[1].1;
                let h2_energy = nearby_h[1].2;
                let h2_vel = nearby_h[1].4;

                // MgH2 FORMATION OCCURS!
                let total_mass = mg24_mass + h1_mass + h2_mass;
                let combined_momentum = mg24_vel * mg24_mass + h1_vel * h1_mass + h2_vel * h2_mass;
                let combined_vel = combined_momentum / total_mass;
                let combined_energy = mg24_energy + h1_energy + h2_energy;
                let center_of_mass = (mg24_pos * mg24_mass + {
                    let h1p = self.protons[h1_idx].as_ref().unwrap().position();
                    let h2p = self.protons[h2_idx].as_ref().unwrap().position();
                    h1p * h1_mass + h2p * h2_mass
                }) / total_mass;

                let mut mgh2 = Proton::new(
                    center_of_mass,
                    combined_vel,
                    Color::from_rgba(180, 180, 190, 255),
                    combined_energy,
                    14, // Mg24 has 12 protons + 2 from H = 14
                );
                mgh2.set_neutron_count(14); // Mg24 has 12 neutrons + 2 from H = 14
                mgh2.set_max_lifetime(-1.0);
                mgh2.set_mgh2(true);
                self.protons[mg24_idx] = Some(mgh2);

                self.protons[h1_idx] = None;
                self.protons[h2_idx] = None;

                use macroquad::rand::gen_range;
                let t: f32 = gen_range(0.0, 1.0);
                let t = t.powf(3.0);
                ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                return;
            }
        }

        // CH4 FORMATION: C12 + 4 H atoms → CH4 molecule
        // Collect all C12 particles (not bonded)
        let mut c12_particles: Vec<(usize, Vec2, f32, f32, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_stable_carbon12() && !proton.is_oxygen16_bonded() {
                    c12_particles.push((i, proton.position(), proton.mass(), proton.energy(), proton.velocity()));
                }
            }
        }

        // Reuse h_atoms
        let mut h_atoms: Vec<(usize, Vec2, f32, f32, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.charge() == 0 && proton.neutron_count() == 1 && !proton.is_crystallized() {
                    h_atoms.push((i, proton.position(), proton.mass(), proton.energy(), proton.velocity()));
                }
            }
        }

        // Check each C12 for nearby H atoms
        for (c12_idx, c12_pos, c12_mass, c12_energy, c12_vel) in c12_particles {
            let mut nearby_h: Vec<(usize, f32, f32, f32, Vec2)> = Vec::new();
            for (h_idx, h_pos, h_mass, h_energy, h_vel) in &h_atoms {
                let dist = c12_pos.distance(*h_pos);
                if dist < proton::CH4_CAPTURE_RANGE {
                    nearby_h.push((*h_idx, *h_mass, *h_energy, dist, *h_vel));
                }
            }

            // Need at least 4 H atoms for methane
            if nearby_h.len() >= 4 {
                nearby_h.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
                let h1_idx = nearby_h[0].0;
                let h2_idx = nearby_h[1].0;
                let h3_idx = nearby_h[2].0;
                let h4_idx = nearby_h[3].0;

                // CH4 FORMATION OCCURS!
                let h1_mass = nearby_h[0].1;
                let h2_mass = nearby_h[1].1;
                let h3_mass = nearby_h[2].1;
                let h4_mass = nearby_h[3].1;

                let h1_energy = nearby_h[0].2;
                let h2_energy = nearby_h[1].2;
                let h3_energy = nearby_h[2].2;
                let h4_energy = nearby_h[3].2;

                let h1_vel = nearby_h[0].4;
                let h2_vel = nearby_h[1].4;
                let h3_vel = nearby_h[2].4;
                let h4_vel = nearby_h[3].4;

                let total_mass = c12_mass + h1_mass + h2_mass + h3_mass + h4_mass;
                let combined_momentum = c12_vel * c12_mass + h1_vel * h1_mass + h2_vel * h2_mass + h3_vel * h3_mass + h4_vel * h4_mass;
                let combined_vel = combined_momentum / total_mass;
                let combined_energy = c12_energy + h1_energy + h2_energy + h3_energy + h4_energy;

                let h_positions_mass = {
                    let h1p = self.protons[h1_idx].as_ref().unwrap().position();
                    let h2p = self.protons[h2_idx].as_ref().unwrap().position();
                    let h3p = self.protons[h3_idx].as_ref().unwrap().position();
                    let h4p = self.protons[h4_idx].as_ref().unwrap().position();
                    h1p * h1_mass + h2p * h2_mass + h3p * h3_mass + h4p * h4_mass
                };
                let center_of_mass = (c12_pos * c12_mass + h_positions_mass) / total_mass;

                let mut ch4 = Proton::new(
                    center_of_mass,
                    combined_vel,
                    Color::from_rgba(120, 200, 150, 255),
                    combined_energy,
                    10, // C12 has 6 protons + 4 from H = 10
                );
                ch4.set_neutron_count(10); // C12 has 6 neutrons + 4 from H = 10
                ch4.set_max_lifetime(-1.0);
                ch4.set_ch4(true);
                self.protons[c12_idx] = Some(ch4);

                self.protons[h1_idx] = None;
                self.protons[h2_idx] = None;
                self.protons[h3_idx] = None;
                self.protons[h4_idx] = None;

                use macroquad::rand::gen_range;
                let t: f32 = gen_range(0.0, 1.0);
                let t = t.powf(3.0);
                ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                return;
            }
        }

        // SIH4 FORMATION: Si28 + 4 H atoms → SiH4 molecule
        // Collect all Si28 particles
        let mut si28_particles: Vec<(usize, Vec2, f32, f32, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.is_silicon28() {
                    si28_particles.push((i, proton.position(), proton.mass(), proton.energy(), proton.velocity()));
                }
            }
        }

        // Reuse h_atoms
        let mut h_atoms: Vec<(usize, Vec2, f32, f32, Vec2)> = Vec::new();
        for i in 0..self.protons.len() {
            if let Some(proton) = &self.protons[i] {
                if proton.is_alive() && proton.charge() == 0 && proton.neutron_count() == 1 && !proton.is_crystallized() {
                    h_atoms.push((i, proton.position(), proton.mass(), proton.energy(), proton.velocity()));
                }
            }
        }

        // Check each Si28 for nearby H atoms
        for (si28_idx, si28_pos, si28_mass, si28_energy, si28_vel) in si28_particles {
            let mut nearby_h: Vec<(usize, f32, f32, f32, Vec2)> = Vec::new();
            for (h_idx, h_pos, h_mass, h_energy, h_vel) in &h_atoms {
                let dist = si28_pos.distance(*h_pos);
                if dist < proton::SIH4_CAPTURE_RANGE {
                    nearby_h.push((*h_idx, *h_mass, *h_energy, dist, *h_vel));
                }
            }

            // Need at least 4 H atoms for silane
            if nearby_h.len() >= 4 {
                nearby_h.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
                let h1_idx = nearby_h[0].0;
                let h2_idx = nearby_h[1].0;
                let h3_idx = nearby_h[2].0;
                let h4_idx = nearby_h[3].0;

                // SiH4 FORMATION OCCURS!
                let h1_mass = nearby_h[0].1;
                let h2_mass = nearby_h[1].1;
                let h3_mass = nearby_h[2].1;
                let h4_mass = nearby_h[3].1;

                let h1_energy = nearby_h[0].2;
                let h2_energy = nearby_h[1].2;
                let h3_energy = nearby_h[2].2;
                let h4_energy = nearby_h[3].2;

                let h1_vel = nearby_h[0].4;
                let h2_vel = nearby_h[1].4;
                let h3_vel = nearby_h[2].4;
                let h4_vel = nearby_h[3].4;

                let total_mass = si28_mass + h1_mass + h2_mass + h3_mass + h4_mass;
                let combined_momentum = si28_vel * si28_mass + h1_vel * h1_mass + h2_vel * h2_mass + h3_vel * h3_mass + h4_vel * h4_mass;
                let combined_vel = combined_momentum / total_mass;
                let combined_energy = si28_energy + h1_energy + h2_energy + h3_energy + h4_energy;

                let h_positions_mass = {
                    let h1p = self.protons[h1_idx].as_ref().unwrap().position();
                    let h2p = self.protons[h2_idx].as_ref().unwrap().position();
                    let h3p = self.protons[h3_idx].as_ref().unwrap().position();
                    let h4p = self.protons[h4_idx].as_ref().unwrap().position();
                    h1p * h1_mass + h2p * h2_mass + h3p * h3_mass + h4p * h4_mass
                };
                let center_of_mass = (si28_pos * si28_mass + h_positions_mass) / total_mass;

                let mut sih4 = Proton::new(
                    center_of_mass,
                    combined_vel,
                    Color::from_rgba(220, 100, 50, 255),
                    combined_energy,
                    18, // Si28 has 14 protons + 4 from H = 18
                );
                sih4.set_neutron_count(18); // Si28 has 14 neutrons + 4 from H = 18
                sih4.set_max_lifetime(-1.0);
                sih4.set_sih4(true);
                self.protons[si28_idx] = Some(sih4);

                self.protons[h1_idx] = None;
                self.protons[h2_idx] = None;
                self.protons[h3_idx] = None;
                self.protons[h4_idx] = None;

                use macroquad::rand::gen_range;
                let t: f32 = gen_range(0.0, 1.0);
                let t = t.powf(3.0);
                ring_manager.add_ring_with_color(center_of_mass, Color::new(0.17 + 0.83*t, 0.8*t, 0.0, 1.0));

                return;
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

                        // Determine charge randomly (50/50 chance for H+ or H-)
                        use macroquad::rand::gen_range;
                        let charge = if gen_range(0.0, 1.0) < 0.5 {
                            1  // H+
                        } else {
                            -1  // H-
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

    /// Get counts of discovered stable elements
    pub fn get_element_counts(&self) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();

        for proton_opt in &self.protons {
            if let Some(proton) = proton_opt {
                if !proton.is_alive() {
                    continue;
                }

                // Track all stable elements and compounds (not O16 bonded pairs)
                let element = if proton.is_sih4() {
                    Some("SiH4")
                } else if proton.is_ch4() {
                    Some("CH4")
                } else if proton.is_h2s() {
                    Some("H2S")
                } else if proton.is_mgh2() {
                    Some("MgH2")
                } else if proton.is_h2o() {
                    Some("H2O")
                } else if proton.is_sulfur32() {
                    Some("S32")
                } else if proton.is_silicon28() {
                    Some("Si28")
                } else if proton.is_magnesium24() {
                    Some("Mg24")
                } else if proton.is_neon20() {
                    Some("Ne20")
                } else if proton.charge() == 6 && proton.neutron_count() == 6 {
                    Some("C12")
                } else if proton.charge() == 2 && proton.neutron_count() == 2 {
                    Some("He4")
                } else if proton.charge() == 1 && proton.neutron_count() == 2 {
                    Some("He3")
                } else if proton.is_stable_hydrogen() {
                    Some("H1")
                } else {
                    None
                };

                if let Some(elem) = element {
                    *counts.entry(elem.to_string()).or_insert(0) += 1;
                }
            }
        }

        counts
    }

    /// Spawn a specific element type at a position with velocity
    pub fn spawn_element(&mut self, element_type: &str, position: Vec2, velocity: Vec2) {
        use crate::constants::proton as pc;

        // Check if at capacity
        if self.get_proton_count() >= self.max_protons {
            return;
        }

        // Find first empty slot
        for i in 0..self.protons.len() {
            if self.protons[i].is_none() || !self.protons[i].as_ref().unwrap().is_alive() {
                let proton = match element_type {
                    "H1" => {
                        // Stable hydrogen
                        let mut p = Proton::new(position, velocity, Color::from_rgba(255, 255, 255, 255), 1.0, 0);
                        p.set_neutron_count(1);
                        p.set_stable_hydrogen(true);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "He3" => {
                        // Helium-3 (charge 1, neutron 2)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(255, 200, 100, 255), 3.0, 1);
                        p.set_neutron_count(2);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "He4" => {
                        // Helium-4 (charge 2, neutron 2)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(255, 255, 100, 255), 4.0, 2);
                        p.set_neutron_count(2);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "C12" => {
                        // Carbon-12 (charge 6, neutron 6)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(100, 100, 100, 255), 12.0, 6);
                        p.set_neutron_count(6);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "Ne20" => {
                        // Neon-20 (charge 10, neutron 10)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(255, 100, 150, 255), 20.0, 10);
                        p.set_neutron_count(10);
                        p.set_neon20(true);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "Mg24" => {
                        // Magnesium-24 (charge 12, neutron 12)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(200, 200, 220, 255), 24.0, 12);
                        p.set_neutron_count(12);
                        p.set_magnesium24(true);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "Si28" => {
                        // Silicon-28 (charge 14, neutron 14)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(160, 130, 90, 255), 28.0, 14);
                        p.set_neutron_count(14);
                        p.set_silicon28(true);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "S32" => {
                        // Sulfur-32 (charge 16, neutron 16)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(220, 220, 80, 255), 32.0, 16);
                        p.set_neutron_count(16);
                        p.set_sulfur32(true);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "H2O" => {
                        // Water molecule (O16 + 2H)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(40, 100, 180, 255), 18.0, 8);
                        p.set_neutron_count(10);
                        p.set_h2o(true);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "H2S" => {
                        // Hydrogen Sulfide (S32 + 2H)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(200, 220, 80, 255), 34.0, 18);
                        p.set_neutron_count(18);
                        p.set_h2s(true);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "MgH2" => {
                        // Magnesium Hydride (Mg24 + 2H)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(180, 180, 190, 255), 26.0, 14);
                        p.set_neutron_count(14);
                        p.set_mgh2(true);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "CH4" => {
                        // Methane (C12 + 4H)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(120, 200, 150, 255), 16.0, 10);
                        p.set_neutron_count(10);
                        p.set_ch4(true);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    "SiH4" => {
                        // Silane (Si28 + 4H)
                        let mut p = Proton::new(position, velocity, Color::from_rgba(220, 100, 50, 255), 32.0, 18);
                        p.set_neutron_count(18);
                        p.set_sih4(true);
                        p.set_max_lifetime(pc::INFINITE_LIFETIME);
                        p
                    },
                    _ => return, // Unknown element type
                };

                self.protons[i] = Some(proton);
                break;
            }
        }
    }
}
