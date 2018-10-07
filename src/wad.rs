use std::fmt;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};

use utils;

pub enum WadType {
    IWAD, PWAD
}

pub struct Header {
    wad_type: WadType,
    num_lumps:  usize,
    dir_offset: usize
}

impl Header { 
    pub fn from_file(mut file: &File) -> Header {
        // load the first 12 bytes of the WAD file, this is the header
        match file.seek(SeekFrom::Start(0)) {
            Ok(_)    => { }
            Err(why) => panic!("Unable to seek to the start of the file. ({})", why.description()) 
        };

        let mut header_raw: [u8; 12] = [0; 12];
        match file.read(&mut header_raw) {
            Ok(_)    => { },
            Err(why) => panic!("Error when reading WAD header: {}", why.description())
        }

        // the wad type is ASCII but just use utf8 converter since it works fine
        let wad_type_str: String = match String::from_utf8(header_raw[0..4].to_vec()) {
            Ok(wtype)      => wtype,
            Err(not_ascii) => panic!("Could not read WAD type from header. Is this a WAD file? ({})", not_ascii.description())
        };

        let wad_type: WadType = match wad_type_str.as_str() {
            "IWAD" => WadType::IWAD, 
            "PWAD" => WadType::PWAD,
            _      => panic!("Could not convert the first 4 bytes of the provided file into a WAD type. Are you sure this is a WAD file?")
        };

        // total entries in the directory
        let num_lumps: usize = utils::u8ref_to_u32(&header_raw[4..8]) as usize;
        // where to seek to for the start of the directory 
        let dir_offset: usize = utils::u8ref_to_u32(&header_raw[8..12]) as usize;

        Header { wad_type, num_lumps, dir_offset }
    }

    pub fn wad_type(&self) -> &WadType {
        &self.wad_type
    }

    pub fn num_lumps(&self) -> usize {
        self.num_lumps
    }

    pub fn dir_offset(&self) -> usize {
        self.dir_offset
    }
}

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

pub struct Wad {
    header: Header,
    directory: Directory
}

impl Wad {
    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn get_directory(&self) -> &Directory {
        &self.directory
    }

    pub fn from_path(path: &str) -> Wad {
        let path = Path::new(path);

        if !path.exists() {
            panic!("WAD file {} not found!", path.display());
        }

        let wad_file = match File::open(path) {
            Ok(file) => file, 
            Err(why) => panic!("Unable to open WAD {}", why.description())
        };

        Wad::from_file(&wad_file)
    }

    pub fn from_file(file: &File) -> Wad {
        let header    = Header::from_file(file);
        let directory = Directory::from_file(file, &header);

        Wad { header, directory }
    }
}