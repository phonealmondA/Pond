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

        // STEP 2: Neutron formation (proximity to atoms)
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

        // STEP 3: Electron capture (for neutral protons)
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

        // STEP 4: Nuclear fusion
        self.handle_nuclear_fusion(ring_manager);

        // STEP 5: Spawn from atom collisions
        self.detect_and_spawn_from_atom_collisions(atom_manager);

        // STEP 6: Cleanup dead protons
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

    /// Clear all protons (except stable ones)
    pub fn clear(&mut self) {
        for proton_opt in &mut self.protons {
            if let Some(proton) = proton_opt {
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

    /// Check if proton is near any atom
    fn is_near_atom(&self, _proton_pos: Vec2, _atom_manager: &AtomManager) -> bool {
        // Simple distance check - 50px proximity threshold
        // TODO: implement atom proximity check when atoms are integrated
        false
    }

    /// Find nearby atom position for electron capture
    fn find_nearby_atom(&self, _proton_pos: Vec2, _atom_manager: &AtomManager) -> Option<Vec2> {
        // TODO: implement when we integrate
        None
    }

    /// Mark atom at position for deletion
    fn mark_atom_at_position(&self, _atom_pos: Vec2, _atom_manager: &mut AtomManager) {
        // TODO: implement when we integrate
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

                        // Spawn energy wave
                        ring_manager.add_ring(center_of_mass);

                        // Delete second proton
                        self.protons[j] = None;

                        println!("Helium-3 formed! D + H → He3 + gamma");
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

                        // Spawn BIG energy waves
                        ring_manager.add_ring(center_of_mass);
                        ring_manager.add_ring(center_of_mass);

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

                        println!("Helium-4 formed! He3 + He3 → He4 + 2H");
                        break;
                    }
                }
            }
        }
    }

    /// Detect atom collisions and spawn protons
    fn detect_and_spawn_from_atom_collisions(&mut self, _atom_manager: &AtomManager) {
        // TODO: implement when we integrate atoms
        // For now, this is a stub
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
                self.protons[i] = Some(Proton::new(position, velocity, color, energy, charge));

                if charge == -1 {
                    println!("Negative proton spawned! Charge: -1, Energy: {}", energy);
                }

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
