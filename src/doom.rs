extern crate regex;
use wad::Entry;

const DOOM_MAP_REQUIRED_LUMPS: [&'static str; 9] = [
    "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES", "SECTORS",
    "BLOCKMAP",
];

const DOOM_MAP_OPTIONAL_LUMPS: [&'static str; 1] = ["REJECT"];

pub fn is_valid_map(mut map_marker: Entry) -> bool {
    let valid_doom1_map_marker = regex::Regex::new("E[0-9]M[0-9]").unwrap();
    let valid_doom2_map_marker = regex::Regex::new("MAP[0-9][0-9]").unwrap();

    if valid_doom1_map_marker.is_match(&map_marker.lump_info().name())
        || valid_doom2_map_marker.is_match(&map_marker.lump_info().name())
    {
        let mut temp = 0; // required lumps have to be in exactly the order of the REQUIRED_LUMPS const list
        let map_name: String = String::from(map_marker.lump_info().name());

        // the map marker is useless other than as an index to start at, so we can just next() its info away
        while let Some(current_entry) = map_marker.next() {
            if temp >= DOOM_MAP_REQUIRED_LUMPS.len() {
                break;
            }

            let current_required_lump = DOOM_MAP_REQUIRED_LUMPS[temp];

            if current_entry.lump_info().name() == current_required_lump {
                temp = temp + 1;
            } else {
                // check if this is an optional lump
                for current_optional_lump in DOOM_MAP_OPTIONAL_LUMPS.iter() {
                    if &current_entry.lump_info().name() == current_optional_lump {
                    }
                }
            }
        }
    }

    false
}
