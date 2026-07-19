# light-poseidon Python Bindings

Python bindings for [light-poseidon](https://github.com/Lightprotocol/light-poseidon), originally by [Mike Rostecki](https://github.com/vadorovsky). This package is a fork providing Python access to the Poseidon hash implementation for the BN254 curve.


## Install from pypi:
```bash
pip3 install light-poseidon

```

## Compatibility

This package is made to be compatible with [Kusama Shield's PoseidonPolkaVM](https://codeberg.org/KusamaShield/PoseidonPolkaVM).

## Installation

```bash
pip install light-poseidon
```

## Usage

```python
from light_poseidon_python import poseidon_hash, poseidon_hash_bytes, Hasher

# One-shot function with uint64 inputs
result = poseidon_hash([1, 2])
print(result)  # 0x115cc0f5e7d690413df64c6b9662e9cf2a3617f2743245519e19607a4417189a

# Byte inputs (32 bytes each, big-endian)
a = b"\x01" * 32
b = b"\x02" * 32
result = poseidon_hash_bytes([a, b])

# Reusable Hasher class
hasher = Hasher(2)  # 2 inputs
result = hasher.hash([1, 2])

# Byte variants
hasher.hash_bytes_be([a, b])  # big-endian
hasher.hash_bytes_le([a, b])  # little-endian
```

## Supported arities

1-12 inputs are supported.

## Development

```bash
# Using Makefile
make install    # Create venv and build
make test       # Run basic tests
make build      # Build wheel
make publish    # Publish to PyPI

# Or manually
cd light-poseidon-python
python3 -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop
```
