

fn main(){
    let a = Some(3);
    if let Some(a) = a{
        println!("a is not null");
    }else{
        println!("a is null")
    }
}