use crate::Player;
use crate::SpriteType;
use crate::Tile;
use crate::settings::{Map, settings};
use crate::sprites::Sprites;
use macroquad::input::*;
use macroquad::math::Rect;
use macroquad::math::Vec2;
use macroquad::prelude::*;
use macroquad::window::{screen_height, screen_width};

pub struct Level {
    map: Map,
    camera: YsortCamera,
    sprites: Vec<Sprites>,
}

impl Level {
    pub async fn new() -> Level {
        let mut level = Level {
            map: Map::mapa1(),
            camera: YsortCamera::new(),
            sprites: Vec::new(),
        };
        YsortCamera::draw(&mut level).await;
        return level;
    }
    pub async fn run(&mut self) {
        if is_key_down(KeyCode::K) {
            panic!("Closing the game");
        }
        println!("level");
        self.camera.run_draw(&mut self.sprites).await;
    }
}

pub struct YsortCamera {
    offset: Vec2,
    half_h: f32,
    half_w: f32,
}

impl YsortCamera {
    pub fn new() -> YsortCamera {
        YsortCamera {
            offset: Vec2 { x: 0.0, y: 0.0 },
            half_h: screen_height() / 2.0,
            half_w: screen_width() / 2.0,
        }
    }

    pub async fn run_draw(&mut self, sprites: &mut [Sprites]) {
        if let Some(pos) = sprites.iter().find_map(|sprite| {
            if let Sprites::Player(p) = sprite {
                Some(Vec2 {
                    x: p.rect.x,
                    y: p.rect.y,
                })
            } else {
                None
            }
        }) {
            self.offset = pos;
        }

        YsortCamera::y_sort(sprites);

        let adjustment_x = 0.0;
        let adjustment_y = 200.0;
        let obstacles = sprites.to_vec();
        for sprite in sprites.iter_mut() {
            let pos = (
                sprite.x() - self.offset.x + self.half_w - sprite.w() + adjustment_x,
                sprite.y() - self.offset.y + self.half_h - sprite.h() + adjustment_y,
            );
            sprite.draw(pos);
            sprite.update(&obstacles);
        }
    }

    pub async fn draw(level: &mut Level) {
        let size = (settings::TILESIZE as f32, settings::TILESIZE as f32);
        let mut origin_x: f32 = 0.0;
        let mut origin_y: f32 = 0.0;

        for row in &level.map.map {
            for tile in row {
                match *tile {
                    "x" => {
                        let rect = Rect::new(origin_x, origin_y, size.0, size.1);
                        let tile = Tile::new(rect).await;
                        level.sprites.push(Sprites::Tile(tile));
                    }
                    "o" => draw_rectangle(origin_x, origin_y, size.0, size.1, BLACK),
                    "p" => {
                        let rect = Rect::new(origin_x, origin_y, size.0, size.1);
                        let player = Player::new(rect).await;
                        level.sprites.push(Sprites::Player(player));
                    }
                    _ => (),
                }
                origin_x += size.0;
            }
            origin_x = 0.0;
            origin_y += size.1;
        }
    }

    pub fn y_sort(sprites: &mut [Sprites]) {
        sprites.sort_by(|a, b| {
            a.y()
                .partial_cmp(&b.y())
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    a.x()
                        .partial_cmp(&b.x())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });
    }
}
