#![recursion_limit = "256"]

use std::time::Instant;

use once_cell::sync::Lazy;

use crate::{math::Vec2, print::{GameTexture, GColor, PrintTypes, TermPrint, RGB}};
pub mod game;
pub mod math;
pub mod gameobject;
pub mod print;
pub mod prelude;
pub mod components;
pub mod physics;

pub type Ret = std::io::Result<()>;
/// if true continue
/// 
/// if false stop program
pub type RetTick = std::io::Result<bool>;
pub type RetType<T> = std::io::Result<T>;
pub static RESET_COLOR:&str = "\x1b[0m";
pub static GAME_STARTED:Lazy<Instant> = Lazy::new(||{
    Instant::now()
});

#[macro_export]
macro_rules! print_vec {
    ($((($x:expr, $y:expr)$c:expr)),* $(,)?) => {
        vec![
            $(
                PrintThing {
                    rel_pos: Vec2::from(($x, $y)),
                    text: TermPrint::from($c),
                }
            ),*
        ]
    };
}
