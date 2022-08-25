use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("Listening on localhost:8080");

    let (tx, mut rx) = broadcast::channel(10);

    let mut counter = 0;

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Accepted a socket");

        counter += 1;

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                println!("Reading from socket");

                tokio::select! {
                    result = socket.read(&mut buf) => {
                        let size = result.unwrap();
                        if size == 0 {
                            break;
                        }

                        let mut msg = String::new();
                        msg.push_str(&String::from_utf8_lossy(&buf[..size]));
                        tx.send((counter, msg, addr)).unwrap();
                    }
                    result = rx.recv() => {
                        let (counter, msg, other_addr) = result.unwrap();
                        if other_addr != addr {
                            let new_msg = &format!(" ({}) {}", counter, msg);
                            socket.write(new_msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
