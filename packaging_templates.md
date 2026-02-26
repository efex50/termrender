# Packaging & CI/CD Templates for Termrender

Bu dosya, `termrender` projesinin Python modülü (wheel) olarak derlenmesi, paketlenmesi ve CI/CD süreçleriyle dağıtılması için gereken tüm şablonları içerir. Diğer ajanlar ihtiyaç duydukları konfigürasyonları buradan alıp kullanabilirler.

## 1. `Cargo.toml` Eklemeleri
Rust projesinin bir Python dinamik kütüphanesi (C-extension) olarak derlenebilmesi için `Cargo.toml` dosyasına aşağıdaki kısımların eklenmesi veya güncellenmesi gereklidir:

```toml
[lib]
name = "termrender"
crate-type = ["cdylib", "rlib"]

[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"] }

[profile.release]
opt-level = "z"  # veya 3 (Performans/Boyut dengesine göre)
lto = "fat"      # Boyutu küçültmek ve performansı artırmak için
codegen-units = 1
panic = "abort"
strip = "symbols"
```

## 2. `pyproject.toml`
Maturin'in projeyi bir Python paketi olarak algılayabilmesi ve `pip install .` komutunun çalışabilmesi için projenin kök dizininde bulunması gereken dosya:

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "termrender"
requires-python = ">=3.8"
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: Implementation :: CPython",
    "Programming Language :: Python :: Implementation :: PyPy",
]
```

## 3. Lokal Derleme Scriptleri

### 3.1. `build.bat` (Windows için)
Lokal Windows ortamında wheel derlemek için:

```bat
@echo off
echo Building Termrender Python Wheel...
pip install maturin
maturin build --release
echo Build complete. Check target/wheels/ directory.
pause
```

### 3.2. `build.sh` (Linux/MacOS için)
Lokal Linux/Mac ortamında wheel derlemek için:

```bash
#!/bin/bash
set -e

echo "Building Termrender Python Wheel..."
pip install maturin
maturin build --release
echo "Build complete. Check target/wheels/ directory."
```

## 4. GitHub Actions CI/CD Pipeline (`.github/workflows/python-release.yml`)
Projeye her tag atıldığında veya main branch'e kod pushlandığında tüm platformlar için wheel paketlerini derleyip otomatik olarak PyPI veya GitHub Releases üzerine yükleyecek şablon:

```yaml
name: Build and Publish Python Wheels

on:
  push:
    branches:
      - main
    tags:
      - 'v*'
  pull_request:

jobs:
  build-wheels:
    name: Build wheels on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]

    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Set up Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.10'

      - name: Build Wheels
        uses: PyO3/maturin-action@v1
        with:
          command: build
          args: --release --out dist
          manylinux: auto

      - name: Upload wheels
        uses: actions/upload-artifact@v4
        with:
          name: wheels-${{ matrix.os }}
          path: dist/*.whl

  publish:
    name: Publish to PyPI
    needs: build-wheels
    if: startsWith(github.ref, 'refs/tags/v')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4
        with:
          path: dist/
          merge-multiple: true

      - name: Publish to PyPI
        uses: PyO3/maturin-action@v1
        with:
          command: upload
          args: --skip-existing dist/*
        env:
          MATURIN_PYPI_TOKEN: ${{ secrets.PYPI_API_TOKEN }}
```

## 5. Python Modülü İçin Başlangıç Noktası (Örnek `src/lib.rs` Modülü)
Maturin'in C kütüphanesini başarılı bir şekilde Python'a sunabilmesi için `src/lib.rs` içerisine bir PyO3 modülü tanımlanmalıdır:

```rust
use pyo3::prelude::*;

/// Python'a açılacak bir örnek fonksiyon
#[pyfunction]
fn merhaba_termrender() -> PyResult<String> {
    Ok("Termrender Python modülü başarıyla yüklendi!".to_string())
}

/// Modül giriş noktası - Python'daki `import termrender` çağrıldığında çalışır.
#[pymodule]
fn termrender(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merhaba_termrender, m)?)?;
    Ok(())
}
```
