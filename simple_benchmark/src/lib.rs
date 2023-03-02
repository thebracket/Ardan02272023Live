// You will need to run this with:
// cargo +nightly bench
//
// It also failed spectacularly during the demo.
#![feature(test)]
extern crate test;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn is_prime(n: u32) -> bool {
    (2..=n / 2).all(|i| n % i != 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn it_works() {
        let result = add(2,2);
        assert_eq!(result, 4);
    }

    #[bench]
    fn bench_add(b: &mut Bencher) {
        // Do your initialization here
        b.iter(|| add(2,4));
    }

    #[bench]
    fn bench_prime(b: &mut Bencher) {
        b.iter(|| {
            for i in 0 .. 50_000 {
                is_prime(black_box(i));
            }
    });
    }
}