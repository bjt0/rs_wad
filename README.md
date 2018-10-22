# Introduction
A Rust library for loading .WAD files (Doom, Quake, etc)

## Features
* Load a WAD archive from a filepath or Rust File 
* Access the archive entries by index or name 
* All raw data in the archive is loaded internally

# Quick Start
## Load WAD archive from path
    let w: Wad = Wad::from_path("./wads/MYWAD.wad");
    
 ## Get lump entry by index 
    let first_entry = w.get_at_index(0).unwrap();
    
## Get lump entry by name 
    let map01_marker = w.get_by_name(String::from("MAP01"));
    
Now you can retrieve the data for the lump you've got
## Get lump name 
    let first_entry = w.get_at_index(0).unwrap();
    println!("{}", first_entry.get_name());
    
## Retrieve the data for the lump 
    let first_entry = w.get_at_index(0).unwrap();
    let first_entry_data = w.get_data_at_index(0).unwrap();
    
first_entry_data is a raw Vec<u8> with all data for the lump at index 0
