use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;
use rand;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{SCREEN_H, SCREEN_W, SNACK_W};

pub struct Snack {
    location: Point2,
    velocity: Vector2,
    w: f32,
    active: bool,
}

impl Snack {
    fn new() -> Snack {
        let s = Snack {
            location: Point2::new(
                rand::random::<f32>() * SCREEN_W,
                rand::random::<f32>() * SCREEN_H - SCREEN_H,
            ),
            velocity: Vector2::new(0.0, rand::random::<f32>() * 2.0 + 0.1),
            w: SNACK_W,
            active: true,
        };
        s
    }

    pub fn update(&mut self) -> GameResult<&Self> {
        /*
         * TODO:
         * 1. Move snack down
         * 2. Set active to false if the snack has left the screen
         * 3. If not active, reset the snack
         */
        self.location += self.velocity;
        if !self.active || self.location.y > SCREEN_H {
            *self = Snack::new();
        }
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        if self.active {
            let draw_params = graphics::DrawParam::new().dest(self.location);
            graphics::draw(ctx, img, draw_params)?;
        }
        Ok(self)
    }

    pub fn collides_with(&mut self, other: Point2) -> bool {
        if self.active {
            let distance = self.location - other;
            if distance.norm() < self.w {
                self.active = false;
                return true;
            }
        }
        false
    }
}

pub fn spawn_snacks(num: usize) -> Vec<Snack> {
    let mut snacks = vec![];
    for _ in 0..num {
        snacks.push(Snack::new());
    }
    snacks
}
