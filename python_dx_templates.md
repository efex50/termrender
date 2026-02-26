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

# PyO3 tarafından derlenmiş saf Rust modülünü (C-extension) içe aktarıyoruz
try:
    import _termrender
except ImportError as e:
    raise ImportError("The Rust backend '_termrender' is not compiled or installed. Please build the project using maturin.") from e


class Engine:
    """
    Pythonic context manager wrapper for the Rust Termrender Engine.
    
    Usage:
        with Engine() as engine:
            engine.width = 100
            engine.render()
    """
    
    def __init__(self, *args, **kwargs):
        # Rust altındaki ham objeyi oluşturuyoruz
        self._raw_engine = _termrender.Engine(*args, **kwargs)

    def __enter__(self) -> 'Engine':
        """Context manager girişi: Gerekiyorsa terminali başlat (Örn. setup_terminal)"""
        # self._raw_engine.setup_terminal() varsayımı
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager çıkışı: Hata olsa dahi terminali eski haline getir (Örn. teardown_terminal)"""
        # self._raw_engine.teardown_terminal() varsayımı
        pass

    # --- Properties (Getter / Setter) ---
    @property
    def width(self) -> int:
        """Get the current render width."""
        return self._raw_engine.get_width()

    @width.setter
    def width(self, val: int):
        """Set the render width."""
        self._raw_engine.set_width(val)

    # --- Snake Case Method Wrappers ---
    def clear_screen(self):
        """
        Clears the terminal screen.
        Wraps the Rust camelCase 'clearScreen' method.
        """
        self._raw_engine.clear_screen()  # Veya camelCase geliyorsa: _raw_engine.clearScreen()

    def render(self):
        """Renders the current frame to the terminal."""
        self._raw_engine.render()


# Diğer modüller, enumlar ve fonksiyonlar dışa aktarılabilir.
def get_version() -> str:
    """Returns the version of the termrender engine."""
    return _termrender.__version__

```

---

## 2. Type Hint Stubs Template (`termrender/__init__.pyi`)

Geliştiricilerin VS Code / PyCharm gibi IDE'lerde `termrender` kütüphanesini kullanırken detaylı fonksiyon açıklamaları ve tip ipuçları görmeleri için `.pyi` stub dosyasının şablonu.

```python
"""
Type stub file for the termrender library.
Provides IDE autocompletion and static type checking.
"""
from typing import Tuple, Optional, Any

class Engine:
    """
    Pythonic context manager wrapper for the Rust Termrender Engine.
    Handles terminal setup and teardown automatically.

    Example:
        ```python
        import termrender
        with termrender.Engine() as engine:
            engine.width = 80
            engine.render()
        ```
    """

    def __init__(self, *args: Any, **kwargs: Any) -> None: ...

    def __enter__(self) -> 'Engine': ...

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None: ...

    @property
    def width(self) -> int:
        """
        int: The current rendering width in terminal columns.
        """
        ...
        
    @width.setter
    def width(self, val: int) -> None: ...

    def clear_screen(self) -> None:
        """
        Clears the entire terminal screen.
        Safe to call multiple times.
        """
        ...
        
    def render(self) -> None:
        """
        Renders the current frame buffer to the terminal.
        """
        ...

def get_version() -> str:
    """
    Returns:
        str: The currently compiled version of the termrender Rust backend.
    """
    ...
```

---

## 3. Demo Game Template (`examples/snake.py`)

Kütüphane kullanılabilir hale geldiğinde kullanıcıların "neler yapabildiğini" hızlıca görmesi için hazırlanmış bir Python Snake oyunu şablonu. (Bu kod, gerçek modül tasarlandığında güncellenebilir).

```python
#!/usr/bin/env python3
"""
Termrender Snake Game Demo
A classic Snake game written entirely in Python, powered by the Termrender Rust Engine.
"""

import time
import termrender

def main():
    print(f"Starting Termrender Snake Engine (v{termrender.get_version()})...")
    
    # Engine context manager ile yönetiliyor: Terminal güvenli bir şekilde başlayıp kapanacak
    with termrender.Engine() as engine:
        engine.width = 40
        # ... Diğer başlangıç ayarları ...
        
        running = True
        try:
            while running:
                # 1. Event (Girdi) İşleme
                # event = engine.poll_events()
                # if event == termrender.Event.QUIT: break
                
                # 2. Oyun Mantığı (Yılanı hareket ettir, elma yedi mi kontrol et vb.)
                # update_snake()
                
                # 3. Çizim (Render) İşlemi
                engine.clear_screen()
                # for segment in snake:
                #     engine.draw_char(segment.x, segment.y, 'O')
                engine.render()
                
                # Saniyede 10 kare (10 FPS)
                time.sleep(0.1)
                
        except KeyboardInterrupt:
            # Kullanıcı CTRL+C bastığında oyundan çık
            pass

if __name__ == "__main__":
    main()
```
