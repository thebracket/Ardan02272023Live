use std::sync::RwLock;

struct MyType(usize);

impl MyType {
    const fn new(n: usize) -> Self { // <-- Notice we've added a constant constructor
        Self(n)
    }
}

static SHARED: RwLock<MyType> = RwLock::new(MyType::new(5));

fn main() {
    for _ in 0..10 {
        std::thread::spawn(|| {
            let read_lock = SHARED.read().unwrap();
            println!("The value of SHARED is {}", read_lock.0)
        });
    }
    std::thread::spawn(|| {
        // Do your big calculation here and store the result.
        let mut write_lock = SHARED.write().unwrap();
        write_lock.0 += 1;
    });
    
    std::thread::sleep(std::time::Duration::from_secs(5));
}