use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{CLAW_H, CLAW_S, CLAW_W};

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

pub struct Claw {
    pub location: Point2,
    body_anchor: Vector2,
    joint_anchor: Vector2, // "wrist"
    w: f32,
    h: f32,
    s: f32,
}

impl Claw {
    pub fn new(location: Point2, body_anchor: Vector2, joint_anchor: Vector2) -> GameResult<Claw> {
        let c = Claw {
            location,
            body_anchor,
            joint_anchor,
            w: CLAW_W,
            h: CLAW_H,
            s: CLAW_S,
        };
        Ok(c)
    }

    pub fn update(&mut self, parent_loc: Point2) -> GameResult<&Self> {
        /*
         * TODO: Update claw location according to body's location
         */
        self.location = parent_loc;
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        let red_color = graphics::Color::new(1.0, 0.0, 0.0, 1.0);

        let body_location = self.location + self.body_anchor;
        let joint_location = self.location + self.joint_anchor;

        let arm = graphics::Mesh::new_line(ctx, &[body_location, joint_location], 10., red_color)?;

        graphics::draw(ctx, &arm, graphics::DrawParam::default())?;

        let draw_params = graphics::DrawParam::new()
            .dest(self.get_origin())
            .scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, img, draw_params)?;
        Ok(self)
    }

    pub fn get_origin(&self) -> Point2 {
        let joint_position = self.location + self.joint_anchor;
        let x = joint_position.x - self.w / 2.0;
        let y = joint_position.y - self.h;
        Point2::new(x, y)
    }

    pub fn movedir(&mut self, dir: Directions) -> Vector2 {
        match dir {
            Directions::Up => {
                self.joint_anchor.y -= self.s;
            }
            Directions::Down => {
                self.joint_anchor.y += self.s;
            }
            Directions::Left => {
                self.joint_anchor.x -= self.s;
            }
            Directions::Right => {
                self.joint_anchor.x += self.s;
            }
        }
        self.joint_anchor
    }
}
