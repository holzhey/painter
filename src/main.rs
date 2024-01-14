use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use painter::{color::Color, position::Position, screen::Screen};

pub mod painter;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut screen = Screen::new(WIDTH, HEIGHT);

    let mut window = Window::new("Firework", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    window.limit_update_rate(Some(Duration::from_micros(16600)));
    let mut color_index = 0;
    let p1 = Position::new(0, 0);
    let p2 = Position::new(WIDTH, HEIGHT);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let color = Color::new(1, 1, color_index);
        screen.fill(&p1, &p2, &color);
        color_index = color_index.checked_add(1).unwrap_or(1);
        window
            .update_with_buffer(screen.get_buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
