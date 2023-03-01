use serde::{Serialize, Deserialize};
use tokio::{net::{TcpListener, TcpStream}, spawn, io::{AsyncReadExt, AsyncWriteExt}};

#[derive(Serialize, Deserialize)]
enum Request {
    Ping,
}

#[derive(Serialize, Deserialize)]
enum Response {
    Error,
    Ack,
}

async fn rpc_server() -> anyhow::Result<()> {
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

                let mut response = Response::Error;
                let request = serde_json::from_slice(&buffer[0 .. n_bytes_read]);
                match request {
                    Err(..) => return,
                    Ok(request) => {
                        match request {
                            Request::Ping => response = Response::Ack,
                        }
                    }
                }

                let bytes = serde_json::to_vec(&response).unwrap();

                socket
                    .write_all(&bytes)
                    .await
                    .expect("failed to write to the socket");
            }
        });
    }

    Ok(())
}

async fn rpc_client() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    let message = serde_json::to_vec(&Request::Ping)?;
    stream.write_all(&message).await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let response: Response = serde_json::from_slice(&buf[0..n])?;
    match response {
        Response::Error => println!("Error!"),
        Response::Ack => println!("Ack!"),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.is_empty() {
        println!("You must run with either --server or --client");
    } else {
        match args[1].as_str() {
            "--server" => rpc_server().await?,
            "--client" => rpc_client().await?,
            _ => println!("You must run with either --server or --client"),
        }
    }
    Ok(())
}