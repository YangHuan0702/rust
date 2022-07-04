use std::sync::mpsc;
use std::thread;

pub fn channel_demo() {
    let (se, rx) = mpsc::channel();

    let s = se.clone();
    let handler1 = thread::spawn( move || {
        let v = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        for i in v {
            se.send(i).unwrap()
        }
    });
    let handler2 = thread::spawn(move ||{
       let v = vec![String::from("hello "),String::from("i love rust"),String::from(" and c++/c and java")];
        for i in v{
            s.send(i).unwrap();
        }
    });

    for i in rx {
        println!("收到消息：{}",i);
    }

}


