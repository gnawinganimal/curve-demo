use ggez::glam::vec2 as glam_vec2;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, DrawMode, FillOptions};
use ggez::event::{self, EventHandler};

pub mod vec2;
pub use vec2::{Vec2, vec2};

pub mod curve;
pub use curve::Curve;

pub mod spline;
pub use spline::Spline;

pub mod hermite;
pub use hermite::{Hermite, herm};

fn main() -> GameResult {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx)?;

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    u: f32,
    mesh: Mesh,
    spline: Spline<Hermite>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        Ok(MyGame {
            u: 0.,
            mesh: Mesh::new_circle(
                ctx, 
                DrawMode::fill(),
                glam_vec2(0., 0.), 
                10.0, 
                0.2, 
                Color::BLACK,
            )?,
            spline: Spline::from([
                herm(
                    vec2(100., 100.),
                    vec2(0., 1000.),
                    vec2(500., 300.),
                    vec2(500., 0.),
                ),
            ].as_slice())
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.u += 0.01;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        if let Some(point) = self.spline.get(self.u) {
            canvas.draw(&self.mesh, glam_vec2(point.0, point.1));
        }

        canvas.finish(ctx)
    }
}
