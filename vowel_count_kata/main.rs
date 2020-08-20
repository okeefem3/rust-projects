// Without regex to practice iteration
fn get_count(string: &str) -> usize {
    return string.to_lowercase()
            .chars()
            .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
            .collect::<Vec<char>>().len();
}
