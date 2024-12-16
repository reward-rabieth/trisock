use std::{io::Read, net::TcpListener, sync::mpsc, thread};

const ADDRESS: &str = "127.0.0.1:9090";
const BUFFER: usize = 32;

fn sleep() {
    thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    let server = TcpListener::bind(ADDRESS).expect("listener cannot bind");

    //set the server to non blocking mode to handle multiple connections

    server
        .set_nonblocking(true)
        .expect("failed to set non-blocking");

    let mut clients = vec![];

    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();

            clients.push(socket.try_clone().expect("failed to clone the client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; BUFFER];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        //log the message received

                        println!("{}: {:?}", addr, msg);

                        //send the message to the main thread via the channel
                        tx.send(msg).expect("failed to send msg to rx");
                    }

                    Err(_) => {
                        println!("closing connection with : {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }
    }
}
