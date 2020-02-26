extern crate regex;

use std::collections::HashMap;
use wad::Entry;

const DOOM_MAP_LUMPS: [&'static str; 10] = [
    "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES", "SECTORS", "REJECT",
    "BLOCKMAP",
];

lazy_static! {
    // the bool value determines whether the given lump is required for a map to be valid
    // the REJECT table is optional because it's only used for quick line of sight checks and so can be omitted
    static ref DOOM_MAP_LUMP_REQUIRED: HashMap<&'static str, bool> = {
        let mut map_lumps = HashMap::new();
        map_lumps.insert("THINGS", true);
        map_lumps.insert("LINEDEFS", true);
        map_lumps.insert("SIDEDEFS", true);
        map_lumps.insert("VERTEXES", true);
        map_lumps.insert("SEGS", true);
        map_lumps.insert("SSECTORS", true);
        map_lumps.insert("NODES", true);
        map_lumps.insert("SECTORS", true);
        map_lumps.insert("REJECT", false);
        map_lumps.insert("BLOCKMAP", true);
        map_lumps
    };
}

pub fn is_valid_map(mut map_marker: Entry) -> bool {
    let valid_doom1_map_marker = regex::Regex::new("E[0-9]M[0-9]").unwrap();
    let valid_doom2_map_marker = regex::Regex::new("MAP[0-9][0-9]").unwrap();

    let map_name: &str = &map_marker.lump_info().name();

    if valid_doom1_map_marker.is_match(map_name) || valid_doom2_map_marker.is_match(map_name) {
        // gather up all the lumps after the map marker if their name exists in the DOOM_MAP_LUMPS list
        // end when we've found a lump that isn't in the DOOM_MAP_LUMPS list
        // this could mean there's a bad lump in between or that we've found all the required lumps
        let mut map_entries = Vec::new();

        while let Some(next_lump) = map_marker.next() {
            let listed_lump = DOOM_MAP_LUMPS.contains(&&next_lump.lump_info().name()[..]); // ew

            if listed_lump {
                map_entries.push(next_lump);
            } else {
                break;
            }
        }

        // we now check the list of map entries that we've found
        // 1. they need to be in the correct order
        // 2. all the ones that are marked as required in the DOOM_MAP_LUMP_REQUIRED hashmap have to be found
        let mut current_entry_index = 0;

        for index in 0..DOOM_MAP_LUMPS.len() {
            let required_map_lump = *DOOM_MAP_LUMP_REQUIRED.get(DOOM_MAP_LUMPS[index]).unwrap();
            let current_entry_match =
                map_entries[current_entry_index].lump_info().name() == DOOM_MAP_LUMPS[index];

            if required_map_lump {
                if current_entry_match {
                    /* println!(
                        "map entry at index {} matches required lump {}",
                        index, DOOM_MAP_LUMPS[index]
                    ); */

                    current_entry_index = current_entry_index + 1;
                } else {
                    return false;
                }
            } else if !required_map_lump {
                if current_entry_match {
                    /* println!(
                        "map entry at index {} matches optional lump {}",
                        index, DOOM_MAP_LUMPS[index]
                    ); */

                    current_entry_index = current_entry_index + 1;
                }
                // if the current entry doesn't match we don't bother to increment the current_entry_index counter
                // we check the same entry against the next lump name in the DOOM_MAP_LUMPS list
                // this ensures that we verify all the entries in the map_entries vec and know that we have all required lumps in the correct order
            }
        }

        return true;
    } else {
        return false;
    }
}