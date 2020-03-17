extern crate byteorder;

pub mod map;
pub mod thing;
pub mod types;

pub struct DoomPoint {
    x_position: i16,
    y_position: i16
}

impl DoomPoint {
    pub fn new(x: i16, y: i16) -> DoomPoint {
        DoomPoint { x_position: x, y_position: y }
    }
}

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
    use doom::map::*;

    #[test]
    fn verify_valid_doom_map() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");

        let pass = w.get_by_name("E1M1");
        assert!(pass.is_some());

        let e1m1_entry = pass.unwrap();
        assert!(is_valid_map(e1m1_entry.clone()));
    }

    #[test]
    fn verify_lump_size() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");
        let mut map_marker = w.get_by_name("E1M1").unwrap();

        let things_lump = map_marker.next().unwrap();
        let things_lump_info = things_lump.lump_info();
        let things_lump_data = things_lump.lump_data();
        let things_lump_raw_data = things_lump_data.raw_data();

        assert!(things_lump_info.wad_size() == things_lump_raw_data.len());

        // each doom thing entry is 10 bytes long
        // ergo, there should be len / 10 things in the lump
        let expected_num_things = 158; // GOETIA1.wad E1M1 has 158 THINGS entries
        let num_things = things_lump_data.len() / 10;

        assert!(num_things == expected_num_things);

        let linedefs_lump = map_marker.next().unwrap();
        let linedefs_lump_info = linedefs_lump.lump_info();
        let linedefs_lump_data = linedefs_lump.lump_data();
        let linedefs_lump_raw_data = linedefs_lump_data.raw_data();

        assert!(linedefs_lump_info.wad_size() == linedefs_lump_raw_data.len());
        let expected_num_linedefs = 645;
        let num_linedefs = linedefs_lump_data.len() / 14;

        assert!(num_linedefs == expected_num_linedefs);

        let sidedefs_lump = map_marker.next().unwrap();
        let sidedefs_lump_info = sidedefs_lump.lump_info();
        let sidedefs_lump_data = sidedefs_lump.lump_data();
        let sidedefs_lump_raw_data = sidedefs_lump_data.raw_data();

        assert!(sidedefs_lump_info.wad_size() == sidedefs_lump_raw_data.len());
        let expected_num_sidedefs = 876;
        let num_sidedefs = sidedefs_lump_data.len() / 30;

        assert!(num_sidedefs == expected_num_sidedefs);

        let vertexes_lump = map_marker.next().unwrap();
        let vertexes_lump_info = vertexes_lump.lump_info();
        let vertexes_lump_data = vertexes_lump.lump_data();
        let vertexes_lump_raw_data = vertexes_lump_data.raw_data();

        assert!(vertexes_lump_info.wad_size() == vertexes_lump_raw_data.len());
        let expected_num_vertexes = 564;
        let num_vertexes = vertexes_lump_data.len() / 4;

        assert!(num_vertexes == expected_num_vertexes);

        let segs_lump = map_marker.next().unwrap();
        let segs_lump_info = segs_lump.lump_info();
        let segs_lump_data = segs_lump.lump_data();
        let segs_lump_raw_data = segs_lump_data.raw_data();

        assert!(segs_lump_info.wad_size() == segs_lump_raw_data.len());
        let expected_num_segs = 946;
        let num_segs = segs_lump_data.len() / 12;

        assert!(num_segs == expected_num_segs);

        let ssectors_lump = map_marker.next().unwrap();
        let ssectors_lump_info = ssectors_lump.lump_info();
        let ssectors_lump_data = ssectors_lump.lump_data();
        let ssectors_lump_raw_data = ssectors_lump_data.raw_data();

        assert!(ssectors_lump_info.wad_size() == ssectors_lump_raw_data.len());
        let expected_num_ssectors = 258;
        let num_ssectors = ssectors_lump_data.len() / 4;

        assert!(num_ssectors == expected_num_ssectors);
    }

    #[test]
    fn verify_missing_optional() {
        let broken: Wad = Wad::from_path("./GOETIA1-BROKEN.wad");
        // in GOETIA1-BROKEN.wad, E1M9 is deliberately missing it's REJECT table
        let missing_e1m9 = broken.get_by_name("E1M9");
        assert!(missing_e1m9.is_some());

        let e1m9_entry = missing_e1m9.unwrap();
        assert!(is_valid_map(e1m9_entry.clone()));
    }

    #[test]
    fn detect_broken_doom_map() {
        let broken: Wad = Wad::from_path("./GOETIA1-BROKEN.wad");
        // in GOETIA1-BROKEN.wad, E1M3 is deliberately broken and doesn't include the SIDEDEFS and SECTORS lumps
        let broken_e1m3 = broken.get_by_name("E1M3");
        assert!(broken_e1m3.is_some());

        let e1m3_entry = broken_e1m3.unwrap();
        assert!(!is_valid_map(e1m3_entry.clone()));
    }

    #[test]
    fn load_all_doom_maps() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");
        let maplist = DoomMap::get_maps(&w);
        assert!(maplist.len() == 9); // there should be 9 maps in GOETIA1.wad 

        for map in maplist {
            println!("{}", map.name());

            for thing in map.things() {
                println!("{:?}", thing.thing_type());
            }
        }
    }
}