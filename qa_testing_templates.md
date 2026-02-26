# QA & Testing Templates for Python Bindings

This document contains the structural templates and boilerplate code for testing the PyO3 Python bindings for the `termrender` engine. Other agents (e.g., Rust Bindings Agent, Architect) should use these templates as a reference for the expected Python API and testing methodology.

## 1. Test Dependencies (`tests/python/requirements.txt`)
```txt
pytest>=8.0.0
psutil>=6.0.0
memory-profiler>=0.61.0
matplotlib>=3.9.0
markdown>=3.6.0
```

## 2. API & Edge Case Tests (`tests/python/test_edge_cases.py`)
This suite validates the API surface and ensures that invalid inputs from Python do not crash the Rust runtime, but rather return appropriate Python exceptions.

```python
import pytest
# import termrender

def test_invalid_color_coordinates():
    # Example: Out of bounds or negative coordinates
    # with pytest.raises(ValueError):
    #     with termrender.Engine() as engine:
    #         # engine.draw_char(-1, 5, "test")
    pass

def test_out_of_bounds_rendering():
    # Example: Extreme coordinates
    # with pytest.raises(ValueError):
    #     with termrender.Engine() as engine:
    #         # engine.draw_char(99999, 99999, "test")
    pass

def test_null_or_invalid_reference():
    # Example: Passing None where a specific PyClass is expected
    # with pytest.raises(TypeError):
    #     with termrender.Engine() as engine:
    #         engine.width = None
    pass

def test_panic_safety():
    # Ensure rust panics (unwraps/expects) are properly caught via PyErr 
    # instead of crashing the python process.
    pass
```

## 3. Memory Leak Detection (`tests/python/test_memory_leak.py`)
This script asserts that creating and dropping large amounts of PyO3-bound Rust objects in Python correctly frees memory via Python's Garbage Collector.

```python
import gc
import psutil
import os
import time
import pytest

# import termrender

def get_memory_usage():
    process = psutil.Process(os.getpid())
    return process.memory_info().rss / (1024 * 1024) # MB

def test_memory_leak_object_creation():
    initial_mem = get_memory_usage()
    
    # Rapidly create and destroy bindings
    for _ in range(10_000):
        # Example Engine creations testing context manager __enter__ / __exit__
        # with termrender.Engine() as engine:
        #     engine.width = 100
        pass
    
    # Force python garbage collection
    gc.collect()
    time.sleep(0.5)
    
    final_mem = get_memory_usage()
    
    # Assert memory hasn't grown by more than a small margin (e.g., 5MB)
    assert final_mem - initial_mem < 5.0, f"Memory leak detected! Grew by {final_mem - initial_mem:.2f} MB"
```

## 4. Performance & FPS Profiling (`tests/python/test_fps_profiling.py`)
This script calculates the overhead introduced by the Python-to-Rust bridge during continuous rendering loops.

```python
import time
import pytest

# import termrender

def test_python_fps_profiling():
    # Measure overhead from Python loop invoking Rust engine methods
    
    # with termrender.Engine() as engine:
    #     start = time.perf_counter()
    #     iterations = 1000
    #     
    #     for _ in range(iterations):
    #         # Simulate an inner game loop
    #         # engine.clear_screen()
    #         # engine.render()
    #         pass
            
    #     end = time.perf_counter()
    
    elapsed = end - start
    fps = iterations / elapsed if elapsed > 0 else 0
    
    print(f"\n[Performance] Average Python-side Render FPS: {fps:.2f}")
    assert fps > 30.0, f"Python to Rust render loop FPS too low! ({fps:.2f})"
```

## 5. Automated QA Report Generator (`tests/python/qa_report_generator.py`)
A utility to aggregate the results of the test suite into a markdown report.

```python
import subprocess
import os
from datetime import datetime

def generate_report():
    report_file = "qa_report_latest.md"
    now_str = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    # Run pytest and collect report
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
