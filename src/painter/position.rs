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

    #[test]
    fn given_coordinates_then_position_is_created() {
        let under_test = Position::new(1, 2);

        assert_eq!(under_test.x, 1);
        assert_eq!(under_test.y, 2);
    }
}
