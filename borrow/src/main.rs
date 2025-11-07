fn main() {
    // example for immutable string
    let s_immutable = String::from("Hello, Rust!");
    let len = calculate_length(&s_immutable);
    let first_word = get_first_word(&s_immutable);

    println!("String: {}", s_immutable);
    println!("Length: {}", len);
    println!("First word: {}", first_word);
    // example for mutable string
    let mut s_mutable = String::from("Hello, Rust!");
    append_str(&mut s_mutable, String::from("!"));
    println!("Mutable String: {}", s_mutable);
}

fn append_str(s: &mut String, str: String) {
    s.push_str(&str);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn get_first_word(s: &String) -> String {
    s.split_whitespace().next().unwrap().to_string()
}
