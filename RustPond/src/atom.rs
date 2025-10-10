// Atom module - Path-following atoms that track ring intersections
// Rust port of AtomManager.h/cpp

use macroquad::prelude::*;
use crate::constants::*;
use crate::ring::Ring;
use std::collections::HashSet;

/// Represents any ring shape (main ring or bounce reflection)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RingShape {
    pub center: Vec2,
    pub radius: f32,
    pub color: Color,
    pub source_ring_id: usize, // ID instead of pointer
    pub bounce_index: i32,     // -1 for main ring, 0+ for bounce shapes
}

impl RingShape {
    pub fn new(center: Vec2, radius: f32, color: Color, source_ring_id: usize, bounce_index: i32) -> Self {
        Self {
            center,
            radius,
            color,
            source_ring_id,
            bounce_index,
        }
    }
}

/// Path-following atom that moves along intersection points
pub struct PathFollowingAtom {
    current_position: Vec2,
    previous_position: Vec2,
    color: Color,
    radius: f32,
    energy: f32,
    lifetime: f32,
    max_lifetime: f32,
    is_alive: bool,
    marked_for_deletion: bool,
    pulse_timer: f32,
    fade_start_time: f32,

    // Track which two shapes this atom follows
    shape1: RingShape,
    shape2: RingShape,
    has_valid_shapes: bool,
}

impl PathFollowingAtom {
    /// Create a new atom at the intersection of two ring shapes
    pub fn new(shape1: RingShape, shape2: RingShape, initial_position: Vec2) -> Self {
        let color = Self::calculate_interference_color(shape1.color, shape2.color);
        let energy = Self::calculate_interference_energy(shape1.color, shape2.color);

        let radius = atom::RADIUS_BASE + (energy * atom::RADIUS_ENERGY_FACTOR);
        let max_lifetime = atom::LIFETIME_BASE + (energy * atom::LIFETIME_ENERGY_FACTOR);
        let fade_start_time = max_lifetime * atom::FADE_START_RATIO;

        Self {
            current_position: initial_position,
            previous_position: initial_position,
            color,
            radius,
            energy,
            lifetime: 0.0,
            max_lifetime,
            is_alive: true,
            marked_for_deletion: false,
            pulse_timer: 0.0,
            fade_start_time,
            shape1,
            shape2,
            has_valid_shapes: true,
        }
    }

    /// Update position based on current intersection of tracked shapes
    pub fn update(&mut self, delta_time: f32, all_current_shapes: &[RingShape]) {
        if !self.is_alive {
            return;
        }

        self.lifetime += delta_time;
        self.pulse_timer += delta_time;

        // Check if atom should die from age
        if self.lifetime >= self.max_lifetime {
            self.is_alive = false;
            return;
        }

        // Find current versions of our tracked shapes
        let (current_shape1, current_shape2) = match self.find_current_shapes(all_current_shapes) {
            Some(shapes) => shapes,
            None => {
                self.has_valid_shapes = false;
                self.is_alive = false;
                return;
            }
        };

        // Check if shapes still intersect
        if !Self::circles_intersect(&current_shape1, &current_shape2) {
            self.is_alive = false;
            return;
        }

        // Update position to current intersection point
        self.previous_position = self.current_position;
        self.current_position = self.calculate_intersection_point(&current_shape1, &current_shape2);
    }

    /// Render the atom with pulsing effects
    pub fn render(&self, segments: u8) {
        if !self.is_alive || !self.has_valid_shapes {
            return;
        }

        // Create pulsing effect based on energy
        let pulse_frequency = atom::PULSE_FREQUENCY_BASE + (self.energy * atom::PULSE_FREQUENCY_ENERGY_FACTOR);
        let pulse_intensity = atom::PULSE_INTENSITY_BASE + (self.energy * atom::PULSE_INTENSITY_ENERGY_FACTOR);
        let pulse = (self.pulse_timer * pulse_frequency).sin() * pulse_intensity + 1.0;

        // Apply pulsing to color
        let mut pulsing_color = self.color;
        pulsing_color.r = (self.color.r * pulse).min(1.0);
        pulsing_color.g = (self.color.g * pulse).min(1.0);
        pulsing_color.b = (self.color.b * pulse).min(1.0);

        // Fade out near end of lifetime
        if self.lifetime > self.fade_start_time {
            let fade_ratio = (self.lifetime - self.fade_start_time) / (self.max_lifetime - self.fade_start_time);
            let fade_amount = 1.0 - fade_ratio;
            pulsing_color.a = fade_amount;
        }

        // Slight size pulsing based on energy
        let size_multiplier = 1.0 + ((self.pulse_timer * pulse_frequency).sin()
            * atom::SIZE_PULSE_FACTOR * self.energy * atom::SIZE_PULSE_ENERGY_FACTOR);
        let current_radius = self.radius * size_multiplier;

        // Draw the atom
        draw_circle(self.current_position.x, self.current_position.y, current_radius, pulsing_color);

        // Optional: draw with polygon for better quality
        if segments > 0 {
            draw_poly(
                self.current_position.x,
                self.current_position.y,
                segments,
                current_radius,
                0.0,
                pulsing_color,
            );
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive && self.has_valid_shapes && !self.marked_for_deletion
    }

    pub fn get_position(&self) -> Vec2 {
        self.current_position
    }

    pub fn get_energy(&self) -> f32 {
        self.energy
    }

    pub fn mark_for_deletion(&mut self) {
        self.marked_for_deletion = true;
    }

    /// Check if this atom is tracking the given shape pair
    pub fn is_tracking_shapes(&self, shape1: &RingShape, shape2: &RingShape) -> bool {
        (self.shape1 == *shape1 && self.shape2 == *shape2) ||
        (self.shape1 == *shape2 && self.shape2 == *shape1)
    }

    /// Find current versions of tracked shapes in the current shape list
    fn find_current_shapes(&self, all_current_shapes: &[RingShape]) -> Option<(RingShape, RingShape)> {
        let mut found1 = None;
        let mut found2 = None;

        for shape in all_current_shapes {
            if found1.is_none() && *shape == self.shape1 {
                found1 = Some(*shape);
            } else if found2.is_none() && *shape == self.shape2 {
                found2 = Some(*shape);
            }

            if found1.is_some() && found2.is_some() {
                break;
            }
        }

        match (found1, found2) {
            (Some(s1), Some(s2)) => Some((s1, s2)),
            _ => None,
        }
    }

    /// Calculate intersection point between two circles
    fn calculate_intersection_point(&self, shape1: &RingShape, shape2: &RingShape) -> Vec2 {
        let dx = shape2.center.x - shape1.center.x;
        let dy = shape2.center.y - shape1.center.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance == 0.0 || distance > shape1.radius + shape2.radius ||
           distance < (shape1.radius - shape2.radius).abs()
        {
            return shape1.center; // Fallback
        }

        // Calculate intersection points using circle-circle intersection formula
        let a = (shape1.radius * shape1.radius - shape2.radius * shape2.radius + distance * distance) / (2.0 * distance);
        let h = (shape1.radius * shape1.radius - a * a).sqrt();

        // Point on line between centers
        let px = shape1.center.x + (a * dx) / distance;
        let py = shape1.center.y + (a * dy) / distance;

        // Choose intersection point closer to previous position
        let intersection1 = vec2(px + (h * dy) / distance, py - (h * dx) / distance);
        let intersection2 = vec2(px - (h * dy) / distance, py + (h * dx) / distance);

        let dist1 = self.previous_position.distance_squared(intersection1);
        let dist2 = self.previous_position.distance_squared(intersection2);

        if dist1 < dist2 { intersection1 } else { intersection2 }
    }

    /// Check if two circles intersect
    fn circles_intersect(shape1: &RingShape, shape2: &RingShape) -> bool {
        let dx = shape2.center.x - shape1.center.x;
        let dy = shape2.center.y - shape1.center.y;
        let distance = (dx * dx + dy * dy).sqrt();

        distance <= shape1.radius + shape2.radius &&
        distance >= (shape1.radius - shape2.radius).abs() &&
        distance > 0.0
    }

    /// Calculate interference color (additive mixing)
    pub fn calculate_interference_color(color1: Color, color2: Color) -> Color {
        Color::new(
            (color1.r + color2.r).min(1.0),
            (color1.g + color2.g).min(1.0),
            (color1.b + color2.b).min(1.0),
            1.0,
        )
    }

    /// Calculate interference energy based on color frequencies
    pub fn calculate_interference_energy(color1: Color, color2: Color) -> f32 {
        use crate::ring::Ring;

        let energy1 = Ring::calculate_frequency_based_speed(color1);
        let energy2 = Ring::calculate_frequency_based_speed(color2);

        let energy_sum = energy1 + energy2;
        let energy_difference = (energy1 - energy2).abs();

        energy_sum + (energy_difference * atom::ENERGY_DIFFERENCE_AMPLIFICATION)
    }

    /// Check if two colors should create interference
    pub fn should_create_interference(color1: Color, color2: Color) -> bool {
        let tolerance = atom::COLOR_TOLERANCE as f32 / 255.0;

        (color1.r - color2.r).abs() > tolerance ||
        (color1.g - color2.g).abs() > tolerance ||
        (color1.b - color2.b).abs() > tolerance
    }
}

/// Manages all atoms, detects intersections, and creates new atoms
pub struct AtomManager {
    atoms: Vec<Option<PathFollowingAtom>>,
    next_slot: usize,
    atom_count: usize,
    max_atoms: usize,
    tracked_intersections: HashSet<u64>,
    cleanup_counter: i32,
}

impl AtomManager {
    pub fn new(max_atoms: usize) -> Self {
        let mut atoms = Vec::with_capacity(max_atoms);
        for _ in 0..max_atoms {
            atoms.push(None);
        }

        Self {
            atoms,
            next_slot: 0,
            atom_count: 0,
            max_atoms,
            tracked_intersections: HashSet::new(),
            cleanup_counter: 0,
        }
    }

    /// Main update method - detects intersections and creates/updates atoms
    pub fn update(&mut self, delta_time: f32, rings: &[Ring], window_size: (f32, f32)) {
        // Get all current shapes
        let all_shapes = self.get_all_shapes(rings);

        // Update atoms (interleaved for performance)
        static mut UPDATE_FIRST_HALF: bool = true;
        unsafe {
            UPDATE_FIRST_HALF = !UPDATE_FIRST_HALF;

            let start_idx = if UPDATE_FIRST_HALF { 0 } else { self.atom_count / 2 };
            let end_idx = if UPDATE_FIRST_HALF { self.atom_count / 2 } else { self.atom_count };

            for i in start_idx..end_idx {
                if let Some(atom) = &mut self.atoms[i] {
                    atom.update(delta_time * atom::DELTA_TIME_COMPENSATION, &all_shapes);
                }
            }
        }

        // Detect new intersections and create atoms
        self.detect_new_intersections(&all_shapes, window_size);

        // Clean up intersection tracking periodically
        self.cleanup_intersection_tracking();
    }

    /// Draw all atoms
    pub fn draw(&self, segments: u8) {
        for i in 0..self.atom_count {
            if let Some(atom) = &self.atoms[i] {
                atom.render(segments);
            }
        }
    }

    /// Clear all atoms
    pub fn clear(&mut self) {
        for atom in &mut self.atoms {
            *atom = None;
        }
        self.atom_count = 0;
        self.next_slot = 0;
        self.tracked_intersections.clear();
    }

    pub fn get_atom_count(&self) -> usize {
        self.atom_count
    }

    pub fn get_max_atoms(&self) -> usize {
        self.max_atoms
    }

    /// Get all shapes from rings (main + bounce shapes)
    fn get_all_shapes(&self, rings: &[Ring]) -> Vec<RingShape> {
        let mut shapes = Vec::new();

        for (ring_id, ring) in rings.iter().enumerate() {
            if !ring.is_alive() {
                continue;
            }

            // Add main ring
            shapes.push(RingShape::new(
                ring.get_center(),
                ring.get_radius(),
                ring.get_color(),
                ring_id,
                -1,
            ));

            // Add bounce shapes
            let bounce_count = ring.get_bounce_shape_count();
            for i in 0..bounce_count {
                let bounce_center = ring.get_bounce_shape_center(i as i32);
                shapes.push(RingShape::new(
                    bounce_center,
                    ring.get_radius(),
                    ring.get_color(),
                    ring_id,
                    i as i32,
                ));
            }
        }

        shapes
    }

    /// Detect new intersections and create atoms
    fn detect_new_intersections(&mut self, all_shapes: &[RingShape], window_size: (f32, f32)) {
        // Simple O(n²) for now - can optimize with spatial grid later
        for i in 0..all_shapes.len() {
            for j in (i + 1)..all_shapes.len() {
                self.check_shape_pair_for_new_intersection(&all_shapes[i], &all_shapes[j], window_size);
            }
        }
    }

    /// Check if a pair of shapes should create a new atom
    fn check_shape_pair_for_new_intersection(&mut self, shape1: &RingShape, shape2: &RingShape, window_size: (f32, f32)) {
        // Don't check intersections between shapes from the same ring
        if shape1.source_ring_id == shape2.source_ring_id {
            return;
        }

        // Check if they should create interference
        if !PathFollowingAtom::should_create_interference(shape1.color, shape2.color) {
            return;
        }

        // Fast intersection check
        let dx = shape2.center.x - shape1.center.x;
        let dy = shape2.center.y - shape1.center.y;
        let distance_squared = dx * dx + dy * dy;

        if distance_squared < EPSILON {
            return;
        }

        let sum_radii = shape1.radius + shape2.radius;
        let diff_radii = (shape1.radius - shape2.radius).abs();

        if distance_squared > sum_radii * sum_radii || distance_squared < diff_radii * diff_radii {
            return;
        }

        // Create unique key for this intersection
        let key = self.create_intersection_key(shape1, shape2);

        // Check if we're already tracking this intersection
        if self.tracked_intersections.contains(&key) {
            return;
        }

        // Check if any existing atom is already tracking this pair
        for i in 0..self.atom_count {
            if let Some(atom) = &self.atoms[i] {
                if atom.is_alive() && atom.is_tracking_shapes(shape1, shape2) {
                    return;
                }
            }
        }

        // Calculate intersection point
        let distance = distance_squared.sqrt();
        let a = (shape1.radius * shape1.radius - shape2.radius * shape2.radius + distance_squared) / (2.0 * distance);
        let h = (shape1.radius * shape1.radius - a * a).sqrt();

        let px = shape1.center.x + (a * dx) / distance;
        let py = shape1.center.y + (a * dy) / distance;

        let intersection_point = vec2(px + (h * dy) / distance, py - (h * dx) / distance);

        // Check if intersection point is within screen bounds
        let margin = atom::INTERSECTION_MARGIN;
        if intersection_point.x >= -margin && intersection_point.x <= window_size.0 + margin &&
           intersection_point.y >= -margin && intersection_point.y <= window_size.1 + margin
        {
            self.tracked_intersections.insert(key);
            self.add_path_following_atom(*shape1, *shape2, intersection_point);
        }
    }

    /// Add a new path-following atom (FIFO system)
    fn add_path_following_atom(&mut self, shape1: RingShape, shape2: RingShape, intersection_point: Vec2) {
        self.atoms[self.next_slot] = Some(PathFollowingAtom::new(shape1, shape2, intersection_point));

        self.next_slot = (self.next_slot + 1) % self.max_atoms;

        if self.atom_count < self.max_atoms {
            self.atom_count += 1;
        }
    }

    /// Create a unique numeric key for an intersection
    fn create_intersection_key(&self, shape1: &RingShape, shape2: &RingShape) -> u64 {
        let (first, second) = if shape1.source_ring_id < shape2.source_ring_id ||
            (shape1.source_ring_id == shape2.source_ring_id && shape1.bounce_index < shape2.bounce_index)
        {
            (shape1, shape2)
        } else {
            (shape2, shape1)
        };

        let key1 = ((first.source_ring_id as u32) << 16) | ((first.bounce_index + 100) as u32 & 0xFFFF);
        let key2 = ((second.source_ring_id as u32) << 16) | ((second.bounce_index + 100) as u32 & 0xFFFF);

        ((key1 as u64) << 32) | (key2 as u64)
    }

    /// Clean up intersection tracking periodically
    fn cleanup_intersection_tracking(&mut self) {
        self.cleanup_counter += 1;

        if self.cleanup_counter >= atom::CLEANUP_INTERVAL {
            self.tracked_intersections.clear();
            self.cleanup_counter = 0;
        }
    }
}
