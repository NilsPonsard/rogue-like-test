use ggez::event::{self, EventHandler};
use ggez::event::{KeyCode, KeyMods};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use std::time::{Duration, Instant};
extern crate mint;
use crate::game_world;

pub const UPDATES_PER_SECOND: f32 = 60.0;
pub const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

pub const VIEWPORT_WIDTH: f32 = 320.0;
pub const VIEWPORT_HEIGHT: f32 = 180.0;

pub const TILE_SIZE: usize = 16;

struct Assets {
    player_image: graphics::Image,
    // shot_image: graphics::Image,
    // rock_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/assets/player/placeholder.png")?;
        // let shot_image = graphics::Image::new(ctx, "/shot.png")?;
        // let rock_image = graphics::Image::new(ctx, "/rock.png")?;
        Ok(Assets {
            player_image,
            // shot_image,
            // rock_image
        })
    }
}

struct Direction {
    right: bool,
    left: bool,
    up: bool,
    down: bool,
}

impl Direction {
    pub fn new() -> Direction {
        Direction {
            right: false,
            left: false,
            up: false,
            down: false,
        }
    }
}

pub struct MainState {
    x: f32,
    y: f32,
    dir: Direction,
    last_update: Instant,
    assets: Assets,
    world: game_world::GameWorld,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);
        let assets = Assets::new(ctx)?;
        let world = game_world::GameWorld::new(ctx, 40, 40)?;

        let s = MainState {
            x: 0.0,
            y: 0.0,
            dir: Direction::new(),
            last_update: Instant::now(),
            assets,
            world,
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            if self.dir.up {
                self.y -= 2.0
            }
            if self.dir.down {
                self.y += 2.0
            }
            if self.dir.right {
                self.x += 2.0
            }
            if self.dir.left {
                self.x -= 2.0
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.world.draw(ctx)?;
        {
            let drawparams =
                graphics::DrawParam::new().dest(ggez::nalgebra::Point2::new(self.x, self.y));
            graphics::draw(ctx, &self.assets.player_image, drawparams)?;
        }
        graphics::present(ctx)
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        repeat: bool,
    ) {
        // println!("{}", repeat);
        if !repeat {
            // do only once by keypress
            match keycode {
                KeyCode::Left => self.dir.left = true,
                KeyCode::Right => self.dir.right = true,
                KeyCode::Up => self.dir.up = true,
                KeyCode::Down => self.dir.down = true,

                _ => {}
            }
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Left => self.dir.left = false,
            KeyCode::Right => self.dir.right = false,
            KeyCode::Up => self.dir.up = false,
            KeyCode::Down => self.dir.down = false,

            _ => {}
        }
    }
}
