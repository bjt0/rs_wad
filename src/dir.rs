use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use utils;
use wad::{CompressionType, EntryType, Header, WadType};

pub struct Directory {
    lumps: Vec<Lump>,
    cache: Vec<LumpData>,
}

impl Directory {
    pub fn get_at_index(&self, index: usize) -> Option<&Lump> {
        self.lumps.get(index)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Lump> {
        self.lumps.iter().find(|lump| lump.name() == name)
    }

    pub fn get_data_at_index(&self, index: usize) -> Option<&LumpData> {
        self.cache.get(index)
    }

    pub fn num_lumps(&self) -> usize {
        self.lumps.len()
    }

    pub fn from_file(mut file: &File, header: &Header) -> Directory {
        file.seek(SeekFrom::Start(header.dir_offset() as u64))
            .unwrap_or_else(|e| panic!("Unable to seek to the start of the directory. ({})", e));

        let mut results: Vec<Lump> = Vec::new();

        if header.wad_type() == WadType::IWAD || header.wad_type() == WadType::PWAD {
            for index in 0..header.num_lumps() {
                let mut entry_raw: [u8; 16] = [0; 16];

                file.read_exact(&mut entry_raw)
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
                    lump_name_str.trim_end_matches(char::from(0)).to_string();

                let lump = Lump {
                    name: trimmed_lump_name_str,
                    offset: dir_offset,
                    index,
                    wad_size: lump_size,
                    mem_size: lump_size,
                    entry_type: EntryType::Doom,
                    compression: CompressionType::None,
                };
                results.push(lump);
            }
        } else if header.wad_type() == WadType::WAD2 {
            for index in 0..header.num_lumps() {
                let mut entry_raw: [u8; 32] = [0; 32];

                file.read_exact(&mut entry_raw)
                    .unwrap_or_else(|e| panic!("Error when reading WAD2 lump {}: {}", index, e));

                // pointer to the start of the lump's data
                let dir_offset: usize = utils::u8ref_to_u32(&entry_raw[0..4]) as usize;
                // the size of the lump in the WAD file (bytes)
                let wad_size: usize = utils::u8ref_to_u32(&entry_raw[4..8]) as usize;
                // the memory size of the lump (bytes)
                let mem_size: usize = utils::u8ref_to_u32(&entry_raw[8..12]) as usize;

                // entry type
                let lump_type_raw: char = char::from(entry_raw[12]);
                let lump_type: EntryType = match lump_type_raw {
                    '@' => EntryType::Palette,
                    'B' => EntryType::StatusBar,
                    'D' => EntryType::Texture,
                    'E' => EntryType::ConsolePic,
                    _ => panic!("Could not determine the entry type of WAD2 lump {}", index),
                };

                // compression type
                let compression_type_raw: u8 = u8::from(entry_raw[13]);
                let compression_type: CompressionType = match compression_type_raw {
                    _ => CompressionType::None,
                };

                // bytes 14 and 15 aren't used for anything
                // push all chars until we run into the null terminator
                let mut lump_name: String = String::from("");
                for name_char_index in 16..32 {
                    let current: char = char::from(entry_raw[name_char_index]);

                    if current == '\0' {
                        break;
                    }

                    lump_name.push(current);
                }

                let lump = Lump {
                    name: lump_name,
                    offset: dir_offset,
                    index,
                    wad_size,
                    mem_size,
                    entry_type: lump_type,
                    compression: compression_type,
                };
                results.push(lump);
            }
        }

        // borrow results to load data
        let mut cache: Vec<LumpData> = Vec::new();

        for lump in &results {
            file.seek(SeekFrom::Start(lump.wad_size as u64))
                .unwrap_or_else(|e| {
                    panic!(
                        "Unable to seek to the location of lump {}. Reason: {}",
                        lump.name, e
                    )
                });

            let mut raw_data = vec![0; lump.wad_size as usize];

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

        return Directory {
            lumps: results,
            cache,
        };
    }
}

pub struct Lump {
    name: String,
    offset: usize,
    index: usize,

    wad_size: usize,
    mem_size: usize,

    entry_type: EntryType,
    compression: CompressionType,
}

impl Lump {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn wad_size(&self) -> usize {
        self.wad_size
    }

    pub fn mem_size(&self) -> usize {
        self.mem_size
    }

    pub fn entry_type(&self) -> EntryType {
        self.entry_type.clone()
    }

    pub fn compression_type(&self) -> CompressionType {
        self.compression.clone()
    }
}

// basically just a wrapper for a u8 vec so that it doesn't look ugly when creating the cache
pub struct LumpData {
    _index: usize,
    data: Vec<u8>,
}

impl LumpData {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
