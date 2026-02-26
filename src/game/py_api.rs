use crate::game::Game;
use pyo3::prelude::*;

/// Python'a sunacağımız geçici API nesnesi.
/// DIKKAT: Sadece Rust'ın GIL'i elinde tuttuğu tick aşamasında geçerlidir.
#[pyclass(unsendable, module = "termrender")]
pub struct PyGameApi {
    game_ptr: *mut Game<'static>,
}

impl PyGameApi {
    pub fn new(game: &mut Game) -> Self {
        Self {
            game_ptr: unsafe { std::mem::transmute::<&mut Game<'_>, *mut Game<'static>>(game) },
        }
    }

    fn get_game<'a>(&self) -> &'a mut Game<'static> {
        unsafe { &mut *self.game_ptr }
    }
}

#[pymethods]
impl PyGameApi {
    #[pyo3(signature = (id, x, y))]
    pub fn set_location(&mut self, id: usize, x: i32, y: i32) -> PyResult<()> {
        let game = self.get_game();
        if let Some(obj) = game.world.get_with_id(id) {
            let _ = obj.set_cords(x, y);
        }
        Ok(())
    }

    #[pyo3(signature = (id))]
    pub fn get_location(&mut self, id: usize) -> PyResult<Option<(i32, i32)>> {
        let game = self.get_game();
        if let Some(obj) = game.world.get_with_id(id) {
            if let Some(coords) = obj.get_cords() {
                return Ok(Some((coords.x, coords.y)));
            }
        }
        Ok(None)
    }

    #[pyo3(signature = (id))]
    pub fn force_rerender(&mut self, id: usize) -> PyResult<()> {
        let game = self.get_game();
        if let Some(obj) = game.world.get_with_id(id) {
            obj.force_rerender();
        }
        Ok(())
    }

    #[pyo3(signature = (key_name))]
    pub fn is_key_pressed(&self, key_name: &str) -> PyResult<bool> {
        let game = self.get_game();

        let key = match key_name {
            "Esc" => Some(crate::game::input::Keys::Esc),
            "Space" => Some(crate::game::input::Keys::Space),
            "Up" => Some(crate::game::input::Keys::Up),
            "Down" => Some(crate::game::input::Keys::Down),
            "Left" => Some(crate::game::input::Keys::Left),
            "Right" => Some(crate::game::input::Keys::Right),
            "E" => Some(crate::game::input::Keys::E),
            "Q" => Some(crate::game::input::Keys::Q),
            "R" => Some(crate::game::input::Keys::R),
            "T" => Some(crate::game::input::Keys::T),
            _ => None,
        };

        if let Some(k) = key {
            Ok(game.input.pressed_keys.contains(&k))
        } else {
            Ok(false)
        }
    }

    #[pyo3(signature = (key_name))]
    pub fn is_key_just_pressed(&self, key_name: &str) -> PyResult<bool> {
        let game = self.get_game();
        let key = match key_name {
            "Esc" => Some(crate::game::input::Keys::Esc),
            "Space" => Some(crate::game::input::Keys::Space),
            "Up" => Some(crate::game::input::Keys::Up),
            "Down" => Some(crate::game::input::Keys::Down),
            "Left" => Some(crate::game::input::Keys::Left),
            "Right" => Some(crate::game::input::Keys::Right),
            "E" => Some(crate::game::input::Keys::E),
            "Q" => Some(crate::game::input::Keys::Q),
            "R" => Some(crate::game::input::Keys::R),
            "T" => Some(crate::game::input::Keys::T),
            _ => None,
        };

        if let Some(k) = key {
            Ok(game.input.just_pressed_keys.contains(&k))
        } else {
            Ok(false)
        }
    }

    pub fn log_debug(&self, msg: &str) -> PyResult<()> {
        crate::game::logger::log(
            crate::game::logger::LogType::Level(crate::game::logger::LogLevel::Debug),
            "py_api",
            msg,
            0,
        );
        Ok(())
    }
    pub fn log_info(&self, msg: &str) -> PyResult<()> {
        crate::game::logger::log(
            crate::game::logger::LogType::Level(crate::game::logger::LogLevel::Info),
            "py_api",
            msg,
            0,
        );
        Ok(())
    }
    pub fn log_warn(&self, msg: &str) -> PyResult<()> {
        crate::game::logger::log(
            crate::game::logger::LogType::Level(crate::game::logger::LogLevel::Warn),
            "py_api",
            msg,
            0,
        );
        Ok(())
    }
    pub fn log_error(&self, msg: &str) -> PyResult<()> {
        crate::game::logger::log(
            crate::game::logger::LogType::Level(crate::game::logger::LogLevel::Error),
            "py_api",
            msg,
            0,
        );
        Ok(())
    }
}
