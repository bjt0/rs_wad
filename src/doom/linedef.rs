use wad::*;
use doom::{byteorder::ReadBytesExt, byteorder::LittleEndian};
use std::io::Cursor;
use bitflags::*;

bitflags! {
    struct LinedefFlags: u16 {
        const IMPASSABLE = 0b0000000000000001;
        const BLOCKS_MONSTERS = 0b0000000000000010;
        const TWO_SIDED = 0b0000000000000100;
        const UPPER_UNPEGGED = 0b0000000000001000;
        const LOWER_UNPEGGED = 0b0000000000010000;
        const SECRET = 0b0000000000100000;
        const BLOCKS_SOUND = 0b0000000001000000;
        const NOT_SHOWN_ON_MAP = 0b0000000010000000;
        const ALWAYS_SHOWN_ON_MAP = 0b0000000100000000;
    }
}

pub struct Linedef {
    vertex_index_1: u16,
    vertex_index_2: u16,
    linedef_flags: LinedefFlags,
    special: u16, 
    sector_tag: u16, 

    // doom.exe uses signed shorts for these indices
    // so that -1 can be used to indicate lack of value 
    // i.e. sidedef_index_back = -1 for one-sided linedefs
    sidedef_index_front: i16,
    sidedef_index_back: i16
}

impl Linedef {
    pub fn from_linedefs_lump(lump: Entry) -> Vec<Linedef> {
        let linedefs = Vec::new();
        let linedef_size_bytes = 14;
        let num_lindefs = lump.lump().data().len() / linedef_size_bytes;

        for index in 0..num_lindefs {
            let offset1 = linedef_size_bytes * index;
            let offset2 = offset1 + linedef_size_bytes;

            let linedef_data = lump.lump().data().bytes()[offset1..offset2].to_vec();            
            let mut read_cursor = Cursor::new(linedef_data);

            let vertex_index_1 = read_cursor.read_u16::<LittleEndian>().unwrap();
            let vertex_index_2 = read_cursor.read_u16::<LittleEndian>().unwrap();
            let linedef_flags = LinedefFlags::from_bits(read_cursor.read_u16::<LittleEndian>().unwrap());
            let special = read_cursor.read_u16::<LittleEndian>().unwrap();
            let sector_tag = read_cursor.read_u16::<LittleEndian>().unwrap();
            let sidedef_index_front = read_cursor.read_i16::<LittleEndian>().unwrap();
            let sidedef_index_back = read_cursor.read_i16::<LittleEndian>().unwrap();
        }

        linedefs
    }
}