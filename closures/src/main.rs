use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| z == x;
    //fn equal_to_x(z: i32) -> bool { z == x }
    //  error[E0434]: can't capture dynamic environment in a fn item
    //  use the `|| { ... }` closure form instead
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;
    //println!("can't use x here: {:?}", x);
    // error[E0382]: borrow of moved value: `x`

    let y = vec![1, 2, 3];
    
    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T1, T2>
    where T1: Fn(T2) -> T2,
          T2: Eq + Hash + Copy
{
    calculation: T1,
    value: HashMap<T2, T2>,
}

impl<T1, T2> Cacher<T1, T2>
    where T1: Fn(T2) -> T2,
          T2: Eq + Hash + Copy
{
    fn new(calculation: T1) -> Cacher<T1, T2> {
        Cacher { calculation, value: HashMap::new(), }
    }

    fn value(&mut self, arg: T2) -> T2 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let calculation_value = (self.calculation)(arg);
                self.value.insert(arg, calculation_value);
                calculation_value
            },
        }
    }
}
