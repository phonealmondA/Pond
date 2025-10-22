use macroquad::prelude::*;

mod constants;
use constants::*;

// Membrane component - represents one lipid molecule in the cell membrane
struct MembraneComponent {
    position: Vec2,      // Current position of the component
    velocity: Vec2,      // Velocity for physics
    angle: f32,          // Angle the component is facing (orientation)
    circle_angle: f32,   // Fixed angle around the cell circle (0 to 2π)
    ideal_radius: f32,   // Ideal distance from center (100 for inner, 125 for outer)
    circle_radius: f32,  // Radius of the lipid head (circle)
    bar_length: f32,     // Length of the lipid tail (bar)
    bar_width: f32,      // Width of the lipid tail (bar)
}

impl MembraneComponent {
    fn new(position: Vec2, angle: f32, circle_angle: f32, ideal_radius: f32) -> Self {
        MembraneComponent {
            position,
            velocity: Vec2::ZERO,
            angle,
            circle_angle,
            ideal_radius,
            circle_radius: LIPID_CIRCLE_RADIUS,
            bar_length: LIPID_BAR_LENGTH,
            bar_width: LIPID_BAR_WIDTH,
        }
    }

    fn update(&mut self, dt: f32) {
        // Apply velocity
        self.position += self.velocity * dt;

        // Apply damping to velocity
        self.velocity *= DAMPING;
    }

    fn draw(&self) {
        let direction = Vec2::new(self.angle.cos(), self.angle.sin());

        // Draw lipid tail (bar) pointing opposite to the direction angle - hydrophobic
        let bar_start = self.position;
        let bar_end = self.position - direction * self.bar_length;
        draw_line(bar_start.x, bar_start.y, bar_end.x, bar_end.y, self.bar_width, LIPID_TAIL_COLOR);

        // Draw lipid head (circle) in the direction of the angle - hydrophilic
        let circle_pos = self.position + direction * self.circle_radius;
        draw_circle(circle_pos.x, circle_pos.y, self.circle_radius, LIPID_HEAD_COLOR);
        draw_circle_lines(circle_pos.x, circle_pos.y, self.circle_radius, LIPID_HEAD_OUTLINE_WIDTH, WHITE);
    }

    fn get_tail_position(&self) -> Vec2 {
        let direction = Vec2::new(self.angle.cos(), self.angle.sin());
        self.position - direction * self.bar_length
    }

    fn get_head_position(&self) -> Vec2 {
        let direction = Vec2::new(self.angle.cos(), self.angle.sin());
        self.position + direction * self.circle_radius
    }
}

// Cell with membrane
struct Cell {
    actual_center: Vec2,      // The actual center position (center of mass)
    center_velocity: Vec2,    // Velocity of the center
    head_position: Vec2,      // The head position (leads ahead during movement for pseudopod formation)
    head_velocity: Vec2,      // Velocity of the head
    input_direction: Vec2,    // Current input direction (WASD)
    stationary_time: f32,     // Time the head has been stationary (for delayed reforming)
    expansion_radius: f32,    // Invisible expanding force radius (0 = inactive)
    expansion_center: Vec2,   // Fixed position of the expansion zone (stays stationary when movement starts)
    expansion_active_time: f32, // Time the expansion has been active during movement
    inner_membrane: Vec<MembraneComponent>,
    outer_membrane: Vec<MembraneComponent>,
}

impl Cell {
    fn new(center: Vec2, num_components: usize) -> Self {
        const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

        let inner_membrane = Self::create_membrane_ring(center, num_components, INNER_MEMBRANE_RADIUS, true);
        let outer_membrane = Self::create_membrane_ring(center, num_components, OUTER_MEMBRANE_RADIUS, false);

        Cell {
            actual_center: center,
            center_velocity: Vec2::ZERO,
            head_position: center,
            head_velocity: Vec2::ZERO,
            input_direction: Vec2::ZERO,
            stationary_time: 0.0,
            expansion_radius: 0.0,
            expansion_center: center,
            expansion_active_time: 0.0,
            inner_membrane,
            outer_membrane,
        }
    }

    fn create_membrane_ring(center: Vec2, num_components: usize, radius: f32, inward_facing: bool) -> Vec<MembraneComponent> {
        const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

        (0..num_components)
            .map(|i| {
                let circle_angle = (i as f32 / num_components as f32) * TWO_PI;
                let position = center + Vec2::new(radius * circle_angle.cos(), radius * circle_angle.sin());

                // Inner membrane faces inward (heads toward center), outer faces outward
                let orientation_angle = if inward_facing {
                    circle_angle + std::f32::consts::PI
                } else {
                    circle_angle
                };

                MembraneComponent::new(position, orientation_angle, circle_angle, radius)
            })
            .collect()
    }

    fn update_head_physics(&mut self, dt: f32) {
        if self.input_direction.length() > 0.0 {
            let acceleration = self.input_direction.normalize() * HEAD_ACCELERATION;
            self.head_velocity += acceleration * dt;
        }

        // Clamp to maximum speed
        if self.head_velocity.length() > HEAD_MAX_SPEED {
            self.head_velocity = self.head_velocity.normalize() * HEAD_MAX_SPEED;
        }

        self.head_velocity *= HEAD_DAMPING;
        self.head_position += self.head_velocity * dt;
    }

    fn update_center_physics(&mut self, dt: f32) {
        let to_head = self.head_position - self.actual_center;
        self.center_velocity += to_head * CENTER_FOLLOW_STRENGTH * dt;

        // Clamp to maximum speed
        if self.center_velocity.length() > CENTER_MAX_SPEED {
            self.center_velocity = self.center_velocity.normalize() * CENTER_MAX_SPEED;
        }

        self.center_velocity *= CENTER_DAMPING;
        self.actual_center += self.center_velocity * dt;
    }

    fn update_expansion_state(&mut self, dt: f32) {
        if self.head_velocity.length() < HEAD_STATIONARY_THRESHOLD {
            // Cell is stationary - grow expansion zone
            self.stationary_time += dt;
            self.expansion_active_time = 0.0; // Reset movement timer

            if self.stationary_time > STATIONARY_DELAY {
                if self.expansion_radius == 0.0 {
                    // Expansion just started - set its fixed center position
                    self.expansion_radius = EXPANSION_INITIAL_RADIUS;
                    self.expansion_center = self.actual_center;
                } else if self.expansion_radius < OUTER_MEMBRANE_RADIUS {
                    self.expansion_radius += EXPANSION_GROWTH_RATE * dt;
                    self.expansion_center = self.actual_center; // Update center while stationary
                } else {
                    self.expansion_radius = OUTER_MEMBRANE_RADIUS;
                    self.expansion_center = self.actual_center; // Update center while stationary
                }
            }
        } else {
            // Cell is moving - keep expansion active for a duration, but stationary
            self.stationary_time = 0.0;

            if self.expansion_radius > 0.0 {
                self.expansion_active_time += dt;

                // Deactivate expansion after persist time
                if self.expansion_active_time >= EXPANSION_PERSIST_TIME {
                    self.expansion_radius = 0.0;
                    self.expansion_active_time = 0.0;
                }
                // Otherwise keep expansion_radius and expansion_center unchanged (stationary)
            }
        }
    }

    fn update(&mut self, dt: f32) {
        self.update_head_physics(dt);
        self.update_center_physics(dt);
        self.update_expansion_state(dt);

        // Apply forces to membrane
        Self::apply_head_push_forces(&mut self.inner_membrane, self.head_position, dt);
        Self::apply_head_push_forces(&mut self.outer_membrane, self.head_position, dt);

        if self.expansion_radius > 0.0 {
            Self::apply_expansion_forces(&mut self.inner_membrane, self.expansion_center, self.expansion_radius, dt);
            Self::apply_expansion_forces(&mut self.outer_membrane, self.expansion_center, self.expansion_radius, dt);
        }

        // Keep membrane layers separated by at least the lipid tail length
        Self::apply_membrane_separation_forces(&mut self.inner_membrane, &mut self.outer_membrane, dt);

        // Update membrane components
        let movement_direction = if self.head_velocity.length() > MOVEMENT_DIRECTION_THRESHOLD {
            self.head_velocity.normalize()
        } else {
            Vec2::ZERO
        };

        Self::update_membrane_ring(&mut self.inner_membrane, self.actual_center, self.head_position, movement_direction, INNER_DESIRED_NEIGHBOR_DISTANCE, dt);
        Self::update_membrane_ring(&mut self.outer_membrane, self.actual_center, self.head_position, movement_direction, OUTER_DESIRED_NEIGHBOR_DISTANCE, dt);
    }

    fn apply_head_push_forces(membrane: &mut Vec<MembraneComponent>, head_center: Vec2, dt: f32) {
        for component in membrane.iter_mut() {
            let to_component = component.position - head_center;
            let distance = to_component.length();

            if distance > 0.0 && distance < HEAD_RADIUS {
                let push_direction = to_component / distance;
                let penetration = HEAD_RADIUS - distance;
                component.velocity += push_direction * penetration * HEAD_PUSH_FORCE * dt;
            }
        }
    }

    fn apply_expansion_forces(membrane: &mut Vec<MembraneComponent>, center: Vec2, expansion_radius: f32, dt: f32) {
        for component in membrane.iter_mut() {
            let to_component = component.position - center;
            let distance = to_component.length();

            if distance > 0.0 && distance < expansion_radius {
                let push_direction = to_component / distance;
                let penetration = expansion_radius - distance;
                component.velocity += push_direction * penetration * EXPANSION_PUSH_FORCE * dt;
            }
        }
    }

    fn apply_membrane_separation_forces(inner_membrane: &mut Vec<MembraneComponent>, outer_membrane: &mut Vec<MembraneComponent>, dt: f32) {
        let min_distance = LIPID_BAR_LENGTH;

        // Calculate approximate center based on inner membrane average position
        let center = {
            let sum: Vec2 = inner_membrane.iter().map(|c| c.position).fold(Vec2::ZERO, |acc, p| acc + p);
            sum / inner_membrane.len() as f32
        };

        // Check each corresponding pair of inner/outer components
        for i in 0..inner_membrane.len() {
            let inner_pos = inner_membrane[i].position;
            let outer_pos = outer_membrane[i].position;

            // === Radial separation force ===
            let delta = outer_pos - inner_pos;
            let distance = delta.length();

            if distance > 0.0 && distance < min_distance {
                // Membranes are too close - apply repulsion forces
                let separation_direction = delta / distance;
                let penetration = min_distance - distance;
                let force_magnitude = penetration * MEMBRANE_SEPARATION_FORCE * dt;

                // Push inner membrane inward, outer membrane outward
                inner_membrane[i].velocity -= separation_direction * force_magnitude;
                outer_membrane[i].velocity += separation_direction * force_magnitude;
            }

            // === Angular alignment force ===
            // Keep paired components at the same angle from center
            let inner_from_center = inner_pos - center;
            let outer_from_center = outer_pos - center;

            let inner_angle = inner_from_center.y.atan2(inner_from_center.x);
            let inner_radius = inner_from_center.length();
            let outer_radius = outer_from_center.length();

            if inner_radius > 0.0 && outer_radius > 0.0 {
                // Calculate where the outer component SHOULD be (same angle as inner, but at outer radius)
                let ideal_outer_pos = center + Vec2::new(inner_angle.cos(), inner_angle.sin()) * outer_radius;

                // Apply force to pull outer component toward ideal angular position
                let alignment_delta = ideal_outer_pos - outer_pos;
                let alignment_force = alignment_delta * MEMBRANE_ALIGNMENT_FORCE * dt;
                outer_membrane[i].velocity += alignment_force;

                // Apply opposite force to inner component to conserve momentum
                inner_membrane[i].velocity -= alignment_force * 0.3; // Weaker counter-force
            }
        }
    }

    fn update_membrane_ring(membrane: &mut Vec<MembraneComponent>, actual_center: Vec2, head_position: Vec2, movement_direction: Vec2, desired_distance: f32, dt: f32) {
        // Update component physics
        for component in membrane.iter_mut() {
            Self::update_component_physics(component, actual_center, head_position, movement_direction, dt);
        }

        // Apply neighbor interaction forces for elastic behavior
        let neighbor_forces = Self::calculate_neighbor_forces(membrane, desired_distance);
        for (component, force) in membrane.iter_mut().zip(neighbor_forces.iter()) {
            component.velocity += *force * dt;
        }
    }

    fn calculate_neighbor_forces(membrane: &[MembraneComponent], desired_distance: f32) -> Vec<Vec2> {
        let num_components = membrane.len();
        let mut forces = vec![Vec2::ZERO; num_components];

        for i in 0..num_components {
            let current_pos = membrane[i].position;
            let prev_idx = if i == 0 { num_components - 1 } else { i - 1 };
            let next_idx = if i == num_components - 1 { 0 } else { i + 1 };

            forces[i] += Self::calculate_spring_force(current_pos, membrane[prev_idx].position, desired_distance);
            forces[i] += Self::calculate_spring_force(current_pos, membrane[next_idx].position, desired_distance);
        }

        forces
    }

    fn calculate_spring_force(from: Vec2, to: Vec2, desired_distance: f32) -> Vec2 {
        let delta = to - from;
        let distance = delta.length();

        if distance > 0.0 {
            let displacement = distance - desired_distance;
            (delta / distance) * displacement * NEIGHBOR_FORCE_STRENGTH
        } else {
            Vec2::ZERO
        }
    }

    fn update_component_physics(component: &mut MembraneComponent, actual_center: Vec2, head_position: Vec2, movement_direction: Vec2, dt: f32) {
        // Apply membrane surface flow and forward migration during movement
        if movement_direction.length() > MOVEMENT_DIRECTION_THRESHOLD {
            Self::apply_membrane_flow(component, movement_direction, dt);
            Self::apply_forward_migration(component, head_position, movement_direction, dt);
        }

        // Update component orientation to point toward/away from actual center
        let to_center = actual_center - component.position;
        let angle_to_center = to_center.y.atan2(to_center.x);

        // Inner membrane: heads toward center, tails outward
        // Outer membrane: heads outward, tails toward center
        if component.ideal_radius == INNER_MEMBRANE_RADIUS {
            component.angle = angle_to_center; // Heads point toward center
        } else {
            component.angle = angle_to_center + std::f32::consts::PI; // Heads point away from center
        }

        // Update position
        component.update(dt);
    }

    fn apply_membrane_flow(component: &mut MembraneComponent, movement_direction: Vec2, dt: f32) {
        let component_dir = Vec2::new(component.circle_angle.cos(), component.circle_angle.sin());
        let tangent = Vec2::new(-component_dir.y, component_dir.x);

        let flow_alignment = tangent.dot(movement_direction);
        let flow_rate = flow_alignment * MEMBRANE_FLOW_SPEED;

        component.circle_angle += flow_rate * dt;

        // Normalize angle to 0-2π range
        const TWO_PI: f32 = 2.0 * std::f32::consts::PI;
        if component.circle_angle > TWO_PI {
            component.circle_angle -= TWO_PI;
        } else if component.circle_angle < 0.0 {
            component.circle_angle += TWO_PI;
        }
    }

    fn apply_forward_migration(component: &mut MembraneComponent, head_position: Vec2, movement_direction: Vec2, dt: f32) {
        let to_component = component.position - head_position;
        let distance_behind = -to_component.dot(movement_direction);

        if distance_behind > 0.0 {
            let flow_factor = (distance_behind / FLOW_DISTANCE_NORMALIZER).min(MAX_FLOW_FACTOR);
            component.velocity += movement_direction * flow_factor * MEMBRANE_FORWARD_FLOW_STRENGTH * dt;
        }
    }

    fn handle_movement(&mut self) {
        // WASD input
        let mut input = Vec2::ZERO;

        if is_key_down(KeyCode::W) {
            input.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            input.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            input.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            input.x += 1.0;
        }

        // Store input direction for physics update
        self.input_direction = input;
    }

    fn draw(&self) {
        // Draw expansion zone if active (blue circle stays stationary)
        if self.expansion_radius > 0.0 {
            draw_circle(self.expansion_center.x, self.expansion_center.y, self.expansion_radius, EXPANSION_ZONE_COLOR);
            draw_circle_lines(self.expansion_center.x, self.expansion_center.y, self.expansion_radius, EXPANSION_ZONE_BORDER_WIDTH, EXPANSION_ZONE_BORDER_COLOR);
        }

        // Draw head zone
        draw_circle(self.head_position.x, self.head_position.y, HEAD_RADIUS, HEAD_ZONE_COLOR);
        draw_circle_lines(self.head_position.x, self.head_position.y, HEAD_RADIUS, HEAD_ZONE_BORDER_WIDTH, HEAD_ZONE_BORDER_COLOR);

        // Draw membrane components
        for component in &self.inner_membrane {
            component.draw();
        }
        for component in &self.outer_membrane {
            component.draw();
        }

        // Draw center markers for reference
        draw_circle(self.actual_center.x, self.actual_center.y, CENTER_MARKER_RADIUS, GREEN);
        draw_circle(self.head_position.x, self.head_position.y, CENTER_MARKER_RADIUS, RED);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Cell Membrane Simulation".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let center = Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
    let mut cell = Cell::new(center, NUM_MEMBRANE_COMPONENTS);

    loop {
        let dt = get_frame_time();

        cell.handle_movement();
        cell.update(dt);

        clear_background(BLACK);
        cell.draw();

        next_frame().await
    }
}
