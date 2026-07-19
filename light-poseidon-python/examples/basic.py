"""Basic usage of the light_poseidon_python library."""

from light_poseidon_python import poseidon_hash, poseidon_hash_bytes, Hasher

# --- One-shot function ---

print("=== poseidon_hash (uint64 inputs) ===")
print(f"hash([0, 0]):   {poseidon_hash([0, 0])}")
print(f"hash([1, 2]):   {poseidon_hash([1, 2])}")
print(f"hash([123, 456]): {poseidon_hash([123, 456])}")
print()

# --- Byte inputs ---

print("=== poseidon_hash_bytes (32-byte inputs) ===")
a = b"\x00" * 32  # 0 as 32 bytes big-endian
b = b"\x01" * 32  # 1 as 32 bytes big-endian
print(f"hash_bytes([0x00..00, 0x01..01]): {poseidon_hash_bytes([a, b])}")
print()

# --- Hasher class (reusable) ---

print("=== Hasher class ===")
hasher = Hasher(2)  # 2 inputs
print(f"hasher.hash([1, 2]):       {hasher.hash([1, 2])}")
print(f"hasher.hash_bytes_be([...]): {hasher.hash_bytes_be([b, a])}")
print(f"hasher.hash_bytes_le([...]): {hasher.hash_bytes_le([b, a])}")
print()

# --- Different arities ---

print("=== Different arities (1-12 inputs) ===")
for n in range(1, 13):
    h = Hasher(n)
    result = h.hash([i + 1 for i in range(n)])
    print(f"  arity={n:2d}: {result[:18]}...")