use std::char;
use std::collections::HashMap;
use std::fs;

struct MirrorDeflection {
    pos_x: (i32, i32),
    neg_x: (i32, i32),
    pos_y: (i32, i32),
    neg_y: (i32, i32),
}

fn main() {
    let message = "TpnQSjdmZdpoohd";
    // let message = "T";
    // let mut mirror_rows: Vec<String> = Vec::new();
    // TODO create structs for these
    // The value of the hashmap is a tuple defining the deflection from x and y
    let mut mirror_grid: HashMap<(usize, usize), MirrorDeflection> = HashMap::new();

    let key_result =
        fs::read_to_string("key.txt").expect("Something went wrong reading the key file");

    let lines = key_result.lines();

    // Each line is a "row" in the grid
    for (i_row, line) in lines.enumerate() {
        let y = i_row + 1; // Y coordinate of the row
        for (i_column, cell) in line.chars().enumerate() {
            let x = i_column + 1; // X coordinate of the column
            let mirror_deflection: MirrorDeflection;
            // Yucky
            match cell {
                '/' => {
                    mirror_deflection = MirrorDeflection {
                        pos_x: (-1, -1), // From a positive x, the mirror will deflect in a negative y direction and bring x velocity to 0
                        neg_x: (1, 1),
                        pos_y: (-1, -1),
                        neg_y: (1, 1),
                    }
                }
                '\\' => {
                    mirror_deflection = MirrorDeflection {
                        pos_x: (-1, 1), // From a positive x, the mirror will deflect in a positive y direction and bring x velocity to 0
                        neg_x: (1, -1),
                        pos_y: (1, -1),
                        neg_y: (-1, 1),
                    }
                }
                _ => {
                    mirror_deflection = MirrorDeflection {
                        pos_x: (0, 0),
                        neg_x: (0, 0),
                        pos_y: (0, 0),
                        neg_y: (0, 0),
                    }
                }
            }
            mirror_grid.insert((x, y), mirror_deflection);
        }
    }

    let mut translated_message: String = "".to_owned();
    for c in message.chars() {
        let mut position = decide_starting_point(c);
        // println!("{:?}", position);

        let mut velocity: (i32, i32) = (0, 0);

        match position.0 {
            0 => velocity = (1, 0),   // starting from the left and heading right
            14 => velocity = (-1, 0), // starting from the right and heading left
            _ => (),
        }

        match position.1 {
            0 => velocity = (0, 1),   // starting from the top rowand heading down
            14 => velocity = (0, -1), // starting from the bottom row and heading up
            _ => (),
        }

        loop {
            // TODO some vector math would be nice instead of matching... But I went full ham on tuples lol
            match velocity {
                (0, 1) => position = (position.0, position.1 + 1),
                (0, -1) => position = (position.0, position.1 - 1),
                (1, 0) => position = (position.0 + 1, position.1),
                (-1, 0) => position = (position.0 - 1, position.1),
                _ => (),
            }

            if !mirror_grid.contains_key(&position) {
                // We are off the grid and can translate point to character
                translated_message.push(decide_char(position));
                break;
            }

            let deflection = mirror_grid.get(&position).unwrap();

            // Calculate new velocity
            match velocity {
                (1, 0) => velocity = deflect_velocity(velocity, deflection.pos_x),
                (-1, 0) => velocity = deflect_velocity(velocity, deflection.neg_x),
                (0, 1) => velocity = deflect_velocity(velocity, deflection.pos_y),
                (0, -1) => velocity = deflect_velocity(velocity, deflection.neg_y),
                _ => (),
            }
        }
    }

    println!("{}", translated_message);
}

fn deflect_velocity(velocity: (i32, i32), deflection: (i32, i32)) -> (i32, i32) {
    return (velocity.0 + deflection.0, velocity.1 + deflection.1);
}

fn decide_starting_point(c: char) -> (usize, usize) {
    let mut char_digit: usize = (c.to_digit(36).unwrap() as usize) - 9;
    if char_digit > 13 {
        char_digit -= 13;
    }

    if 'A' <= c && c <= 'M' {
        return (0, char_digit);
    } else if 'a' <= c && c <= 'm' {
        return (char_digit, 0);
    } else if 'N' <= c && c <= 'Z' {
        return (char_digit, 14);
    }
    return (14, char_digit);
}

fn decide_char(point: (usize, usize)) -> char {
    let result: char;
    match point.0 {
        0 => {
            // A - M
            result = char::from_digit((point.1 as u32) + 9, 36).unwrap_or_default();
            return result.to_ascii_uppercase();
        }
        14 => {
            // n - z
            result = char::from_digit((point.1 as u32) + 22, 36).unwrap_or_default();
            return result;
        }
        _ => (),
    }

    match point.1 {
        0 => {
            // a - m
            result = char::from_digit((point.0 as u32) + 9, 36).unwrap_or_default();
            return result;

        }
        14 => {
            // N - Z
            result = char::from_digit((point.0 as u32) + 22, 36).unwrap_or_default();
            return result.to_ascii_uppercase();
        }
        _ => (),
    }

    return '\0';
}
