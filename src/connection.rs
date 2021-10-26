use std::net::{TcpStream, ToSocketAddrs, Shutdown};
use std::io::{Read, Write};
use std::fmt::Display;

pub struct Connection {
    socket : TcpStream,
    auto_flush: bool
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        Connection {
            socket : self.socket.try_clone().expect("Failed to clone"),
            auto_flush: self.auto_flush.clone()
        }
    }
    fn clone_from(&mut self, source: &Self) {
        self.socket = source.socket.try_clone().expect("Failed to clone");
        self.auto_flush = source.auto_flush.clone();
    }
}
impl Connection {
    pub fn new<A : ToSocketAddrs>(address : A) -> Connection {
        Connection {
            socket : TcpStream::connect(address).expect("Couldn't connect to Minecraft, is it running?"),
            auto_flush: true
        }
    }
    pub fn send<T : Display>(self,parts : Vec<T>) {
        self.clone().drain();
        print!("send:");
        let mut cnt = 0;
        for i in &parts {
            self.clone().socket.write(i.to_string().as_bytes()).expect("Failed to write from socket");
            print!("{}",i.to_string());
            if cnt == 0 {
                self.clone().socket.write("(".as_bytes()).expect("Failed to write from socket");
                print!("(");
            }
            else if cnt < parts.len() - 1 {
                self.clone().socket.write(",".as_bytes()).expect("Failed to write from socket");
                print!(")");
            }
            cnt+=1;
        }
        self.clone().socket.write(")\n".as_bytes()).expect("Failed to write from socket");
        println!(")\n");
        if self.clone().auto_flush {
            self.flush();
        }
    }
    pub fn send_s<T : Display>(self,str : T) {
        self.clone().drain();
        self.clone().socket.write(str.to_string().as_bytes()).expect("Failed to write from socket");
        if self.clone().auto_flush {
            self.clone().flush();
        }
    }
    pub fn drain(self) {
        self.clone().socket.set_nonblocking(true).expect("Failed to set non-blocking mode");
        let mut c : [u8;1] = [0];
        while self.clone().socket.read(&mut c).is_ok() {
            eprint!("{}",c[0]);
        }
    }
    pub fn flush(mut self) {
        self.socket.flush().expect("Failed to flush");
    }
    pub fn receive(self) -> String {
        let mut data : Vec<u8> = Vec::new();
        loop {
            let mut read : [u8;1] = [0];
            self.clone().socket.read(&mut read).expect("Failed to read from socket");
            if read[0] != b'\n' {
                data.push(read[0]);
            }
            else {
                break;
            }
        }
        String::from_utf8(data).expect("Failed to convert Vec<u8> to string")
    }
    pub fn close(self) {
        self.socket.shutdown(Shutdown::Both).expect("Failed to close");
    }
    pub fn auto_flush(mut self,flush : bool) {
        self.auto_flush = flush;
        if flush {
            self.flush();
        }
    }
}