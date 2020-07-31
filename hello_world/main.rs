#[cfg(test)]
mod tests() {

    #[test]
    fn it_works() {
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
}

fn same_necklace(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    }
    let mut owned_str: String = str1.to_owned();
    owned_str.push_str(str1);
    return owned_str.contains(str2);
}