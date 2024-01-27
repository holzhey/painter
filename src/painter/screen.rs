pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn get_value(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn plot(&mut self, x: usize, y: usize, color: u32) {
        let pos = (y * self.width) + x;
        self.buffer[pos] = color
    }

    pub fn hscroll(&mut self, _ix: usize) {
        // TODO: improve algo
        for line in self.buffer.chunks_mut(self.width) {
            line.copy_within(2..line.len(), 1);
        }
    }

    pub fn vscroll(&mut self, _i: usize) {
        self.buffer
            .copy_within(self.width..self.width * self.height, 0);
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
        assert_eq!(under_test.height, HEIGHT);
        assert_eq!(under_test.get_buffer().len(), WIDTH * HEIGHT);
        assert_eq!(under_test.get_buffer().iter().find(|&v| *v != 0), None);
    }

    #[test]
    fn given_rgb_then_color_is_computed() {
        let under_test = Color::new(1, 2, 3);

        assert_eq!(under_test.get_value(), 66051);
    }

    macro_rules! color_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (r, g, b, expected) = $value;
                let under_test = Color::new(r, g, b);
                assert_eq!(under_test.get_value(), expected)
           }
        )*
        };
    }

    color_tests! {
         blue_color: (0,0,1,1),
         red_color: (1,0,0,65536),
         green_color: (0,1,0,256),
         all_rgb: (1,2,3,66051),
         min_rgb: (0,0,0,0),
         max_rgb: (255,255,255,16777215),
    }
}
