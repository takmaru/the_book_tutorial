fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("word:'{}'", word);
    let word = first_word(&my_string);
    println!("word:'{}'", word);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("word:'{}'", word);
    let word = first_word(my_string_literal);
    println!("word:'{}'", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
