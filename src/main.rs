mod iter;
//use std::error::Error;

fn main() {
    println!("Fibonacci sequence | iterator implementation");
    let mut f = iter::fibonacci::fibonacci();
    loop {
        let x = f.next().unwrap();
        if x > 100 {
            break;
        }
    }
}
