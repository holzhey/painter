use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use painter::{color::Color, pixel::Pixel, position::Position, screen::Screen};

pub mod painter;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut screen = Screen::new(WIDTH, HEIGHT);

    let mut window = Window::new("Firework", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    window.limit_update_rate(Some(Duration::from_micros(16600)));
    let mut color = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color_rgb = Color::new(1, 1, color);
                let pixel = Pixel::new(Position::new(x, y), color_rgb);
                screen.plot(&pixel);
            }
            color = color.checked_add(1).unwrap_or(1);
        }

        window
            .update_with_buffer(screen.get_buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
