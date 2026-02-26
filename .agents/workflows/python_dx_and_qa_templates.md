---
description: Termrender Python DX and QA Testing Templates
---

# Termrender Python Developer Experience (DX) Templates

Bu dosya, Rust tabanlı `termrender` kütüphanesini Python tarafında "Pythonic" (DX dostu) bir yapıya kavuşturmakla görevli olan ajanların referans alacağı **şablonları** içerir. 

Eğer Rust tarafında PyO3 C-extension modülü `_termrender` adıyla dışa aktarılıyorsa, saf Python (Pure Python) tarafındaki sarmalayıcılar bu dosyadaki standartlara uygun olarak yazılmalıdır.

---

## 1. Python Wrapper Template (`termrender/__init__.py`)

Kullanıcıların `_termrender` yerine doğrudan `termrender` kütüphanesini içeri aktarması ve nesne tabanlı (Object-Oriented), bağlam (Context Manager) yönetimli bir deneyim yaşaması sağlanmalıdır.

```python
"""
Termrender - High performance terminal rendering engine.
This module provides a Pythonic wrapper around the PyO3 Rust bindings.
"""

from typing import Tuple

try:
    import _termrender
except ImportError as e:
    raise ImportError("The Rust backend '_termrender' is not compiled or installed. Please build the project using maturin.") from e

class Engine:
    """
    Pythonic context manager wrapper for the Rust Termrender Engine.
    """
    
    def __init__(self, *args, **kwargs):
        self._raw_engine = _termrender.Engine(*args, **kwargs)

    def __enter__(self) -> 'Engine':
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        pass

    @property
    def width(self) -> int:
        return self._raw_engine.get_width()

    @width.setter
    def width(self, val: int):
        self._raw_engine.set_width(val)

    def clear_screen(self):
        self._raw_engine.clear_screen()

    def render(self):
        self._raw_engine.render()


def get_version() -> str:
    return _termrender.__version__

```

---

## 2. Type Hint Stubs Template (`termrender/__init__.pyi`)

Geliştiricilerin VS Code / PyCharm gibi IDE'lerde `termrender` kütüphanesini kullanırken detaylı fonksiyon açıklamaları ve tip ipuçları görmeleri için `.pyi` stub dosyasının şablonu.

```python
"""
Type stub file for the termrender library.
"""
from typing import Tuple, Optional, Any

class Engine:
    def __init__(self, *args: Any, **kwargs: Any) -> None: ...

    def __enter__(self) -> 'Engine': ...

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None: ...

    @property
    def width(self) -> int: ...
        
    @width.setter
    def width(self, val: int) -> None: ...

    def clear_screen(self) -> None: ...
        
    def render(self) -> None: ...

def get_version() -> str: ...
```

---

## 3. Demo Game Template (`examples/snake.py`)

Kütüphane kullanılabilir hale geldiğinde kullanıcıların "neler yapabildiğini" hızlıca görmesi için hazırlanmış bir Python Snake oyunu şablonu.

```python
#!/usr/bin/env python3
"""
Termrender Snake Game Demo
"""

import time
import termrender

def main():
    print(f"Starting Termrender Snake Engine (v{termrender.get_version()})...")
    
    with termrender.Engine() as engine:
        engine.width = 40
        
        running = True
        try:
            while running:
                engine.clear_screen()
                engine.render()
                time.sleep(0.1)
                
        except KeyboardInterrupt:
            pass

if __name__ == "__main__":
    main()
```

# QA & Testing Templates for Python Bindings

This document contains the structural templates and boilerplate code for testing the PyO3 Python bindings for the `termrender` engine.

## 1. Test Dependencies (`tests/python/requirements.txt`)
```txt
pytest>=8.0.0
psutil>=6.0.0
memory-profiler>=0.61.0
matplotlib>=3.9.0
markdown>=3.6.0
```

## 2. API & Edge Case Tests (`tests/python/test_edge_cases.py`)
```python
import pytest

def test_invalid_color_coordinates():
    pass

def test_out_of_bounds_rendering():
    pass

def test_null_or_invalid_reference():
    pass

def test_panic_safety():
    pass
```

## 3. Memory Leak Detection (`tests/python/test_memory_leak.py`)
```python
import gc
import psutil
import os
import time
import pytest

def get_memory_usage():
    process = psutil.Process(os.getpid())
    return process.memory_info().rss / (1024 * 1024) # MB

def test_memory_leak_object_creation():
    initial_mem = get_memory_usage()
    
    for _ in range(100_000):
        pass
    
    gc.collect()
    time.sleep(0.5)
    
    final_mem = get_memory_usage()
    assert final_mem - initial_mem < 5.0, f"Memory leak detected! Grew by {final_mem - initial_mem:.2f} MB"
```

## 4. Performance & FPS Profiling (`tests/python/test_fps_profiling.py`)
```python
import time
import pytest

def test_python_fps_profiling():
    start = time.perf_counter()
    iterations = 1000
    
    for _ in range(iterations):
        pass
        
    end = time.perf_counter()
    elapsed = end - start
    fps = iterations / elapsed if elapsed > 0 else 0
    
    print(f"\n[Performance] Average Python-side Render FPS: {fps:.2f}")
    assert fps > 30.0, f"Python to Rust render loop FPS too low! ({fps:.2f})"
```

## 5. Automated QA Report Generator (`tests/python/qa_report_generator.py`)
```python
import subprocess
import os
from datetime import datetime

def generate_report():
    report_file = "qa_report_latest.md"
    now_str = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    result = subprocess.run(
        ["pytest", "-v", "--tb=short"], 
        capture_output=True, 
        text=True,
        cwd=os.path.dirname(os.path.abspath(__file__))
    )
    
    with open(report_file, "w") as f:
        f.write("# Termrender Python bindings QA Report\n")
        f.write(f"**Generated:** {now_str}\n\n")
        if result.returncode == 0:
            f.write("✅ All API tests passed.\n\n")
        else:
            f.write("❌ Tests failed. See details below.\n\n")
            
        f.write("## Pytest Output\n```text\n")
        f.write(result.stdout)
        if result.stderr:
            f.write("\n" + result.stderr)
        f.write("\n```\n")
        
if __name__ == "__main__":
    generate_report()
```
