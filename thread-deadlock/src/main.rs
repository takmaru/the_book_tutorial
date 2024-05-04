use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    {
        let counter1 = Arc::clone(&counter1);
        let counter2 = Arc::clone(&counter2);
        let handle = thread::spawn(move || {
            for i in 0..10 {
                println!("thread 1: counter1 lock");
                let mut num1 = counter1.lock().unwrap();
                println!("thread 1: counter1 locked");
                thread::sleep(Duration::from_secs(1));
                println!("thread 1: counter2 lock");
                let mut num2 = counter2.lock().unwrap();
                println!("thread 1: counter2 locked");
                *num1 += 1;
                *num2 += 2;
                println!("thread 1: unlock");
            }
        });
        handles.push(handle);
    }
    {
        let counter1 = Arc::clone(&counter1);
        let counter2 = Arc::clone(&counter2);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                println!("thread 2: counter2 lock");
                let mut num2 = counter2.lock().unwrap();
                println!("thread 2: counter2 locked");
                thread::sleep(Duration::from_secs(1));
                println!("thread 2: counter1 lock");
                let mut num1 = counter1.lock().unwrap();
                println!("thread 2: counter1 locked");
                *num2 += 2;
                *num1 += 1;
                println!("thread 2: unlock");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter1: {}, counter2: {}", *counter1.lock().unwrap(), *counter2.lock().unwrap());
}
