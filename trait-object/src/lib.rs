pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
    //pub components: Vec<Box<dyn Clone>>,
    //error[E0038]: the trait `Clone` cannot be made into an object
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen() {
        struct Component1 {}
        impl Draw for Component1 { fn draw(&self) { println!("Component1 draw()"); } }
        struct Component2 {}
        impl Draw for Component2 { fn draw(&self) { println!("Component2 draw()"); } }
        struct Component3 {}
        impl Draw for Component3 { fn draw(&self) { println!("Component3 draw()") } }
        
        let screen = Screen { components: vec![
            Box::new(Component1 {}),
            Box::new(Component2 {}),
            Box::new(Component3 {}),
        ] };
        screen.run();
        screen.run();
        screen.run();
    }
}
