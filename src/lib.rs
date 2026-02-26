#![recursion_limit = "256"]
#![allow(unsafe_op_in_unsafe_fn)]

use std::time::Instant;

use once_cell::sync::Lazy;

pub mod components;
pub mod game;
pub mod gameobject;
pub mod math;
pub mod physics;
pub mod prelude;
pub mod print;

pub type Ret = std::io::Result<()>;
/// if true continue
///
/// if false stop program
pub type RetTick = std::io::Result<bool>;
pub type RetType<T> = std::io::Result<T>;
pub static RESET_COLOR: &str = "\x1b[0m";
pub static GAME_STARTED: Lazy<Instant> = Lazy::new(|| Instant::now());

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

use pyo3::prelude::*;

#[pymodule]
fn termrender(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<math::Vec2>()?;
    m.add_class::<math::Vec2f>()?;
    m.add_class::<physics::AABB>()?;
    m.add_class::<print::PyGColor>()?;
    m.add_class::<print::PyTermPrint>()?;
    m.add_class::<print::PyPrintType>()?;
    m.add_class::<print::PyGameTexture>()?;
    m.add_class::<game::PyEngine>()?;
    m.add_class::<game::py_api::PyGameApi>()?;
    Ok(())
}
