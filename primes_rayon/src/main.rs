use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn is_prime(n: u32) -> bool {
    (2..=n / 2).all(|i| n % i != 0)
}

fn main() {
    const MAX:u32 = 200_000;
    let now = std::time::Instant::now();

    let count = (2..MAX)
        .into_par_iter()
        .filter(|n| is_prime(*n))
        .count();

    let duration = now.elapsed();
    println!("Found {count} primes in {} seconds", duration.as_secs_f32());
}
