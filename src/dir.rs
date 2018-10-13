use std::fmt;
use std::fs::File;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};

use utils;
use wad::Header;

pub struct Directory {
    lumps: Vec<Lump>
}

impl Directory {
    pub fn get_at_index(&self, index: usize) -> &Lump {
        if index >= self.lumps.len() {
            panic!("Tried to index a lump that was outside of the WAD's lump count.");
        }

        &self.lumps[index]
    }

    pub fn num_lumps(&self) -> usize {
        self.lumps.len()
    }

    pub fn from_file(mut file: &File, header: &Header) -> Directory {
        match file.seek(SeekFrom::Start(header.dir_offset() as u64)) {
            Ok(_)    => { }
            Err(why) => panic!("Unable to seek to the start of the file. ({})", why.description()) 
        };

        struct Entry(String, u32, usize, usize); // name, index, size, offset
        let mut entries: Vec<Entry> = Vec::new();

        for index in 0..header.num_lumps() {
            let mut entry_raw: [u8; 16] = [0; 16];

            match file.read(&mut entry_raw) {
                Ok(_) => { },
                Err(why) => panic!("Error when reading lump {}: {}", index, why.description())
            }

            // pointer to the start of the lump's data
            let dir_offset: usize = utils::u8ref_to_u32(&entry_raw[0..4]) as usize;
            // the size of the lump in bytes
            let lump_size: usize = utils::u8ref_to_u32(&entry_raw[4..8]) as usize;

            let lump_name_str: String = match String::from_utf8(entry_raw[8..16].to_vec()) {
                Ok(wtype)      => wtype,
                Err(not_ascii) => panic!("Could not read entry name for {}: {}", index, not_ascii.description())
            };

            let e = Entry { 0: lump_name_str, 1: index as u32, 2: lump_size, 3: dir_offset };
            entries.push(e);
        }

        let mut results: Vec<Lump> = Vec::new();

        for entry in entries {
            if entry.2 > 0 {
                // seek to raw data location
                match file.seek(SeekFrom::Start(entry.3 as u64)) {
                    Ok(_)    => { }
                    Err(why) => panic!("Unable to seek to the data for lump {}. Reason: {}", entry.0, why.description()) 
                };

                let mut raw_data: Vec<u8> = Vec::with_capacity(entry.2);

                match file.read_exact(&mut raw_data) {
                    Ok(_) => { },
                    Err(why) => panic!("Error when reading data for lump {}. Reason: {}", entry.0, why.description())
                }

                let result = Lump { name: entry.0.clone(), index: entry.1, size: entry.2, location: entry.3, data: raw_data };
                results.push(result);
            } else {
                // this is a marker
                let data: Vec<u8> = Vec::new();
                let result = Lump { name: entry.0.clone(), index: entry.1, size: entry.2, location: entry.3, data };

                results.push(result);
            }
        }

        Directory { lumps: results }
    }
}

// TODO: make the concept of a "lump" into a trait and use it as a generic store for both WAD1 and WAD2 lumps
// WAD2 lumps have a different format including compression type, etc
pub struct Lump {
    name:     String, 
    index:    u32, 
    size:     usize,
    location: usize,
    data:     Vec<u8>
}

impl fmt::Display for Lump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: index: {}, size: {}, location: {}", self.name, self.index, self.size, self.location)
    }
}