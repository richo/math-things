use std::thread;
use rand::{self, Rng};
use multi_persistence;

const CURRENT_BEST: usize = 277777788888899;
const NUM_THREADS: usize = 8;

fn work() {
    println!("Starting");
    let mut rng = rand::thread_rng();
    loop {
        let v = rng.gen();
        if v < CURRENT_BEST {
            continue
        }
        let ret = multi_persistence::persistence(v);
        if ret > 9 {
            println!("{}: {}", ret, v);
        }
        if ret > 11 {
            std::process::exit(0);
            unreachable!();
        }
    }
}

fn main() {
    let mut threads = vec![];
    for i in 0..NUM_THREADS {
        threads.push(thread::spawn(work));
    }
    for i in threads {
        i.join();
    }
    println!("done");
}
