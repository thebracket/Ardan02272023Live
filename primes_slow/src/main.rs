fn is_prime(n: u32) -> bool {
    (2..=n / 2).all(|i| n % i != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_hundred_primes() {
        // List obtained from: https://en.wikipedia.org/wiki/Prime_number
        let primes: Vec<u32> = (2..100).filter(|n| is_prime(*n)).collect();
        assert_eq!(
            primes,
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }
}

fn main() {
    const MAX: u32 = 500_000;

    /*let mut count = 0;
    let now = std::time::Instant::now();
    for i in 2 .. MAX {
        if is_prime(i) {
            count += 1;
        }
    }
    let time = now.elapsed();
    println!("Found {count} primes in {} seconds", time.as_secs_f32());*/

    use std::sync::atomic::AtomicUsize;

    let now = std::time::Instant::now();

    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    const N_THREADS: u32 = 8;

    let mut threads = Vec::with_capacity(N_THREADS as usize);
    let group = MAX / N_THREADS;

    for i in 0..N_THREADS {
        let counter = i;
        threads.push(std::thread::spawn(move || {
            let range = u32::max(2, counter*group) .. (i+1)*group;
            COUNTER.fetch_add(
                range.filter(|n| is_prime(*n)).count(),
                std::sync::atomic::Ordering::Relaxed
            );
        }));
    }
    
    for thread in threads {
        let _ = thread.join();
    }

    let time = now.elapsed();
    println!("{} seconds for {MAX} iterations", time.as_secs_f32());
    println!("Found {} primes", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
