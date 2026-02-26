---
description: PyO3 FFI and Packaging Templates for Termrender
---
# Termrender: PyO3 FFI Şablonları ve Veri Geçiş Modelleri

Bu belge, **Rust Binding Ajanı** ve **Python Geliştirici Ajanı** için, Rust (Core) ve Python katmanları arasında veri geçişlerini (FFI) sağlarken kullanılacak PyO3 şablonlarını içermektedir.

## 1. Basit Veri Tipleri (Value Types)

`Vec2` gibi matematiksel tipler referans yerine kopyalanarak veya taşınarak (Value Type) yönetilmelidir. Performans için gereklidir.

```rust
use pyo3::prelude::*;
use crate::math::Vec2;

#[pyclass(name = "Vec2")]
#[derive(Clone, Copy)]
pub struct PyVec2 {
    pub inner: Vec2,
}

#[pymethods]
impl PyVec2 {
    #[new]
    pub fn new(x: i32, y: i32) -> Self {
        PyVec2 { inner: Vec2::new(x, y) }
    }

    #[getter]
    pub fn x(&self) -> i32 { self.inner.x }
    
    #[setter]
    pub fn set_x(&mut self, value: i32) { self.inner.x = value; }

    #[getter]
    pub fn y(&self) -> i32 { self.inner.y }

    #[setter]
    pub fn set_y(&mut self, value: i32) { self.inner.y = value; }

    fn __add__(&self, other: &PyVec2) -> PyVec2 {
        PyVec2 { inner: self.inner + other.inner }
    }
}
```

## 2. World / Game API Proxy (Referans Yönetimi)

Rust tarafındaki `Game` objesi `&mut` olarak döngü (loop) içerisindedir. Bu nedenle Python'a `&mut Game` doğrudan verilemez (Borrow Checker hatası). Çözüm: Pointer tabanlı bir Proxy sınıfı oluşturmak ve bunu yalnızca `with_gil` bloğu içerisinde Python'a sunmaktır.

```rust
use pyo3::prelude::*;
use crate::game::Game;

/// Python'a sunacağımız geçici API nesnesi.
/// DIKKAT: Sadece Rust'ın GIL'i elinde tuttuğu tick aşamasında geçerlidir.
#[pyclass(unsendable)]
pub struct PyGameApi {
    game_ptr: *mut Game<'static>, 
}

impl PyGameApi {
    pub fn new(game: &mut Game) -> Self {
        Self {
            game_ptr: game as *mut _ as *mut Game<'static>
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

    #[pyo3(signature = (key_name))]
    pub fn is_key_pressed(&self, key_name: &str) -> PyResult<bool> {
        let game = self.get_game();
        Ok(false) 
    }
}
```

## 3. Python Sistem Köprüsü (PySystemBridge)

Rust'taki `GameSystem` trait'ini implement eden ve Python'daki `class` instance'ını tetikleyen köprü. 

```rust
use pyo3::prelude::*;
use crate::game::systems::{GameSystem, Message};
use crate::game::Game;
use std::time::Duration;

pub struct PySystemBridge {
    pub python_instance: PyObject,
}

impl GameSystem for PySystemBridge {
    fn _process_loop(&mut self, delta: Duration, sys_name: &String, game: &mut Game) -> crate::RetTick {
        Python::with_gil(|py| {
            let api = PyGameApi::new(game);
            let args = (delta.as_secs_f32(), sys_name.clone(), api);
            
            if let Err(e) = self.python_instance.call_method1(py, "process_loop", args) {
                e.print(py);
            }
        });
        Ok(true)
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
```

## 4. Initialization (Motorun Python'dan Başlatılması)

Termrender motorunun Python scripti içerisinden bir Python paketi gibi import edilip ayağa kaldırılması için gereken başlangıç (Entry Point) noktası.

```rust
use pyo3::prelude::*;

#[pyfunction]
fn run_engine(py: Python<'_>, entry_system: PyObject) -> PyResult<()> {
    let mut game = crate::game::Game::new("Termrender PyO3".into(), 60.0, 60.0, 60.0);
    let bridge = PySystemBridge { python_instance: entry_system };
    let _ = game.main_loop();
    Ok(())
}

#[pymodule]
fn termrender(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyVec2>()?;
    m.add_class::<PyGameApi>()?;
    m.add_function(wrap_pyfunction!(run_engine, m)?)?;
    Ok(())
}
```

# Packaging & CI/CD Templates for Termrender

Projenin bir Python dinamik kütüphanesi olarak derlenmesi için kurallar.

## 1. `Cargo.toml`
```toml
[lib]
name = "termrender"
crate-type = ["cdylib", "rlib"]

[dependencies]
pyo3 = { version = "0.22", features = ["extension-module", "abi3-py310"] }

[profile.release]
opt-level = "z"  
lto = "fat"      
codegen-units = 1
panic = "abort"
strip = "symbols"
```

## 2. `pyproject.toml`
```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "termrender"
requires-python = ">=3.10"
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: Implementation :: CPython",
]
```

## 3. Derleme Kodları
- Win: `pip install maturin && maturin build --release`
- Linux: `pip install maturin && maturin build --release`

Ajanlar geliştirme aşamasında wrapper ve implementasyon yaparken buradaki kuralları baz almalıdır.
