extern crate byteorder;

pub mod map;
pub mod thing;
pub mod types;
pub mod linedef;
pub mod sidedef;
pub mod vertex;

#[derive(Debug)]
pub struct DoomPoint {
    x_position: i16,
    y_position: i16
}

impl DoomPoint {
    pub fn x(&self) -> i16 {
        self.x_position
    }

    pub fn y(&self) -> i16 {
        self.y_position
    }

    pub fn new(x: i16, y: i16) -> DoomPoint {
        DoomPoint { x_position: x, y_position: y }
    }
}

#[derive(Debug)]
pub enum DoomDirection {
    Unknown,
    East, 
    Northeast,
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast
}

impl DoomDirection {
    pub fn from_angle(angle: u16) -> Self {
        match angle {
            0 => DoomDirection::East, 
            45 => DoomDirection::Northeast,
            90 => DoomDirection::North,
            135 => DoomDirection::Northwest,
            180 => DoomDirection::West,
            225 => DoomDirection::Southwest,
            270 => DoomDirection::South,
            315 => DoomDirection::Southeast,
            _ => DoomDirection::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use wad::*;
    use doom::*;

    #[test]
    fn verify_valid_doom_map() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");

        let pass = w.get_by_name("E1M1");
        assert!(pass.is_some());

        let e1m1_entry = pass.unwrap();
        assert!(map::is_valid_map(e1m1_entry.clone()));
    }

    #[test]
    fn verify_doom_map_lump_data() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");
        let mut map_marker = w.get_by_name("E1M1").unwrap();

        let things_wad_entry = map_marker.next().unwrap();
        let linedefs_wad_entry = map_marker.next().unwrap();
        let sidedefs_wad_entry = map_marker.next().unwrap();
        let vertexes_wad_entry = map_marker.next().unwrap();
        let segs_wad_entry = map_marker.next().unwrap();
        let ssectors_wad_entry = map_marker.next().unwrap();

        let expected_num_things = 158;
        let num_things = things_wad_entry.lump().data().len() / 10;
        assert!(num_things == expected_num_things);

        let expected_num_linedefs = 645;
        let num_linedefs = linedefs_wad_entry.lump().data().len() / 14;
        assert!(num_linedefs == expected_num_linedefs);

        let expected_num_sidedefs = 876;
        let num_sidedefs = sidedefs_wad_entry.lump().data().len() / 30;
        assert!(num_sidedefs == expected_num_sidedefs);

        let expected_num_vertexes = 564;
        let num_vertexes = vertexes_wad_entry.lump().data().len() / 4;
        assert!(num_vertexes == expected_num_vertexes);

        let expected_num_segs = 946;
        let num_segs = segs_wad_entry.lump().data().len() / 12;
        assert!(num_segs == expected_num_segs);

        let expected_num_ssectors = 258;
        let num_ssectors = ssectors_wad_entry.lump().data().len() / 4;
        assert!(num_ssectors == expected_num_ssectors);
    }

    #[test]
    fn verify_missing_optional() {
        let broken: Wad = Wad::from_path("./GOETIA1-BROKEN.wad");
        // in GOETIA1-BROKEN.wad, E1M9 is deliberately missing it's REJECT table
        let missing_e1m9 = broken.get_by_name("E1M9");
        assert!(missing_e1m9.is_some());

        let e1m9_entry = missing_e1m9.unwrap();
        assert!(map::is_valid_map(e1m9_entry.clone()));
    }

    #[test]
    fn detect_broken_doom_map() {
        let broken: Wad = Wad::from_path("./GOETIA1-BROKEN.wad");
        // in GOETIA1-BROKEN.wad, E1M3 is deliberately broken and doesn't include the SIDEDEFS and SECTORS lumps
        let broken_e1m3 = broken.get_by_name("E1M3");
        assert!(broken_e1m3.is_some());

        let e1m3_entry = broken_e1m3.unwrap();
        assert!(!map::is_valid_map(e1m3_entry.clone()));
    }

    #[test]
    fn verify_all_thing_types_loaded() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");
        let maplist = map::DoomMap::get_maps(&w);
        
        assert!(maplist.len() == 9); // there should be 9 maps in GOETIA1.wad 

        for map in maplist {
            for thing in map.things() {
                let unknown_thing = match thing.thing_type() {
                    types::ThingType::Unknown(_) => true,
                    _ => false
                };
                
                assert_eq!(unknown_thing, false);
            }
        }
    }

    // #[test]
    // fn load_linedefs() {
    //     let w: Wad = Wad::from_path("./GOETIA1.wad");
    //     let maplist = map::DoomMap::get_maps(&w);

    //     for map in maplist {
    //         for linedef in map.linedefs() {
                
    //         }
    //     }
    // }
}