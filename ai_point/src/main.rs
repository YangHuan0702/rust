mod r#box;
mod drop_demo;

use drop_demo::*;
use r#box::*;
use std::mem::drop;
use std::rc::Rc;

fn main() {
    // use_box();
    // use_box2();
    let r = Rc::new(Vec::new());

    // drop trait demo
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };
    // drop(c);
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // println!("CustomSmartPointers created.");

    let b = Some(2);
    rc_demo();
}


fn rc_demo(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}