#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

/// ​lazy-static crate​​ 可以取代一些创建单例的复杂代码。 
/// 以下是一个全局可变 vector:
fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("called {}", ARRAY.lock().unwrap().len());
}
