use std::io::Result;
use std::fmt::Arguments;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    type Kilomteters = i32;

    let x: i32 = 5;
    let y: Kilomteters = 5;

    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));

    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
}

fn takes_long_type(f: Thunk) {
}
/*
fn returns_long_type() -> Thunk {
}
*/

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
}

fn bar() -> ! {
    panic!("")
}