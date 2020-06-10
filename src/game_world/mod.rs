use ggez::event::{self, EventHandler};
use ggez::event::{KeyCode, KeyMods};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use std::time::{Duration, Instant};


pub enum Tile {
    floor,
    wall,
}


pub struct GameWorld {
    data: Vec<Vec<Tile>>,
}

impl GameWorld {
    pub fn new(width: u32, height: u32) -> GameWorld {
        let mut data: Vec<Vec<Tile>> = Vec::new();
        for i in 0..width {
            let mut line: Vec<Tile> = Vec::new();
            for j in 0..height {
                line.push(Tile::wall);
            }
            data.push(line);
        }

        GameWorld {
            data
        }
    }
}