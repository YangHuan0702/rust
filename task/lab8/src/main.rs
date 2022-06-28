
use std::collections::HashMap;


fn main() {
    let mut a = String::from("Add Sally to Engineering");
    let mut b = String::from("Add Amir to Sales");

    let mut index : u8 = 0;

    let mut value = Vec::new();
    let mut key = Vec::new();
    for item in a.split_whitespace() {
        if index == 1 {
            value.push(item);
        }else if index == 3 {
            key.push(item);
        }
        index += 1;
    }
    index = 0;

    for item in b.split_whitespace() {
        if index == 1 {
            value.push(item);
        }else if index == 3 {
            key.push(item);
        }
        index += 1;
    }

    let mut user_map : HashMap<_,_> = key.into_iter().zip(value.into_iter()).collect();

    println!("{:?}",user_map);
}



fn demo1(){
    // let mut users :HashMap<&str,&Vec<&str>> = HashMap::new();

    // let a = "Add Sally to Engineering";
    // let b = "Add Amir to Sales";

    // let ra = a.split(" ").collect::<Vec<&str>>();
    // let rb = b.split(" ").collect::<Vec<&str>>();

    // let ra_key = &ra[3];
    // let rb_key = &rb[3];

    // let a_v = users.get(*ra_key);
    // if a_v.is_none() {
    //     let mut v : Vec<&str> = Vec::new();
    //     users.insert(ra_key, &v);
    //     v.push(&&ra[1]);
    // }
    
    // let b_v = users.get(*rb_key);
    // if b_v.is_none() {
    //     let mut v : Vec<&str> = Vec::new();
    //     users.insert(rb_key, &v);
    //     v.push(&&ra[1]);
    // }

    // println!("{:?}",&users);
}


