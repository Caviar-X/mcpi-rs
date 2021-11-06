//! Minecraft structs
//! Type ['Minecraft'] represents a connection between Minecraft and the mcpi-rs library
//! the initlize method depends on your bukkit port
//! # Warning
//! All the examples assume that you connect it in default way
//!
//! All the positions are measured by mc_get_pos_int (It may have some tiny deviation)
//! # Examples
//!
//! set a connection between minecraft and rust (default port)
//! ```
//! use mcpi_rs::minecraft::Minecraft;
//! let mc = Minecraft::connect();
//! ```
//! set a connection between mincraft and rust (cunsomize port)
//! ```
//! use mcpi_rs::minecraft::Minecraft;
//! use std::net::{IpAddr, Ipv4Addr};
//!
//! let mc = Minecraft::connect_to(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),4173));
//! ```
//! send a message to minecraft
//!
//! ```
//! use mcpi_rs::minecraft::Minecraft;
//!
//! let mc = Minecraft::connect();
//! mc.mc_post_to_chat("Hello World!");
//! ```

use crate::connection::Connection;
use std::net::{ToSocketAddrs, SocketAddr, IpAddr, Ipv4Addr};
use crate::items::Block;
use std::fmt::Display;
pub const DEFAULT_PORT : u16 = 4711;

/// The Minecraft type
#[derive(Clone)]
pub struct Minecraft {
    connection : Connection
}
/// convert a string to (f64,f64,f64)
/// # Examples
/// ```
/// use mcpi_rs::minecraft::pos_decode;
///
/// let a = String::from("1.0,2.0,3.0");
/// assert_eq!(pos_decode(a).0,1.0);
///
/// let b = String::from("1.1,2.5,3.4");
/// assert_eq!(pos_decode(b).1,2.5);
/// ```
pub fn pos_decode(pos_str : impl ToString) -> (f64,f64,f64) {
    let s = pos_str.to_string();
    let vec = s.split(",").collect::<Vec<&str>>();
    (vec[0].parse().expect("Failed to parse1"),vec[1].parse().expect("Failed to parse2"),vec[2].split_whitespace().collect::<Vec<&str>>()[0].parse().expect("Failed to parse3"))
}
/// convert a string to (i32,i32,i32)
///
/// if you want to convert to (f64.f64,f64),please use `pos_decode`
///
/// # Examples
/// ```
/// use mcpi_rs::minecraft::pos_decode_int;
///
/// let a = String::from("1,2,3");
/// assert_eq!(pos_decode_int(a),(1,2,3));
///
/// let b = String::from("3,5,4");
/// assert_eq!(pos_decode_int(b).1,3);
/// ```
pub fn pos_decode_int(pos_str : impl ToString) -> (i32,i32,i32) {
    let s = pos_str.to_string().replace("\n","");
    let vec = s.split(",").collect::<Vec<&str>>();
    (vec[0].parse().expect("Failed to parse1"),vec[1].parse().expect("Failed to parse 2"),vec[2].parse().expect("Failed to parse 3"))
}
/// convert a tuple with 3 params to string
/// # Example
/// ```
/// use mcpi_rs::minecraft::pos_to_string;
///
/// assert_eq!(pos_to_string((1,2,3)),"1,2,3");
/// ```
pub fn pos_to_string(pos : (impl Display,impl Display,impl Display)) -> String {
    format!("{},{},{}",pos.0,pos.1,pos.2)
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////
impl Minecraft {
    /// construct a minecraft type
    /// # Examples
    /// ```
    /// use mcpi_rs::minecraft::Minecraft;
    /// use mcpi_rs::connection::Connection;
    /// use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let mc = Minecraft::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),4173));
    /// ```
    pub fn new(connection : Connection) -> Minecraft {
        Minecraft {
            connection
        }
    }
    /// set a connection between minecraft and rust (default port)
    /// # Example
    /// ```
    /// use mcpi_rs::minecraft::Minecraft;
    ///
    /// let mc = Minecraft::connect();
    /// ```
    pub fn connect() -> Minecraft {
        Minecraft::new(Connection::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),DEFAULT_PORT)))
    }
    /// set a connection between minecraft and rust (customize port)
    /// # Example
    /// ```
    /// use mcpi_rs::minecraft::Minecraft;
    ///
    /// let mc = Minecraft::connect_to(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),4173));
    /// ```
    pub fn connect_to<A : ToSocketAddrs>(address : A) -> Minecraft {
        self::Minecraft::new(Connection::new(address))
    }
    /// get a block's data from minecraft
    /// # Example
    /// ```
    /// use mcpi_rs::minecraft::Minecraft;
    /// use mcpi_rs::items::{Block, STONE};
    ///
    /// let mc = Minecraft::connect();
    ///
    /// mc.clone().mc_set_block((12,35,64),Block::from_item(STONE));
    ///
    /// let block : Block = mc.clone().mc_get_block((12,35,64));
    ///
    /// assert_eq!(block,Block::from_item(STONE))
    /// ```
    pub fn mc_get_block(self,pos : (i32,i32,i32)) -> Block{
        self.clone().connection.send(vec!["world.getBlock",pos_to_string(pos).as_str()]);
        Block::decode(self.clone().connection.receive())
    }
    /// update a data of a block in a position
    /// # Example
    ///
    /// ```
    /// use mcpi_rs::minecraft::Minecraft;
    /// use mcpi_rs::items::{Block, STONE};
    ///
    /// let mc = Minecraft::connect();
    ///
    /// mc.clone().mc_set_block((1,2,3),Block::from_item(STONE));
    ///
    /// assert_eq!(mc.clone().mc_get_block((1,2,3)),Block::from_item(STONE));
    /// ```
    pub fn mc_set_block(self,pos : (i32,i32,i32),block : Block) {
        self.clone().connection.send_s(format!("world.setBlock({},{},{},{})",pos.0,pos.1,pos.2,block.to_string()));
    }
    /// update the data of the block from pos_begin to pos_end
    ///
    /// # Example
    ///
    /// ```
    /// use mcpi_rs::minecraft::Minecraft;
    /// use mcpi_rs::items::{Block, DIAMOND_ORE};
    ///
    /// let mc = Minecraft::connect();
    ///
    /// mc.clone().mc_set_blocks((12,13,14),(12,13,16),Block::from_item(DIAMOND_ORE));
    ///
    /// assert_eq!(mc.clone().mc_get_block((12,13,15)),Block::from_item(STONE));
    /// ```
    pub fn mc_set_blocks(self,pos_begin : (i32,i32,i32),pos_end : (i32,i32,i32),block : Block) {
        self.clone().connection.send_s(format!("world.setBlocks({},{},{})",pos_to_string(pos_begin),pos_to_string(pos_end),block.to_string()));
    }
    /// get the height of the world
    /// # Example
    ///
    /// ```
    ///use mcpi_rs::minecraft::Minecraft;
    ///
    /// let mc = Minecraft::connect();
    ///
    ///
    ///
    /// ```
    pub fn mc_get_pos_y(self,x : i32,z : i32) -> i32{
        self.clone().connection.send(vec!["world.getHeight",x.to_string().as_str(),z.to_string().as_str()]);
        self.clone().connection.receive().split_whitespace().collect::<Vec<&str>>()[0].parse().unwrap()
    }
    /// send a message to minecraft
    ///
    /// # Examples
    ///
    /// ```
    /// use mcpi_rs::minecraft::Minecraft;
    /// use std::fmt::Display;
    ///
    /// let mc = Minecraft::connect();
    ///
    /// mc.mc_post_to_chat(114514);
    ///
    /// mc.mc_post_to_chat("Hello World");
    ///
    /// #[derive(Clone)]
    /// struct lay {
    ///     data : i32,
    ///     param : i128
    /// }
    ///
    /// impl Display for lay {
    ///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    ///         write!("{} {}",self.clone().data,self.clone().param)
    ///    }
    /// }
    /// let l = lay {
    ///     data : 0,
    ///     param : 114514
    /// };
    /// mc.mc_post_to_chat(l);
    /// ```
    pub fn mc_post_to_chat(self,message : impl Display) {
        self.connection.send(vec!["chat.post",message.to_string().as_str()]);

    }
    /// get the position of the player (**cannot use it when the server has multi players**)
    ///
    /// # Example
    ///
    /// ```
    /// use mcpi_rs::minecraft::Minecraft;
    ///
    /// let mc = Minecraft::connect();
    ///
    /// mc.clone().mc_set_pos((12.25f64,100.11789f64,13f64));
    ///
    /// assert_eq!(mc.clone().mc_get_pos(),(12.25f64,100.11789f64,13f64));
    /// ```
    pub fn mc_get_pos(self) -> (f64,f64,f64) {
        self.clone().connection.send_s("player.getPos()");
        pos_decode(self.clone().connection.receive())
    }
    pub fn mc_get_pos_int(self) -> (i32,i32,i32) {
        self.clone().connection.send_s("player.getTile()");
        let receive = self.clone().connection.receive();
        let pos_exact = pos_decode_int(receive);
        (pos_exact.0 as i32,pos_exact.1 as i32,pos_exact.2 as i32)
    }
    pub fn mc_set_pos(self,pos : (f32,f32,f32)) {
        self.connection.send(vec!["player.setPos",pos_to_string(pos).as_str()]);
    }
    pub fn mc_set_pos_int(self,pos : (i32,i32,i32)) {
        self.connection.send(vec!["player.setTile",pos_to_string(pos).as_str()]);
    }
    pub fn mc_world_setting(self,key : impl ToString,val : bool) {
        self.connection.send(vec!["player.setting",key.to_string().as_str(),val.to_string().as_str()]);
    }
    pub fn mc_get_pos_entity_int(self,id : i32) -> (i32,i32,i32) {
        self.clone().connection.send(vec!["entity.getTile",id.to_string().as_str()]);
        let pos = pos_decode(self.clone().connection.receive());
        (pos.0 as i32,pos.1 as i32,pos.2 as i32)
    }
    pub fn mc_set_pos_entity_int(self,id : i32,pos : (i32,i32,i32)) {
        self.connection.send(vec!["entity.setTile",id.to_string().as_str(),pos_to_string(pos).as_str()]);
    }
    pub fn mc_get_pos_entity(self,id : i32) -> (f64,f64,f64) {
        self.clone().connection.send(vec!["entity.getPos",id.to_string().as_str()]);
        pos_decode(self.clone().connection.receive())
    }
    pub fn mc_set_pos_entity(self,id : i32,pos : (f64,f64,f64)) {
        self.connection.send(vec!["entity.setPos",id.to_string().as_str(),pos_to_string(pos).as_str()]);
    }
    pub fn mc_set_camera_normal(self) {
        self.connection.send_s("camera.mode.setNormal()");
    }
    pub fn auto_flush(self,auto : bool) {
        self.connection.auto_flush(auto);
    }
}