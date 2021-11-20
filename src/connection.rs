//! The connection module of mcpi-rs
//! set a connection between minecraft and mcpi-rs
//!
//! # Example
//!
//! ```
//! use mcpi_rs::prelude::*;
//! use std::net::{SocketAddr, Ipv4Addr, IpAddr};
//!
//! let mut conn = Connection::new(SocketAddr::new(IpAddr::V4(Ipv4Addr(127,0,0,1)),1000));
//!
//! conn.send_s("Hello World");
//! ```
use std::fmt::Display;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
/// The connection struct
pub struct Connection {
    socket: TcpStream,
    auto_flush: bool,
}
impl Clone for Connection {
    fn clone(&self) -> Self {
        Connection {
            socket: self.socket.try_clone().expect("Failed to clone"),
            auto_flush: self.auto_flush,
        }
    }
    fn clone_from(&mut self, source: &Self) {
        self.socket = source.socket.try_clone().expect("Failed to clone");
        self.auto_flush = source.auto_flush;
    }
}
impl Connection {
    /// construct a connection
    ///
    /// # Example
    /// ```
    /// use mcpi_rs::prelude::*;
    /// use std::net::{SocketAddr, Ipv4Addr, IpAddr};
    ///
    /// let mut conn = Connection::new(SocketAddr::new(IpAddr::V4(Ipv4Addr(127,0,0,1)),1000));
    /// ```
    pub fn new<A: ToSocketAddrs>(address: A) -> Connection {
        Connection {
            socket: TcpStream::connect(address)
                .expect("Couldn't connect to Minecraft, is it running?"),
            auto_flush: true,
        }
    }
    /// send some data to address
    /// # Example
    /// ```
    /// use mcpi_rs::prelude::*;
    /// let a = Connection::new("127.0.0.1:1000");
    /// a.send(vec!["one","two"]);
    /// ```
    pub fn send<T: Display>(self, parts: Vec<T>) {
        for (cnt, i) in parts.iter().enumerate() {
            self.clone()
                .socket
                .write_all(i.to_string().as_bytes())
                .expect("Failed to write from socket");
            if cnt == 0 {
                self.clone()
                    .socket
                    .write_all("(".as_bytes())
                    .expect("Failed to write from socket");
            } else if cnt < parts.len() - 1 {
                self.clone()
                    .socket
                    .write_all(",".as_bytes())
                    .expect("Failed to write from socket");
            }
        }
        self.clone()
            .socket
            .write_all(")\n".as_bytes())
            .expect("Failed to write from socket");
        if self.auto_flush {
            self.flush();
        }
    }
    /// send a string to address (non-format)
    /// # Example
    /// ```
    /// use mcpi_rs::prelude::*;
    /// let c = Connection::new("127.0.0.1:1000");
    /// c.send_s("Hello World!");
    /// ```
    pub fn send_s<T: Display>(self, str: T) {
        self.clone().drain();
        self.clone()
            .socket
            .write_all(str.to_string().as_bytes())
            .expect("Failed to write from socket");
        self.clone()
            .socket
            .write_all("\n".as_bytes())
            .expect("Failed to write from socket");
        if self.auto_flush {
            self.flush();
        }
    }
    /// drains the socket of incoming data
    pub fn drain(self) {
        self.socket
            .set_nonblocking(true)
            .expect("Failed to set non-blocking mode");
        let mut c: [u8; 1] = [0];
        while self.clone().socket.read(&mut c).is_ok() {
            eprint!("{}", c[0]);
        }
    }
    /// flush the stream
    pub fn flush(mut self) {
        self.socket.flush().expect("Failed to flush");
    }
    /// receive the data of the address
    /// # Example
    /// ```
    /// use mcpi_rs::prelude::*;
    /// let c = Connection::new("127.0.0.1:1000");
    /// c.send_s("This string will let the server send back some data");
    /// println!("receive: {}",c.receive());
    /// ```
    pub fn receive(self) -> String {
        self.socket
            .try_clone()
            .unwrap()
            .set_nonblocking(false)
            .unwrap();
        let mut b: BufReader<TcpStream> = BufReader::new(self.socket.try_clone().unwrap());
        let mut s = String::new();
        b.read_line(&mut s).expect("Failed to read line");
        s
    }
    /// close the connection
    /// ```
    /// use mcpi_rs::prelude::*;
    /// let c = Connection::new("127.0.0.1:1000");
    /// // -- snip --
    /// c.close();
    /// ```
    pub fn close(self) {
        self.socket
            .shutdown(Shutdown::Both)
            .expect("Failed to close");
    }
    /// switch the auto flush
    /// # Example
    /// ```
    /// use mcpi_rs::prelude::*;
    /// let c = Connection::new("127.0.0.1:1000");
    /// c.auto_flush(true);//auto setting
    /// ```
    pub fn auto_flush(mut self, flush: bool) {
        self.auto_flush = flush;
        if flush {
            self.flush();
        }
    }
}
