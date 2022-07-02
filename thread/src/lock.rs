use std::thread;
use std::sync::{Mutex,Arc};

pub fn lock_demo() {
    let count = Arc::new(Mutex::new(0));
    let mut handlers = Vec::new();
    for _ in 0..10 {
        let count = count.clone();
        let handler = thread::spawn(move ||{
            let mut num = count.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    for handler in handlers{
        handler.join();
    }

    println!("count :{:?}",count);
}