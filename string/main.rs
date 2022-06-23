fn main(){

    // let x = string::from("123");  heap
    let x = "123"; // stack
    let b = x;

    println!("{}",b);
}