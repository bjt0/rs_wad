use wad::Entry;
use doom::{byteorder::ReadBytesExt, byteorder::LittleEndian, DoomDirection, DoomPoint};
use std::io::Cursor;
use num::{FromPrimitive};
use num_derive::{FromPrimitive};

#[derive(Debug, FromPrimitive)]
pub enum DoomMonsterType {
    Unknown = -1,
    Zombieman = 18, 
    ShotgunGuy = 9,
    Imp = 3001,
    Demon = 3002
}

#[derive(Debug)]
pub enum DoomThingType {
    Unknown,
    DoomMonster(DoomMonsterType)
}

impl DoomThingType {
    pub fn from_type_number(num: u16) -> Self {
        let monster_type = DoomMonsterType::from_u16(num);

        if monster_type.is_some() { return DoomThingType::DoomMonster(monster_type.unwrap()) };

        return DoomThingType::Unknown;
    }
}

pub enum DoomThingFlags {
    None,
    EasyDifficulty,
    MediumDifficulty,
    HardDifficulty,
    Ambush,
    DeathmatchOnly
}

pub struct DoomThing {
    location: DoomPoint,
    direction: DoomDirection,
    thing_type: DoomThingType,
    thing_flags: DoomThingFlags
}

impl DoomThing {
    pub fn thing_type(&self) -> &DoomThingType {
        &self.thing_type
    }

    pub fn from_things_lump(lump: Entry) -> Vec<DoomThing> {
        let mut things = Vec::new();
        let thing_size_bytes = 10;
        let num_things = lump.lump_info().wad_size() / thing_size_bytes;

        for index in 0..num_things {
            let offset1 = thing_size_bytes * index;
            let offset2 = offset1 + thing_size_bytes;
            let thing_data = lump.lump_data().raw_data()[offset1..offset2].to_vec();
            let mut read_cursor = Cursor::new(thing_data);

            let location = DoomPoint::new(
                read_cursor.read_i16::<LittleEndian>().unwrap(), 
                read_cursor.read_i16::<LittleEndian>().unwrap()
            );

            let direction = DoomDirection::from_angle(
                read_cursor.read_u16::<LittleEndian>().unwrap()
            );

            let thing_type = DoomThingType::from_type_number(
                read_cursor.read_u16::<LittleEndian>().unwrap()
            );

            let thing_flags = read_cursor.read_u16::<LittleEndian>().unwrap();
            
            let result = DoomThing { 
                location,
                direction, 
                thing_type,
                thing_flags: DoomThingFlags::None
            };

            things.push(result);
        }

        things
    }
}