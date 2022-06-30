use std::ops::Deref;

pub fn use_box(){
    let m = Box::new(85);
    println!("{}",m);
}

pub fn use_box2(){
    let x = 5;
    let y = MyBox::new(x);

    // assert_eq!(5,y);
    assert_eq!(5,*y);
}

pub struct MyBox<T>(T);

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T> MyBox<T>  {
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}

