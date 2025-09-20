use crate::sprites::{Sprite, SpriteType};
use macroquad::math::Rect;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Tile {
    pub rect: Rect,
    texture: Texture2D,
    group: SpriteType,
}

impl Sprite for Tile {
    async fn new(rect: Rect) -> Tile {
        let texture = load_texture("./assets/rock.png").await.unwrap();
        Tile {
            rect,
            texture,
            group: SpriteType::Obstacle,
        }
    }

    fn draw(&self) {
        match self.group {
            SpriteType::Obstacle => draw_texture(&self.texture, self.rect.x, self.rect.y, WHITE),
            SpriteType::Enemy => {
                draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE)
            }
            _ => draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK),
        }
    }
}
