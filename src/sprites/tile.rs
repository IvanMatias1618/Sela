use crate::settings::settings;
use crate::sprites::SpriteType;
use macroquad::math::Rect;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Tile {
    texture: Option<Texture2D>,
    group: SpriteType,
    pub hitbox: Rect,
    pub rect: Rect,
}

impl Tile {
    pub async fn new(group: SpriteType, surface: Vec2, texture: Option<Texture2D>) -> Tile {
        let rect = Rect::new(
            surface.x,
            surface.y,
            settings::TILESIZE as f32,
            settings::TILESIZE as f32,
        );
        Tile {
            texture,
            group,
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
        let texture = &self.texture;
        match self.group {
            SpriteType::Obstacle => match texture {
                Some(tex) => {
                    draw_texture(&tex, pos.0, pos.1, WHITE);
                }
                None => (),
            },
            SpriteType::Enemy => {
                draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE)
            }
            _ => draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK),
        }
    }
}
