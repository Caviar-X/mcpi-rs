use crate::connection::Connection;
use std::net::{ToSocketAddrs, SocketAddr, IpAddr, Ipv4Addr};
use crate::items::Block;
use std::fmt::Display;

pub const DEFAULT_PORT : u16 = 4711;
#[derive(Clone)]
pub struct Minecraft {
    connection : Connection
}
pub fn pos_decode(pos_str : impl ToString) -> (f64,f64,f64) {
    let s = pos_str.to_string();
    let vec = s.split("\\,").collect::<Vec<&str>>();
    (vec[0].parse().expect("Failed to parse"),vec[1].parse().expect("Failed to parse"),vec[2].parse().expect("Failed to parse"))
}
pub fn pos_to_string(pos : (impl Display,impl Display,impl Display)) -> String {
    format!("{},{},{}",pos.0,pos.1,pos.2)
}

impl Minecraft {
    pub fn new(connection : Connection) -> Minecraft {
        Minecraft {
            connection
        }
    }

    pub fn connect() -> Minecraft {
        Minecraft::new(Connection::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),DEFAULT_PORT)))
    }
    pub fn connect_to<A : ToSocketAddrs>(address : A) -> Minecraft {
        self::Minecraft::new(Connection::new(address))
    }
    pub fn mc_get_block(self,pos : (i32,i32,i32)) -> Block{
        self.clone().connection.send(vec!["world.getBlock",pos_to_string(pos).as_str()]);
        Block::decode(self.clone().connection.receive())
    }
    pub fn mc_set_block(self,pos : (i32,i32,i32),block : Block) {
        self.clone().connection.send_s(format!("world.setBlock({},{},{},{})\n",pos.0,pos.1,pos.2,block.to_string()));
    }
    pub fn mc_set_blocks(self,pos_begin : (i32,i32,i32),pos_end : (i32,i32,i32),block : Block) {
        self.clone().connection.send_s(format!("world.setBlocks({},{},{})\n",pos_to_string(pos_begin),pos_to_string(pos_end),block.to_string()));
    }
    pub fn mc_get_pos_y(self,x : i32,z : i32) {
        self.connection.send(vec!["world.getHeight",x.to_string().as_str(),z.to_string().as_str()]);
    }
    pub fn mc_post_to_chat(self,message : impl Display) {
        self.connection.send(vec!["chat.post",message.to_string().as_str()]);
    }
    pub fn mc_get_pos(self) -> (f64,f64,f64) {
        self.clone().connection.send(vec!["player.getPos"]);
        pos_decode(self.clone().connection.receive())
    }
    pub fn mc_get_pos_int(self) -> (i32,i32,i32) {
        let pos_exact = self.mc_get_pos();
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
        self.connection.send(vec!["camera.mode.setNormal"]);
    }
    pub fn auto_flush(self,auto : bool) {
        self.connection.auto_flush(auto);
    }
}