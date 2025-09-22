use crate::Player;
use crate::Tile;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub enum Sprites {
    Tile(Tile),
    Player(Player),
}

impl Sprites {
    pub fn update(&mut self, obstacles: &[Sprites]) {
        match self {
            Sprites::Player(player) => player.update(obstacles),
            _ => (),
        }
    }
    pub fn draw(&self, pos: (f32, f32)) {
        match self {
            Sprites::Tile(tile) => tile.draw(pos),
            Sprites::Player(player) => {
                //let adjust_pos = (pos.0 - 12.0, pos.1 - 140.0);
                player.draw(pos);
            }
        }
    }
    pub fn y(&self) -> f32 {
        match self {
            Sprites::Tile(tile) => tile.rect.y,
            Sprites::Player(player) => player.rect.y,
        }
    }

    // Si quieres ordenar también por x como desempate:
    pub fn x(&self) -> f32 {
        match self {
            Sprites::Tile(tile) => tile.rect.x,
            Sprites::Player(player) => player.rect.x,
        }
    }
    pub fn w(&self) -> f32 {
        match self {
            Sprites::Tile(tile) => tile.rect.w,
            Sprites::Player(player) => player.rect.h,
        }
    }

    // Si quieres ordenar también por x como desempate:
    pub fn h(&self) -> f32 {
        match self {
            Sprites::Tile(tile) => tile.rect.h,
            Sprites::Player(player) => player.rect.h,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpriteType {
    Enemy,
    Player,
    Obstacle,
}
