pub struct Canvas {
    width: usize,
    _height: usize,
    buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            _height: height,
            buffer: vec![0; width * height],
        }
    }

    pub fn plot(&mut self, x: usize, y: usize, color: u32) {
        let pos = (y * self.width) + x;
        self.buffer[pos] = color
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    #[test]
    fn given_coordinates_then_position_is_created() {
        let under_test = Position::new(1, 2);

        assert_eq!(under_test.x, 1);
        assert_eq!(under_test.y, 2);
    }

    #[test]
    fn given_dimensions_then_empty_buffer_is_provided() {
        let under_test = Canvas::new(WIDTH, HEIGHT);

        assert_eq!(under_test.width, WIDTH);
        assert_eq!(under_test._height, HEIGHT);
        assert_eq!(under_test.get_buffer().len(), WIDTH * HEIGHT);
        assert_eq!(under_test.get_buffer().iter().find(|&v| *v != 0), None);
    }
}
