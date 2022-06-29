fn main() {
    annoy_demo(3);
}

struct LazyGet<T>
    where
        T: Fn(i32) -> i32
{
    value: Option<i32>,
    exec_fn: T,
}

impl LazyGet<T> {
    fn new(exec_fn: T) -> LazyGet<T> {
        LazyGet {
            exec_fn,
            value: None,
        }
    }

    fn value(&mut self, age: i32) -> i32 {
        match self.value {
            Some(t) =>t,
            None => {
                let r = self.exec_fn(age);
                self.value = r;
                r
            }
        }
    }
}

fn annoy_demo(a: u8) {
    let func = |num| {
        println!("------------:{}", num);
        num + 1
    };
    let c = func(a) + 1;
    println!("{}", c);
}


