use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::graphics::{ComponentStorageManager};


pub struct DebugSystem {
    listener: TcpListener,
    receiver: Receiver<Task>,
    sender: Sender<Task>,
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
    
    pub fn pop_task(&self, data: &mut ComponentStorageManager) {
        if let Ok(mut task) = self.receiver.try_recv() {
            println!("Command: {}", task.command);
            task.execute(data);
        }
    }
}


fn handle_client(mut stream: TcpStream, tx: Sender<Task>) {
    let mut data = [0 as u8; 512]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            // stream.write(&data[0..size]).unwrap();
            tx.send(Task::new(String::from_utf8_lossy(&data[0..size]).to_string(), stream.try_clone().unwrap())).unwrap();
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


#[derive(Debug)]
struct Task {
    command: String,
    stream: TcpStream
}

impl Task {
    fn new(command: String, stream: TcpStream) -> Self {
        Self {
            command: command,
            stream: stream
        }
    }

    fn get_command(&self) -> DebugCommand {
        DebugCommand::from(self.command.trim())
    }

    fn execute(&mut self, data: &mut ComponentStorageManager) {
        let response: String = match self.get_command() {
            DebugCommand::VERTICES => {
                let vertices: usize = data.mesh_manager.iter().map(|(_, m)| {
                    m.positions.len()
                }).sum();
                vertices.to_string()
            },
            DebugCommand::ILLEGAL => String::from("Unknown command")
        };
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}

#[derive(Debug, PartialEq)]
enum DebugCommand {
    VERTICES,
    ILLEGAL,
}

impl From<&str> for DebugCommand {
    fn from(c: &str) -> Self {
        match c {
            "vertices" => DebugCommand::VERTICES,
            _ => DebugCommand::ILLEGAL
        }
    }
}
