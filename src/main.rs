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

    loop {
        let delta_time = get_frame_time();
        let window_size = (screen_width(), screen_height());

        // FPS counter
        fps_timer += delta_time;
        frame_count += 1;
        if fps_timer >= 1.0 {
            fps = frame_count as f32 / fps_timer;
            fps_timer = 0.0;
            frame_count = 0;
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

        // Draw UI
        draw_text(&format!("FPS: {:.0}", fps), 10.0, 30.0, 30.0, GREEN);
        draw_text(&format!("Rings: {}", ring_manager.get_ring_count()), 10.0, 60.0, 30.0, GREEN);
        draw_text(&format!("Atoms: {}", atom_manager.get_atom_count()), 10.0, 90.0, 30.0, GREEN);
        draw_text(&format!("Protons: {}", proton_manager.get_proton_count()), 10.0, 120.0, 30.0, GREEN);
        draw_text("RustPond v0.2", 10.0, 150.0, 20.0, GRAY);
        draw_text("Click: Spawn Ring | C: Cycle Color | Space: Clear All | H: Delete H | P: Pause | ESC: Exit", 10.0, 180.0, 20.0, GRAY);

        // Show current ring color
        let color_info = ring_manager.get_current_frequency_info();
        draw_text(&color_info, 10.0, 210.0, 18.0, LIGHTGRAY);

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

        // Spawn ring on click (only when not paused)
        if !paused && is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            ring_manager.add_ring(vec2(mouse_pos.0, mouse_pos.1));
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

        next_frame().await
    }
}

