use std::thread;
use std::sync::Mutex;

fn main(){

    let numbers = &Mutex::new(vec![1,2,3]);
    
    let guards: Vec<_> = (0..3).map(|i| {
        thread::scoped( move || {
            // the lock() call will return us a reference to the value inside the Mutex,
            // and block any other calls to lock() until said reference goes out of scope.
            let mut array = numbers.lock().unwrap();
            array[i] += 1;
            println!("numbers[{}] is {}", i, array[i]);
        })
    }).collect();
    
}
