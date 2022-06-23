
#[derive(Debug)]
struct Target{
    higth : u8,
    writer : u8,
}

fn main(){

    // let higth = 10;
    // let writer = 20;

    // println!("r = {}" , get_r(higth,writer));
    
    // let target = (10,20);
    let t = Target{
        higth : 10,
        writer : 20,
    };
    println!("r = {:#?}" , t);
}

fn get_r(t : &Target) -> u8 {
    t.higth * t.writer
}

// fn get_r(a : (u8,u8)) -> u8 {
//     a.0 * a.1
// }

// fn get_r( higth : u8, writer : u8) -> u8 {
//     higth * writer
// }