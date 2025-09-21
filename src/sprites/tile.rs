use crate::settings::settings;
use crate::sprites::SpriteType;
use macroquad::math::Rect;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Tile {
    texture: Texture2D,
    group: SpriteType,
    pub hitbox: Rect,
    pub rect: Rect,
}

impl Tile {
    pub async fn new(rect: Rect) -> Tile {
        let texture = load_texture("./assets/rock.png").await.unwrap();
        Tile {
            texture,
            group: SpriteType::Obstacle,
            hitbox: Rect::new(
                rect.x + 25.0,
                rect.y + 20.0,
                settings::TILESIZE as f32 - 50.0,
                settings::TILESIZE as f32 - 40.0,
            ),
            rect,
        }
    }

    pub fn draw(&self, pos: (f32, f32)) {
        match self.group {
            SpriteType::Obstacle => {
                draw_texture(&self.texture, pos.0, pos.1, WHITE);
            }
            SpriteType::Enemy => {
                draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE)
            }
            _ => draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK),
        }
    }
}
