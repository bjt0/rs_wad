pub mod wad;

#[cfg(test)]
mod tests {
    use wad::*;

    #[test]
    fn read_header() {
        let w: Wad = Wad::from_path("./GOETIA1.wad");
    }
}