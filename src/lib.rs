pub mod dir;
pub mod doom;
pub mod utils;
pub mod wad;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use doom::*;
    use wad::*;

    #[test]
    fn read_header() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");

        // GOETIA1.wad is a PWAD
        let pwad_check: bool = match w.get_header().wad_type() {
            WadType::PWAD => true,
            _ => false,
        };

        assert!(pwad_check);

        // GOETIA1.wad has 152 lumps in it
        assert_eq!(w.get_header().num_lumps(), 152);
    }

    #[test]
    fn read_lump() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");
        // GOETIA1.wad has 152 lumps in it
        assert_eq!(w.num_lumps(), 152);

        // the first lump in GOETIA1 is WIMAP0 (an intermission screen background)
        let wimap_entry = w.get_at_index(0).unwrap();
        let wimap = wimap_entry.lump_info();

        assert_eq!(wimap.name(), "WIMAP0");

        // verify length of data
        let wimap_data = wimap_entry.lump_data();
        assert_eq!(wimap.wad_size(), wimap_data.len());
    }

    #[test]
    fn read_q1_lump() {
        let w: Wad = Wad::from_path("./METAL.WAD");

        // METAL.WAD has 99 lumps in it
        assert_eq!(w.num_lumps(), 99);

        // the first lump in METAL.WAD is PALETTE
        let palette_entry = w.get_at_index(0).unwrap();
        let palette = palette_entry.lump_info();

        assert_eq!(palette.name(), "PALETTE");

        // entry type is palette
        assert_eq!(palette.entry_type(), EntryType::Palette);
        // it has no compression
        assert_eq!(palette.compression_type(), CompressionType::None);

        // because it has no compression, the WAD size should be equal to the size in memory
        assert_eq!(palette.wad_size(), palette.mem_size());
    }

    #[test]
    fn read_lump_by_name() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");

        // this one should fail because GOETIA1.wad is a Doom 1 WAD
        // Doom 1 WADs have map name convention EXMX
        // Doom 2 WADs have map name convention MAPXX
        let op = w.get_by_name("MAP01");
        assert!(op.is_none());

        let pass = w.get_by_name("E1M1");
        assert!(pass.is_some());

        // check name and size
        let e1m1_entry = pass.unwrap();
        let e1m1 = e1m1_entry.lump_info();
        assert_eq!(e1m1.name(), "E1M1");

        let e1m1_data = e1m1_entry.lump_data();
        assert_eq!(e1m1.wad_size(), e1m1_data.len());
    }

    #[test]
    fn verify_valid_doom_map() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");

        let pass = w.get_by_name("E1M1");
        assert!(pass.is_some());

        let e1m1_entry = pass.unwrap();
        assert!(is_valid_map(e1m1_entry.clone()));
    }

    #[test]
    fn verify_missing_optional() {
        let broken: Wad = Wad::from_path("./GOETIA1-BROKEN.wad");
        // in GOETIA1-BROKEN.wad, E1M9 is deliberately missing it's REJECT table
        let missing_e1m9 = broken.get_by_name("E1M9");
        assert!(missing_e1m9.is_some());

        let e1m9_entry = missing_e1m9.unwrap();
        assert!(is_valid_map(e1m9_entry.clone()));
    }

    #[test]
    fn detect_broken_doom_map() {
        let broken: Wad = Wad::from_path("./GOETIA1-BROKEN.wad");
        // in GOETIA1-BROKEN.wad, E1M3 is deliberately broken and doesn't include the SIDEDEFS and SECTORS lumps
        let broken_e1m3 = broken.get_by_name("E1M3");
        assert!(broken_e1m3.is_some());

        let e1m3_entry = broken_e1m3.unwrap();
        assert!(!is_valid_map(e1m3_entry.clone()));
    }
}
