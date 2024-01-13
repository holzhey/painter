use super::pixel::Pixel;

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
        let pos = (pixel.position.y * self.width) + pixel.position.x;
        self.buffer[pos] = pixel.color.get_color_value()
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use crate::painter::{color::Color, position::Position};

    use super::*;

    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;

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
}
