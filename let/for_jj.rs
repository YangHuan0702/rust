fn main(){
    print_asc();
}


fn print_asc(){
    let array = [1,2,3,4,5,6,7,8,9];
    
    let mut i : usize = array.len();

    for element in array {
        let mut j = 1;
        while j <= element {
            print!("{} * {} = {}   ",j , element, j * element);        
            j += 1;
        }
        println!("");
    }
}

fn print_desc(){
    let array = [1,2,3,4,5,6,7,8,9];

    let mut i: usize = array.len();

    while i > 0 {
        let mut j: usize = 0;
        while j < i {
            print!("{} * {} = {}   ",j +1 , array[i - 1], array[j] * array[i - 1]);        
            j += 1;
        }
        println!("");
        i -= 1;
    } 
}