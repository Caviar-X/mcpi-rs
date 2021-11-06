use std::net::{TcpStream, ToSocketAddrs, Shutdown};
use std::io::{Read, Write, BufReader, BufRead};
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
        let mut cnt = 0;
        for i in &parts {
            self.clone().socket.write(i.to_string().as_bytes()).expect("Failed to write from socket");
            if cnt == 0 {
                self.clone().socket.write("(".as_bytes()).expect("Failed to write from socket");
            }
            else if cnt < parts.len() - 1 {
                self.clone().socket.write(",".as_bytes()).expect("Failed to write from socket");
            }
            cnt+=1;
        }
        self.clone().socket.write(")\n".as_bytes()).expect("Failed to write from socket");
        if self.clone().auto_flush {
            self.flush();
        }
    }
    pub fn send_s<T : Display>(self,str : T) {
        self.clone().drain();
        self.clone().socket.write(str.to_string().as_bytes()).expect("Failed to write from socket");
        self.clone().socket.write("\n".as_bytes()).expect("Failed to write from socket");
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
        self.clone().socket.try_clone().unwrap().set_nonblocking(false).unwrap();
        let mut b : BufReader<TcpStream> = BufReader::new(self.clone().socket.try_clone().unwrap());
        let mut s = String::new();
        b.read_line(&mut s).expect("Failed to read line");
        s
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