use std::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let control_listener = TcpListener::bind("localhost:7770").await?;

    let control_socket = async {
        let (socket, _) = control_listener.accept().await?;

        loop {
            socket.readable().await?;

            let mut buf = [0; 1024];

            match socket.try_read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let msg = std::str::from_utf8(&buf).unwrap();
                    let msg = msg.trim_matches(char::from(0));

                    println!("read {} bytes, {:?}", n, msg);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok::<_, io::Error>(())
    };
    tokio::pin!(control_socket);

    loop {
        tokio::select! {
            cmd = &mut control_socket => {
                println!("received command: {cmd:?}");
                break;
            },
        }
    }

    Ok(())
}
