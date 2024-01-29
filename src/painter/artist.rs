use super::screen::{Canvas, Color, Position};

pub fn fill(screen: &mut Canvas, top_left: &Position, bottom_right: &Position, color: &Color) {
    for y in top_left.y..bottom_right.y {
        for x in top_left.x..bottom_right.x {
            screen.plot(x, y, color.get_value())
        }
    }
}

pub fn line(screen: &mut Canvas, p1: &Position, p2: &Position, color: &Color) {
    let mut x0 = p1.x as i32;
    let mut y0 = p1.y as i32;
    let x1 = p2.x as i32;
    let y1 = p2.y as i32;
    let dx = x1.abs_diff(x0) as i32;
    let sx: i32 = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1.abs_diff(y0) as i32);
    let sy: i32 = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        screen.plot(x0 as usize, y0 as usize, color.get_value());
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            if x0 == x1 {
                break;
            }
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            if y0 == y1 {
                break;
            }
            err += dx;
            y0 += sy;
        }
    }
}

pub fn circle(screen: &mut Canvas, center: &Position, radius: f64, color: &Color) {
    for angle in 0..360 {
        let arad = (angle as f64).to_radians();
        let point = Position::new(
            ((center.x as f64) + radius * arad.cos()) as usize,
            ((center.y as f64) + radius * arad.sin()) as usize,
        );
        screen.plot(point.x, point.y, color.get_value());
    }
}

pub fn circle_segmented(
    screen: &mut Canvas,
    center: &Position,
    radius: f64,
    color: &Color,
    segments: u8,
) {
    let inc: f64 = 360_f64 / segments as f64;
    let mut angle = 0_f64;
    let mut old_pos = create_position(angle, center.x, center.y, radius);
    loop {
        angle += inc;
        let new_pos = create_position(angle, center.x, center.y, radius);
        line(screen, &old_pos, &new_pos, color);
        old_pos = new_pos;
        if angle > 360_f64 {
            break;
        }
    }
}

fn create_position(angle: f64, x: usize, y: usize, radius: f64) -> Position {
    let arad = angle.to_radians();
    Position::new(
        ((x as f64) + radius * arad.cos()) as usize,
        ((y as f64) + radius * arad.sin()) as usize,
    )
}

#[cfg(test)]
mod tests {
    use crate::painter::screen::Color;

    use super::*;

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    #[test]
    fn given_coordinates_with_color_then_region_is_filled() {
        let mut screen = Canvas::new(WIDTH, HEIGHT);
        let top_left = Position::new(0, 0);
        let bottom_right = Position::new(WIDTH, HEIGHT);
        let color = Color::new(0, 0, 1);

        fill(&mut screen, &top_left, &bottom_right, &color);

        assert_eq!(screen.get_buffer().iter().find(|&v| *v != 1), None);
    }

    macro_rules! line_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (p1, p2, color, expected) = $value;
                let mut screen = Canvas::new(2, 2);
                line(&mut screen, &p1, &p2, &color);
                assert_eq!(screen.get_buffer().to_owned(), expected);
            }
        )*
        };
    }

    line_tests! {
        top: (Position::new(0, 0), Position::new(1, 0), Color::new(0, 0, 1), vec![1, 1, 0, 0]),
        bottom: (Position::new(0, 1), Position::new(1, 1), Color::new(0, 0, 1), vec![0, 0, 1, 1]),
        left: (Position::new(0, 0), Position::new(0, 1), Color::new(0, 0, 1), vec![1, 0, 1, 0]),
        right: (Position::new(1, 0), Position::new(1, 1), Color::new(0, 0, 1), vec![0, 1, 0, 1]),
        diag: (Position::new(0, 0), Position::new(1, 1), Color::new(0, 0, 1), vec![1, 0, 0, 1]),
    }

    #[test]
    fn given_coordinate_with_radius_and_color_then_circle_is_drawn() {
        let mut screen = Canvas::new(3, 3);

        circle(
            &mut screen,
            &Position::new(1, 1),
            1_f64,
            &Color::new(0, 0, 1),
        );

        // FIXME: Current assertion seems incorrect
        assert_eq!(
            screen.get_buffer().to_owned(),
            vec![1, 1, 0, 1, 1, 1, 0, 1, 0]
        );
    }
}
