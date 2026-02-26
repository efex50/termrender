import time
import pytest

# import termrender

def test_python_fps_profiling():
    # Will measure overhead from Python loop invoking Rust engine methods
    
    # Initialize engine
    # with termrender.Engine() as engine:
    #     start = time.perf_counter()
    #     iterations = 1000
    #     
    #     for _ in range(iterations):
    #         # engine.clear_screen()
    #         # ... print things ...
    #         # engine.render()
    #         pass
            
    #     end = time.perf_counter()
    
    elapsed = end - start
    fps = iterations / elapsed if elapsed > 0 else 0
    
    # We can assert to ensure the FPS is above an acceptable threshold.
    print(f"\n[Performance] Average Python-side Render FPS: {fps:.2f}")
    assert fps > 30.0, f"Python to Rust render loop FPS too low! ({fps:.2f})"
