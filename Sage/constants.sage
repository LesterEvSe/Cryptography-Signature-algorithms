from sage.all import GF
import secrets

# Also known as secp256r1 or prime256v1 https://neuromancer.sk/std/nist/P-256
p256_FIELD = 0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff
ORDER = 0xffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551
e = 65_537

E = EllipticCurve(GF(p256_FIELD), [
    0xffffffff00000001000000000000000000000000fffffffffffffffffffffffc,
    0x5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b
])

G = E(
    0x6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296,
    0x4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5
)

FIELD = GF(p256_FIELD)

def gen_random_num():
    random_bytes = os.urandom(32) 
    random_int = int.from_bytes(random_bytes, "big")
    return (random_int % (ORDER - 1)) + 1