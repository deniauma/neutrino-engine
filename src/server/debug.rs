use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};


pub struct DebugSystem {
    listener: TcpListener,
    receiver: Receiver<String>,
    sender: Sender<String>,
}

impl DebugSystem {
    pub fn new(addr: &str) -> Self {
        let (tx, rx) = channel();
        Self {
            listener: TcpListener::bind(addr).unwrap(),
            receiver: rx,
            sender: tx,
        }
    }

    pub fn accept_connections(&self) {
        let listener_thread = self.listener.try_clone().unwrap();
        let tx_thread = self.sender.clone();
        thread::spawn(move|| {
            for stream in listener_thread.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("New connection: {}", stream.peer_addr().unwrap());
                        let tx_new_thread = tx_thread.clone();
                        thread::spawn(move|| {
                            // connection succeeded
                            handle_client(stream, tx_new_thread)
                        });
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        /* connection failed */
                    }
                }
            }
        });
    }
    
    pub fn execute(&self) {
        if let Ok(command) = self.receiver.try_recv() {
            println!("Command: {}", command);
        }
    }
}


fn handle_client(mut stream: TcpStream, tx: Sender<String>) {
    let mut data = [0 as u8; 512]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            tx.send(String::from_utf8_lossy(&data).to_string()).unwrap();
            stream.flush().unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
