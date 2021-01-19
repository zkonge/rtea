import pytest

from rtea import tea16_encrypt, tea16_decrypt, qqtea_encrypt, qqtea_decrypt
from rtea._pytea import (
    pytea16_encrypt,
    pytea16_decrypt,
    pyqqtea_encrypt,
    pyqqtea_decrypt,
)


@pytest.mark.benchmark(group="tea16-encrypt")
def test_pytea16_encrypt(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 8

    def func():
        pytea16_encrypt(text, key)

    benchmark(func)


@pytest.mark.benchmark(group="tea16-decrypt")
def test_pytea16_decrypt(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 8

    cipher_text = pytea16_encrypt(text, key)

    def func():
        pytea16_decrypt(cipher_text, key)

    benchmark(func)


@pytest.mark.benchmark(group="tea16-encrypt")
def test_tea16_encrypt(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 8

    def func():
        tea16_encrypt(text, key)

    benchmark(func)


@pytest.mark.benchmark(group="tea16-decrypt")
def test_tea16_decrypt(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 8

    cipher_text = tea16_encrypt(text, key)

    def func():
        tea16_decrypt(cipher_text, key)

    benchmark(func)


# qqtea encrypt benchmark


@pytest.mark.benchmark(group="qqtea-encrypt16")
def test_pyqqtea_encrypt16(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 16

    def func(key, text):
        pyqqtea_encrypt(text, key)

    benchmark(func, key, text)


@pytest.mark.benchmark(group="qqtea-encrypt256")
def test_pyqqtea_encrypt256(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 256

    def func(key, text):
        pyqqtea_encrypt(text, key)

    benchmark(func, key, text)


@pytest.mark.benchmark(group="qqtea-encrypt4096")
def test_pyqqtea_encrypt4096(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 4096

    def func(key, text):
        pyqqtea_encrypt(text, key)

    benchmark(func, key, text)


@pytest.mark.benchmark(group="qqtea-encrypt16")
def test_qqtea_encrypt16(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 16

    def func(key, text):
        qqtea_encrypt(text, key)

    benchmark(func, key, text)


@pytest.mark.benchmark(group="qqtea-encrypt256")
def test_qqtea_encrypt256(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 256

    def func(key, text):
        qqtea_encrypt(text, key)

    benchmark(func, key, text)


@pytest.mark.benchmark(group="qqtea-encrypt4096")
def test_qqtea_encrypt4096(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 4096

    def func(key, text):
        qqtea_encrypt(text, key)

    benchmark(func, key, text)


# qqtea decrypt benchmark


@pytest.mark.benchmark(group="qqtea-decrypt16")
def test_pyqqtea_decrypt16(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 16

    cipher_text = pyqqtea_encrypt(text, key)

    def func(key, cipher_text):
        pyqqtea_decrypt(cipher_text, key)

    benchmark(func, key, cipher_text)


@pytest.mark.benchmark(group="qqtea-decrypt256")
def test_pyqqtea_decrypt256(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 256

    cipher_text = pyqqtea_encrypt(text, key)

    def func(key, cipher_text):
        pyqqtea_decrypt(cipher_text, key)

    benchmark(func, key, cipher_text)


@pytest.mark.benchmark(group="qqtea-decrypt4096")
def test_pyqqtea_decrypt4096(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 4096

    cipher_text = pyqqtea_encrypt(text, key)

    def func(key, cipher_text):
        pyqqtea_decrypt(cipher_text, key)

    benchmark(func, key, cipher_text)


@pytest.mark.benchmark(group="qqtea-decrypt16")
def test_qqtea_decrypt16(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 16

    cipher_text = qqtea_encrypt(text, key)

    def func(key, cipher_text):
        qqtea_decrypt(cipher_text, key)

    benchmark(func, key, cipher_text)


@pytest.mark.benchmark(group="qqtea-decrypt256")
def test_qqtea_decrypt256(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 256

    cipher_text = qqtea_encrypt(text, key)

    def func(key, cipher_text):
        qqtea_decrypt(cipher_text, key)

    benchmark(func, key, cipher_text)


@pytest.mark.benchmark(group="qqtea-decrypt4096")
def test_qqtea_decrypt4096(benchmark):
    key, text = b"\xff" * 16, b"\xff" * 4096

    cipher_text = qqtea_encrypt(text, key)

    def func(key, cipher_text):
        qqtea_decrypt(cipher_text, key)

    benchmark(func, key, cipher_text)
