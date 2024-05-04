fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // ２つ同時に可変の参照を作ることはできない
    // let r1 = &mut s;
    {
        let r1 = &mut s;
        println!("r1: {}", r1);
    }
    let r2 = &mut s;
    println!("r2: {}", r2);

    // 不変の参照がある間、可変の参照を作ることはできない
    // let r1 = &s;
    // let r2 = &s;
    {
        let r1 = &s;
        let r2 = &s;
        println!("r1: {}, r2: {}", r1, r2);
    }
    let r3 = &mut s;
    println!("r3: {}", r3);

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
