// https://www.desmos.com/calculator/lms03mpiz8

fn y(t: i32, initial_velocity: i32) -> i32 {
    return -(i32::pow(t, 2) - t)/2 + initial_velocity * t;
}

fn max_y(initial_y_velocity: i32) -> i32 {
    if initial_y_velocity < 0 { // if we start going down
        return 0
    }
    return y(initial_y_velocity, initial_y_velocity);
}

fn x(t: i32, initial_velocity: i32) -> i32 {
    if t >= initial_velocity {
        t*(t+1)/2
    } else {
        t*initial_velocity - t*(t-1)/2
    }
}

fn lands_in_target(x_min: i32, x_max: i32, y_min: i32, y_max: i32, velocity: (i32, i32)) -> bool {
    let mut current_x = 0;
    let mut current_y = 0;
    let mut current_x_velocity = velocity.0;
    let mut current_y_velocity = velocity.1;
    loop {
        if current_y < y_min || current_x > x_max { // we've gone too deep/far!
            break;
        }
        if current_x >= x_min && current_x <= x_max && current_y >= y_min && current_y <= y_max {
            return true;
        }
        current_x += current_x_velocity;
        if current_x_velocity > 0 {
            current_x_velocity -= 1;
        }
        current_y += current_y_velocity;
        current_y_velocity -= 1;
    }
    false
}



fn main() {
    // target area: x=277..318, y=-92..-53
    let x_min = 277;
    let x_max = 318;
    let y_min = -92;
    let y_max = -53;
    let mut highest_y_position = 0;
    for x_velocity in 0..x_max {
        for y_velocity in 0..=-y_min {
            if lands_in_target(x_min, x_max, y_min, y_max, (x_velocity, y_velocity)) {
                if max_y(y_velocity) > highest_y_position {
                    highest_y_position = max_y(y_velocity);
                }
            }
        }
    }
    println!("{}", highest_y_position);

}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_y() {
        assert_eq!(max_y(0), 0);
        assert_eq!(max_y(-1), 0);
        assert_eq!(max_y(1), 1);
        assert_eq!(max_y(2), 3);
        assert_eq!(max_y(3), 6);
        assert_eq!(max_y(4), 10);
        assert_eq!(max_y(5), 15);
    }

    #[test]
    fn test_lands_in_target() {
        assert_eq!(lands_in_target(20, 30, -10, -5, (7, 2)), true);
        assert_eq!(lands_in_target(20, 30, -10, -5, (6, 3)), true);
        assert_eq!(lands_in_target(20, 30, -10, -5, (9, 0)), true);
        assert_eq!(lands_in_target(20, 30, -10, -5, (17, -4)), false);
    }

    #[test]
    fn find_highest() {
        // target area: x=20..30, y=-10..-5
        let x_min = 20;
        let x_max = 30;
        let y_min = -10;
        let y_max = -5;
        let mut highest_y_position = 0;
        for x_velocity in 0..x_max {
            for y_velocity in 0..=-y_min {
                if lands_in_target(x_min, x_max, y_min, y_max, (x_velocity, y_velocity)) {
                    if max_y(y_velocity) > highest_y_position {
                        highest_y_position = max_y(y_velocity);
                    }
                }
            }
        }
        assert_eq!(highest_y_position, 45);
    }
}
