struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("ohter stuff") };
    println!("CustomSmartPointer created.");

    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    //c.drop();
    //error[E0040]: explicit use of destructor method
    drop(c);
    //println!("c = {}", c.data);
    //error[E0382]: borrow of moved value: `c`
    println!("CustomSmartPointer dropped before the end of main.");
}
