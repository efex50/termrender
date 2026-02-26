"""
Type stub file for the termrender library.
Provides IDE autocompletion and static type checking.
"""
from typing import Tuple, Optional, Any, Callable

class Vec2:
    """
    2D Vector class mapping from Rust math::Vec2
    """
    def __init__(self, x: int, y: int) -> None: ...
    @property
    def x(self) -> int: ...
    @x.setter
    def x(self, value: int) -> None: ...
    @property
    def y(self) -> int: ...
    @y.setter
    def y(self, value: int) -> None: ...
    def __add__(self, other: 'Vec2') -> 'Vec2': ...


class Engine:
    """
    Pythonic context manager wrapper for the Rust Termrender Engine.
    Handles terminal setup and teardown automatically.

    Example:
        ```python
        import termrender
        with termrender.Engine() as engine:
            engine.width = 80
            engine.height = 24
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

    @property
    def height(self) -> int:
        """
        int: The current rendering height in terminal rows.
        """
        ...
        
    @height.setter
    def height(self, val: int) -> None: ...

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
        
    def poll_events(self) -> Any:
        """
        Poll keypress and terminal events.
        
        Returns:
            Event object or None if no event occurred.
        """
        ...

def get_version() -> str:
    """
    Returns:
        str: The currently compiled version of the termrender Rust backend.
    """
    ...
