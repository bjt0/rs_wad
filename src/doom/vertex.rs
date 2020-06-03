use wad::{Lump, FromLump};
use doom::{DoomPoint, byteorder::ReadBytesExt, byteorder::LittleEndian};
use std::io::Cursor;

pub struct Vertex {
    location: DoomPoint
}

impl Vertex {
    pub fn x(&self) -> i16 {
        self.location.x()
    }

    pub fn y(&self) -> i16 {
        self.location.y()
    }
}

impl FromLump<Vec<Vertex>> for Vertex {
    fn from_lump(lump: &Lump) -> Vec<Vertex> {
        let mut vertexes = Vec::new();
        let vertex_size_bytes = 4;
        let num_vertexes = lump.data().len() / vertex_size_bytes;

        for index in 0..num_vertexes {
            let offset1 = vertex_size_bytes * index;
            let offset2 = offset1 + vertex_size_bytes;

            let vertex_data = lump.data().bytes()[offset1..offset2].to_vec();            
            let mut read_cursor = Cursor::new(vertex_data);

            vertexes.push(Vertex {
                location: DoomPoint::new(
                    read_cursor.read_i16::<LittleEndian>().unwrap(), 
                    read_cursor.read_i16::<LittleEndian>().unwrap()
                )
            });
        }

        vertexes
    }
}