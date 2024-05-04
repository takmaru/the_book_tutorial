fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    //assert_eq!(5, y);
    //error[E0277]: can't compare `{integer}` with `&{integer}`
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    //error[E0614]: type `MyBox<{integer}>` cannot be dereferenced

    let m = MyBox::new(String::from("Rust"));
    hello(&m);  // 参照外し型強制：MyBox<T>のderefでMyBox<String>から&Stringになり、Stringのderefで&Stringから&strになる
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name:&str) {
    println!("Hello, {}!", name);
}
