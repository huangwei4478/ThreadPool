use std::io;
use std::thread;
use std::time::Duration;
use ThreadPool::ThreadPool;
fn main() {
    let pool = ThreadPool::new(4);

    let mut input_string = String::new();
    println!("Input x to quit, other to continue working.");
    loop {
        io::stdin().read_line(&mut input_string).unwrap();
        if input_string.trim() == "x" {
            break;
        }
        println!("You wrote {}", input_string);
        let string2 = input_string.clone();
        input_string.clear();
        pool.execute(|| {
            handle_counting(string2);
        });
    }
}

fn handle_counting(input_string: String) {
    thread::sleep(Duration::from_secs(3));
    println!("Worker has worked {} for 3 seconds", input_string.trim());
}
