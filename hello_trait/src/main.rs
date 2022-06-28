
fn main() {
    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from(
    //         "of course, as you probably already know, people",
    //     ),
    //     reply: false,
    //     retweet: false,
    // };
    // println!("1 new tweet: {}", tweet.summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {

    let mut r_index = 0;
    let mut current_max_index = 0;
    for &item in list {
        if item > list[current_max_index] {
            current_max_index = r_index;
        }
        r_index += 1;
    }
    &list[current_max_index]
}
