use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

mod beats;

struct MainState {
    pos_x: f32,
    progression: Vec<u64>,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        //let filename = "data/Pop-Rock-Loop1.wav";
        //let mut progression = beats::find_beats(filename).unwrap();
        let _progression: Vec<f32> = vec![
            16.13, 15.63, 15.13, 14.62, 14.12, 13.62, 13.11, 12.61, 12.1, 11.6, 11.09, 10.59,
            10.09, 9.58, 9.08, 8.57, 8.07, 7.57, 7.06, 6.56, 6.05, 5.55, 5.05, 4.54, 4.04, 3.53,
            3.03, 2.52, 2.02, 1.52, 1.01, 0.51, 0.01,
        ];
        let progression = beats::beats_to_intervals(_progression, 50.0);
        println!("{:?}", progression);
        let max = progression.first().unwrap();
        let s = MainState {
            pos_x: 0.0,
            progression,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        match self.progression.pop() {
            Some(interval) => {
                self.pos_x = self.pos_x + interval as f32;
                println!("{} -> {}",self.pos_x, self.pos_x + interval as f32);
            }
            None => (),
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.pos_x, 380.0),
            10.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
