from sys import version_info
from random import seed

if version_info >= (3, 9):
    from random import randbytes
else:
    from random import getrandbits as __getrandbits

    def randbytes(n: int) -> bytes:
        return __getrandbits(n * 8).to_bytes(n, "big")


import pytest

from rtea import tea16_encrypt, tea16_decrypt, qqtea_encrypt, qqtea_decrypt, is_debug
from rtea._pytea import (
    pytea16_encrypt,
    pytea16_decrypt,
    pyqqtea_encrypt,
    pyqqtea_decrypt,
)


def test_tea16_encrypt():
    seed(114514)
    for i in range(32):
        key, text = randbytes(16), randbytes(8)
        assert pytea16_encrypt(text, key) == tea16_encrypt(text, key), f"in round {i}"


def test_tea16_decrypt():
    seed(114514)
    for i in range(32):
        key, text = randbytes(16), randbytes(8)
        cipher_text = pytea16_encrypt(text, key)
        assert pytea16_decrypt(cipher_text, key) == tea16_decrypt(cipher_text, key), f"in round {i}"


@pytest.mark.parametrize("n", [4, 8, 256, 4096])
@pytest.mark.skipif(not is_debug(), reason="Impossible stable output in RELEASE mode")
def test_qqtea_encrypt_n(n):
    seed(114514)
    for i in range(32):
        key, text = randbytes(16), randbytes(n)
        assert pyqqtea_encrypt(text, key) == qqtea_encrypt(text, key), f"in round {i}"


@pytest.mark.parametrize("n", [4, 8, 256, 4096])
@pytest.mark.skipif(not is_debug(), reason="Impossible stable output in RELEASE mode")
def test_qqtea_decrypt_n(n):
    seed(114514)
    for i in range(32):
        key, text = randbytes(16), randbytes(n)
        text = pyqqtea_encrypt(text, key)
        assert pyqqtea_decrypt(text, key) == qqtea_decrypt(text, key), f"in round {i}"


@pytest.mark.parametrize("n", [4, 8, 256, 4096])
def test_qqtea_mixed_n(n):
    seed(114514)
    for i in range(32):
        key, text = randbytes(16), randbytes(n)

        cipher_text = qqtea_encrypt(text, key)
        assert pyqqtea_decrypt(cipher_text, key) == qqtea_decrypt(cipher_text, key), f"in round {i}"

        cipher_text = pyqqtea_encrypt(text, key)
        assert pyqqtea_decrypt(cipher_text, key) == qqtea_decrypt(cipher_text, key), f"in round {i}"
