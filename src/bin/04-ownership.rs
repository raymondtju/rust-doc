fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..2];
    println!("{slice}");
    let slice = &s[..];

    let mut text: String = String::from("abc Hello, world!");
    text.clear();
    println!("{}", first_word(&text));
}

// manually slice the length first word
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    println!("{:?}", bytes.iter().enumerate());

    for (i, &item) in bytes.iter().enumerate() {
        // println!("{item}");
        if item == 32 {
            // return i; // return index
            return &s[0..i];
        }
    }

    // return s.len(); // return index
    return &s[..];
}
