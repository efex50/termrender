#!/bin/bash
set -e

echo "Building Termrender Python Wheel..."
pip install maturin
maturin build --release
echo "Build complete. Check target/wheels/ directory."
