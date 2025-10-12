// RustPond - Main entry point
// Rust port of the Pond physics simulation

mod constants;
mod proton;
mod ring;
mod atom;
mod proton_manager;

use macroquad::prelude::*;
use ring::RingManager;
use atom::AtomManager;
use proton_manager::ProtonManager;
use std::collections::HashSet;

// UI State structures
#[derive(PartialEq)]
enum MenuState {
    None,
    Elements,
    Controls,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ElementType {
    H1,
    He3,
    He4,
    C12,
    Ne20,
    Mg24,
    Si28,
    S32,
}

impl ElementType {
    fn name(&self) -> &str {
        match self {
            ElementType::H1 => "H1",
            ElementType::He3 => "He3",
            ElementType::He4 => "He4",
            ElementType::C12 => "C12",
            ElementType::Ne20 => "Ne20",
            ElementType::Mg24 => "Mg24",
            ElementType::Si28 => "Si28",
            ElementType::S32 => "S32",
        }
    }

    fn color(&self) -> Color {
        match self {
            ElementType::H1 => Color::from_rgba(255, 255, 255, 255),
            ElementType::He3 => Color::from_rgba(255, 200, 100, 255),
            ElementType::He4 => Color::from_rgba(255, 255, 100, 255),
            ElementType::C12 => Color::from_rgba(100, 100, 100, 255),
            ElementType::Ne20 => Color::from_rgba(255, 100, 150, 255),
            ElementType::Mg24 => Color::from_rgba(200, 200, 220, 255),
            ElementType::Si28 => Color::from_rgba(160, 130, 90, 255),
            ElementType::S32 => Color::from_rgba(220, 220, 80, 255),
        }
    }

    fn all() -> Vec<ElementType> {
        vec![
            ElementType::H1,
            ElementType::He3,
            ElementType::He4,
            ElementType::C12,
            ElementType::Ne20,
            ElementType::Mg24,
            ElementType::Si28,
            ElementType::S32,
        ]
    }
}

#[derive(Clone)]
struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: String,
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            label: label.to_string(),
        }
    }

    fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    fn draw(&self) {
        // Button background
        draw_rectangle(self.x, self.y, self.width, self.height, Color::from_rgba(50, 50, 50, 200));
        // Button border
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, WHITE);
        // Button text
        let text_dims = measure_text(&self.label, None, 20, 1.0);
        let text_x = self.x + (self.width - text_dims.width) / 2.0;
        let text_y = self.y + (self.height + text_dims.height) / 2.0 - 2.0;
        draw_text(&self.label, text_x, text_y, 20.0, WHITE);
    }
}

fn draw_elements_menu(discovered: &HashSet<ElementType>, counts: &std::collections::HashMap<String, usize>, window_size: (f32, f32)) {
    // Semi-transparent background overlay
    draw_rectangle(0.0, 0.0, window_size.0, window_size.1, Color::from_rgba(0, 0, 0, 180));

    // Menu panel
    let menu_width = 400.0;
    let menu_height = 500.0;
    let menu_x = (window_size.0 - menu_width) / 2.0;
    let menu_y = (window_size.1 - menu_height) / 2.0;

    draw_rectangle(menu_x, menu_y, menu_width, menu_height, Color::from_rgba(30, 30, 30, 255));
    draw_rectangle_lines(menu_x, menu_y, menu_width, menu_height, 3.0, WHITE);

    // Title
    let title = "DISCOVERED ELEMENTS";
    let title_dims = measure_text(title, None, 30, 1.0);
    draw_text(title, menu_x + (menu_width - title_dims.width) / 2.0, menu_y + 40.0, 30.0, YELLOW);

    // Element list
    let mut y_offset = menu_y + 80.0;
    let line_height = 40.0;

    for element in ElementType::all() {
        if discovered.contains(&element) {
            let count = counts.get(element.name()).unwrap_or(&0);
            let text = format!("{} ({})", element.name(), count);

            // Draw element circle
            draw_circle(menu_x + 30.0, y_offset, 12.0, element.color());

            // Draw element text
            draw_text(&text, menu_x + 60.0, y_offset + 7.0, 24.0, WHITE);

            y_offset += line_height;
        }
    }

    // Instructions
    let instructions = "Click an element to select it | Click outside to close";
    let inst_dims = measure_text(instructions, None, 18, 1.0);
    draw_text(instructions, menu_x + (menu_width - inst_dims.width) / 2.0, menu_y + menu_height - 20.0, 18.0, GRAY);
}

fn draw_controls_menu(fps: f32, ring_manager: &RingManager, atom_manager: &AtomManager, proton_manager: &ProtonManager, window_size: (f32, f32), color_info: &str) {
    // Semi-transparent background overlay
    draw_rectangle(0.0, 0.0, window_size.0, window_size.1, Color::from_rgba(0, 0, 0, 180));

    // Menu panel
    let menu_width = 600.0;
    let menu_height = 550.0;
    let menu_x = (window_size.0 - menu_width) / 2.0;
    let menu_y = (window_size.1 - menu_height) / 2.0;

    draw_rectangle(menu_x, menu_y, menu_width, menu_height, Color::from_rgba(30, 30, 30, 255));
    draw_rectangle_lines(menu_x, menu_y, menu_width, menu_height, 3.0, WHITE);

    // Title
    let title = "CONTROLS & STATS";
    let title_dims = measure_text(title, None, 30, 1.0);
    draw_text(title, menu_x + (menu_width - title_dims.width) / 2.0, menu_y + 40.0, 30.0, YELLOW);

    // Stats section
    let mut y_offset = menu_y + 80.0;
    draw_text("STATS:", menu_x + 20.0, y_offset, 24.0, LIGHTGRAY);
    y_offset += 35.0;

    draw_text(&format!("FPS: {:.0}", fps), menu_x + 40.0, y_offset, 20.0, GREEN);
    y_offset += 28.0;
    draw_text(&format!("Rings: {}", ring_manager.get_ring_count()), menu_x + 40.0, y_offset, 20.0, GREEN);
    y_offset += 28.0;
    draw_text(&format!("Atoms: {}", atom_manager.get_atom_count()), menu_x + 40.0, y_offset, 20.0, GREEN);
    y_offset += 28.0;
    draw_text(&format!("Protons: {}", proton_manager.get_proton_count()), menu_x + 40.0, y_offset, 20.0, GREEN);
    y_offset += 28.0;
    draw_text(&format!("Current: {}", color_info), menu_x + 40.0, y_offset, 18.0, LIGHTGRAY);

    // Controls section
    y_offset += 40.0;
    draw_text("CONTROLS:", menu_x + 20.0, y_offset, 24.0, LIGHTGRAY);
    y_offset += 35.0;

    let controls = vec![
        "Left Click: Spawn energy ring",
        "Right Click & Drag: Spawn selected element with velocity",
        "C: Cycle ring color",
        "R: Clear all non-stable particles",
        "Space: Clear all non-stable particles",
        "H: Delete all stable hydrogen",
        "Z: Clear all protons",
        "P: Pause/unpause simulation",
        "Esc: Exit game",
    ];

    for control in controls {
        draw_text(control, menu_x + 40.0, y_offset, 18.0, WHITE);
        y_offset += 26.0;
    }

    // Instructions
    let instructions = "Click outside to close";
    let inst_dims = measure_text(instructions, None, 18, 1.0);
    draw_text(instructions, menu_x + (menu_width - inst_dims.width) / 2.0, menu_y + menu_height - 20.0, 18.0, GRAY);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "RustPond - Nuclear Physics Simulation".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Initialize managers
    let mut ring_manager = RingManager::new();
    let mut atom_manager = AtomManager::new(100);
    let mut proton_manager = ProtonManager::new(300);

    let mut frame_count = 0;
    let mut fps_timer = 0.0;
    let mut fps = 0.0;
    let mut paused = false;

    // UI State
    let mut menu_state = MenuState::None;
    let mut discovered_elements: HashSet<ElementType> = HashSet::new();
    let mut selected_element: Option<ElementType> = None;

    // Right-click drag state for element spawning
    let mut right_click_start: Option<Vec2> = None;
    let mut is_dragging_right = false;

    // Create buttons
    let elements_button = Button::new(10.0, 10.0, 120.0, 40.0, "Elements");
    let controls_button = Button::new(0.0, 10.0, 120.0, 40.0, "Controls"); // x will be set in loop

    loop {
        let delta_time = get_frame_time();
        let window_size = (screen_width(), screen_height());

        // Update controls button position (top right)
        let mut controls_button_positioned = controls_button.clone();
        controls_button_positioned.x = window_size.0 - controls_button.width - 10.0;

        // FPS counter
        fps_timer += delta_time;
        frame_count += 1;
        if fps_timer >= 1.0 {
            fps = frame_count as f32 / fps_timer;
            fps_timer = 0.0;
            frame_count = 0;
        }

        // Update discovered elements
        let element_counts = proton_manager.get_element_counts();
        for (element_name, _) in &element_counts {
            let element_type = match element_name.as_str() {
                "H1" => Some(ElementType::H1),
                "He3" => Some(ElementType::He3),
                "He4" => Some(ElementType::He4),
                "C12" => Some(ElementType::C12),
                "Ne20" => Some(ElementType::Ne20),
                "Mg24" => Some(ElementType::Mg24),
                "Si28" => Some(ElementType::Si28),
                "S32" => Some(ElementType::S32),
                _ => None,
            };
            if let Some(et) = element_type {
                discovered_elements.insert(et);
            }
        }

        // Update systems (only if not paused)
        if !paused {
            ring_manager.update(delta_time, window_size);
            atom_manager.update(delta_time, ring_manager.get_all_rings(), window_size);
            proton_manager.update(delta_time, window_size, &mut atom_manager, &mut ring_manager);
        }

        // Render
        clear_background(BLACK);

        // Draw everything
        ring_manager.draw(18);
        // atom_manager.draw(12);  // Atoms are hidden - only used for backend calculations
        proton_manager.draw(24);
        proton_manager.draw_labels();

        // Draw UI - buttons and menus

        // Draw buttons (always visible)
        elements_button.draw();
        controls_button_positioned.draw();

        // Draw selected element indicator
        if let Some(elem) = selected_element {
            let text = format!("Selected: {}", elem.name());
            let text_dims = measure_text(&text, None, 24, 1.0);
            let text_x = (window_size.0 - text_dims.width) / 2.0;
            draw_rectangle(text_x - 10.0, 10.0, text_dims.width + 20.0, 40.0, Color::from_rgba(30, 30, 30, 200));
            draw_text(&text, text_x, 35.0, 24.0, elem.color());
        }

        // Draw menus
        match menu_state {
            MenuState::Elements => {
                draw_elements_menu(&discovered_elements, &element_counts, window_size);
            },
            MenuState::Controls => {
                draw_controls_menu(fps, &ring_manager, &atom_manager, &proton_manager, window_size, &ring_manager.get_current_frequency_info());
            },
            MenuState::None => {},
        }

        // Show PAUSED indicator
        if paused {
            let pause_text = "PAUSED";
            let pause_font_size = 60.0;
            let text_dims = measure_text(pause_text, None, pause_font_size as u16, 1.0);
            let pause_x = (window_size.0 - text_dims.width) / 2.0;
            let pause_y = window_size.1 / 2.0;

            // Draw with red outline
            draw_text(pause_text, pause_x + 2.0, pause_y + 2.0, pause_font_size, BLACK);
            draw_text(pause_text, pause_x - 2.0, pause_y - 2.0, pause_font_size, BLACK);
            draw_text(pause_text, pause_x + 2.0, pause_y - 2.0, pause_font_size, BLACK);
            draw_text(pause_text, pause_x - 2.0, pause_y + 2.0, pause_font_size, BLACK);
            draw_text(pause_text, pause_x, pause_y, pause_font_size, RED);
        }

        // Input handling
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Toggle pause with P key
        if is_key_pressed(KeyCode::P) {
            paused = !paused;
        }

        // Mouse input handling
        let mouse_pos = mouse_position();

        // Left click handling
        if is_mouse_button_pressed(MouseButton::Left) {
            match menu_state {
                MenuState::None => {
                    // Check button clicks
                    if elements_button.contains_point(mouse_pos.0, mouse_pos.1) {
                        menu_state = MenuState::Elements;
                    } else if controls_button_positioned.contains_point(mouse_pos.0, mouse_pos.1) {
                        menu_state = MenuState::Controls;
                    } else if !paused {
                        // Spawn ring if not clicking UI
                        ring_manager.add_ring(vec2(mouse_pos.0, mouse_pos.1));
                    }
                },
                MenuState::Elements => {
                    // Check if clicking an element in the menu
                    let menu_width = 400.0;
                    let menu_height = 500.0;
                    let menu_x = (window_size.0 - menu_width) / 2.0;
                    let menu_y = (window_size.1 - menu_height) / 2.0;

                    // Check if clicking inside menu
                    if mouse_pos.0 >= menu_x && mouse_pos.0 <= menu_x + menu_width &&
                       mouse_pos.1 >= menu_y && mouse_pos.1 <= menu_y + menu_height {
                        // Check which element was clicked
                        let mut y_offset = menu_y + 80.0;
                        let line_height = 40.0;

                        for element in ElementType::all() {
                            if discovered_elements.contains(&element) {
                                // Check if mouse is over this element
                                if mouse_pos.1 >= y_offset - line_height / 2.0 && mouse_pos.1 < y_offset + line_height / 2.0 {
                                    selected_element = Some(element);
                                    menu_state = MenuState::None;
                                    break;
                                }
                                y_offset += line_height;
                            }
                        }
                    } else {
                        // Clicked outside, close menu
                        menu_state = MenuState::None;
                    }
                },
                MenuState::Controls => {
                    // Check if clicking outside menu to close
                    let menu_width = 600.0;
                    let menu_height = 550.0;
                    let menu_x = (window_size.0 - menu_width) / 2.0;
                    let menu_y = (window_size.1 - menu_height) / 2.0;

                    if mouse_pos.0 < menu_x || mouse_pos.0 > menu_x + menu_width ||
                       mouse_pos.1 < menu_y || mouse_pos.1 > menu_y + menu_height {
                        menu_state = MenuState::None;
                    }
                },
            }
        }

        // Right click drag for element spawning (only when not paused and element is selected)
        if !paused && selected_element.is_some() && menu_state == MenuState::None {
            if is_mouse_button_pressed(MouseButton::Right) {
                right_click_start = Some(vec2(mouse_pos.0, mouse_pos.1));
                is_dragging_right = true;
            }

            if is_dragging_right && is_mouse_button_down(MouseButton::Right) {
                // Currently dragging, could draw a line showing the drag vector if desired
            }

            if is_dragging_right && is_mouse_button_released(MouseButton::Right) {
                // Spawn element with velocity based on drag
                if let Some(start_pos) = right_click_start {
                    let end_pos = vec2(mouse_pos.0, mouse_pos.1);
                    let drag_vector = end_pos - start_pos;

                    // Velocity is proportional to drag distance (scale by 2 for better feel)
                    let velocity = drag_vector * 2.0;

                    if let Some(elem) = selected_element {
                        proton_manager.spawn_element(elem.name(), start_pos, velocity);
                    }
                }

                right_click_start = None;
                is_dragging_right = false;
            }
        }

        // Cycle color with C key (only when not paused)
        if !paused && is_key_pressed(KeyCode::C) {
            ring_manager.cycle_to_next_color();
        }

        // Clear all with R key
        if is_key_pressed(KeyCode::R) {
            ring_manager.clear();
            atom_manager.clear();
            proton_manager.clear();
        }

        // Clear all with Space bar
        if is_key_pressed(KeyCode::Space) {
            ring_manager.clear();
            atom_manager.clear();
            proton_manager.clear();
        }

        // Delete all stable H protons with H key
        if is_key_pressed(KeyCode::H) {
            proton_manager.delete_stable_hydrogen();
        }

        // Clear all protons with Z key (including immortal elements)
        if is_key_pressed(KeyCode::Z) {
            proton_manager.clear_all();
        }

        next_frame().await
    }
}

