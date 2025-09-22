use crate::Player;
use crate::SpriteType;
use crate::Tile;
use crate::settings::settings;
use crate::sprites::Sprites;
use crate::{FUNC_MAP, import_folder};

use macroquad::input::*;
use macroquad::math::Rect;
use macroquad::math::Vec2;
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use macroquad::window::{screen_height, screen_width};

pub fn pick_random_path(images: &Vec<String>) -> Option<&String> {
    images.choose()
}

pub struct Level {
    camera: YsortCamera,
    sprites: Vec<Sprites>,
    background: Texture2D,
    background_rect: Rect,
}

impl Level {
    pub async fn new() -> Level {
        let background = load_texture("./assets/mapa0.png").await.unwrap();
        let background_rect = Rect::new(0.0, 0.0, background.width(), background.height());
        let mut level = Level {
            camera: YsortCamera::new(),
            sprites: Vec::new(),
            background,
            background_rect,
        };
        //YsortCamera::draw(&mut level).await;
        level.create_level().await;
        return level;
    }

    async fn create_map(&mut self) {
        let size = (settings::TILESIZE as f32, settings::TILESIZE as f32);
        //let mut origin_x: f32 = 0.0;
        //let mut origin_y: f32 = 0.0;
        //new
        for (style, layout) in FUNC_MAP.iter() {
            let mut origin_x: f32 = 0.0;
            let mut origin_y: f32 = 0.0;
            if let Some(_) = FUNC_MAP.get(*style) {
                if *style == "boundary" {
                    let map = layout("./assets/map0/mapa0_colisiones.csv");

                    for row in map.iter() {
                        for n in row.iter() {
                            match *n {
                                7 => {
                                    let surf = Vec2::new(origin_x, origin_y);
                                    //let texture =
                                    //Some(load_texture("./assets/rock.png").await.unwrap());
                                    let tile = Tile::new(SpriteType::Obstacle, surf, None).await;
                                    self.sprites.push(Sprites::Tile(tile));
                                }
                                _ => (),
                            }
                            origin_x += size.0;
                        }
                        origin_x = 0.0;
                        origin_y += size.1;
                    }
                } else if *style == "grass" {
                    let map = layout("./assets/map0/mapa0_vegetacion.csv");
                    let images = import_folder("./assets/flowers/");

                    for row in map.iter() {
                        for n in row.iter() {
                            match *n {
                                -1 => (),
                                _ => {
                                    let surf = Vec2::new(origin_x, origin_y);
                                    if let Some(image) = pick_random_path(&images) {
                                        let texture = Some(load_texture(image).await.unwrap());
                                        let tile =
                                            Tile::new(SpriteType::Obstacle, surf, texture).await;
                                        self.sprites.push(Sprites::Tile(tile));
                                    } else {
                                        let texture =
                                            Some(load_texture("./assets/rock.png").await.unwrap());
                                        let tile =
                                            Tile::new(SpriteType::Obstacle, surf, texture).await;
                                        self.sprites.push(Sprites::Tile(tile));
                                    }
                                }
                            }
                            origin_x += size.0;
                        }
                        origin_x = 0.0;
                        origin_y += size.1;
                    }
                }
            }
        }
    }

    async fn create_level(&mut self) {
        self.create_map().await;
        let rect = Rect::new(1200.0, 1200.0, 128.0, 128.0);
        let player = Player::new(rect).await;
        self.sprites.push(Sprites::Player(player));
    }
    pub async fn run(&mut self) {
        if is_key_down(KeyCode::K) {
            panic!("Closing the game");
        }
        println!("level");

        self.camera
            .run_draw(&self.background, &self.background_rect, &mut self.sprites)
            .await;
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
    pub async fn run_draw(
        &mut self,
        back: &Texture2D,
        back_rect: &Rect,
        sprites: &mut Vec<Sprites>,
    ) {
        // 1. Offset de cámara pixel-perfect
        if let Some(p) = sprites.iter().find_map(|s| {
            if let Sprites::Player(p) = s {
                Some(p)
            } else {
                None
            }
        }) {
            //       self.offset.x = (p.rect.x + p.rect.w / 2.0).floor();
            //self.offset.y = p.rect.y.floor();
            self.offset.x = (p.rect.x + p.rect.w / 2.0).floor();
            // Aplica un desplazamiento vertical de 50 píxeles
            const CAMERA_Y_SHIFT: f32 = 150.0;
            self.offset.y = (p.rect.y).floor() + CAMERA_Y_SHIFT;
        }

        // 2. Fondo
        let bg_x = (back_rect.x - self.offset.x + self.half_w).floor();
        let bg_y = (back_rect.y - self.offset.y + self.half_h).floor();
        draw_texture(back, bg_x, bg_y, WHITE);

        // 3. Profundidad
        YsortCamera::y_sort(sprites);
        let obstacles = sprites.clone();

        // 4. Sprites pixel-snapped
        for sprite in sprites.iter_mut() {
            let sx = (sprite.x() - self.offset.x + self.half_w + 12.0/* Ajuste */).floor();
            let sy = (sprite.y() - self.offset.y + self.half_h + 140.0/* Ajuste */).floor();

            sprite.draw((sx, sy));
            sprite.update(&obstacles);
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
