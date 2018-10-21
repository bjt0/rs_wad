pub mod wad;
pub mod dir;
pub mod utils;

#[cfg(test)]
mod tests {
    use wad::*;

    #[test]
    fn read_header() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");

        // GOETIA1.wad is a PWAD 
        let pwad_check: bool = match w.get_header().wad_type() {
            WadType::PWAD => true,
            _             => false
        };

        assert_eq!(pwad_check, true);

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
        assert_eq!(wimap.get_size(), wimap_data.len());
    }

    #[test]
    fn read_lump_by_name() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");

        // this one should fail because GOETIA1.wad is a Doom 1 WAD 
        let op = w.get_by_name(String::from("MAP01"));
        assert_eq!(op.is_none(), true);

        let pass = w.get_by_name(String::from("E1M1"));
        assert_eq!(pass.is_some(), true);

        // check name and size 
        let e1m1 = pass.unwrap();
        assert_eq!(e1m1.get_name(), "E1M1");
        
        let e1m1_data = w.get_data_at_index(e1m1.get_index()).unwrap();
        assert_eq!(e1m1.get_size(), e1m1_data.len());
    }
}