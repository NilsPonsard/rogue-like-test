use ggez::{graphics, Context, GameResult};

use crate::game_state;

struct WorldAsset {
    floor: graphics::Image,
    wall: graphics::Image,
}

impl WorldAsset {
    fn new(ctx: &mut Context) -> GameResult<WorldAsset> {
        let floor = graphics::Image::new(ctx, "/assets/map/floor.png")?;
        let wall = graphics::Image::new(ctx, "/assets/map/wall.png")?;

        Ok(WorldAsset { floor, wall })
    }
}

pub enum Tile {
    Floor,
    Wall,
}

pub struct GameWorld {
    data: Vec<Vec<Tile>>,
    assets: WorldAsset,
}

impl GameWorld {
    pub fn new(ctx: &mut Context, width: u32, height: u32) -> GameResult<GameWorld> {
        let mut data: Vec<Vec<Tile>> = Vec::new();
        for _i in 0..width {
            let mut line: Vec<Tile> = Vec::new();
            for _j in 0..height {
                line.push(Tile::Wall);
            }
            data.push(line);
        }
        let assets = WorldAsset::new(ctx)?;
        Ok(GameWorld { data, assets })
    }
    pub fn draw(&mut self, ctx: &mut Context, x: i32, y: i32) -> GameResult<()> {
        // let diff = x
        for i in 0..self.data.len() {
            let line = &self.data[i];
            for j in 0..line.len() {
                let drawparams =
                    graphics::DrawParam::new().dest(ggez::nalgebra::Point2::<f32>::new(
                        (i * game_state::TILE_SIZE) as f32,
                        (j * game_state::TILE_SIZE) as f32,
                    ));
                match line[j] {
                    Tile::Floor => {
                        graphics::draw(ctx, &self.assets.floor, drawparams)?;
                    }
                    Tile::Wall => {
                        graphics::draw(ctx, &self.assets.wall, drawparams)?;
                    }
                }
            }
        }

        Ok(())
    }
}
