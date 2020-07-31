#[cfg(test)]
mod tests {

    #[test]
    fn same_necklace_tests() {
        println!("Hello Tests!");
        assert!(same_necklace("nicole", "icolen")); // true
        assert!(same_necklace("nicole", "lenico")); // true
        assert!(!same_necklace("nicole", "coneli")); // false
        assert!(same_necklace("aabaaaaabaab", "aabaabaabaaa")); // true
        assert!(!same_necklace("abc", "cba")); // false
        assert!(!same_necklace("xxyyy", "xxxyy")); // false
        assert!(!same_necklace("xyxxz", "xxyxz")); // false
        assert!(same_necklace("x", "x")); // true
        assert!(!same_necklace("x", "xx")); // false
        assert!(!same_necklace("x", "")); // false
        assert!(same_necklace("", "")); // true
    }

    fn same_necklace(str1: &str, str2: &str) -> bool {
        let start = Instant::now();

        if str1.len() != str2.len() {
            return false;
        }
        let mut owned_str: String = str1.to_owned();
        owned_str.push_str(str1);

        let contains = owned_str.contains(str2);
        let duration = start.elapsed();
        println!("Time elapsed in same_necklace() is: {:?}", duration);
        return contains;
    }

    // Something like this
    // fn repeats(string: &str) -> usize {
    //     let start = Instant::now();
    //     let split_string = string.split("");
    //     let unique_chars = set(split_string);
    //     let repetable_string = unique_chars.join("");
    //     let repeats = string.matches(repetable_string).count();
    //     let duration = start.elapsed();
    //     println!("Time elapsed in same_necklace() is: {:?}", duration);
    //     return repeats;
    // }
}

fn main() {
    println!("Hello, world!");
}

