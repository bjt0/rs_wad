# Introduction
A Rust library for loading .WAD files (Doom, Quake, etc)

## Features
* Load a WAD archive from a filepath or Rust File 
* Access the archive entries by index or name 
* All raw data in the archive is loaded internally

## Quick Start
### Load WAD file from a path and then get the first lump in the wad
    let w: Wad = Wad::from_path("./wads/MYWAD.wad");
    // this returns an option depending on whether there is a lump at index 0 
    let first_entry = w.get_at_index(0).unwrap();

#### Get a WAD entry by name or index
    // same as get_at_index method, but depends on whether a lump with the given name exists
    let e1m1_entry = w.get_by_name("E1M1").unwrap();

#### Get raw lump data 
    let e1m1_entry = w.get_by_name("E1M1").unwrap();
    let e1m1_info = e1m1_entry.lump_info(); // returns a reference to Lump struct
    let e1m1_data = e1m1_entry.lump_data(); // returns a reference to LumpData struct

    // the E1M1 lump is what is known as a marker 
    // it signifies the start of the Doom map E1M1 (Hangar in DOOM.WAD)
    // all the map data is contained in subsequent lumps, such as the THINGS lump
    // this contains all data for monster and items in the level
    assert_eq!(e1m1_info.wad_size(), 0);
    assert_eq!(e1m1_data.len(), 0);