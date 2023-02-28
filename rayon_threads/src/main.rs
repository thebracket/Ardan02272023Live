use std::{thread::sleep, time::Duration};
use rayon::spawn;

fn hello(n: u64) {
    println!("Hello from thread {n}");
    sleep(Duration::from_secs(n));
    println!("Bye from thread {n}");
}

fn main() {
    rayon::scope(|s| {
        for i in 0..8 {
            let my_i = i;
            s.spawn(move |_| hello(my_i));
        }
    });

    /*let mut threads = Vec::new();
    for i in 0..8 {
        let my_i = i;
        threads.push(spawn(move || hello(my_i)));
    }
    for t in threads {
        t.join();
    }*/
}
