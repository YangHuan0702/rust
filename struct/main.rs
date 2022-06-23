
struct User {
    name:String,
    age : u8,
    email : String,
}


fn main(){
    let yanghuan = User {
        name : String::from("halosky"),
        age : 23,
        email : String::from("626090763@qq.com")
    };

    let user2 = User{
        age : 12,
        ..yanghuan
    };
    println!("{}",yanghuan.name);

}