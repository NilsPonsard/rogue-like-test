use ggez::event;
use ggez::{ContextBuilder, GameResult};

mod game_state;
mod game_world;

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup {
            title: "An easy, good game".to_owned(),
            samples: ggez::conf::NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(ggez::conf::WindowMode {
            width: 1280.0,
            height: 720.0,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: false,
        })
        .build()
        .expect("aieee, could not create ggez context!");

    let scale_x: f32 = ggez::graphics::screen_coordinates(&ctx).w / game_state::VIEWPORT_WIDTH;
    let scale_y: f32 = ggez::graphics::screen_coordinates(&ctx).h / game_state::VIEWPORT_HEIGHT;

    let param = ggez::graphics::DrawParam::default().scale(ggez::mint::Vector2 {
        x: scale_x,
        y: scale_y,
    });
    let transform = param.to_matrix();
    ggez::graphics::set_transform(&mut ctx, transform);
    ggez::graphics::apply_transformations(&mut ctx)?;
    ggez::graphics::set_default_filter(&mut ctx, ggez::graphics::FilterMode::Nearest);
    let mut game = game_state::MainState::new(&mut ctx)?;

    ggez::graphics::Scale::uniform(10.0);
    event::run(&mut ctx, &mut event_loop, &mut game)
}
