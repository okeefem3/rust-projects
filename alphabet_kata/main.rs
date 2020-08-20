fn alphabet_position(text: &str) -> String {
    let mut positions = Vec::new();
    for c in text.chars() {
        let char_digit = c.to_digit(36);
        let char_position: u32;
        match char_digit {
            None => char_position = 26,
            Some(i) => char_position = i - 9,
        }
        if char_position > 0 && char_position <= 25 {
            positions.push(char_position.to_string())
        }
    }
    
    return positions.join(" ");
};

// Better version
fn alphabet_position(text: &str) -> String {
    let mut positions = Vec::new();
    for c in text.chars() {
        match c.to_digit(36) {
            Some(i) if 9 < i && i < 36 => positions.push((i - 9).to_string()),
            _ => {},

        }
    }
    return positions.join(" ");
}
