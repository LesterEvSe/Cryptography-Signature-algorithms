from Crypto.PublicKey import RSA
from Crypto.Signature.pkcs1_15 import PKCS115_SigScheme
from Crypto.Hash import SHA256
import binascii

def to_bytes_convert(val):
    bytes = (val.bit_length() + 7) // 8
    return list(val.to_bytes(bytes, byteorder='little'))

# Generate 1024-bit RSA key pair (private + public key)

p = 0xdce473d0e0e8ba2a2ba9ca355e252206c4ba5e0c9cb3c6d9ecc5c4db11a71da7a9268e4cd19a6b75d4ced8846c48415674607eac5d7275b51ee2520893f6c26ab61ae47844e118e0b3fe8f080677c1c8df927e67ee22476308442fcad36c56ff1fb8ecf3852da0cdc4c14ea5bd990e26e26806f579e6b6735b13f079ce59ff9f
q = 0xf80a8fd7913b43b220d08ef45f914987e32f27915a22d92299829bce01743d5e5f96bc7d3d96e99c06d1e4572c8fc0ac11c50d22c1cb8893d106720de6f6758909e66f7a625b1fb1df330dbb688531f22771c25067a401f982caa26ec6672ea29f7e44332f0304f2332b2196ca6de4f860bab0eefda326b7b0d3aa7706280313
n = 0xc350edcb6348c3adcc4fca30e66a08765dfc3c09bd8fef18849b9f3a7894e5497013e5f1b8af2753e942e41dad43a6de67eade5dd39f0921dd3821eb6e2c7eaf42400769d6d006b3cbc7fb0699706832b53014865ed59b538cd9d15e8f6374cc6ab9bac2e44e38086a573f01623edf458882f060b3b500ab2c29fdfdc6c1e4212ebab6a1aca2b1b4818252c5aec146ebb7a0cd777dc8ef7e51b3140e329d4832a6660564bda3d790533d3795c2e145737ec5fb283e3484060890b9592b0437e4ca74f6c32a66cda4cf2cb767962f0b844d1eee6439663ad2a52b587981b39e6b5ddf2307a8c4ca971175ad519dc3a9552052e2a84eb8cebad9396906759aea09
d = 0x4e02c4f7b77018b81222cc5f3ba89e38a8d156298dcd2f725b84ccd44dd52abf6a44885343b0b1c49226657674fdd06884990ab7ef0e565770443b480d7938204815cae03b2c5b80f4a3ab5fb7696e34d2a9a18909d7c772578fa8505c751cf188af262931870c5a501f08cb44b257fb2dcb1eb7810f8a74d3eef1c3aadf467e46ce33d8fc919f6fd6f9fac479606182dd151c013792bb636143e40eb73286744127c037908d217891a2672efdd30c3efd7b60c690b3c092d3908c075128bf4bdd67a615f63baa521ceca52950f77d6c69d0d010b158c5c62d0d3acc02be036782e51da665ed37d6de01412c8d89b2978f4652977e9a127610ac790b57049901
e = 0x10001


#keyPair = RSA.generate(bits=2048)
keyPair = RSA.construct((n, e, d))
print(f"Primes p, q:\np = {to_bytes_convert(keyPair.p)}\nq = {to_bytes_convert(keyPair.q)}", end='\n\n')
print(f"Public key:\nn = {to_bytes_convert(keyPair.n)}\ne = {to_bytes_convert(keyPair.e)}", end='\n\n')
print(f"Private key:\nn = {to_bytes_convert(keyPair.n)}\nd = {to_bytes_convert(keyPair.d)}", end='\n\n')

pubKey = keyPair.publickey()


# Sign the message using the PKCS#1 v1.5 signature scheme (RSASP1)
msg = b'Message for RSA signing'
hash = SHA256.new(msg)
signer = PKCS115_SigScheme(keyPair)
signature = signer.sign(hash)
print("Signature:", binascii.hexlify(signature))


# Verify valid PKCS#1 v1.5 signature (RSAVP1)
msg = b'Message for RSA signing'
hash = SHA256.new(msg)
verifier = PKCS115_SigScheme(pubKey)
try:
    verifier.verify(hash, signature)
    print("Signature is valid.")
except:
    print("Signature is invalid.")

# Verify invalid PKCS#1 v1.5 signature (RSAVP1)
msg = b'A tampered message'
hash = SHA256.new(msg)
verifier = PKCS115_SigScheme(pubKey)
try:
    verifier.verify(hash, signature)
    print("Signature is valid.")
except:
    print("Signature is invalid.")