fn add_fancy_hat(){
    println!("add_fancy_hat");    
}

fn remove_fancy_hat(){
    println!("remove_fancy_hat");
}

fn move_fancy_hat(num_space:u8){
    println!("move_fancy_hat:{}",num_space);
}


fn main(){
    let a = 7;
    match a {
        3 => add_fancy_hat(),
        6 => remove_fancy_hat(),
        _ => move_fancy_hat(a),
    }
}