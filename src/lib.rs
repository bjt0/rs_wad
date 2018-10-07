pub mod wad;
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
        assert_eq!(w.get_directory().num_lumps(), 152);
    }
}