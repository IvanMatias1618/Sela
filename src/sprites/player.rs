use crate::Sprites;
use crate::Tile;
use crate::sprites::{Sprite, SpriteType};
use macroquad::input::*;
use macroquad::math::Vec2;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Player {
    rect: Rect,
    texture: Texture2D,
    group: SpriteType,
    dir: Vec2,
    speed: u8,
}

impl Sprite for Player {
    async fn new(rect: Rect) -> Player {
        let texture = load_texture("./assets/ella.png").await.unwrap();
        let dir = Vec2::new(0.0, 0.0);
        Player {
            rect,
            texture,
            group: SpriteType::Player,
            dir,
            speed: 4,
        }
    }

    fn draw(&self) {
        draw_texture(&self.texture, self.rect.x, self.rect.y, WHITE);
    }
}

impl Player {
    pub fn update(&mut self, obstacles: &[Tile]) {
        self.input(obstacles);
    }
    fn input(&mut self, obstacles: &[Tile]) {
        self.set_dir();
        self.safe_normalize();
        self.set_pos(obstacles);
        self.dir.x = 0.0;
        self.dir.y = 0.0;
    }

    fn safe_normalize(&mut self) {
        if self.dir.length() > 0.0 {
            self.dir = self.dir.normalize();
        } else {
            self.dir = Vec2::ZERO;
        }
    }

    fn set_pos(&mut self, obstacles: &[Tile]) {
        let x = self.rect.x;
        self.rect.x += self.dir.x * self.speed as f32;
        for obs in obstacles {
            if self.rect.overlaps(&obs.rect) {
                self.rect.x = x;
            } else {
                println!("NOPcollission");
            }
        }

        let y = self.rect.y;
        self.rect.y += self.dir.y * self.speed as f32;
        for obs in obstacles {
            if self.rect.overlaps(&obs.rect) {
                self.rect.y = y;
            }
        }
    }

    fn set_dir(&mut self) {
        if is_key_down(KeyCode::A) {
            self.dir.x = -1.0;
            println!("A");
        } else if is_key_down(KeyCode::D) {
            self.dir.x = 1.0;
            println!("D");
        }

        if is_key_down(KeyCode::W) {
            self.dir.y = -1.0;
            println!("W");
        } else if is_key_down(KeyCode::S) {
            self.dir.y = 1.0;
            println!("S");
        }
    }

    fn collidex(&mut self, dir: f32, obstacles: &[Tile]) -> bool {
        let x = self.rect.x;
        self.rect.x += dir * self.speed as f32;
        for obs in obstacles {
            if self.rect.overlaps(&obs.rect) {
                return true;
            }
        }
        self.rect.x = x;

        return false;
    }

    fn collidey(&mut self, dir: f32, obstacles: &[Tile]) -> bool {
        let y = self.rect.y;
        self.rect.y += dir * self.speed as f32;
        for obs in obstacles {
            if self.rect.overlaps(&obs.rect) {
                return true;
            }
        }
        self.rect.y = y;
        return false;
    }
}
