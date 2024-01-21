use super::{color::Color, pixel::Pixel, position::Position};

pub struct Screen {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn plot(&mut self, pixel: &Pixel) {
        self.plot_coordinates(
            pixel.position.x,
            pixel.position.y,
            pixel.color.get_color_value(),
        )
    }

    pub fn fill(&mut self, top_left: &Position, bottom_right: &Position, color: &Color) {
        for y in top_left.y..bottom_right.y {
            for x in top_left.x..bottom_right.x {
                self.plot_coordinates(x, y, color.get_color_value())
            }
        }
    }

    pub fn line(&mut self, p1: &Position, p2: &Position, color: &Color) {
        let mut x0 = p1.x as i32;
        let mut y0 = p1.y as i32;
        let x1 = p2.x as i32;
        let y1 = p2.y as i32;
        let dx = x1.abs_diff(x0) as i32;
        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1.abs_diff(y0) as i32);
        let sy: i32 = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.plot_coordinates(x0 as usize, y0 as usize, color.get_color_value());
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                if x0 == x1 {
                    break;
                }
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 {
                    break;
                }
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn circle(&mut self, center: &Position, radius: f64, color: &Color) {
        for angle in 0..360 {
            let arad = (angle as f64).to_radians();
            let point = Position::new(
                ((center.x as f64) + radius * arad.cos()) as usize,
                ((center.y as f64) + radius * arad.sin()) as usize,
            );
            self.plot_coordinates(point.x, point.y, color.get_color_value());
        }
    }

    fn plot_coordinates(&mut self, x: usize, y: usize, color: u32) {
        let pos = (y * self.width) + x;
        self.buffer[pos] = color
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use crate::painter::{color::Color, position::Position};

    use super::*;

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    #[test]
    fn given_dimensions_then_empty_buffer_is_provided() {
        let under_test = Screen::new(WIDTH, HEIGHT);

        assert_eq!(under_test.width, WIDTH);
        assert_eq!(under_test.height, HEIGHT);
        assert_eq!(under_test.get_buffer().len(), WIDTH * HEIGHT);
        assert_eq!(under_test.get_buffer().iter().find(|&v| *v != 0), None);
    }

    #[test]
    fn given_plot_position_then_buffer_is_modified() {
        let mut under_test = Screen::new(WIDTH, HEIGHT);
        let pixel = Pixel::new(Position::new(1, 0), Color::new(1, 2, 3));

        under_test.plot(&pixel);

        assert_eq!(under_test.get_buffer().get(1), Some(66051).as_ref());
    }

    #[test]
    fn given_coordinates_with_color_then_region_is_filled() {
        let mut under_test = Screen::new(WIDTH, HEIGHT);
        let top_left = Position::new(0, 0);
        let bottom_right = Position::new(WIDTH, HEIGHT);
        let color = Color::new(0, 0, 1);

        under_test.fill(&top_left, &bottom_right, &color);

        assert_eq!(under_test.get_buffer().iter().find(|&v| *v != 1), None);
    }

    macro_rules! line_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (p1, p2, color, expected) = $value;
                let mut under_test = Screen::new(2, 2);
                under_test.line(&p1, &p2, &color);
                assert_eq!(under_test.get_buffer().to_owned(), expected);
            }
        )*
        };
    }

    line_tests! {
        top: (Position::new(0, 0), Position::new(1, 0), Color::new(0, 0, 1), vec![1, 1, 0, 0]),
        bottom: (Position::new(0, 1), Position::new(1, 1), Color::new(0, 0, 1), vec![0, 0, 1, 1]),
        left: (Position::new(0, 0), Position::new(0, 1), Color::new(0, 0, 1), vec![1, 0, 1, 0]),
        right: (Position::new(1, 0), Position::new(1, 1), Color::new(0, 0, 1), vec![0, 1, 0, 1]),
        diag: (Position::new(0, 0), Position::new(1, 1), Color::new(0, 0, 1), vec![1, 0, 0, 1]),
    }

    #[test]
    fn given_coordinate_with_radius_and_color_then_circle_is_drawn() {
        let mut under_test = Screen::new(3, 3);

        under_test.circle(&Position::new(1, 1), 1_f64, &Color::new(0, 0, 1));

        // FIXME: Current assertion seems incorrect
        assert_eq!(
            under_test.get_buffer().to_owned(),
            vec![1, 1, 0, 1, 1, 1, 0, 1, 0]
        );
    }
}
