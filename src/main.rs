use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, Write};

fn main() {
    let pa = String::from_utf8(vec![0xe7, 0x88, 0xac]).unwrap();
    let mut count = 0;
    loop {
        stdout().flush().unwrap();
        print!("{}\t", pa);
        count += 1;
        sleep(Duration::from_millis(500));
        if count > 100 {
            break;
        }
    }
}
