use crate::ch::channel_demo;
use crate::lock::lock_demo;

mod lock;
mod th;
mod ch;

fn main() {
    // th::thread_spance();
    // channel_demo();
    lock_demo();
}
