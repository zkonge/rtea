from struct import pack, unpack


def xor(a: bytes, b: bytes) -> bytes:
    return bytes(x ^ y for x, y in zip(a, b))


def pytea16_encrypt(plain: bytes, key: bytes) -> bytes:
    op = 0xFFFFFFFF
    v0, v1 = unpack(">LL", plain[:8])
    k0, k1, k2, k3 = unpack(">LLLL", key)

    delta = 0x9E3779B9
    s = 0

    for _ in range(16):
        s = (s + delta) & op
        v0 += ((v1 << 4) + k0) ^ (v1 + s) ^ ((v1 >> 5) + k1)
        v0 &= op
        v1 += ((v0 << 4) + k2) ^ (v0 + s) ^ ((v0 >> 5) + k3)
        v1 &= op

    r = pack(">LL", v0, v1)
    return r


def pytea16_decrypt(cipher, key):
    op = 0xFFFFFFFF
    v0, v1 = unpack(">LL", cipher[:8])
    k0, k1, k2, k3 = unpack(">LLLL", key)

    delta = 0x9E3779B9
    s = (delta << 4) & op

    for i in range(16):
        v1 -= ((v0 << 4) + k2) ^ (v0 + s) ^ ((v0 >> 5) + k3)
        v1 &= op
        v0 -= ((v1 << 4) + k0) ^ (v1 + s) ^ ((v1 >> 5) + k1)
        v0 &= op
        s = (s - delta) & op
    return pack(">LL", v0, v1)


def pyqqtea_encrypt(plain: bytes, key: bytes) -> bytes:
    fill_count = 9 - (len(plain) + 1) % 8
    fills = b"\xdc" * fill_count
    plain = bytes([(fill_count - 2) | 0xF8]) + fills + plain + b"\0" * 7

    t_read = b"\0" * 8
    t_outp = b"\0" * 8
    result = b""
    outp = b"\0" * 8
    for i in range(0, len(plain), 8):
        outp = xor(plain[i : i + 8], t_read)
        t_read = xor(pytea16_encrypt(outp, key), t_outp)
        t_outp = outp
        result += t_read
    return result


def pyqqtea_decrypt(v: bytes, secret_key: bytes):
    l = len(v)
    prePlain = pytea16_decrypt(v, secret_key)
    pos = (prePlain[0] & 0x07) + 2
    r = prePlain
    preCrypt = v[0:8]
    for i in range(8, l, 8):
        x = xor(pytea16_decrypt(xor(v[i : i + 8], prePlain), secret_key), preCrypt)
        prePlain = xor(x, preCrypt)
        preCrypt = v[i : i + 8]
        r += x
    if r[-7:] != b"\0" * 7:
        return None
    return r[pos + 1 : -7]
