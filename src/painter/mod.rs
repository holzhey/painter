pub struct Screen {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

pub struct Point {
    x: usize,
    y: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0, (width * height).try_into().unwrap()],
        }
    }

    pub fn plot(&mut self, point: Point, color: u32) {
        let pos = (point.y * self.width) + point.x;
        self.buffer[pos] = color
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
