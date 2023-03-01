use tokio::{net::TcpListener, spawn, io::{AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    let listener = TcpListener::bind("127.0.0.1:8123").await?;
    loop {
        let (mut socket, address) = listener.accept().await?;
        spawn(async move {
            let mut buffer = vec![0; 1024];
            loop {
                let n_bytes_read = socket
                    .read(&mut buffer)
                    .await
                    .expect("failed to read data from socket");

                if n_bytes_read == 0 {
                    return;
                }

                socket
                    .write_all(&buffer[0 .. n_bytes_read])
                    .await
                    .expect("failed to write to the socket");
            }
        });
    }

    Ok(())
}