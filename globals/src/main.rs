use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

static SHARED: AtomicUsize = AtomicUsize::new(5);
static SHARED_MUTEX: Mutex<usize> = Mutex::new(5);

struct MyType(usize);

impl MyType {
    fn new(n: usize) -> Self {
        Self(n)
    }
}

use once_cell::sync::Lazy;

static SHARED_MYTYPE: Lazy<Mutex<MyType>> = Lazy::new(|| Mutex::new(MyType::new(5)));

fn main() {
    /*SHARED.fetch_add(1, Ordering::Relaxed);
    println!("{}", SHARED.load(Ordering::Relaxed));

    println!("{}", *SHARED_MUTEX.lock().unwrap());

    println!("{}", SHARED_MYTYPE.lock().unwrap().0);*/

    let lock = SHARED_MUTEX.lock().unwrap();
    std::mem::drop(lock);
    // std::mem::forget(lock); // Don't do this!
    println!("{}", *SHARED_MUTEX.lock().unwrap());
}
