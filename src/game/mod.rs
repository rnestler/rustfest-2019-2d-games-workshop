use ggez::audio::SoundSource;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;

mod claw;
mod config;
mod crab;
mod player;
mod state;
use crate::game::claw::Directions;
mod assets;
mod snacks;

pub use crate::game::config::{SCREEN_H, SCREEN_W};
pub use crate::game::state::State;

type Point2 = na::Point2<f32>;

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for s in self.snacks.iter_mut() {
            s.update()?;
        }
        self.crab.update(self.screen_width)?;
        self.player1.update(self.crab.location)?;
        self.player2.update(self.crab.location)?;
        self.collision_check();
        /*
         * TODO: Play the background music
         */
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(
            ctx,
            &self.assets.bg_image,
            graphics::DrawParam::new().dest(Point2::new(0.0, 0.0)),
        )?;

        for s in self.snacks.iter() {
            s.draw(ctx, &self.assets.snack_image)?;
        }
        self.crab.draw(ctx, &self.assets.crab_image)?;
        self.player1.draw(ctx, &self.assets.claw_left)?;
        self.player2.draw(ctx, &self.assets.claw_right)?;

        self.render_ui(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods
    ) {
        /*
        * TODO: Provide 2 key matches. One for player 1 and the other for player 2
        */
    }
}
