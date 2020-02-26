pub mod map;
pub mod thing;

pub struct DoomPoint {
    x_position: i16,
    y_position: i16
}

pub enum DoomDirection {
    East, 
    Northeast,
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast
}

#[cfg(test)]
mod tests {
    use wad::*;
    use doom::map::*;

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