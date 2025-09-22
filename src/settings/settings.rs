use macroquad::prelude::*;

//pub const WIDTH: u16 = 1920;
//pub const HEIGHT: u16 = 780;
//pub const FPS: u8 = 60;
pub const TILESIZE: f64 = 128.0;

pub struct Map {
    pub map: Vec<Vec<&'static str>>,
}

impl Map {
    pub fn mapa1() -> Map {
        Map {
            map: vec![
                vec!["x", "x", "x", "x", "x", "x", "x", "x", "x"],
                vec!["x", "o", "o", "o", "o", "o", "o", "o", "x"],
                vec!["x", "o", "o", "o", "o", "o", "o", "o", "x"],
                vec!["x", "x", "x", "x", "o", "o", "o", "o", "o"],
                vec!["x", "o", "o", "x", "o", "p", "o", "o", "o"],
                vec!["x", "o", "x", "o", "o", "o", "o", "o", "o"],
                vec!["x", "o", "o", "o", "o", "o", "o", "o", "x"],
                vec!["x", "o", "o", "o", "o", "o", "o", "o", "x"],
                vec!["x", "x", "x", "x", "x", "x", "x", "x", "x"],
            ],
        }
    }
}
