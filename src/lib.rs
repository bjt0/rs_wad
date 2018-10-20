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
    fn read_directory() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");
        
        // GOETIA1.wad has 152 lumps in it
        assert_eq!(w.num_lumps(), 152);

        // the first lump in GOETIA1 is WIMAP0 (an intermission screen background)
        let wimap = w.get_at_index(0);
        assert_eq!(wimap.get_name(), "WIMAP0");

        // verify length of data
        let wimap_data = w.get_data_at_index(0);
        assert_eq!(wimap.get_size(), wimap_data.len());
    }
}