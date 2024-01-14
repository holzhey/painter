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
        // TODO: Improve performance by filling slices
        for y in top_left.y..bottom_right.y {
            for x in top_left.x..bottom_right.x {
                self.plot_coordinates(x, y, color.get_color_value())
            }
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
    fn given_position_then_buffer_is_modified() {
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
}
