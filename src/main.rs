use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use painter::{Point, Screen};

pub mod painter;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut screen = Screen::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Firework",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap();
    window.limit_update_rate(Some(Duration::from_micros(16600)));
    let mut color = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                screen.plot(Point::new(x, y), color);
            }
            color = color.checked_add(1).unwrap();
        }

        window
            .update_with_buffer(screen.get_buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
