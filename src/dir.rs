use std::fmt;
use std::fs::File;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};

use utils;
use wad::Header;

pub struct Directory {
    lumps: Vec<Lump>,
    cache: Vec<LumpData>
}

impl Directory {
    pub fn get_at_index(&self, index: usize) -> Option<&Lump> {
        if index >= self.lumps.len() {
            return None
        }

        Some(&self.lumps[index])
    }

    pub fn get_data_at_index(&self, index: usize) -> Option<&Vec<u8>> {
        if index >= self.lumps.len() {
            return None
        }

        Some(&self.cache[index].data)
    }

    pub fn num_lumps(&self) -> usize {
        self.lumps.len()
    }

    pub fn from_file(mut file: &File, header: &Header) -> Directory {
        match file.seek(SeekFrom::Start(header.dir_offset() as u64)) {
            Ok(_)    => { }
            Err(why) => panic!("Unable to seek to the start of the directory. ({})", why.description()) 
        };

        let mut results: Vec<Lump> = Vec::new();

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

            // this is technically just ASCII, so I should probably change it but it works for now
            let lump_name_str: String = match String::from_utf8(entry_raw[8..16].to_vec()) {
                Ok(wtype)      => wtype,
                Err(not_ascii) => panic!("Could not read entry name for {}: {}", index, not_ascii.description())
            };

            // remove trailing unicode NULLs from the conversion
            let trimmed_lump_name_str: String = lump_name_str.trim_right_matches(char::from(0)).to_string();

            let lump = Lump { name: trimmed_lump_name_str, index, size: lump_size, location: dir_offset };
            results.push(lump);
        }

        // borrow results to load data
        let mut cache: Vec<LumpData> = Vec::new();

        for lump in &results {
            let position = match file.seek(SeekFrom::Start(lump.location as u64)) {
                Ok(pos)  => pos,
                Err(why) => panic!("Unable to seek to the location of lump {}. Reason: {}", lump.name, why.description()) 
            };

            let mut raw_data = vec![0; lump.size];
            let lump_read = match file.read_exact(&mut raw_data) {
                Ok(_) => { },
                Err(why) => panic!("Error when reading data for lump {}. Reason: {}", lump.name, why.description())
            };

            let lump_data = LumpData { index: lump.index, data: raw_data };
            cache.push(lump_data);
        }

        Directory { lumps: results, cache }
    }
}

// TODO: make the concept of a "lump" into a trait and use it as a generic store for both WAD1 and WAD2 lumps
// WAD2 lumps have a different format including compression type, etc
pub struct Lump {
    name:     String, 
    index:    usize, 
    size:     usize,
    location: usize,
}

impl Lump {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

impl fmt::Display for Lump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: index: {}, size: {}, location: {}", self.name, self.index, self.size, self.location)
    }
}

// basically just a wrapper for a u8 vec so that it doesn't look ugly when creating the cache
pub struct LumpData {
    index: usize,
    data:  Vec<u8> 
}