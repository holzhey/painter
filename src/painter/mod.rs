pub struct Screen {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

pub struct Position {
    x: usize,
    y: usize,
}

pub trait Color {
    fn get_color_value(self) -> u32;
}

pub struct ColorRGB {
    r: u8,
    g: u8,
    b: u8,
}

impl Color for ColorRGB {
    fn get_color_value(self) -> u32 {
        self.b as u32
    }
}

pub struct Pixel {
    position: Position,
    color: dyn Color,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0, (width * height).try_into().unwrap()],
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

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Pixel {
    pub fn new(position: Position, color: impl Color) -> Self {
        Self { position, color }
    }
}

impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
