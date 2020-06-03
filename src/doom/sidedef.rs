use wad::{Lump, FromLump};

pub struct Sidedef {
    
}

impl Sidedef {
    
}

impl FromLump<Vec<Sidedef>> for Sidedef {
    fn from_lump(lump: &Lump) -> Vec<Sidedef> {
        let sidedefs = Vec::new();

        sidedefs
    }
}