use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use dir::{Directory, Lump, LumpData};
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
pub enum EntryType {
    Doom,
    Palette,
    StatusBar,
    Texture,
    ConsolePic,
}

pub struct Header {
    wad_type: WadType,
    num_lumps: usize,
    dir_offset: usize,
}

impl Header {
    pub fn from_file(mut file: &File) -> Header {
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

        Header {
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

    pub fn get_at_index(&self, index: usize) -> Option<Entry> {
        Entry::from_index(self, index)
    }

    pub fn get_by_name(&self, name: &str) -> Option<Entry> {
        Entry::by_name(self, name)
    }

    pub fn from_path(path: &str) -> Wad {
        let path = Path::new(path);
        let wad_file = File::open(path).unwrap_or_else(|e| panic!("Unable to open WAD {}", e));

        Wad::from_file(&wad_file)
    }

    pub fn from_file(file: &File) -> Wad {
        let header = Header::from_file(file);
        let directory = Directory::from_file(file, &header);

        Wad { header, directory }
    }
}

// represents a return type for retrieving lumps from wad file
pub struct Entry<'a> {
    wad: &'a Wad,
    lump: &'a Lump,
    data: &'a LumpData,
}

impl<'a> Entry<'a> {
    pub fn owner(&self) -> &'a Wad {
        self.wad
    }

    pub fn lump_info(&self) -> &'a Lump {
        self.lump
    }

    pub fn lump_data(&self) -> &'a LumpData {
        self.data
    }

    pub fn from_index(wad: &'a Wad, index: usize) -> Option<Entry<'a>> {
        let lump_ref = wad.directory.get_at_index(index);
        let data_ref = wad.directory.get_data_at_index(index);

        if lump_ref.is_none() || data_ref.is_none() {
            return None;
        }

        let ld: &Lump = lump_ref.unwrap();
        let dt: &LumpData = data_ref.unwrap();

        let result = Entry {
            wad,
            lump: ld,
            data: dt,
        };
        Some(result)
    }

    pub fn by_name(wad: &'a Wad, name: &str) -> Option<Entry<'a>> {
        let lump_ref = wad.directory.get_by_name(&name);

        if lump_ref.is_none() {
            return None;
        }

        let ld: &Lump = lump_ref.unwrap();
        // we need the index of the lump with the given name to get the data ref
        let data_ref = wad.directory.get_data_at_index(ld.index());

        if data_ref.is_none() {
            return None;
        }

        let dt: &LumpData = data_ref.unwrap();

        let result = Entry {
            wad,
            lump: ld,
            data: dt,
        };
        Some(result)
    }
}
