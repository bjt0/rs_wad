pub mod dir;
pub mod utils;
pub mod wad;

#[cfg(test)]
mod tests {
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
        let wimap = w.get_at_index(0).unwrap();
        assert_eq!(wimap.get_name(), "WIMAP0");

        // verify length of data
        let wimap_data = w.get_data_at_index(0).unwrap();
        assert_eq!(wimap.wad_size(), wimap_data.len());
    }

    #[test]
    fn read_q1_lump() {
        let w: Wad = Wad::from_path("./METAL.WAD");

        // GOETIA1.wad has 99 lumps in it
        assert_eq!(w.num_lumps(), 99);

        // the first lump in METAL.WAD is PALETTE 
        let palette = w.get_at_index(0).unwrap();
        assert_eq!(palette.get_name(), "PALETTE");

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
        let op = w.get_by_name("MAP01");
        assert!(op.is_none());

        let pass = w.get_by_name("E1M1");
        assert!(pass.is_some());

        // check name and size
        let e1m1 = pass.unwrap();
        assert_eq!(e1m1.get_name(), "E1M1");

        let e1m1_data = w.get_data_at_index(e1m1.get_index()).unwrap();
        assert_eq!(e1m1.wad_size(), e1m1_data.len());
    }
}
