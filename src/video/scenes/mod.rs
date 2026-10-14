pub mod part1;
pub mod part2;
pub mod part3;
pub mod part4;
pub mod part5;
pub mod part6;

use crate::video::ctx::Ctx;

/// Draws whichever parts are currently on screen. Each part fades itself in and out.
pub fn draw(c: &Ctx) {
    part1::draw(c);
    part2::draw(c);
    part3::draw(c);
    part4::draw(c);
    part5::draw(c);
    part6::draw(c);
}
