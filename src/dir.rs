use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use utils;
use wad::Header;

pub struct Directory {
    lumps: Vec<Lump>,
    cache: Vec<LumpData>,
}

impl Directory {
    pub fn get_at_index(&self, index: usize) -> Option<&Lump> {
        if index >= self.lumps.len() {
            return None;
        }

        Some(&self.lumps[index])
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Lump> {
        self.lumps.iter().find(|lump| lump.get_name() == name)
    }

    pub fn get_data_at_index(&self, index: usize) -> Option<&Vec<u8>> {
        self.cache.get(index).map(|lump| &lump.data)
    }

    pub fn num_lumps(&self) -> usize {
        self.lumps.len()
    }

    pub fn from_file(mut file: &File, header: &Header) -> Directory {
        file.seek(SeekFrom::Start(header.dir_offset() as u64))
            .unwrap_or_else(|e| panic!("Unable to seek to the start of the directory. ({})", e));

        let mut results: Vec<Lump> = Vec::new();

        for index in 0..header.num_lumps() {
            let mut entry_raw: [u8; 16] = [0; 16];

            file.read(&mut entry_raw)
                .unwrap_or_else(|e| panic!("Error when reading lump {}: {}", index, e));

            // pointer to the start of the lump's data
            let dir_offset: usize = utils::u8ref_to_u32(&entry_raw[0..4]) as usize;
            // the size of the lump in bytes
            let lump_size: usize = utils::u8ref_to_u32(&entry_raw[4..8]) as usize;

            // this is technically just ASCII, so I should probably change it but it works for now
            let lump_name_str: String = String::from_utf8(entry_raw[8..16].to_vec())
                .unwrap_or_else(|e| panic!("Could not read entry name for {}: {}", index, e));

            // remove trailing unicode NULLs from the conversion
            let trimmed_lump_name_str: String =
                lump_name_str.trim_right_matches(char::from(0)).to_string();

            let lump = Lump {
                name: trimmed_lump_name_str,
                index,
                size: lump_size,
                location: dir_offset,
            };
            results.push(lump);
        }

        // borrow results to load data
        let mut cache: Vec<LumpData> = Vec::new();

        for lump in &results {
            file.seek(SeekFrom::Start(lump.location as u64))
                .unwrap_or_else(|e| {
                    panic!(
                        "Unable to seek to the location of lump {}. Reason: {}",
                        lump.name, e
                    )
                });

            let mut raw_data = vec![0; lump.size];

            file.read_exact(&mut raw_data).unwrap_or_else(|e| {
                panic!(
                    "Error when reading data for lump {}. Reason: {}",
                    lump.name, e
                )
            });

            let lump_data = LumpData {
                _index: lump.index,
                data: raw_data,
            };
            cache.push(lump_data);
        }

        Directory {
            lumps: results,
            cache,
        }
    }
}

// TODO: make the concept of a "lump" into a trait and use it as a generic store for both WAD1 and WAD2 lumps
// WAD2 lumps have a different format including compression type, etc
pub struct Lump {
    name: String,
    index: usize,
    size: usize,
    location: usize,
}

impl Lump {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

impl fmt::Display for Lump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: index: {}, size: {}, location: {}",
            self.name, self.index, self.size, self.location
        )
    }
}

// basically just a wrapper for a u8 vec so that it doesn't look ugly when creating the cache
pub struct LumpData {
    _index: usize,
    data: Vec<u8>,
}
