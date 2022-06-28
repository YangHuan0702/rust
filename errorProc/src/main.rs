use std::fs::File;
use std::io::ErrorKind;

pub trait Summary{
    fn summarize(&self) -> String;
}

fn main() {

}


fn panic_demo(){
    let f = File::open("hello.txt");
    let file = match f {
        Ok(t)=> t,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Couldn't create Hello file: {}", error),
            },
            other_error => {
                panic!("Open File hello.text failed: {}", other_error);
            }
        },
    };
}