use std::thread;

pub fn thread_spance(){
    let v = vec![1,3,4,5];
    let handler = thread::spawn(move ||{
       println!("{:?}",v);
    });
    handler.join().unwrap();
}

pub fn base_thread(){

    let handler = thread::spawn(||{
        println!("create to thread")
    });

    handler.join();
    for i in 1..6{
        println!("main thread exec");
    }
}
