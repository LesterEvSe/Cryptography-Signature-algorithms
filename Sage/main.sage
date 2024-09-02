import hashlib
load('constants.sage')

def generate_key_pair():
    d = gen_random_num()
    return (d * G, int(d))

def sign_message(msg, d):
    sha256 = hashlib.sha256()
    sha256.update(msg.encode('utf-8'))
    hashed = int(sha256.hexdigest(), 16)

    k = gen_random_num()
    R = k * G
    r = int(R[0])

    return (r % ORDER, (inverse_mod(k, ORDER) * (e + r * d)) % ORDER)
    
def verify_signature(msg, r, s, Q):
    sha256 = hashlib.sha256()
    sha256.update(msg.encode('utf-8'))
    hashed = int(sha256.hexdigest(), 16)
    
    # print(s)
    w = inverse_mod(s, ORDER)
    u1 = int(e * w) % ORDER
    u2 = int(r * w) % ORDER
    
    R = u1 * G + u2 * Q
    print(r % ORDER)
    print(int(R[0]) % ORDER)
    return int(R[0]) % ORDER == r


msg = "abc"
(Q, d) = generate_key_pair()
(r, s) = sign_message(msg, d)
print(verify_signature(msg, r, s, Q))
