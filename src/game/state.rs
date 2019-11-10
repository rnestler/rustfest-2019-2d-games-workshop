use ggez::audio::SoundSource;
use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::assets::Assets;
use crate::game::config::{CLAW_W, CRAB_H, CRAB_W, NUM_SNACKS};
use crate::game::crab::Crab;
use crate::game::player::Player;
use crate::game::snacks::{spawn_snacks, Snack};

pub struct State {
    pub player1: Player,
    pub player2: Player,
    pub crab: Crab,
    pub snacks: Vec<Snack>,
    pub screen_width: f32,
    pub assets: Assets,
}

impl State {
    pub fn new(ctx: &mut Context) -> ggez::GameResult<State> {
        println!("Play Crab!");
        println!("Player 1, use WASD!");
        println!("Player 2, use IJKL!");
        println!("Have fun!");

        let assets = Assets::new(ctx)?;
        let (width, height) = ggez::graphics::drawable_size(ctx);
        let crab_origin = Point2::new(width / 2.0 - (CRAB_W / 2.0), height - CRAB_H);

        let s = State {
            player1: Player::new(
                crab_origin,
                Vector2::new(CLAW_W - 20., CRAB_H / 2.),
                Vector2::new(-30., -20.),
            )?,
            player2: Player::new(
                crab_origin,
                Vector2::new(CRAB_W + 30.0, CRAB_H / 2.),
                Vector2::new(170.0, -20.0),
            )?,
            crab: Crab::new(crab_origin)?,
            snacks: spawn_snacks(NUM_SNACKS),
            screen_width: width,
            assets: assets,
        };
        Ok(s)
    }

    pub fn render_ui(&self, ctx: &mut Context) -> GameResult<&Self> {
        let score_1 = graphics::Text::new((
            format!("Player 1: #{}", self.player1.score),
            self.assets.font,
            38.0,
        ));
        let score_2 = graphics::Text::new((
            format!("Player 2: #{}", self.player2.score),
            self.assets.font,
            38.0,
        ));

        graphics::draw(
            ctx,
            &score_1,
            graphics::DrawParam::new().dest(Point2::new(10.0, 10.0)),
        )?;

        graphics::draw(
            ctx,
            &score_2,
            graphics::DrawParam::new().dest(Point2::new(self.screen_width - 180.0, 10.0)),
        )?;

        Ok(self)
    }

    pub fn collision_check(&mut self) {
        let c1 = self.player1.claw.get_origin();
        let c2 = self.player2.claw.get_origin();

        for snack in &mut self.snacks {
            if snack.collides_with(c1) {
                let _ = self.assets.snap_sound.play();
                self.player1.increase_score();
            }
            if snack.collides_with(c2) {
                let _ = self.assets.snap_sound.play();
                self.player2.increase_score();
            }
        }
    }
}
