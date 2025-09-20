use crate::Player;
use crate::Tile;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub enum Sprites {
    Tile(Tile),
    Player(Player),
}

impl Sprites {
    pub fn draw(&self) {
        match self {
            Sprites::Tile(tile) => tile.draw(),
            Sprites::Player(player) => player.draw(),
            // Sprites::Obstacle(obs) => obs.draw(),
        }
    }
}

pub trait Sprite {
    async fn new(rect: Rect) -> Self;
    fn draw(&self);
    //fn set_dir(dir:Vec2<(f32,f32)>);
    //fn set_state();
}

#[derive(Debug, Clone, Copy)]
pub enum SpriteType {
    Enemy,
    Player,
    Obstacle,
}
