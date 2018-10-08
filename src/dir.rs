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

        let mut results: Vec<Lump> = Vec::new();
        // each directory entry is 16 bytes
        for index in 0..header.num_lumps() {
            let mut entry_raw: [u8; 16] = [0; 16];

            match file.read(&mut entry_raw) {
                Ok(_) => { },
                Err(why) => panic!("Error when reading lump size for lump {}: {}", index, why.description())
            }

            // pointer to the start of the lump's data
            let dir_offset: usize = utils::u8ref_to_u32(&entry_raw[0..4]) as usize;
            // the size of the lump in bytes
            let lump_size: usize = utils::u8ref_to_u32(&entry_raw[4..8]) as usize;

            let lump_name_str: String = match String::from_utf8(entry_raw[8..16].to_vec()) {
                Ok(wtype)      => wtype,
                Err(not_ascii) => panic!("Could not read entry name for {}. Reason: {}", index, not_ascii.description())
            };

            let result: Lump = Lump { name: lump_name_str, index, size: lump_size, location: dir_offset };
            results.push(result);
        }

        Directory { lumps: results }
    }
}

pub struct Lump {
    name:     String, 
    index:    usize, 
    size:     usize,
    location: usize
}

impl fmt::Display for Lump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. index: {}, size: {}, location: {}", self.name, self.index, self.size, self.location)
    }
}