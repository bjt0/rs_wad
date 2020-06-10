use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use utils;

#[derive(Copy, Clone, PartialEq)]
pub enum WadType {
    IWAD,
    PWAD,
    WAD2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CompressionType {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LumpDataType {
    Doom,
    Marker,
    Palette,
    StatusBar,
    Texture,
    ConsolePic,
}

pub struct WadHeader {
    wad_type: WadType,
    num_lumps: usize,
    dir_offset: usize,
}

impl WadHeader {
    pub fn from_file(mut file: &File) -> WadHeader {
        // load the first 12 bytes of the WAD file, this is the header
        file.seek(SeekFrom::Start(0))
            .unwrap_or_else(|e| panic!("Unable to seek to the start of the file. ({})", e));

        let mut header_raw: [u8; 12] = [0; 12];
        file.read_exact(&mut header_raw)
            .unwrap_or_else(|e| panic!("Error when reading WAD header: {}", e));

        let wad_type: WadType = match &header_raw[0..4] {
            b"IWAD" => WadType::IWAD, 
            b"PWAD" => WadType::PWAD,
            b"WAD2" => WadType::WAD2, // for Quake 
            _       => panic!("Could not convert the first 4 bytes of the provided file into a WAD type. Are you sure this is a WAD file?")
        };

        // total entries in the directory
        let num_lumps: usize = utils::u8ref_to_u32(&header_raw[4..8]) as usize;
        // where to seek to for the start of the directory
        let dir_offset: usize = utils::u8ref_to_u32(&header_raw[8..12]) as usize;

        WadHeader {
            wad_type,
            num_lumps,
            dir_offset,
        }
    }

    pub fn wad_type(&self) -> WadType {
        self.wad_type
    }

    pub fn num_lumps(&self) -> usize {
        self.num_lumps
    }

    pub fn dir_offset(&self) -> usize {
        self.dir_offset
    }
}

pub struct Wad {
    header: WadHeader,
    lumps: Vec<Lump>
}

impl Wad {
    pub fn get_header(&self) -> &WadHeader {
        &self.header
    }

    pub fn lumps(&self) -> &Vec<Lump> {
        &self.lumps
    }

    pub fn get_at_index(&self, index: usize) -> Option<Entry> {
        Entry::from_index(self, index)
    }

    pub fn get_by_name(&self, name: &str) -> Option<Entry> {
        Entry::by_name(self, name)
    }

    pub fn from_path(path: &str) -> Wad {
        let path = Path::new(path);
        let mut wad_file = File::open(path).unwrap_or_else(|e| panic!("Unable to open WAD {}", e));

        Wad::from_file(&mut wad_file)
    }

    pub fn from_file(file: &mut File) -> Wad {
        let header = WadHeader::from_file(file);

        file.seek(SeekFrom::Start(header.dir_offset() as u64))
        .unwrap_or_else(|e| panic!("Unable to seek to the start of the directory. ({})", e));

        struct WadLump {
            name: String,
            offset: usize,
            index: usize,
            wad_size: usize,
            mem_size: usize,
            data_type: LumpDataType,
            compression: CompressionType,
        }

        let mut wadlumps: Vec<WadLump> = Vec::new();

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

                wadlumps.push(WadLump {
                    name: trimmed_lump_name_str,
                    offset: dir_offset,
                    index,
                    wad_size: lump_size,
                    mem_size: lump_size,
                    data_type: LumpDataType::Doom,
                    compression: CompressionType::None,
                });
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
                let lump_type: LumpDataType = match lump_type_raw {
                    '@' => LumpDataType::Palette,
                    'B' => LumpDataType::StatusBar,
                    'D' => LumpDataType::Texture,
                    'E' => LumpDataType::ConsolePic,
                    _ => panic!("Could not determine the entry type of WAD2 lump {}", index),
                };

                // compression type
                let compression_type_raw = entry_raw[13];
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

                wadlumps.push(WadLump {
                    name: lump_name,
                    offset: dir_offset,
                    index,
                    wad_size,
                    mem_size,
                    data_type: lump_type,
                    compression: compression_type,
                });
            }
        }

        let mut lumps = Vec::new();

        // now create lumps from the WAD lump details
        for wadlump in &wadlumps {
            file.seek(SeekFrom::Start(wadlump.offset as u64))
                .unwrap_or_else(|e| {
                    panic!(
                        "Unable to seek to the location of lump {}. Reason: {}",
                        wadlump.name, e
                    )
                });

            let mut raw_data = vec![0; wadlump.wad_size as usize];

            file.read_exact(&mut raw_data).unwrap_or_else(|e| {
                panic!(
                    "Error when reading data for lump {}. Reason: {}",
                    wadlump.name, e
                )
            });

            let is_marker = raw_data.is_empty();

            lumps.push(Lump {
                name: wadlump.name.clone(),
                data: LumpData {
                    bytes: raw_data, 
                    data_type: if is_marker { LumpDataType::Marker } else { wadlump.data_type },
                    compression: wadlump.compression
                }
            });
        }

        Wad { header, lumps }
    }
}

pub struct Lump {
    name: String,
    data: LumpData,
}

impl Lump {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn data(&self) -> &LumpData {
        &self.data
    }
}

// basically just a wrapper for a u8 vec so that it doesn't look ugly when creating the cache
pub struct LumpData {
    bytes: Vec<u8>,
    data_type: LumpDataType,
    compression: CompressionType,
}

impl LumpData {
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn data_type(&self) -> LumpDataType {
        self.data_type
    }

    pub fn compression_type(&self) -> CompressionType {
        self.compression
    }
}

// represents a return type for retrieving lumps from wad file
#[derive(Copy, Clone)]
pub struct Entry<'a> {
    wad: &'a Wad,
    lump: &'a Lump,
}

impl<'a> Entry<'a> {
    pub fn owner(&self) -> &'a Wad {
        self.wad
    }

    pub fn lump(&self) -> &'a Lump {
        self.lump
    }

    pub fn from_index(wad: &'a Wad, index: usize) -> Option<Entry<'a>> {
        if index > wad.lumps.len() {
            return None;
        }
        
        let result = Entry {
            wad,
            lump: &wad.lumps[index]
        };

        Some(result)
    }

    pub fn by_name(wad: &'a Wad, name: &str) -> Option<Entry<'a>> {
        let lump = wad.lumps.iter().find(|lump| lump.name() == name)?;

        Some(Entry{
            wad,
            lump,
        })
    }
}

impl<'a> Iterator for Entry<'a> {
    type Item = Entry<'a>;

    /// Returns the entry at the next index
    fn next(&mut self) -> Option<Entry<'a>> {
        let next_entry_index = match self.wad.lumps.iter().position(|l| std::ptr::eq(l, self.lump())) {
            None => return None,
            Some(entry_index) => entry_index + 1
        };

        if next_entry_index >= self.wad.lumps().len() { return None; }
        let next_entry = self.wad.get_at_index(next_entry_index).unwrap();
        self.lump = next_entry.lump();

        Some(Entry {
            wad: self.wad, 
            lump: next_entry.lump()
        })
    }
}
