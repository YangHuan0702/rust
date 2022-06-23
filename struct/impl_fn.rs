#[derive(Debug)]
struct Target{
    higth : u8,
    writer : u8,
}

impl Target {
    fn get_r(&self) -> u8 { 
        dbg!(self.higth * self.writer)
     }

}

fn main(){
    let t = Target{
        higth : 10,
        writer : 20,
    };
    println!("r = {:#?}, r :{}" , t,t.get_r());
}

