use std::any::Any;

pub struct BlockEvent {
    pos : (i32,i32,i32),
    face : i32,
    entity_id : i32
}
impl BlockEvent {
    pub fn hit(pos : (i32,i32,i32),face : i32,entity_id : i32) -> BlockEvent {
        BlockEvent {
            pos,
            face,
            entity_id
        }
    }
}
impl ToString for BlockEvent {
    fn to_string(&self) -> String {
        format!("BlockEvent({:?},{},{},{},{},{}",self.type_id(),self.pos.0,self.pos.1,self.pos.2,self.face,self.entity_id)
    }
}
pub fn decode_xyz(encoded : String) {
    let mut ret : (i32,i32,i32) = (0,0,0);
    let mut cnt = 0;
    for i in encoded.split("\\,") {
        if i == "" || i == " " || i == "\n"  {
            continue;
        }
        match cnt {
            0 => {ret.0 = i.parse().expect("Failed to parse");}
            1 => {ret.1 = i.parse().expect("Failed to parse");}
            2 => {ret.2 = i.parse().expect("Failed to parse");}
            _ => break
        };
        cnt+=1;
    }
}

