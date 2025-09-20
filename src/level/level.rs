use crate::Player;
use crate::Sprite;
use crate::SpriteType;
use crate::Tile;
use crate::settings::{Map, settings};
use crate::sprites::Sprites;
use macroquad::input::*;
use macroquad::math::Rect;
use macroquad::prelude::*;
use macroquad::window::{screen_height, screen_width};

pub struct Level {
    map: Map,
    sprites: Vec<Tile>,
    player: Option<Player>,
    //for the visible_sprites groups should we use Box and implement a Trait Sprite for tile, obstacle and player,
    // is there a difference between good and bad?
}

impl Level {
    pub async fn new() -> Level {
        let mut level = Level {
            map: Map::mapa1(),
            sprites: Vec::new(),
            player: None,
        };
        level.draw_map().await;
        return level;
    }
    pub fn run(&mut self) {
        if is_key_down(KeyCode::K) {
            panic!("Closing the game");
        }
        println!("level");
        self.sprites.iter().for_each(|sprite| sprite.draw());
        if let Some(player) = self.player.as_mut() {
            player.draw();
            player.update(&self.sprites);
        } else {
            println!("Game Over");
        }
    }

    async fn draw_map(&mut self) {
        let size = (settings::TILESIZE as f32, settings::TILESIZE as f32);
        let mut origin_x: f32 = 0.0;
        let mut origin_y: f32 = 0.0;

        for row in &self.map.map {
            for tile in row {
                match *tile {
                    "x" => {
                        let rect = Rect::new(origin_x, origin_y, size.0, size.1);
                        let tile = Tile::new(rect).await;
                        self.sprites.push(tile);
                    }
                    "o" => draw_rectangle(origin_x, origin_y, size.0, size.1, BLACK),
                    "p" => {
                        let rect = Rect::new(origin_x, origin_y, size.0, size.1);
                        let mut player = Player::new(rect).await;
                        self.player = Some(player);
                    }
                    _ => (),
                }
                origin_x += size.0;
            }
            origin_x = 0.0;
            origin_y += size.1;
        }
    }
}

pub struct YsortCamera {
    pub sprites: Vec<Sprites>,
}

impl YsortCamera {
    pub fn new() -> YsortCamera {
        YsortCamera {
            sprites: Vec::new(),
        }
    }
}
