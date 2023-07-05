#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref MYUSERSIP: Mutex<Vec<String>> = Mutex::new(Vec::new());
}
fn main() {

    let handle = thread::spawn(move || {
        MYUSERSIP.lock().unwrap().push(String::from("127.0.0.1:57700"));
        MYUSERSIP.lock().unwrap().push(String::from("127.0.0.1:57711"));
        thread::sleep(Duration::from_secs(1));
    });
    let _ = handle.join();
    println!("{:?}", MYUSERSIP.lock().unwrap());

    let ip = "127.0.0.1:57711";
    MYUSERSIP.lock().unwrap().push(String::from("127.0.0.1:57777"));
    MYUSERSIP.lock().unwrap().push(String::from("127.0.0.1:57799"));
    println!("{:?}", MYUSERSIP.lock().unwrap());
    MYUSERSIP.lock().unwrap().retain(|item| if *item == ip {false} else {true});

    println!("{:?}", MYUSERSIP.lock().unwrap());
}