pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn get_color_value(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_rgb_then_color_is_computed() {
        let under_test = Color::new(1, 2, 3);

        assert_eq!(under_test.get_color_value(), 66051);
    }
}
