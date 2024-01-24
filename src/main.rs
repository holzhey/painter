use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use painter::{
    artist::{circle, fill, line},
    color::RGB,
    screen::Canvas,
    screen::Position,
};

pub mod painter;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut screen = Canvas::new(WIDTH, HEIGHT);

    let mut window = Window::new("Firework", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    window.limit_update_rate(Some(Duration::from_micros(16600)));
    let mut color_index = 0;
    let p1 = Position::new(0, 0);
    let p2 = Position::new(WIDTH, HEIGHT);
    let mut angle = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let color = RGB::new(1, 1, color_index);
        fill(&mut screen, &p1, &p2, &color);
        draw_triangle(angle, &mut screen);
        angle += 1;
        circle(
            &mut screen,
            &Position::new(320, 180),
            60_f64,
            &RGB::new(50, 60, 150),
        );
        color_index = color_index.checked_add(1).unwrap_or(1);
        window
            .update_with_buffer(screen.get_buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
    for _ in 0..WIDTH {
        screen.scroll(1, 1);
        window
            .update_with_buffer(screen.get_buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}

fn draw_triangle(angle: usize, screen: &mut Canvas) {
    let p1 = get_coordinates(angle);
    let p2 = get_coordinates(angle + 120);
    let p3 = get_coordinates(angle + 240);
    let white = RGB::new(255, 255, 255);
    line(screen, &p1, &p2, &white);
    line(screen, &p2, &p3, &white);
    line(screen, &p3, &p1, &white);
}

fn get_coordinates(angle: usize) -> Position {
    let arad = (angle as f64).to_radians();
    Position::new(
        (320_f64 + 250_f64 * arad.cos()) as usize,
        (180_f64 + 100_f64 * arad.sin()) as usize,
    )
}
