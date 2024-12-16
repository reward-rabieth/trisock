use std::{
    io::{self, ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc::{channel, Receiver, TryRecvError},
    thread,
    time::Duration,
};

const ADDRESS: &str = "127.0.0.1:9090";
const BUFFER: usize = 32;
fn main() {
    //establish a tcp connection to the specified address
    let mut client = TcpStream::connect(ADDRESS).expect("stream could not connect");
    //the client can check  for incoming messages without blocking the entire thread

    client
        .set_nonblocking(true)
        .expect("setting unblocking call failed");
    //channel for communication between threads
    let (tx, rx) = channel::<String>();

    // Spawn a different thread to handle server commnication

    thread::spawn(move || loop {
        let mut buff = vec![0; BUFFER];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                //process and display incoming messages

                // into_vector consumes the original vector and take ownership of the vector
                //take_while  continues taking element from the iterator
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                //buff is no loner usable after this point
                println!("Message received: {:?}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                //handle disconnection
                println!("Disconnected");
                break;
            }
        }

        //receive the messages from the  server

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(32, 0);
                client.write_all(&buff).expect("Writing to socket failed");
                println!("Message sent {:?}", msg);
            }

            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a Message:");
    loop {
        let mut buff = String::new();
        //Read the user input and send it to the other thread
        io::stdin()
            .read_line(&mut buff)
            .expect("Reading from stdin failed");
        let msg = buff.trim().to_string();

        //exit the loop on ":quit" or if the channel is disconnected

        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }

    println!("Bye bye");
}
