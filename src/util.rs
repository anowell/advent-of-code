pub fn group_chars_by(input: &str, f: &Fn(char) -> bool) -> String {
    let mut cleaned = String::new();
    let mut prev_check = false;
    for c in input.chars() {
        if f(c) {
            cleaned.push(c);
            prev_check = true
        } else if prev_check {
            cleaned.push(' ');
            prev_check = false;
        }
    }
    cleaned
}