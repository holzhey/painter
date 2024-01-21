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

    macro_rules! color_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (r, g, b, expected) = $value;
                let under_test = Color::new(r, g, b);
                assert_eq!(under_test.get_color_value(), expected)
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
