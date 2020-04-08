#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate num_derive;
extern crate bitflags;

pub mod doom;
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
        assert_eq!(w.lumps().len(), 152);

        // the first lump in GOETIA1 is WIMAP0 (an intermission screen background)
        let wimap_entry = w.get_at_index(0).unwrap();
        assert_eq!(wimap_entry.lump().name(), "WIMAP0");

        let wimap_size = 66888;
        // verify length of data
        assert_eq!(wimap_entry.lump().data().len(), wimap_size);
    }

    #[test]
    fn read_q1_lump() {
        let w: Wad = Wad::from_path("./METAL.WAD");

        // METAL.WAD has 99 lumps in it
        assert_eq!(w.lumps().len(), 99);

        // the first lump in METAL.WAD is PALETTE
        let palette_entry = w.get_at_index(0).unwrap();
        let palette = palette_entry.lump();

        assert_eq!(palette.name(), "PALETTE");

        // entry type is palette
        assert_eq!(palette.data().data_type(), LumpDataType::Palette);
        // it has no compression
        assert_eq!(palette.data().compression_type(), CompressionType::None);

        let palette_size = 768;
        // because it has no compression, the WAD size should be equal to the size in memory
        assert_eq!(palette.data().len(), palette_size);
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
        assert_eq!(e1m1_entry.lump().name(), "E1M1");

        let e1m1_size = 0;
        assert_eq!(e1m1_entry.lump().data().len(), e1m1_size);
    }
}
