use std::fs::File;
use std::path::Path;
use std::error::Error;

pub enum WadType {
    IWAD, PWAD
}

pub struct Header {

}

pub struct Directory {

}

pub struct Wad {
    header: Header,
    directory: Directory
}

impl Wad {
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

    pub fn from_file(mut file: &File) -> Wad {
        let header    = Header { };
        let directory = Directory { };

        Wad { header, directory }
    }
}