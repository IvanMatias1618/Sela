mod level;
mod settings;
mod sprites;

use level::Level;
use macroquad::prelude::*;
use settings::debug;
pub use sprites::player::Player;
pub use sprites::sprite::*;
pub use sprites::tile::Tile;

#[macroquad::main("Sela")]
async fn main() {
    debug::debug(&"EMpezamos");
    let mut level = Level::new().await;
    loop {
        clear_background(BLACK);

        level.run().await;
        next_frame().await
    }
}
