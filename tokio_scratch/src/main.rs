use std::time::Duration;

use tokio::{join, task::spawn_blocking};

async fn hello(n: u32) {
    println!("Hello {n}");
    if n < 10 {
        hello_child(n*10).await;
    }
}

async fn hello_child(n: u32) {
    println!("Hello again! {n}");
    //tokio::time::sleep(Duration::from_secs(1)).await;
    let _ = spawn_blocking(|| std::thread::sleep(Duration::from_secs(1))).await;
}

fn not_async() {
    println!("Hello");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    not_async();
    join!(
        hello(1), hello(2), hello(3), hello(4)
    );
    Ok(())
}
