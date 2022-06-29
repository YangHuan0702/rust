use std::collections::HashMap;

fn main() {
    let mut a = String::from("Add Sally to Engineering");

    let mut index = 0;

    for i in a.split(" "){
        if index == 1 {
            panic!("error index must be greater than 1");
        }else if index == 3{

        }
        index += 1;
    }

}
