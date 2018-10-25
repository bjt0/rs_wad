use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use dir::{Directory, Lump};
use utils;

pub enum WadType {
    IWAD,
    PWAD,
    WAD2,
}

pub struct Header {
    wad_type: WadType,
    num_lumps: usize,
    dir_offset: usize,
}

impl Header {
    pub fn from_file(mut file: &File) -> Header {
        // load the first 12 bytes of the WAD file, this is the header
        match file.seek(SeekFrom::Start(0)) {
            Ok(_) => {}
            Err(why) => panic!(
                "Unable to seek to the start of the file. ({})",
                why.description()
            ),
        };

        let mut header_raw: [u8; 12] = [0; 12];
        match file.read(&mut header_raw) {
            Ok(_) => {}
            Err(why) => panic!("Error when reading WAD header: {}", why.description()),
        }

        // the wad type is ASCII but just use utf8 converter since it works fine
        let wad_type_str: String = match String::from_utf8(header_raw[0..4].to_vec()) {
            Ok(wtype) => wtype,
            Err(not_ascii) => panic!(
                "Could not read WAD type from header. Is this a WAD file? ({})",
                not_ascii.description()
            ),
        };

        let wad_type: WadType = match wad_type_str.as_str() {
            "IWAD" => WadType::IWAD, 
            "PWAD" => WadType::PWAD,
            "WAD2" => WadType::WAD2, // for Quake 
            _      => panic!("Could not convert the first 4 bytes of the provided file into a WAD type. Are you sure this is a WAD file?")
        };

        // total entries in the directory
        let num_lumps: usize = utils::u8ref_to_u32(&header_raw[4..8]) as usize;
        // where to seek to for the start of the directory
        let dir_offset: usize = utils::u8ref_to_u32(&header_raw[8..12]) as usize;

        Header {
            wad_type,
            num_lumps,
            dir_offset,
        }
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

pub struct Wad {
    header: Header,
    directory: Directory,
}

impl Wad {
    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn num_lumps(&self) -> usize {
        self.directory.num_lumps()
    }

    pub fn get_at_index(&self, index: usize) -> Option<&Lump> {
        self.directory.get_at_index(index)
    }

    pub fn get_by_name(&self, name: String) -> Option<&Lump> {
        self.directory.get_by_name(name)
    }

    pub fn get_data_at_index(&self, index: usize) -> Option<&Vec<u8>> {
        self.directory.get_data_at_index(index)
    }

    pub fn from_path(path: &str) -> Wad {
        let path = Path::new(path);

        if !path.exists() {
            panic!("WAD file {} not found!", path.display());
        }

        let wad_file = match File::open(path) {
            Ok(file) => file,
            Err(why) => panic!("Unable to open WAD {}", why.description()),
        };

        Wad::from_file(&wad_file)
    }

    pub fn from_file(file: &File) -> Wad {
        let header = Header::from_file(file);
        let directory = Directory::from_file(file, &header);

        Wad { header, directory }
    }
}
