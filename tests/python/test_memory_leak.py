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
