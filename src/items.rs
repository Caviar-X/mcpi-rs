use self::Item::Id;
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq,Hash)]
pub enum Item {
    Id(u32),
}
impl Item {
    pub fn unwrap(self) -> u32 {
        match self {
            Id(x) => x,
        }
    }
}
pub const AIR: Item = Id(0);
pub const STONE: Item = Id(1);
pub const GRASS: Item = Id(2);
pub const DIRT: Item = Id(3);
pub const COBBLESTONE: Item = Id(4);
pub const WOOD_PLANKS: Item = Id(5);
pub const SAPLING: Item = Id(6);
pub const BEDROCK: Item = Id(7);
pub const WATER_FLOWING: Item = Id(8);
pub const WATER: Item = WATER_FLOWING;
pub const WATER_STATIONARY: Item = Id(9);
pub const LAVA_FLOWING: Item = Id(10);
pub const LAVA: Item = LAVA_FLOWING;
pub const LAVA_STATIONARY: Item = Id(11);
pub const SAND: Item = Id(12);
pub const GRAVEL: Item = Id(13);
pub const GOLD_ORE: Item = Id(14);
pub const IRON_ORE: Item = Id(15);
pub const COAL_ORE: Item = Id(16);
pub const WOOD: Item = Id(17);
pub const LEAVES: Item = Id(18);
pub const GLASS: Item = Id(20);
pub const LAPINS_LAZULI_ORE: Item = Id(21);
pub const LAPINS_LAZULI_BLOCK: Item = Id(22);
pub const SANDSTONE: Item = Id(24);
pub const BED: Item = Id(26);
pub const COBWEB: Item = Id(30);
pub const GRASS_TALL: Item = Id(31);
pub const WOOL: Item = Id(35);
pub const FLOWER_YELLOW: Item = Id(37);
pub const FLOWER_CYAN: Item = Id(38);
pub const MUSHROOM_BROWN: Item = Id(39);
pub const MUSHROOM_RED: Item = Id(40);
pub const GOLD_BLOCK: Item = Id(41);
pub const IRON_BLOCK: Item = Id(42);
pub const STONE_SLAB_DOUBLE: Item = Id(43);
pub const STONE_SLAB: Item = Id(44);
pub const BRICK_BLOCK: Item = Id(45);
pub const TNT: Item = Id(46);
pub const BOOKSHELF: Item = Id(47);
pub const MOSS_STONE: Item = Id(48);
pub const OBSIDIAN: Item = Id(49);
pub const TORCH: Item = Id(50);
pub const FIRE: Item = Id(51);
pub const STAIRS_WOOD: Item = Id(53);
pub const CHEST: Item = Id(54);
pub const DIAMOND_ORE: Item = Id(56);
pub const DIAMOND_BLOCK: Item = Id(57);
pub const CRAFTING_TABLE: Item = Id(58);
pub const FARMLAND: Item = Id(60);
pub const FURNACE_INACTIVE: Item = Id(61);
pub const FURNACE_ACTIVE: Item = Id(62);
pub const DOOR_WOOD: Item = Id(64);
pub const LADDER: Item = Id(65);
pub const STAIRS_COBBLESTONE: Item = Id(67);
pub const DOOR_IRON: Item = Id(71);
pub const REDSTONE_ORE: Item = Id(73);
pub const SNOW: Item = Id(78);
pub const ICE: Item = Id(79);
pub const SNOW_BLOCK: Item = Id(80);
pub const CACTUS: Item = Id(81);
pub const CLAY: Item = Id(82);
pub const SUGAR_CANE: Item = Id(83);
pub const FENCE: Item = Id(85);
pub const GLOWSTONE_BLOCK: Item = Id(89);
pub const BEDROCK_INVISIBLE: Item = Id(95);
pub const STONE_BRICK: Item = Id(98);
pub const GLASS_PANE: Item = Id(102);
pub const MELON: Item = Id(103);
pub const FENCE_GATE: Item = Id(107);
pub const GLOWING_OBSIDIAN: Item = Id(246);
pub const NETHER_REACTOR_CORE: Item = Id(247);
#[derive(Clone)]
pub struct Block {
    id : Item,
    data : i32
}
impl Block {
    pub fn new(data : i32,id : Item) -> Block {
        Block {
            id,
            data
        }
    }
    pub fn new_without_data(id : Item) -> Block{
        Block {
            id,
            data: 0
        }
    }
    pub fn from_item(id : Item) -> Block {
        Block {
            id,
            data : 0
        }
    }
    pub fn decode(s : String) -> Block{
        return if s.find(",").is_none() {
            Block::new_without_data(Id(s.clone().parse::<u32>().expect("Failed to parse")))
        } else {
            let vec = s.split(",").collect::<Vec<&str>>();
            let id = Id(vec[0].parse::<u32>().expect("Failed to parse"));
            let data = vec[1].parse::<i32>().expect("Failed to parse");
            Block::new(data, id)
        };
    }
}
impl ToString for Block {
    fn to_string(&self) -> String {
        format!("{}",self.clone().id.unwrap())
    }
}