use crate::Sprites;
use macroquad::input::*;
use macroquad::math::Vec2;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Player {
    hitbox: Rect,
    pub rect: Rect,
    texture: Texture2D,
    dir: Vec2,
    speed: u8,
}

impl Player {
    pub async fn new(rect: Rect) -> Player {
        let texture = load_texture("./assets/ella.png").await.unwrap();
        let dir = Vec2::new(0.0, 0.0);
        let hitbox = Rect::new(rect.x + 10.0, rect.y + 10.0, rect.w - 20.0, rect.h - 20.0);
        Player {
            hitbox,
            rect,
            texture,
            dir,
            speed: 4,
        }
    }

    pub fn draw(&self, pos: (f32, f32)) {
        draw_texture(&self.texture, pos.0, pos.1, WHITE);
    }

    pub fn update(&mut self, obstacles: &[Sprites]) {
        self.input(obstacles);
    }
    fn input(&mut self, obstacles: &[Sprites]) {
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

    fn set_pos(&mut self, obstacles: &[Sprites]) {
        let x = self.hitbox.x;
        self.hitbox.x += self.dir.x * self.speed as f32;

        let x_rect = self.rect.x;
        self.rect.x += self.dir.x * self.speed as f32;

        for obs in obstacles {
            match obs {
                Sprites::Tile(tile) => {
                    if self.hitbox.overlaps(&tile.hitbox) {
                        self.hitbox.x = x;
                        self.rect.x = x_rect;
                    }
                }
                _ => (),
            }
        }

        let y = self.hitbox.y;
        self.hitbox.y += self.dir.y * self.speed as f32;

        let y_rect = self.rect.y;
        self.rect.y += self.dir.y * self.speed as f32;

        for obs in obstacles {
            match obs {
                Sprites::Tile(tile) => {
                    if self.rect.overlaps(&tile.hitbox) {
                        self.hitbox.y = y;
                        self.rect.y = y_rect;
                    }
                }
                _ => (),
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
}
