use std::sync::Mutex;

fn is_prime(n: u32) -> bool {
    (2..=n / 2).all(|i| n % i != 0)
}

fn main() {
    let mut n = 5;
    let mut my_func = || {
        n*=5;
    };
    my_func();

    static PRIMES: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    const MAX: u32 = 500_000;
    const N_THREADS: u32 = 8;
    let group = MAX / N_THREADS;
    let mut threads = Vec::new();

    let now = std::time::Instant::now();

    for i in 0..N_THREADS {
        // counter = i;
        threads.push(std::thread::spawn(move || {
            let range = u32::max(2, i * group)..(i + 1) * group;
            let my_primes: Vec<u32> = range.filter(|n| is_prime(*n)).collect();
            PRIMES.lock().unwrap().extend(my_primes);
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }

    let time = now.elapsed();
    println!("{} seconds for {MAX} iterations", time.as_secs_f32());

    println!(
        "Found {} prime numbers in the range 2..{MAX}",
        PRIMES.lock().unwrap().len()
    );
}
