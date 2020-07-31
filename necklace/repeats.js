


function repeats(str: string): number {
    let start = performance.now();
    let split_string = str.split("");
    let unique_chars = new Set(split_string);
    let repeatable_string = Array.from(unique_chars).join("");
    let regexMatch = new RegExp(repeatable_string,"g");
    let repeats = str.match(regexMatch).length;
    let end = performance.now();
    console.log(end - start);
    return repeats;
}

console.log(repeats("abc"), 1); // 1
console.log(repeats("abcabcabc"), 3); // 3
console.log(repeats("abcabcabcx"), 1); // 1
console.log(repeats("aaaaaa"), 6); // 6
console.log(repeats("a"), 1); // 3
console.log(repeats(""), 1); // 1
