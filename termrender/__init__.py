"""
Termrender - High performance terminal rendering engine.
This module provides a Pythonic wrapper around the PyO3 Rust bindings.
"""

from typing import Tuple, Any

# PyO3 tarafından derlenmiş saf Rust modülünü (C-extension) içe aktarıyoruz.
# maturin build veya maturin develop ile derlenmiş olması gerekir.
try:
    # When installed via maturin, the C-extension is typically named `termrender`
    # However since our python folder is also named `termrender`, it can be found under
    # termrender.termrender (site-packages/termrender/termrender.pyd)
    from termrender import termrender as _termrender
except ImportError as e:
    raise ImportError("The Rust backend 'termrender' is not compiled or installed. Please build the project using maturin develop.") from e

# Rust tarafındaki primitive ve utils nesnelerini direkt dışa açıyoruz (e.g. Vec2)
try:
    Vec2 = _termrender.Vec2
except AttributeError:
    pass

class Engine:
    """
    Pythonic context manager wrapper for the Rust Termrender Engine.
    
    Usage:
        import termrender
        with termrender.Engine() as engine:
            engine.width = 100
            engine.render()
    """
    
    def __init__(self, *args, **kwargs):
        """
        Initializes the Termrender engine.
        Any positional or keyword arguments are passed down to the Rust Engine struct.
        """
        self._raw_engine = _termrender.Engine(*args, **kwargs)

    def __enter__(self) -> 'Engine':
        """
        Context manager girişi: Gerekiyorsa terminali başlat (Örn. raw mode, screen clear vb.)
        """
        if hasattr(self._raw_engine, 'setup_terminal'):
            self._raw_engine.setup_terminal()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """
        Context manager çıkışı: Hata olsa dahi terminali eski haline getir.
        """
        if hasattr(self._raw_engine, 'teardown_terminal'):
            self._raw_engine.teardown_terminal()

    # --- Properties (Getter / Setter) ---
    @property
    def width(self) -> int:
        """Get the current render width."""
        return self._raw_engine.get_width()

    @width.setter
    def width(self, val: int):
        """Set the render width."""
        self._raw_engine.set_width(val)
        
    @property
    def height(self) -> int:
        """Get the current render height."""
        return self._raw_engine.get_height()

    @height.setter
    def height(self, val: int):
        """Set the render height."""
        self._raw_engine.set_height(val)

    # --- Snake Case Method Wrappers ---
    def clear_screen(self):
        """
        Clears the terminal screen.
        Wraps the Rust camelCase 'clearScreen' or 'clear_screen' method.
        """
        if hasattr(self._raw_engine, 'clear_screen'):
            self._raw_engine.clear_screen()
        elif hasattr(self._raw_engine, 'clearScreen'):
            self._raw_engine.clearScreen()

    def render(self):
        """Renders the current frame to the terminal."""
        self._raw_engine.render()
        
    def poll_events(self) -> Any:
        """
        Polls for terminal events (like keystrokes or resize).
        """
        if hasattr(self._raw_engine, 'poll_events'):
            return self._raw_engine.poll_events()
        elif hasattr(self._raw_engine, 'pollEvents'):
            return self._raw_engine.pollEvents()

    def __getattr__(self, name):
        """Forward any missing methods/attributes to the raw Rust engine."""
        return getattr(self._raw_engine, name)

def get_version() -> str:
    """Returns the version of the termrender engine."""
    return getattr(_termrender, '__version__', 'unknown')
