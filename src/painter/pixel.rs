use super::{color::Color, position::Position};

pub struct Pixel {
    pub position: Position,
    pub color: Color,
}

impl Pixel {
    pub fn new(position: Position, color: Color) -> Self {
        Self { position, color }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_position_and_color_then_pixel_is_created() {
        let position_x = 1;
        let position_y = 2;
        let color_r = 10;
        let color_g = 20;
        let color_b = 30;
        let under_test = Pixel::new(
            Position::new(position_x, position_y),
            Color::new(color_r, color_g, color_b),
        );

        assert_eq!(under_test.position.x, position_x);
        assert_eq!(under_test.position.y, position_y);
        assert_eq!(under_test.color.get_color_value(), 660510);
    }
}
