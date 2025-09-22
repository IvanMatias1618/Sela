mod level;
mod settings;
mod sprites;
mod support;

use level::Level;
use macroquad::prelude::*;

use settings::debug;
pub use sprites::player::Player;
pub use sprites::sprite::*;
pub use sprites::tile::Tile;
pub use support::{FUNC_MAP, import_folder};

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
