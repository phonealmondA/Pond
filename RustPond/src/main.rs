// RustPond - Main entry point
// Rust port of the Pond physics simulation

mod constants;
mod proton;

use macroquad::prelude::*;
use proton::Proton;

fn window_conf() -> Conf {
    Conf {
        window_title: "RustPond - Nuclear Physics Simulation".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Test with a few protons
    let mut protons: Vec<Proton> = Vec::new();

    // Spawn initial protons
    for i in 0..5 {
        let x = 200.0 + (i as f32 * 100.0);
        let y = 200.0;
        let velocity = vec2((i as f32 - 2.0) * 20.0, 50.0);
        let energy = 50.0 + (i as f32 * 10.0);

        protons.push(Proton::new(
            vec2(x, y),
            velocity,
            WHITE,
            energy,
            1, // charge
        ));
    }

    let mut frame_count = 0;
    let mut fps_timer = 0.0;
    let mut fps = 0.0;

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

        // Update
        for proton in &mut protons {
            proton.update(delta_time, window_size);
        }

        // Render
        clear_background(BLACK);

        // Draw protons with LOD
        for proton in &protons {
            let segments = calculate_lod(proton.radius());
            proton.render(segments);
        }

        // Draw UI
        draw_text(&format!("FPS: {:.0}", fps), 10.0, 30.0, 30.0, GREEN);
        draw_text(&format!("Protons: {}", protons.len()), 10.0, 60.0, 30.0, GREEN);
        draw_text("RustPond v0.1 - Press ESC to exit", 10.0, 90.0, 20.0, GRAY);

        // Input
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Spawn proton on click
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            protons.push(Proton::new(
                vec2(mouse_pos.0, mouse_pos.1),
                vec2(rand::gen_range(-100.0, 100.0), rand::gen_range(-100.0, 100.0)),
                WHITE,
                100.0,
                1,
            ));
        }

        next_frame().await
    }
}

// LOD system for performance
fn calculate_lod(radius: f32) -> i32 {
    if radius < 3.0 {
        6
    } else if radius < 6.0 {
        12
    } else if radius < 12.0 {
        18
    } else {
        24
    }
}
